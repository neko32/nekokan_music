#!/bin/bash

source ~/.nekokan_music.bash
cd ${NEKOKAN_MUSIC_DB}
git add .
DT=`date +%Y-%m-%d`
git commit -m "${DT} checkin"
git push
