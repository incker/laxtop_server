FROM rust:buster

RUN apt update \
    && curl -sL https://deb.nodesource.com/setup_14.x -o nodesource_setup.sh \
    && bash nodesource_setup.sh \
    && apt install -y nodejs \
    && node -v \
    && npm -v \
    && apt install -y nginx \
    && apt install -f \
    && apt update \
    && rustup default nightly \
    && rustup override set nightly \
    && rustup update \
    && cargo install cargo-watch \
    && cargo install diesel_cli --no-default-features --features mysql

WORKDIR /usr/src/app

EXPOSE 3000

VOLUME ["/usr/local/cargo"]
