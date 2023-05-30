FROM rust:1.65.0 AS constructeur
WORKDIR /usr/src

RUN apt update && \
apt-get install libpq-dev -y && \
apt-get install clang llvm-dev libclang-dev -y && \
USER=root cargo new authentification_ms && \
cargo install diesel_cli --no-default-features --features postgres

WORKDIR /usr/src/authentification_ms
COPY Cargo.toml Cargo.lock .env diesel.toml ./
COPY src ./src
RUN cargo build --release

EXPOSE 9008
USER 1000 

CMD ["./target/release/authentification_ms"]
