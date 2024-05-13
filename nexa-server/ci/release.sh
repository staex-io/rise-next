#!/bin/sh
set -ex
export OPENSSL_STATIC=1
export OPENSSL_DIR=/opt/openssl
export OPENSSL_LIB_DIR=/opt/openssl/lib64
export OPENSSL_NO_VENDOR=1
rust_flags="-Ccodegen-units=1 -Cstrip=symbols -Copt-level=3 -Cincremental=false -Clto=yes -Cembed-bitcode=yes"
target=x86_64-unknown-linux-musl
env RUSTFLAGS="$rust_flags" \
    cargo build \
    --quiet \
    --release \
    --target "$target" \
    --no-default-features
mkdir -p binaries
mv ../target/"$target"/release/nexa-server binaries/
