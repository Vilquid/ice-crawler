FROM rust:1.65.0 AS constructeur
WORKDIR /usr/src

RUN apt update && \
apt install dnsutils -y && \
USER=root cargo new smtp_ms && \
apt install tcpdump -y && \
passwd -d root
WORKDIR /usr/src/smtp_ms
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

EXPOSE 9001
USER 1000 

CMD ["./target/release/smtp_ms"] 
