# BUILDER
FROM rust:slim as builder

WORKDIR /app
COPY . .

RUN apt-get update 
RUN apt-get install -y libssl-dev pkg-config openssl g++

RUN cargo build --release

# FINAL IMAGE

FROM debian:bookworm-slim
WORKDIR /app
COPY --from=builder /app/target/release/vector_store ./

ENV SERVER_ADDRESS=0.0.0.0
ENV SERVER_PORT=4000
ENV DATA_DIR="../data/"
CMD ["./vector_store"]
