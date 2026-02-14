@echo off
chcp 65001 >nul
cd /d "%~dp0..\nekokan_music_wa"
echo [nekokan_music] フロントエンドを起動します（http://127.0.0.1:8081）...
trunk serve
