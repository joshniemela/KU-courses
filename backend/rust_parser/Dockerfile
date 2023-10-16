FROM rust:slim

ENV TARGET x86_64-unknown-linux-musl

RUN rustup target add $TARGET

COPY . /app

WORKDIR /app

RUN cargo build --release --locked --target "$TARGET"



# run the target
RUN chmod +x /app/target/x86_64-unknown-linux-musl/release/rust_parser
