// Importa o tipo `Status`, que representa códigos de status HTTP (ex: 200, 404, 500).
use rocket::http::Status;

// Representa a requisição HTTP atual, usada pelo Rocket para montar a resposta.
use rocket::request::Request;

// Define os tipos necessários para implementar uma resposta personalizada para Rocket.
// `Responder` é o trait que permite que um tipo seja convertido em uma resposta HTTP.
// `RocketResult` é o tipo alias para `Result<Response<'static>, Status>`.
use rocket::response::{Responder, Result as RocketResult};

// Permite serializar o erro como JSON automaticamente.
use rocket::serde::json::Json;

// Importa a estrutura de erro que será convertida em resposta HTTP.
use crate::errors::api_error::ApiError;

/// Implementa o trait `Responder` para que `ApiError` possa ser retornado diretamente por rotas.
///
/// Isso permite retornar um erro como este:
/// ```rust
/// return Err(ApiError::not_found("Usuário não encontrado"));
/// ```
/// E o Rocket automaticamente serializa o `ApiError` como JSON,
/// além de configurar o código HTTP correto com base no campo `.status`.
impl<'r> Responder<'r, 'static> for ApiError {
    fn respond_to(self, req: &'r Request<'_>) -> RocketResult<'static> {
        // Cria uma resposta HTTP baseada no conteúdo serializado como JSON
        rocket::response::Response::build_from(Json(self.clone()).respond_to(req)?)
            // Define o status HTTP da resposta com base no campo `status` do erro
            .status(Status::from_code(self.status).unwrap_or(Status::InternalServerError))
            // Finaliza a construção da resposta e retorna `Ok(Response)`
            .ok()
    }
}
