#!/bin/sh
set -ex
rsync -aP binaries/nexa-server root@rtmp.staex.io:/usr/local/bin/nexa-server
