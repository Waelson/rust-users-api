// Declara explicitamente o uso do Rocket (boa prática em binários principais)
extern crate rocket;

// Módulos internos da aplicação
mod context;
mod controllers;
mod db;
mod logger;
mod models;
mod repository;
mod routes;
mod services;

// Importa o contexto da aplicação, que contém todas as dependências compartilhadas (injeção via `.manage`)
use context::AppContext;

// Importa o controlador de usuários
use controllers::user_controller::UserController;

// Importa o tipo `Db` que representa o pool de conexões do Rocket com SQLx
use db::Db;

// Importa o repositório responsável por acesso direto ao banco
use repository::user_repository::UserRepository;

// Importa o serviço de usuários, que encapsula regras de negócio
use services::user_service::UserService;

// Rocket config e utilitários para manipulação de figment (configuração flexível do Rocket)
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket::Config;

// Rocket Database para suporte a `.attach(Db::init())` e `.fetch(...)`
use rocket_db_pools::Database;

// Para ler variáveis de ambiente como `DATABASE_URL` e `APP_PORT`
use std::env;

/// Função principal da aplicação Rocket. É assíncrona pois lida com operações de IO (inicialização do Rocket).
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Inicializa o sistema de logs (tracing + RUST_LOG)
    logger::init();
    tracing::info!("🚀 Inicializando aplicação");

    // Lê a URL do banco da variável de ambiente DATABASE_URL, ou usa padrão local
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:root@localhost:3306/rust_db".to_string());

    // Lê a porta da variável APP_PORT, ou usa 8080 como padrão
    let port: u16 = env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    // Cria o bloco de configuração de banco para figment
    let mut dbs = Map::new();
    dbs.insert(
        "mysql".to_string(),
        Value::from(map! {
            "url" => database_url
        }),
    );

    // Cria o `figment`, que é o sistema de configuração do Rocket
    // Define: banco, porta, e endereço de bind (0.0.0.0 = aceita conexões externas)
    let figment = Config::figment()
        .merge(("databases", Value::from(dbs)))
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    // Inicializa o Rocket em estado `Build`, com a configuração e o fairing do banco
    let rocket = rocket::custom(figment).attach(Db::init());

    // Transforma Rocket<Build> em Rocket<Ignite> para permitir `Db::fetch(...)`
    let ignite = rocket.ignite().await?;

    // Busca o pool de conexões MySQL já inicializado pelo Rocket
    let db = Db::fetch(&ignite).expect("Failed to fetch DB");
    let pool = db.inner().clone();

    // Criação manual das dependências da aplicação
    let repo = UserRepository::new(pool);
    let service = UserService::new(repo);
    let controller = UserController::new(service);
    let ctx = AppContext {
        user_controller: controller,
    };

    // Reconstrói o Rocket final:
    // - reaproveita o `figment` já configurado
    // - anexa novamente o fairing do banco
    // - registra o `AppContext`
    // - monta as rotas
    rocket::custom(ignite.figment().clone())
        .attach(Db::init())
        .manage(ctx)
        .mount("/users", routes::user_routes())
        .launch()
        .await?;

    Ok(())
}
