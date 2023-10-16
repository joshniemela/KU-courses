FROM rust_parser AS PARSER

FROM clojure:temurin-20-lein-alpine AS BUILD


COPY . /code

WORKDIR /code

RUN lein uberjar

FROM eclipse-temurin:20-alpine

WORKDIR /app

COPY --from=PARSER /app/target/x86_64-unknown-linux-musl/release/rust_parser /usr/local/bin/rust_parser

COPY --from=BUILD /code/target/uberjar/*-standalone.jar ./app.jar

CMD ["java", "-jar", "app.jar", "-f"]
