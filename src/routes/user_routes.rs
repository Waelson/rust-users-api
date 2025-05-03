// Importa o contexto da aplicação, que centraliza as dependências da aplicação como o UserController.
use crate::context::AppContext;

// Importa o tipo de erro padronizado da API, que encapsula mensagens, status e causas.
use crate::errors::ApiError;

// Importa os tipos de dados utilizados na criação e leitura de usuários:
// - `NewUser`: representa os dados recebidos na criação de um usuário
// - `User`: representa um usuário já persistido com ID
use crate::models::user::{NewUser, User};

// Importa macros e estruturas do Rocket para definição de rotas e injeção de dependências.
use rocket::{
    get,               // Macro para rotas GET
    post,              // Macro para rotas POST
    routes,            // Gera a lista de rotas automaticamente
    serde::json::Json, // Permite (de)serialização automática entre JSON <-> struct
    State,             // Fornece acesso compartilhado ao estado global, como `AppContext`
};

// Importa a macro `instrument` da biblioteca `tracing`, usada para criar spans e logs estruturados
use tracing::instrument;

/// Handler para a rota HTTP POST `/users`
///
/// Esse endpoint é usado para criar um novo usuário. Ele espera um corpo JSON com os dados
/// do usuário e delega a criação ao `UserController`.
///
/// # Parâmetros
/// - `ctx`: estado compartilhado (`AppContext`) contendo o controller
/// - `user`: JSON recebido com os campos `name`, `email`, `birth_date`
///
/// # Retorno
/// - `201 Created` com o JSON do `User` criado, se sucesso
/// - `400` ou `500` encapsulados via `ApiError`, se falhar
#[post("/", format = "json", data = "<user>")]
#[instrument(skip(ctx))] // registra span de tracing, omitindo `ctx` da saída
pub async fn create_user(
    ctx: &State<AppContext>,
    user: Json<NewUser>,
) -> Result<Json<User>, ApiError> {
    // Converte Json<NewUser> para NewUser e delega a criação ao controller
    let created = ctx.user_controller.create_user(user.into_inner()).await?;

    // Retorna o usuário criado como JSON
    Ok(Json(created))
}

/// Handler para a rota HTTP GET `/users/<id>`
///
/// Este endpoint busca um usuário pelo seu identificador numérico (`id`), recebido na URL.
///
/// # Parâmetros
/// - `ctx`: contexto compartilhado da aplicação
/// - `id`: identificador inteiro extraído da URL
///
/// # Retorno
/// - `200 OK` com o JSON do usuário, se encontrado
/// - `404 Not Found` (via `ApiError`) se o usuário não existir
#[get("/<id>")]
#[instrument(skip(ctx))] // registra o span, omitindo `ctx`
pub async fn get_user(ctx: &State<AppContext>, id: i32) -> Result<Json<User>, ApiError> {
    // Busca o usuário pelo ID via controller
    let user = ctx.user_controller.get_user(id).await?;

    // Retorna JSON do usuário se encontrado
    Ok(Json(user))
}

/// Função auxiliar que registra todas as rotas disponíveis para o recurso `/users`.
///
/// Deve ser usada no `main.rs` com `.mount("/users", routes())`
pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, get_user]
}
