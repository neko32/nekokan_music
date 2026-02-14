@echo off
chcp 65001 >nul
setlocal
REM このスクリプトがあるディレクトリの親 = プロジェクトルート
cd /d "%~dp0.."

echo [nekokan_music] バックエンドを別ウィンドウで起動します（ポート 12989）...
set "ROOT=%~dp0.."
start "nekokan_music_server" /D "%ROOT%" cmd /k "cargo run -p nekokan_music_server"

REM バックエンドの起動待ち
timeout /t 3 /nobreak >nul

echo [nekokan_music] フロントエンドを起動します（http://127.0.0.1:8081）...
cd nekokan_music_wa
trunk serve
