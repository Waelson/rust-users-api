// Importa o derive `Serialize` da biblioteca Rocket para permitir que o erro seja convertido para JSON.
// Isso é necessário para que o erro possa ser enviado como resposta HTTP pela API.
use rocket::serde::Serialize;

/// `ApiError` representa a estrutura padronizada de erro retornada pela API HTTP.
///
/// Esse tipo é usado nos controladores para encapsular qualquer tipo de falha da aplicação
/// e devolvê-la de forma consistente para o cliente (ex: frontend, outro microserviço).
///
/// Ele é convertido automaticamente para JSON na resposta:
/// ```json
/// {
///   "status": 400,
///   "message": "Erro de validação",
///   "cause": [ "Email é obrigatório" ]
/// }
/// ```
#[derive(Debug, Clone, Serialize)]
#[serde(crate = "rocket::serde")] // especifica o caminho da dependência serde usada pelo Rocket
pub struct ApiError {
    /// Código de status HTTP que será retornado (ex: 400, 404, 500)
    pub status: u16,

    /// Mensagem principal que resume o tipo de erro
    pub message: String,

    /// Lista com uma ou mais causas específicas do erro (mensagens detalhadas)
    pub cause: Vec<String>,
}

impl ApiError {
    /// Cria um erro do tipo "Validação" com status HTTP 400.
    ///
    /// Use quando dados de entrada estão faltando ou incorretos.
    pub fn validation(msg: &str) -> Self {
        Self {
            status: 400,
            message: "Erro de validação".into(),
            cause: vec![msg.into()],
        }
    }

    /// Cria um erro do tipo "Não encontrado" com status HTTP 404.
    ///
    /// Use quando um recurso (usuário, produto, etc) não for localizado no banco.
    pub fn not_found(msg: &str) -> Self {
        Self {
            status: 404,
            message: "Recurso nao encontrado".into(),
            cause: vec![msg.into()],
        }
    }

    /// Cria um erro do tipo "Regra de negócio violada" com status HTTP 409.
    ///
    /// Use quando o input é válido, mas o domínio rejeita a ação (ex: "saldo insuficiente").
    pub fn business(msg: &str) -> Self {
        Self {
            status: 409,
            message: "Regra de negocio".into(),
            cause: vec![msg.into()],
        }
    }

    /// Cria um erro interno com status HTTP 500.
    ///
    /// Use para falhas inesperadas do sistema (falha de banco, timeout, parsing, etc).
    /// `msg`: título amigável (geralmente fixo)
    /// `detail`: descrição técnica para debugging
    pub fn internal(msg: &str, detail: String) -> Self {
        Self {
            status: 500,
            message: msg.into(),
            cause: vec![detail],
        }
    }
}
