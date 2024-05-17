#!/bin/sh
exec curl https://web3.staex.io/v1/photo \
    --verbose \
    -F photo=@/tmp/drone.png
