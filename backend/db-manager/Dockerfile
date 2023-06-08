FROM clojure:temurin-20-lein-alpine AS BUILD

COPY . /code

WORKDIR /code

RUN lein uberjar

FROM eclipse-temurin:20-alpine

WORKDIR /app

COPY --from=BUILD /code/target/uberjar/*-standalone.jar ./app.jar

CMD ["java", "-jar", "app.jar"]
