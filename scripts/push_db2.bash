#!/bin/bash

# --- 設定 ---
source ~/.nekokan_music.bash
ROUTING_KEY=${PG_ROUTING_KEY}
SUMMARY="[Critical] Nekokan Music DB Git Push Failed"

# --- 1. エラー時の処理 (Catchに相当) ---
error_handler() {
  local exit_code=$1
  local last_command=$2
  
  # 正常終了 (0) 以外の場合にPagerDutyへ通知
  if [ "$exit_code" -ne 0 ]; then
    echo "エラー検知: コマンド '$last_command' が 終了ステータス $exit_code で失敗したまる！"
    echo $ROUTING_KEY
    
    curl -v -X POST https://events.pagerduty.com/v2/enqueue \
    -H 'Content-Type: application/json' \
    -d '{
      "payload": {
        "summary": "'"$SUMMARY"'",
        "severity": "critical",
        "source": "'"$(hostname)"'",
        "custom_details": {
          "exit_code": "'"$exit_code"'",
          "last_command": "'"$last_command"'"
        }
      },
      "routing_key": "'"$ROUTING_KEY"'",
      "event_action": "trigger"
    }'
  fi
}

# --- 2. 準備 ---
# エラーが発生したら即座にスクリプトを中断する設定
set -e
# スクリプト終了時に必ず error_handler を実行する (引数に終了ステータスと最後に実行したコマンドを渡す)
trap 'error_handler $? "$BASH_COMMAND"' EXIT

# --- 3. メイン処理 (Tryに相当) ---
cd "${NEKOKAN_MUSIC_DB}"

# ファイルに変更がない場合、git add/commitでエラーになるのを防ぐためのガードを入れるとより安全まる
git add .

# 変更がある場合のみコミット
if ! git diff-index --quiet HEAD; then
    DT=$(date +%Y-%m-%d)
    git commit -m "${DT} checkin"
    git push
else
    echo "変更がないのでスキップするまる。"
fi

echo "すべての処理が正常に完了したまる！"

