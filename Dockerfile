# ---------------------------
# Etapa 1: Compilação (builder)
# ---------------------------

# Usa a imagem oficial do Rust como base para o build
FROM rust:latest as builder

# Define o diretório de trabalho dentro do container
WORKDIR /usr/src/app

# Copia todo o código-fonte da aplicação para dentro do container
COPY . .

# Atualiza os pacotes do sistema e instala dependências necessárias para compilar crates como sqlx
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Compila o projeto em modo release para gerar binário otimizado
RUN cargo build --release

# ---------------------------
# Etapa 2: Imagem final enxuta
# ---------------------------

# Usa uma imagem Debian mínima (sem ferramentas de desenvolvimento) para reduzir o tamanho final
FROM debian:bookworm-slim

# Instala apenas as dependências de runtime necessárias para executar binários Rust que usam TLS
RUN apt-get update && apt-get install -y \
    libssl3 ca-certificates && \
    rm -rf /var/lib/apt/lists/*  # Limpa cache do apt para reduzir o tamanho da imagem

# Define o diretório onde o binário será executado
WORKDIR /app

# Copia o binário já compilado da etapa anterior
COPY --from=builder /usr/src/app/target/release/user-api .

# Expõe a porta 8000 (padrão do Rocket, se não configurado via `APP_PORT`)
EXPOSE 8000

# Comando de entrada: inicia o binário da aplicação
CMD ["./user-api"]


# Para construir a imagem, execute:
# docker build -t user-api .
# Para executar o container, use:
# docker run -p 8000:8000 user-api
# Para executar o container em modo interativo, use:
# docker run -it -p 8000:8000 user-api
