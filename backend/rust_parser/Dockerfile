FROM rust:slim AS builder

ENV TARGET x86_64-unknown-linux-musl

RUN rustup target add $TARGET

COPY . /app

WORKDIR /app

RUN cargo build --release --locked --target "$TARGET"
