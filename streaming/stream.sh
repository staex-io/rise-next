#!/bin/sh
video_file=${location}
hostname=127.0.0.1:1935
exec ffmpeg \
    -loglevel error \
    -re \
    -i "$video_file" \
    -vcodec libx264 \
    -preset ultrafast \
    -pix_fmt yuv420p \
    -tune zerolatency \
    -f flv \
    rtmp://$hostname/live/drone
