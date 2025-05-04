# 🧑‍💻 API de Manutenção de Usuários

API RESTful escrita em Rust para gerenciamento de usuários, utilizando Rocket como framework web, MySQL como banco de dados e OpenTelemetry para tracing distribuído via Jaeger.

---

## 📌 O que faz?

A `user-api` permite:

- Criar usuários com nome e e-mail
- Listar todos os usuários cadastrados
- Visualizar detalhes de um usuário por ID
- Observar as requisições via Jaeger com spans instrumentados nas camadas `controller`, `service` e `repository`

Além disso, a aplicação já vem com:

- CORS habilitado
- Middleware de logging via `tracing`
- Integração completa com OpenTelemetry
- Monitoramento via Jaeger UI

---

## 🧱 Componentes da arquitetura

```text
┌───────────────┐      ┌───────────────┐
│  Rocket Web   │─────▶│ UserController│
└───────────────┘      └──────┬────────┘
                              │
                         ┌────▼───────┐
                         │ UserService│
                         └────┬───────┘
                              │
                        ┌─────▼────────┐
                        │UserRepository│
                        └─────┬────────┘
                              │
                        ┌─────▼──────┐
                        │   MySQL    │
                        └────────────┘

```

### Outros componentes:

- **user-api**: aplicação principal escrita em Rust com Rocket
- **MySQL**: banco de dados para persistência dos usuários
- **otel-collector**: recebe spans OTLP e exporta para o Jaeger
- **Jaeger**: UI para visualização dos spans
- **Docker Compose**: orquestra todos os serviços locais

---

## ▶️ Como executar

### 1. Clone o repositório

```bash
git clone https://github.com/sua-org/user-api.git
cd user-api
```

### 2. Clone o repositório

Suba os serviços com Docker Compose

```bash
docker-compose up --build
```

Isso irá subir:
| Serviço | Porta | Descrição |
|-----------------|-------|------------------------------------------|
| user-api | 8080 | API de usuários escrita em Rust |
| MySQL | 3306 | Banco relacional |
| otel-collector | 4318 | Recebe spans OTLP da aplicação |
| Jaeger UI | 16686 | Visualizador de tracing via navegador |

## ✅ Como testar

### 1. Criar um usuário

```bash
curl --request POST \
  --url http://localhost:8080/users \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Alice",
    "email": "alice@example.com"
}'
```

### 2. Listar todos os usuários

```bash
curl http://localhost:8080/users
```

### 3. Buscar usuário por ID

```bash
curl http://localhost:8080/users/1
```

### 4. Visualizar tracing

1. Acesse: http://localhost:16686
2. Selecione o serviço user-api
3. Veja os spans das requisições HTTP instrumentadas

## 📎 Notas adicionais

O banco de dados é inicializado com sql/init.sql

A aplicação utiliza:

- tracing
- OpenTelemetry
- sqlx
- rocket_db_pools

## 🔍 Executar localmente (sem Docker)

Configure as variáveis de ambiente:

```bash
export DATABASE_URL="mysql://root:root@localhost:3306/rust_db"
export APP_PORT=8080
```

E execute o serviço:

```bash
cargo run
```

## 🛠️ Tecnologias

- 🦀 Rust
- 🚀 Rocket
- 🐬 MySQL
- 🔭 OpenTelemetry
- 📈 Jaeger
- 🐳 Docker Compose
