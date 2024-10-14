FROM rust:1.81.0 as build

WORKDIR /app

COPY . .

RUN cargo build --release

FROM gcr.io/distroless/cc-debian12

COPY --from=build /app/target/release/gogoanime-api /bin/gogoanime-api

CMD ["gogoanime-api"]
