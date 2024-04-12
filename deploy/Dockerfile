FROM rust:1.77.2-alpine3.19 as builder
RUN apk add --update alpine-sdk
WORKDIR /build
COPY ./contracts-rs /build/contracts-rs
COPY ./agent /build/agent
COPY ./Cargo.toml /build/Cargo.toml
COPY ./Cargo.lock /build/Cargo.lock
RUN cargo build --package agent


FROM alpine:3.19
COPY --from=builder /build/target/debug/agent /rise-next/agent
ENTRYPOINT ["/rise-next/agent"]
