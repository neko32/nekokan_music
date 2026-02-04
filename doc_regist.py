import re
import os
import chromadb
import json

# JSONデータの読み込み（実際にはファイル読み込みなどを想定）

def _personnel_to_text(personnel: dict) -> str:
    """personnel を doc_content 用のテキストに変換。leader/sidemen または conductor/orchestra/soloists に対応。"""
    if not personnel:
        return ""
    parts = []
    if "leader" in personnel and personnel["leader"]:
        names = [p["name"] for p in personnel["leader"]]
        parts.append(f"Leader: {', '.join(names)}")
    if "sidemen" in personnel and personnel["sidemen"]:
        names = [p["name"] for p in personnel["sidemen"]]
        parts.append(f"Sidemen: {', '.join(names)}")
    if "conductor" in personnel and personnel["conductor"]:
        names = [p["name"] for p in personnel["conductor"]]
        parts.append(f"Conductor: {', '.join(names)}")
    if "orchestra" in personnel and personnel["orchestra"]:
        names = [p["name"] for p in personnel["orchestra"]]
        parts.append(f"Orchestra: {', '.join(names)}")
    if "soloists" in personnel and personnel["soloists"]:
        names = [p["name"] for p in personnel["soloists"]]
        parts.append(f"Soloists: {', '.join(names)}")
    if not parts:
        return ""
    return "Personnel: " + "; ".join(parts) + ". "


def _primary_artist(personnel: dict) -> str:
    """メタデータ用の代表アーティスト名。soloists または leader の先頭、なければ空文字。"""
    if not personnel:
        return ""
    if personnel.get("soloists") and len(personnel["soloists"]) > 0:
        return personnel["soloists"][0]["name"]
    if personnel.get("leader") and len(personnel["leader"]) > 0:
        return personnel["leader"][0]["name"]
    return ""


def register_doc(filename: str):
    with open(filename, "r", encoding="utf-8") as f:
        raw_data = json.load(f)
    chromadb_host = os.getenv("VDB_SRV_HOST") or "localhost"
    chromadb_port: int = int(os.getenv("VDB_SRV_PORT") or "8000")

    print(f"connecting to chromadb at {chromadb_host}:{chromadb_port}...")

    client = chromadb.HttpClient(host=chromadb_host, port=chromadb_port, tenant = "neko32", database = "jazzlib")
    collection = client.get_or_create_collection(name="nekokan_music")

    personnel_text = _personnel_to_text(raw_data.get("personnel") or {})
    primary_artist = _primary_artist(raw_data.get("personnel") or {})

    documents = []
    metadatas = []
    ids = []

    # トラックごとに処理
    for track in raw_data["tracks"]:
        # 検索対象となるテキスト（タイトル + personnel + 作曲家 + 感想）
        doc_content = (
            f"Title: {raw_data['title']} - {track['title']}. "
            f"{personnel_text}"
            f"Composer: {', '.join(track['composer'])}. "
            f"score: {raw_data['score']}. "
            f"Review: {raw_data['comment']}"
        )
        
        documents.append(doc_content)
        
        # フィルタリング用のメタデータ
        metadatas.append({
            "album_id": raw_data["id"],
            "label": raw_data["label"],
            "release_year": raw_data["release_year"],
            "record_year": ", ".join(str(y) for y in raw_data["record_year"]),
            "track_no": track["no"],
            "soloist": primary_artist,
            "primary_composer": track["composer"][0]
        })
        
        # IDの生成（アルバムIDとトラック番号を組み合わせ）
        ids.append(f"{raw_data['id']}_T{track['no']}")

    # ChromaDBへ一括登録
    collection.add(
        documents=documents,
        metadatas=metadatas,
        ids=ids
    )

    print(f"Added {len(documents)} tracks to collection.")

def main() -> None:
    register_doc("db/Andre_Navarra__Prokofiev_Cello_Concerto.json")
    register_doc("db/Akio_Saejima__HumptyDumpty.json")

if __name__ == "__main__":
    main()