# Etapa 1: Build da aplicação
FROM rust:latest as builder

WORKDIR /usr/src/app
COPY . .

# Instala dependências antecipadamente
RUN apt-get update && apt-get install -y libssl-dev pkg-config

RUN cargo build --release

# Etapa 2: Imagem final enxuta
FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /usr/src/app/target/release/user-api .

EXPOSE 8000
CMD ["./user-api"]
