// Importa os tipos de modelo da camada de domínio:
// - `NewUser`: dados necessários para a criação de um usuário (nome, email, nascimento)
// - `User`: estrutura representando um usuário persistido com ID
use crate::models::user::{NewUser, User};

// Importa o tipo de erro da camada de apresentação da API.
// O `ApiError` encapsula status HTTP, mensagens e causas humanas.
use crate::errors::ApiError;

// Importa o serviço responsável pela lógica de negócio relacionada a usuários.
// O serviço é consumido pelo controller.
use crate::services::user_service::UserService;

// Importa a macro `info!`, usada para emitir logs estruturados com o nível `info`.
// Ideal para observabilidade e diagnóstico em produção.
use tracing::info;

/// `UserController` representa a camada intermediária entre a camada HTTP (rotas)
/// e a camada de serviço (lógica de negócio). É responsável por:
///
/// - Orquestrar chamadas ao serviço
/// - Tratar erros e convertê-los em `ApiError`
/// - Executar validações ou autorizações futuras
/// - Gerar logs estruturados para monitoramento
///
/// Ao manter a lógica de negócios no serviço e o tratamento de entrada/saída aqui,
/// o código permanece modular e testável.
#[derive(Clone)]
pub struct UserController {
    /// Serviço responsável por interações com o domínio de usuários,
    /// incluindo persistência, regras de negócio, etc.
    pub service: UserService,
}

impl UserController {
    /// Construtor do `UserController`, recebendo a instância do `UserService`.
    ///
    /// # Parâmetros
    /// - `service`: instância já inicializada de `UserService`
    ///
    /// # Retorno
    /// - `UserController` com o serviço injetado
    pub fn new(service: UserService) -> Self {
        Self { service }
    }

    /// Cria um novo usuário na base de dados.
    ///
    /// # Parâmetros
    /// - `user`: struct contendo os dados de entrada validados (nome, email, data de nascimento)
    ///
    /// # Retorno
    /// - `Ok(User)`: usuário criado com sucesso
    /// - `Err(ApiError)`: erro técnico convertido de `AppError` para erro HTTP sem expor detalhes internos
    ///
    /// # Observações
    /// - O método apenas repassa para o service e converte o erro para `ApiError` via `From<AppError>`
    pub async fn create_user(&self, user: NewUser) -> Result<User, ApiError> {
        self.service.create_user(user).await.map_err(ApiError::from)
    }

    /// Busca um usuário existente pelo seu ID.
    ///
    /// # Parâmetros
    /// - `id`: identificador único do usuário (inteiro positivo)
    ///
    /// # Retorno
    /// - `Ok(User)`: se o usuário for encontrado no banco de dados
    /// - `Err(ApiError)`: se ocorrer falha técnica ou o usuário não existir
    ///
    /// # Observações
    /// - O método gera um log com o ID buscado, útil para rastreamento
    /// - O erro da camada de serviço é convertido para `ApiError` de forma centralizada
    pub async fn get_user(&self, id: i32) -> Result<User, ApiError> {
        info!("Buscando usuário com id = {}", id); // Log de auditoria
        self.service.get_user(id).await.map_err(ApiError::from)
    }
}
