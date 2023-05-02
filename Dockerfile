# Dockerfile-base

ARG RUST_VERSION=1.62.1
FROM rust:1.62.1 AS Builder

RUN rustup target add x86_64-unknown-linux-musl

RUN apt-get update -qq && \
    apt-get install -y \
    musl-tools \
    musl-dev \
    sqlite3 \
    libsqlite3-dev

RUN update-ca-certificates

WORKDIR /app

COPY ./ .

RUN cargo build --release

RUN cargo install diesel_cli

RUN diesel setup --database-url="chat.db"

ENV PORT=8000

EXPOSE 8000

CMD ["cargo","run","--release"]
