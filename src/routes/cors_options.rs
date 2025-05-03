// Importa o tipo `Status`, usado para retornar códigos HTTP como 200, 404, etc.
// Também importa o atributo `#[options]` usado para definir rotas HTTP OPTIONS.
use rocket::{http::Status, options};

/// Rota `OPTIONS` utilizada para responder a requisições **preflight** do CORS (Cross-Origin Resource Sharing).
///
/// ### O que é Preflight?
/// Quando uma aplicação web em um domínio diferente (ex: `app.frontend.com`)
/// faz uma requisição `POST`, `PUT`, `DELETE` ou utiliza cabeçalhos customizados (ex: `Authorization`),
/// o navegador envia automaticamente uma requisição `OPTIONS` antes da requisição principal.
///
/// Essa requisição `OPTIONS` é chamada de **preflight** e tem o objetivo de verificar com o servidor se:
/// - o método é permitido (`POST`, `PUT`, etc)
/// - os cabeçalhos solicitados são aceitos (ex: `Content-Type`, `Authorization`)
/// - a origem da requisição tem permissão
///
/// ### Quando essa rota é chamada?
/// - Quando o navegador envia `OPTIONS /users` ou `OPTIONS /qualquer-rota` com cabeçalhos CORS.
/// - Se não houver uma rota `OPTIONS` correspondente, o Rocket retornará erro `404`.
///
/// ### Sobre o `"/<_..>"`
/// O path com `"/<_..>"` indica que **essa rota responde a qualquer caminho** (é um _catch-all_).
/// Assim, você não precisa definir uma rota OPTIONS específica para cada endpoint da API.
///
/// ### Retorno
/// Retorna um `Status::Ok` (HTTP 200), sinalizando ao navegador que ele pode prosseguir com a requisição principal.
///
/// > A lógica de injetar os headers CORS apropriados (como `Access-Control-Allow-*`) deve ser feita via `Fairing`,
///   geralmente no middleware `CORS` que você já implementou.
#[options("/<_..>")]
pub async fn preflight() -> Status {
    // Retorna HTTP 200 OK
    Status::Ok
}
