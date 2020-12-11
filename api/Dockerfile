FROM rust:buster

RUN apt update \
    && apt install -y nginx nodejs npm \
    && rustup default nightly \
    && rustup override set nightly \
    && rustup update \
    && cargo install cargo-watch \
    && cargo install diesel_cli --no-default-features --features mysql

WORKDIR /usr/src/app

EXPOSE 3000

VOLUME ["/usr/local/cargo"]
