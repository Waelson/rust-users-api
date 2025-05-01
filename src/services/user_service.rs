// Importa os tipos de modelo da aplicação:
// - `NewUser`: estrutura com os dados de entrada para criação de usuário
// - `User`: estrutura completa representando um usuário persistido
use crate::models::user::{NewUser, User};

// Importa o repositório responsável pelas interações com o banco de dados
use crate::repository::user_repository::UserRepository;

/// A `UserService` representa a **camada de serviço da aplicação**.
///
/// Seu principal papel é **encapsular a lógica de negócio relacionada a usuários**,
/// orquestrando ações entre controladores e repositórios.
/// Essa camada permite adicionar validações, regras de negócio, cálculos e manipulações futuras
/// sem poluir os repositórios (acesso a dados) ou controladores (HTTP).
#[derive(Clone)]
pub struct UserService {
    /// Repositório responsável por interagir diretamente com o banco de dados.
    pub repo: UserRepository,
}

impl UserService {
    /// Cria uma nova instância do serviço com o repositório injetado.
    ///
    /// Esse padrão permite **injeção de dependência explícita**, facilitando testes e modularidade.
    pub fn new(repo: UserRepository) -> Self {
        Self { repo }
    }

    /// Cria um novo usuário na base de dados, delegando a lógica de persistência ao repositório.
    ///
    /// # Parâmetros
    /// - `user`: dados do novo usuário (nome, email, data de nascimento).
    ///
    /// # Retorno
    /// - `Ok(User)`: usuário criado com sucesso.
    /// - `Err(sqlx::Error)`: erro durante a operação de escrita no banco.
    pub async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error> {
        self.repo.create_user(user).await
    }

    /// Recupera um usuário da base de dados pelo seu identificador.
    ///
    /// # Parâmetros
    /// - `id`: ID único do usuário.
    ///
    /// # Retorno
    /// - `Ok(User)`: usuário encontrado.
    /// - `Err(sqlx::Error)`: erro na consulta ou usuário inexistente.
    pub async fn get_user(&self, id: i32) -> Result<User, sqlx::Error> {
        self.repo.get_user(id).await
    }
}
