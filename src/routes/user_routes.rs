// Importa o contexto da aplicação, que carrega as dependências injetadas, como o controlador de usuários.
use crate::context::AppContext;

// Importa os tipos de dados usados na criação e leitura de usuários.
use crate::models::user::{NewUser, User};

// Importa utilitários do Rocket para definir rotas.
use rocket::routes;
use rocket::{
    get,               // macro para definir uma rota GET
    http::Status,      // enum que representa códigos HTTP como 200, 404, 500 etc
    post,              // macro para definir uma rota POST
    serde::json::Json, // wrapper que serializa/deserializa JSON automaticamente
    State,             // injeta estado global compartilhado, como o `AppContext`
};

// Importa macro do `tracing` para adicionar observabilidade (logs estruturados + spans).
use tracing::instrument;

/// Rota POST `/users`
/// Cria um novo usuário a partir dos dados JSON fornecidos no corpo da requisição.
///
/// # Parâmetros
/// - `ctx`: injeção automática do `AppContext` via `&State<_>`, contendo o `UserController`.
/// - `user`: JSON com os dados de entrada para o novo usuário (`NewUser`)
///
/// # Retorno
/// - `201 Created` com o JSON do `User` se sucesso
/// - `500 InternalServerError` se falhar ao persistir
#[post("/", format = "json", data = "<user>")]
pub async fn create_user(
    ctx: &State<AppContext>,
    user: Json<NewUser>,
) -> Result<Json<User>, Status> {
    // Converte Json<NewUser> → NewUser (desempacota)
    // Chama a função assíncrona para persistir no banco via o controller
    let created = ctx.user_controller.create_user(user.into_inner()).await?;

    // Retorna 200 OK com o JSON do usuário criado
    Ok(Json(created))
}

/// Rota GET `/users/<id>`
/// Busca um usuário pelo seu ID. Usa tracing para log estruturado.
///
/// # Parâmetros
/// - `ctx`: acesso ao AppContext contendo os controladores
/// - `id`: identificador inteiro passado na URL
///
/// # Retorno
/// - `200 OK` com JSON do usuário
/// - `404 Not Found` se não existir
///
/// A macro `#[instrument]` registra automaticamente um *span* com o parâmetro `id`,
/// permitindo rastrear essa chamada no Jaeger ou logs estruturados.
#[get("/<id>")]
#[instrument(skip(ctx))]
pub async fn get_user(ctx: &State<AppContext>, id: i32) -> Result<Json<User>, Status> {
    // Chama o controller para buscar o usuário
    let user = ctx.user_controller.get_user(id).await?;

    // Retorna o JSON se encontrado
    Ok(Json(user))
}

/// Registra as rotas disponíveis para o recurso `/users`.
/// Deve ser usada com `.mount("/users", routes())` no `main.rs`.
pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, get_user]
}
