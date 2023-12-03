#!/bin/bash

exec docker run --rm \
  --user "$(id -u):$(id -g)" \
  -it \
  -v "${PWD}":/rise \
  --entrypoint="" \
  --workdir /rise \
  oven/bun "$@"
