import re
import os
import time
import json
import chromadb
import httpx
from chromadb.utils import embedding_functions

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

def calculate_passage(score: int) -> str:
    if score == 6:
        return "評価: 殿堂入り、最高、至高。"
    elif score == 5:
        return "評価: 大変良い。"
    elif score == 4:
        return "評価: 良い。"
    elif score == 3:
        return "評価: 普通。"
    elif score == 2:
        return "評価: 悪い。"
    elif score == 1:
        return "評価: 大変悪い。"
    else:
        return "評価: 不明。"
    


def register_doc(filename: str):
    with open(filename, "r", encoding="utf-8") as f:
        raw_data = json.load(f)
    chromadb_host = os.getenv("VDB_SRV_HOST") or "localhost"
    chromadb_port: int = int(os.getenv("VDB_SRV_PORT") or "8000")

    model_name = "intfloat/multilingual-e5-small"
    emb_fn = embedding_functions.SentenceTransformerEmbeddingFunction(model_name=model_name)

    personnel_text = _personnel_to_text(raw_data.get("personnel") or {})
    primary_artist = _primary_artist(raw_data.get("personnel") or {})

    documents = []
    metadatas = []
    ids = []

    # トラックごとに処理
    for track in raw_data["tracks"]:
        # 検索対象となるテキスト（タイトル + personnel + 作曲家 + 感想）
        doc_content = (
            f"passage: {calculate_passage(raw_data['score'])}"
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
            "disc_no": track["disc_no"],
            "track_no": track["no"],
            "soloist": primary_artist,
            "primary_composer": track["composer"][0]
        })
        
        # IDの生成（ディスク番号とアルバムIDとトラック番号を組み合わせ）
        ids.append(f"{track['disc_no']}_{raw_data['id']}_T{track['no']}")

    # ChromaDBへ接続・登録（接続拒否時にリトライ）
    max_attempts = 10
    delay_sec = 2
    last_exc = None
    for attempt in range(1, max_attempts + 1):
        try:
            print(f"connecting to chromadb at {chromadb_host}:{chromadb_port}... (attempt {attempt}/{max_attempts})")
            client = chromadb.HttpClient(host=chromadb_host, port=chromadb_port, tenant="neko32", database="jazzlib")
            # sentence_transformer (E5) でコレクション作成。既存が default の場合はサーバー側でコレクション削除が必要。
            collection = client.get_or_create_collection(
                name="nekokan_music", embedding_function=emb_fn,
                metadata = {"hnsw:space": "cosine"}
            )
            collection.add(documents=documents, metadatas=metadatas, ids=ids)
            print(f"Added {len(documents)} tracks to collection.")
            return
        except (httpx.ConnectError, ValueError) as e:
            last_exc = e
            if attempt < max_attempts:
                print(f"Connection refused, retrying in {delay_sec}s...")
                time.sleep(delay_sec)
            else:
                raise last_exc from e

def main() -> None:
    done_path = "done.txt"
    processed: set[str] = set()
    if os.path.exists(done_path):
        with open(done_path, "r", encoding="utf-8") as f:
            processed = {line.strip() for line in f if line.strip()}

    files = os.listdir("db")
    siz = len(files)
    for idx, file in enumerate(files):
        print(f"[{idx+1}/{siz}] {file}")
        if file in processed:
            print(f"Skipped {file} (already done).")
            continue
        register_doc(f"db/{file}")
        with open(done_path, "a", encoding="utf-8") as f:
            f.write(file + "\n")
        processed.add(file)
        print(f"Added {file} to collection.")

if __name__ == "__main__":
    main()