FROM rust:1.65.0 AS constructeur
WORKDIR /usr/src

RUN rustup target add x86_64-unknown-linux-musl

RUN USER=root cargo new analyse_ms
WORKDIR /usr/src/analyse_ms
COPY Cargo.toml Cargo.lock ./
RUN cargo build --release

COPY src ./src
RUN cargo install --target x86_64-unknown-linux-musl --path .

FROM alpine
COPY --from=constructeur /usr/local/cargo/bin/analyse_ms .
USER 1000
CMD ["./analyse_ms"]