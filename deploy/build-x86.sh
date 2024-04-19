#!/bin/sh

cleanup() {
    rm -rf "$workdir"
}

set -e
tag=ghcr.io/staex-io/rise-next/agent:latest
docker run -it --rm \
    --user "$(id -u):$(id -g)" \
    --volume "$PWD":/src \
    --workdir /src \
    registry.gitlab.com/staex/mcc/ci:latest \
    sh -c '
set -e
export CARGO_HOME=$PWD/.cargo
cargo build --package agent --release --target x86_64-unknown-linux-musl
'

workdir="$(mktemp -d)"
cp target/x86_64-unknown-linux-musl/release/agent "$workdir"
docker build --tag "$tag" -f- "$workdir" <<'EOF'
FROM debian:bullseye
COPY agent /bin/nexa-agent
RUN apt-get update -qq && apt-get -qq install --no-install-recommends ffmpeg && rm -rf /var/lib/apt/lists/*
CMD ["/bin/nexa-agent"]
EOF
docker push "$tag"
