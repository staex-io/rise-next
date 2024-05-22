#!/bin/sh
set -ex
rsync -aP binaries/streaming root@rtmp.staex.io:/usr/local/bin/nexa-stream
