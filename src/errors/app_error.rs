// Importa o tipo `ApiError`, responsável por estruturar erros que serão serializados como resposta HTTP.
// O `AppError`, utilizado internamente na aplicação, será convertido em `ApiError` ao ser propagado para os controllers.
use super::api_error::ApiError;

// Importa a macro derive `Error` da crate `thiserror`, que gera automaticamente a implementação do trait `std::error::Error`.
// Isso permite usar `AppError` com ergonomia idiomática em Rust, inclusive com o operador `?`.
use thiserror::Error;

/// Enum `AppError` representa todos os erros possíveis que podem ocorrer nas **camadas internas da aplicação**.
///
/// Ele é utilizado como tipo de erro padrão nas funções das camadas de **serviço (service)** e **repositório (repository)**.
///
/// Com ele, é possível categorizar os erros de forma clara, separar responsabilidades e aplicar lógica
/// apropriada de tratamento e logging, além de facilitar testes e conversão para respostas HTTP (`ApiError`).
#[derive(Debug, Error)]
pub enum AppError {
    /// Erro de validação representa falhas causadas por entradas malformadas ou inválidas,
    /// como campos obrigatórios ausentes, formato de e-mail incorreto ou violação de regras simples.
    /// Erros de validação com múltiplas causas.
    #[error("Erro de validação: {0:?}")]
    ValidationError(Vec<String>),

    /// Erro de negócio representa regras de domínio que não foram satisfeitas,
    /// como "usuário já cadastrado", "saldo insuficiente", "você não pode excluir seu próprio usuário", etc.
    #[error("Erro de negócio: {0}")]
    BusinessError(String),

    /// Erro específico para situações onde o recurso requisitado não foi encontrado.
    /// Exemplo: buscar um usuário por ID e ele não existir na base.
    /// Separa esse caso de outras falhas de negócio para permitir mapeamento claro para `HTTP 404`.
    #[error("Recurso não encontrado: {0}")]
    NotFoundError(String),

    /// Erro interno representa falhas inesperadas, geralmente técnicas:
    /// - Erros de banco de dados (conexão, constraint, etc)
    /// - Falhas de I/O, timeout, parsing
    /// - Problemas que não deveriam ocorrer durante operação normal
    ///
    /// Esses erros devem ser registrados com detalhes para diagnóstico posterior.
    #[error("Erro interno: {0}")]
    InternalError(String),
}

/// Permite a conversão automática de `AppError` para `ApiError`,
/// que é o tipo esperado pelas rotas HTTP para gerar respostas padronizadas.
///
/// Isso permite usar o operador `?` nos controllers sem precisar mapear manualmente cada erro.
///
/// A conversão define qual código HTTP será retornado e estrutura o corpo da resposta:
/// - `ValidationError` → HTTP 400
/// - `BusinessError` → HTTP 422
/// - `NotFoundError` → HTTP 404
/// - `InternalError` → HTTP 500
impl From<AppError> for ApiError {
    fn from(err: AppError) -> Self {
        match err {
            AppError::ValidationError(errors) => ApiError::validation(errors),
            AppError::BusinessError(msg) => ApiError::business(&msg),
            AppError::NotFoundError(msg) => ApiError::not_found(&msg),
            AppError::InternalError(msg) => ApiError::internal("Erro interno", msg),
        }
    }
}
