import chromadb
import sys
import os

def query(query_text: str) -> None:
    chromadb_host = os.getenv("VDB_SRV_HOST") or "localhost"
    chromadb_port: int = int(os.getenv("VDB_SRV_PORT") or "8000")

    print(f"connecting to chromadb at {chromadb_host}:{chromadb_port}...")

    # 1. サーバーに接続
    client = chromadb.HttpClient(host=chromadb_host, port=chromadb_port, tenant = "neko32", database = "jazzlib")

    # 2. コレクションを取得
    collection = client.get_collection(name="nekokan_music")

    # 3. テキストでクエリを送る
    # query_texts に探したい内容を文章で入れるだけ！
    results = collection.query(
        query_texts=[query_text], 
        n_results=5,  # 上位2件を取得
        include=["documents", "metadatas", "distances"] # 何を返してほしいか指定
    )

    # 4. 結果を表示
    print("--- 検索結果 ---")
    for i in range(len(results['ids'][0])):
        print(f"順位: {i+1}")
        print(f"ID: {results['ids'][0][i]}")
        print(f"内容: {results['documents'][0][i]}")
        print(f"メタデータ: {results['metadatas'][0][i]}")
        print(f"スコア(距離): {results['distances'][0][i]}") # 数値が小さいほど似ている
        print("-" * 20)


if __name__ == "__main__":

    if len(sys.argv) > 1:
        query_text = " ".join(sys.argv[1:])
    else:
        query_text = input("検索クエリを入力してください: ")

    query(query_text)