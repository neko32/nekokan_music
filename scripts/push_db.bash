#!/bin/bash

cd /opt/srv/nekokan_music_server/nekokan_music/db
git add .
DT=`date +%Y-%m-%d`
git commit -m "${DT} checkin"
git push
