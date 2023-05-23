FROM rust:1.65.0 AS constructeur
WORKDIR /usr/src

RUN USER=root cargo new domain_ms
WORKDIR /usr/src/domain_ms
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

EXPOSE 9000
USER 1000 

CMD ["./target/release/domain_ms"] 
