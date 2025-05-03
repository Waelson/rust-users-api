// Importa o tipo de erro da camada de domínio, usado para representar falhas
// técnicas (como erro no banco de dados) ou regras de negócio (como "usuário não encontrado")
use crate::errors::AppError;

// Importa os tipos de modelo da aplicação:
// - `NewUser`: estrutura com os dados de entrada para criação de usuário
// - `User`: estrutura completa representando um usuário persistido
use crate::models::user::{NewUser, User};

// Importa o repositório responsável pelas interações com o banco de dados.
// O repositório é responsável apenas por ler/gravar dados, sem lógica de negócio.
use crate::repository::user_repository::UserRepository;

/// `UserService` representa a **camada de serviço** da aplicação para o domínio de usuários.
///
/// Esta camada tem como responsabilidades:
/// - Orquestrar chamadas ao repositório
/// - Adicionar lógica de negócio (como validações, transformações e regras)
/// - Isolar o controlador (camada HTTP) da persistência direta
///
/// O uso da camada de serviço facilita testes unitários, manutenção e expansão do sistema.
#[derive(Clone)]
pub struct UserService {
    /// Repositório de usuários, utilizado para acesso ao banco de dados.
    pub repo: UserRepository,
}

impl UserService {
    /// Construtor da `UserService`, com injeção explícita do repositório de usuários.
    ///
    /// Esse padrão favorece desacoplamento e facilita testes (por exemplo, usando mocks).
    ///
    /// # Parâmetros
    /// - `repo`: instância do repositório de usuários (`UserRepository`)
    ///
    /// # Retorno
    /// - Uma instância de `UserService` com o repositório injetado.
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    /// Cria um novo usuário na base de dados.
    ///
    /// Este método apenas delega para o repositório, mas futuramente pode incluir validações,
    /// verificação de duplicidade de email, envio de notificações, etc.
    ///
    /// # Parâmetros
    /// - `user`: estrutura contendo os dados do novo usuário (nome, email, nascimento)
    ///
    /// # Retorno
    /// - `Ok(User)`: se o usuário for criado com sucesso
    /// - `Err(AppError)`: erro técnico convertido no repositório (ex: erro de SQL)
    pub async fn create_user(&self, user: NewUser) -> Result<User, AppError> {
        self.repo.create_user(user).await
    }

    /// Busca um usuário pelo seu ID.
    ///
    /// Essa função encapsula:
    /// - A chamada ao repositório para buscar o usuário
    /// - A lógica de retorno de erro de negócio, caso o usuário não exista
    ///
    /// # Parâmetros
    /// - `id`: identificador numérico do usuário (ex: 42)
    ///
    /// # Retorno
    /// - `Ok(User)`: se o usuário for encontrado
    /// - `Err(AppError::BusinessError)`: se não encontrado
    /// - `Err(AppError::InternalError)`: se ocorrer falha técnica (ex: banco indisponível)
    pub async fn get_user(&self, id: i32) -> Result<User, AppError> {
        match self.repo.get_user(id).await {
            // Propaga erro técnico sem mascarar (falha no banco, conexão, etc.)
            Err(e) => Err(e),

            // Retorna erro de negócio se não encontrou o usuário
            Ok(None) => Err(AppError::NotFoundError("Usuário não encontrado".into())),

            // Retorna o usuário encontrado com sucesso
            Ok(Some(user)) => Ok(user),
        }
    }
}
