#!/bin/sh
exec curl https://web3.staex.io/v1/video \
    --verbose \
    -F video=@/tmp/drone.mov
