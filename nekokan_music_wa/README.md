# nekokan_music_wa

Yew + Trunk + wasm-bindgen による音楽JSON編集のWebAssemblyフロントエンドです。

## 技術スタック

- Rust (wasm32-unknown-unknown)
- Yew 0.21
- Trunk
- wasm-bindgen

## 開発手順

### 1. 環境

- Rust (stable)
- wasm32 ターゲット: `rustup target add wasm32-unknown-unknown`
- Trunk: `cargo install trunk`

### 2. バックエンドAPIの起動

リポジトリルートで API サーバーを起動します（db フォルダと dist を参照するため）。

```powershell
cd c:\dev\nekokan_music
cargo run -p nekokan_music_server
```

サーバーは `http://127.0.0.1:12989` で待ち受け、`/api/list`, `/api/files/*`, `/api/save` を提供します。  
静的ファイルは `nekokan_music_wa/dist` から配信されます。

### 3. フロントエンドの開発

Trunk で開発サーバーを起動（API を 12989 にプロキシ）:

```powershell
cd nekokan_music_wa
trunk serve --open
```

ブラウザで `http://127.0.0.1:8081` が開きます。

### 4. 本番ビルド

```powershell
cd nekokan_music_wa
trunk build
```

出力は `nekokan_music_wa/dist` です。  
本番ではリポジトリルートで `cargo run -p nekokan_music_server` を実行し、この dist を配信します。

## レイアウト

- サイドバー: 300px（db 内 JSON ファイル一覧）
- コンテンツ: 最大 900px、左右余白 100px
- ベースカラー: #7297c5、セカンダリー: #666666
