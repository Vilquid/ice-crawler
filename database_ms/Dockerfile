FROM rust:1.65.0 AS constructeur
WORKDIR /usr/src

RUN USER=root cargo new database_ms && \
apt update && \
apt install tcpdump -y && \
passwd -d root
WORKDIR /usr/src/database_ms
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

EXPOSE 9009
USER 1000 

CMD ["./target/release/database_ms"] 
