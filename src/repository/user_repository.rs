// Importa os modelos da camada de domínio:
// - `NewUser`: estrutura usada para criar um novo usuário (dados de entrada)
// - `User`: estrutura completa que representa um usuário armazenado no banco
use crate::models::user::{NewUser, User};

// Importa `NaiveDate` do chrono, que representa uma data sem fuso horário.
// É utilizado para armazenar e recuperar a data de nascimento dos usuários.
use chrono::NaiveDate;

// Importa:
// - `sqlx`: biblioteca para acesso ao banco de dados (MySQL)
// - `MySqlPool`: tipo que representa um pool de conexões MySQL gerenciado pelo sqlx
// - `Row`: trait que permite extrair colunas por nome do resultado da query
use rocket_db_pools::sqlx::{self, MySqlPool, Row};

/// `UserRepository` é responsável por **acesso direto ao banco de dados**.
/// Contém métodos para criar e consultar usuários.
/// Ele **não deve conter lógica de negócio** — apenas interações com SQL.
///
/// A ideia é manter o repositório como uma camada desacoplada e reutilizável,
/// que pode ser mockada em testes ou substituída por outro backend (ex: Redis, API externa).
#[derive(Clone)]
pub struct UserRepository {
    /// Pool de conexões com o banco de dados MySQL.
    /// Esse pool é gerenciado automaticamente pelo Rocket + sqlx,
    /// e permite executar queries simultâneas com reuso eficiente de conexões.
    pub pool: MySqlPool,
}

impl UserRepository {
    /// Cria uma nova instância do repositório com um pool de conexões injetado.
    /// Esse padrão permite desacoplar a camada de banco de dados da aplicação principal.
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Insere um novo usuário na tabela `users` do banco de dados.
    ///
    /// # Parâmetros
    /// - `user`: struct contendo nome, email e data de nascimento.
    ///
    /// # Retorno
    /// - `Ok(User)`: usuário criado com sucesso, incluindo ID atribuído pelo banco.
    /// - `Err(sqlx::Error)`: erro de conexão ou falha ao executar o SQL.
    pub async fn create_user(&self, user: NewUser) -> Result<User, sqlx::Error> {
        // Executa a query SQL de inserção com placeholders (`?`) para evitar SQL Injection
        let rec = sqlx::query("INSERT INTO users (name, email, birth_date) VALUES (?, ?, ?)")
            .bind(&user.name) // Associa `user.name` ao primeiro ?
            .bind(&user.email) // Associa `user.email` ao segundo ?
            .bind(user.birth_date) // Associa `user.birth_date` ao terceiro ?
            .execute(&self.pool) // Executa a query usando o pool de conexões
            .await?; // Propaga erro de execução com `?`

        // Obtém o ID gerado automaticamente pela inserção no banco (auto-incremento)
        let id = rec.last_insert_id() as i32;

        // Retorna o usuário recém-criado com ID preenchido
        Ok(User {
            id,
            name: user.name,
            email: user.email,
            birth_date: user.birth_date,
        })
    }

    /// Busca um usuário por ID na tabela `users`.
    ///
    /// # Parâmetros
    /// - `user_id`: ID do usuário a ser buscado (chave primária)
    ///
    /// # Retorno
    /// - `Ok(User)`: usuário encontrado
    /// - `Err(sqlx::Error)`: usuário não encontrado ou erro de conexão
    pub async fn get_user(&self, user_id: i32) -> Result<User, sqlx::Error> {
        // Prepara a query de seleção para buscar um único usuário pelo ID
        let row = sqlx::query("SELECT id, name, email, birth_date FROM users WHERE id = ?")
            .bind(user_id) // Substitui o `?` pelo valor de `user_id`
            .fetch_one(&self.pool) // Busca uma única linha (ou retorna erro)
            .await?; // Propaga erro em caso de falha

        // Mapeia os campos do resultado para a struct `User`
        Ok(User {
            id: row.get("id"),                                 // extrai coluna "id" como i32
            name: row.get("name"),                             // extrai coluna "name" como String
            email: row.get("email"),                           // extrai coluna "email" como String
            birth_date: row.get::<NaiveDate, _>("birth_date"), // extrai coluna "birth_date" como NaiveDate
        })
    }
}
