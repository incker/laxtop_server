FROM rust:stretch

# npm need to be installed after node
# install npm from source as it looks like conflicts with new node version
RUN apt update \
    && curl -sL https://deb.nodesource.com/setup_13.x | bash - \
    && apt install -y nginx nodejs \
    && curl -L https://npmjs.org/install.sh | sh \
    && rustup default nightly \
    && rustup override set nightly \
    && rustup update \
    && cargo install cargo-watch \
    && cargo install diesel_cli --no-default-features --features mysql

WORKDIR /usr/src/app

EXPOSE 3000

VOLUME ["/usr/local/cargo"]
