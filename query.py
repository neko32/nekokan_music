import chromadb
import sys
import os
import time
import httpx
from chromadb.utils import embedding_functions

E5_MODEL = "intfloat/multilingual-e5-small"

def query(query_text: str) -> None:
    chromadb_host = os.getenv("VDB_SRV_HOST") or "localhost"
    chromadb_port: int = int(os.getenv("VDB_SRV_PORT") or "8000")

    max_attempts = 5
    delay_sec = 2
    last_exc = None
    for attempt in range(1, max_attempts + 1):
        try:
            print(f"connecting to chromadb at {chromadb_host}:{chromadb_port}... (attempt {attempt}/{max_attempts})")
            client = chromadb.HttpClient(host=chromadb_host, port=chromadb_port, tenant="neko32", database="jazzlib")
            emb_fn = embedding_functions.SentenceTransformerEmbeddingFunction(model_name=E5_MODEL)
            collection = client.get_collection(name="nekokan_music", embedding_function=emb_fn)
            results = collection.query(
                query_texts=[f"query: {query_text}"],
                n_results=5,
                include=["documents", "metadatas", "distances"]
            )
            break
        except (httpx.ConnectError, ValueError) as e:
            last_exc = e
            if attempt < max_attempts:
                print(f"Connection failed, retrying in {delay_sec}s...")
                time.sleep(delay_sec)
            else:
                raise last_exc from e

    # 結果を表示
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