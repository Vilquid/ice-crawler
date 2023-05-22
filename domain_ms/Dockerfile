FROM rust:1.65.0 AS constructeur
WORKDIR /usr/src

RUN USER=root cargo new dns_ms
WORKDIR /usr/src/dns_ms
COPY ../dns_ms/Cargo.toml Cargo.lock ./
COPY ../dns_ms/src ./src
RUN cargo build --release

EXPOSE 9000
USER 1000 

CMD ["./target/release/dns_ms"] 
