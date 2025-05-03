// Importa o contexto da aplicação, que centraliza as dependências globais compartilhadas como o UserController.
// Esse contexto é injetado automaticamente nas rotas por meio do `State<AppContext>`.
use crate::context::AppContext;

// Importa o tipo `ApiError`, que representa um erro HTTP estruturado na API,
// contendo status code, mensagem de erro e causas detalhadas.
use crate::errors::ApiError;

// Importa a rota `preflight`, responsável por responder requisições `OPTIONS` do CORS.
use crate::routes::cors_options::preflight;

// Importa os modelos que representam as estruturas de entrada (`NewUser`) e saída (`User`) da API.
// `NewUser` é usado ao criar um novo usuário e `User` representa um usuário persistido, incluindo o `id`.
use crate::models::user::{NewUser, User};

// Importa macros e utilitários do Rocket para definição de rotas e serialização de dados.
// - `get` e `post` são macros para definir rotas HTTP GET e POST.
// - `routes!` agrega as rotas para montagem no servidor.
// - `Json` permite converter structs para JSON automaticamente na resposta.
// - `State` permite acessar o contexto global da aplicação (`AppContext`) de forma segura.
use rocket::{get, post, routes, serde::json::Json, State};

// Importa a macro `#[instrument]` da crate `tracing`, que cria automaticamente um *span*
// para rastrear a execução da função, útil para observabilidade (logs, tracing distribuído, Jaeger, etc).
use tracing::instrument;

/// Rota POST `/users`
///
/// Essa rota permite a criação de um novo usuário. Espera-se que o corpo da requisição contenha
/// os dados JSON compatíveis com a struct `NewUser` (nome, email, data de nascimento).
///
/// A macro `#[instrument(skip(ctx))]` cria um *span* de tracing para monitoramento e logs,
/// mas ignora o campo `ctx` por conter referências complexas que não são úteis na saída.
///
/// # Parâmetros
/// - `ctx`: instância de `AppContext` compartilhada, contendo o `UserController`.
/// - `user`: JSON com os dados de entrada serializados automaticamente como `NewUser`.
///
/// # Retorno
/// - `Ok(Json<User>)`: usuário criado com sucesso.
/// - `Err(ApiError)`: erro de validação, regra de negócio ou erro interno.
#[post("/", format = "json", data = "<user>")]
#[instrument(skip(ctx))]
pub async fn create_user(
    ctx: &State<AppContext>,
    user: Json<NewUser>,
) -> Result<Json<User>, ApiError> {
    // Converte Json<NewUser> para NewUser e chama o controller para criar o usuário
    let created = ctx.user_controller.create_user(user.into_inner()).await?;

    // Retorna o usuário criado em formato JSON
    Ok(Json(created))
}

/// Rota GET `/users/<id>`
///
/// Essa rota permite buscar um usuário existente pelo seu identificador numérico (`id`).
/// A macro `#[instrument]` adiciona rastreamento estruturado com o parâmetro `id`.
///
/// # Parâmetros
/// - `ctx`: instância compartilhada de `AppContext`, contendo o controller.
/// - `id`: identificador inteiro extraído do path da URL.
///
/// # Retorno
/// - `Ok(Json<User>)`: usuário encontrado com sucesso.
/// - `Err(ApiError)`: se o usuário não for encontrado ou ocorrer um erro interno.
#[get("/<id>")]
#[instrument(skip(ctx))]
pub async fn get_user(ctx: &State<AppContext>, id: i32) -> Result<Json<User>, ApiError> {
    // Chama o controller para buscar o usuário pelo ID
    let user = ctx.user_controller.get_user(id).await?;

    // Retorna o usuário encontrado como JSON
    Ok(Json(user))
}

/// Registra todas as rotas relacionadas ao recurso `/users`.
///
/// A função `routes()` retorna um vetor contendo todas as rotas que devem ser montadas no endpoint `/users`.
/// Inclui as rotas de:
/// - Criação (`POST /users`)
/// - Consulta por ID (`GET /users/<id>`)
/// - Preflight (`OPTIONS /users/*`) para suporte a CORS
///
/// Essa função é usada no `main.rs` com `.mount("/users", routes())`.
pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, get_user, preflight]
}
