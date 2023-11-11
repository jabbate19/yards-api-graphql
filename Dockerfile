FROM rust:1.71-slim-buster

WORKDIR /app

RUN apt-get update && apt-get -y install libssl-dev pkg-config

COPY . .

RUN cargo build --release

CMD ["./target/release/graphql-rs"]
