// Importa os tipos de modelo da camada de domínio:
// - `NewUser`: dados para criar um novo usuário
// - `User`: estrutura representando um usuário completo retornado pela API
use crate::models::user::{NewUser, User};

// Importa a camada de serviço, que encapsula a lógica de negócio para o recurso "usuário"
use crate::services::user_service::UserService;

// Importa o enum `Status`, usado para retornar status HTTP padronizados (200, 404, 500, etc)
use rocket::http::Status;

// Importa a macro de logging `info!` do `tracing`, usada para gerar logs estruturados
use tracing::info;

/// `UserController` é responsável por **intermediar as chamadas entre os handlers HTTP e a camada de serviço**.
/// Ele encapsula validações de status HTTP e loga eventos úteis da aplicação.
/// Pode ser facilmente expandido para incluir lógica de autenticação, autorização ou cache.
#[derive(Clone)]
pub struct UserController {
    /// Serviço que contém a lógica de negócio relacionada a usuários.
    pub service: UserService,
}

impl UserController {
    /// Cria uma nova instância do controlador com o serviço de usuários injetado.
    pub fn new(service: UserService) -> Self {
        Self { service }
    }

    /// Cria um novo usuário na base de dados.
    ///
    /// # Parâmetros
    /// - `user`: struct contendo os dados de entrada (nome, email, nascimento)
    ///
    /// # Retorno
    /// - `Ok(User)`: usuário criado com sucesso
    /// - `Err(Status::InternalServerError)`: erro inesperado na persistência
    pub async fn create_user(&self, user: NewUser) -> Result<User, Status> {
        self.service
            .create_user(user) // chama a camada de serviço para persistir o usuário
            .await // espera a operação assíncrona
            .map_err(|_| Status::InternalServerError) // em caso de erro, transforma para HTTP 500
    }

    /// Busca um usuário pelo seu ID.
    ///
    /// # Parâmetros
    /// - `id`: identificador numérico do usuário (esperado no path)
    ///
    /// # Retorno
    /// - `Ok(User)`: se o usuário for encontrado
    /// - `Err(Status::NotFound)`: se o usuário não existir ou ocorrer erro de leitura
    pub async fn get_user(&self, id: i32) -> Result<User, Status> {
        info!("Buscando usuário com id = {}", id); // loga o ID da requisição
        self.service
            .get_user(id) // delega a lógica de busca ao serviço
            .await // aguarda o resultado
            .map_err(|_| Status::NotFound) // transforma erro genérico em HTTP 404
    }
}
