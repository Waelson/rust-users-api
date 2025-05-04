// Declara explicitamente o uso do Rocket.
// Essa linha é necessária apenas em binários que usam macros do Rocket fora do escopo do crate root.
extern crate rocket;

// Módulos internos da aplicação (camadas separadas por responsabilidade)
mod context; // Injeção de dependências via AppContext
mod controllers; // Lógica de controle da API (HTTP -> Service)
mod db; // Inicialização do pool de conexões com banco via Rocket
mod errors; // Tipos customizados de erro (AppError e ApiError)
mod logger; // Sistema de logs baseado em tracing
mod middlewares; // Middleware do Rocket (ex: CORS)
mod models; // Estruturas de dados do domínio (User, NewUser)
mod repository; // Acesso direto ao banco de dados
mod routes; // Definição de rotas HTTP
mod services; // Camada de regras de negócio
mod trace;

// Importa o AppContext, que injeta o controlador no Rocket via `.manage()`
use context::AppContext;

// Importa o controlador que media as chamadas entre rotas e regras de negócio
use controllers::user_controller::UserController;

// Importa o pool de conexões com o banco gerenciado pelo Rocket
use db::Db;

// Repositório responsável por interações SQL com a tabela `users`
use repository::user_repository::UserRepository;

// Middleware que adiciona headers CORS à resposta HTTP
use middlewares::cors::CORS;

// Serviço de usuários contendo regras de negócio
use services::user_service::UserService;

// Utilitários do Rocket para manipular configuração via Figment (sistema de config extensível)
use rocket::figment::{
    util::map,
    value::{Map, Value},
};
use rocket::Config;

// Trait necessária para `.attach(Db::init())` e `.fetch()` de pools no Rocket
use rocket_db_pools::Database;

// Para acessar variáveis de ambiente como `DATABASE_URL` e `APP_PORT`
use std::env;

use trace::init_tracer;

/// Função principal que inicia o servidor Rocket.
/// Marcada como `#[rocket::main]` para habilitar await no escopo principal.
#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // Inicializa logs e tracing com Jaeger via OTLP
    init_tracer().expect("Failed to initialize OpenTelemetry tracer");

    tracing::info!("🚀 Inicializando aplicação");

    // Lê a variável de ambiente `DATABASE_URL`, ou usa valor padrão local
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://root:root@localhost:3306/rust_db".to_string());

    // Lê a porta do servidor via variável `APP_PORT`, ou usa 8080 como fallback
    let port: u16 = env::var("APP_PORT")
        .ok()
        .and_then(|p| p.parse::<u16>().ok())
        .unwrap_or(8080);

    // Monta a configuração do banco de dados em formato aceito pelo Rocket (`figment`)
    let mut dbs = Map::new();
    dbs.insert(
        "mysql".to_string(),
        Value::from(map! {
            "url" => database_url
        }),
    );

    // Cria a configuração Rocket (`figment`) combinando as variáveis de banco, porta e endereço de bind
    // `0.0.0.0` permite aceitar conexões externas (ideal para rodar no Docker ou VMs)
    let figment = Config::figment()
        .merge(("databases", Value::from(dbs)))
        .merge(("port", port))
        .merge(("address", "0.0.0.0"));

    // Cria o Rocket em estado `Build`, aplicando a configuração inicial + attach do banco
    let rocket = rocket::custom(figment).attach(Db::init());

    // Transforma o Rocket para o estado `Ignite`, necessário para acessar recursos como pool de DB
    let ignite = rocket.ignite().await?;

    // Busca o pool MySQL já inicializado pelo Rocket
    let db = Db::fetch(&ignite).expect("Failed to fetch DB");
    let pool = db.inner().clone();

    // Injeta manualmente as dependências seguindo o padrão de injeção explícita:
    // Repository → Service → Controller → AppContext
    let repo = UserRepository::new(pool);
    let service = UserService::new(repo);
    let controller = UserController::new(service);
    let ctx = AppContext {
        user_controller: controller,
    };

    // Reconstrói e lança a aplicação Rocket com:
    // - mesmo `figment` reaproveitado
    // - banco de dados reaplicado
    // - contexto de aplicação (`AppContext`) injetado com `.manage(ctx)`
    // - middleware de CORS aplicado com `.attach(CORS)`
    // - rotas montadas no endpoint `/users`
    rocket::custom(ignite.figment().clone())
        .attach(Db::init())
        .attach(CORS)
        .manage(ctx)
        .mount("/users", routes::user_routes())
        .launch()
        .await?;

    // Encerramento com sucesso
    Ok(())
}
