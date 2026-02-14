@echo off
chcp 65001 >nul
cd /d "%~dp0.."
echo [nekokan_music] バックエンドを起動します（http://127.0.0.1:12989）...
cargo run -p nekokan_music_server
