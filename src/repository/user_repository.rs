// Importa os modelos da camada de domínio:
// - `NewUser`: estrutura usada para criar um novo usuário (dados de entrada)
// - `User`: estrutura completa que representa um usuário armazenado no banco
use crate::models::user::{NewUser, User};

// Importa a enum `AppError`, usada para representar erros técnicos ou de negócio
// que podem ocorrer durante operações de repositório.
use crate::errors::AppError;

// Importa do `sqlx`:
// - `MySqlPool`: representa um pool de conexões para o banco MySQL
// - `Row`: permite acesso a colunas pelo nome
// - `self`: traz o namespace sqlx inteiro, incluindo query, fetch_optional etc.
use rocket_db_pools::sqlx::{self, MySqlPool, Row};

/// `UserRepository` representa a camada de **persistência de dados do domínio de usuários**.
///
/// Ele deve conter **somente interações com o banco de dados**,
/// sem aplicar regras de negócio ou lógica de aplicação.
///
/// A separação do repositório em relação ao serviço garante:
/// - Código mais testável
/// - Possibilidade de reutilização (ex: outro controller, outra API)
/// - Facilidade de substituição do backend (ex: mudança de banco ou arquitetura CQRS)
#[derive(Clone)]
pub struct UserRepository {
    /// Conjunto de conexões reutilizáveis para o banco de dados MySQL.
    /// Isso permite que múltiplas requisições concorrentes sejam tratadas de forma eficiente.
    pub pool: MySqlPool,
}

impl UserRepository {
    /// Cria uma nova instância do repositório de usuários com o pool fornecido.
    ///
    /// Esse padrão segue o princípio de injeção de dependência,
    /// permitindo maior flexibilidade e facilidade em testes automatizados.
    ///
    /// # Parâmetros
    /// - `pool`: pool de conexões MySQL gerenciado pelo Rocket/SQLx
    ///
    /// # Retorno
    /// - Instância de `UserRepository`
    pub fn new(pool: MySqlPool) -> Self {
        Self { pool }
    }

    /// Insere um novo usuário na base de dados.
    ///
    /// Este método executa uma instrução SQL do tipo `INSERT`, utilizando parâmetros bind
    /// para prevenir ataques de injeção de SQL.
    ///
    /// # Parâmetros
    /// - `user`: estrutura com `name`, `email`, `birth_date`
    ///
    /// # Retorno
    /// - `Ok(User)`: struct preenchida com o ID gerado automaticamente
    /// - `Err(AppError::InternalError)`: falha técnica (ex: conexão, sintaxe SQL, timeout)
    pub async fn create_user(&self, user: NewUser) -> Result<User, AppError> {
        let rec = sqlx::query("INSERT INTO users (name, email, birth_date) VALUES (?, ?, ?)")
            .bind(&user.name) // Associa o nome ao primeiro ?
            .bind(&user.email) // Associa o email ao segundo ?
            .bind(user.birth_date) // Associa a data ao terceiro ?
            .execute(&self.pool) // Executa no pool de conexões
            .await
            .map_err(|err| {
                AppError::InternalError(format!("Erro ao inserir usuário no banco: {}", err))
            })?;

        let id = rec.last_insert_id() as i32;

        Ok(User {
            id,
            name: user.name,
            email: user.email,
            birth_date: user.birth_date,
        })
    }

    /// Busca um usuário pelo ID.
    ///
    /// Executa uma consulta `SELECT` na tabela `users`, com a cláusula `WHERE id = ?`.
    /// Caso o usuário exista, os dados são convertidos em uma instância de `User`.
    ///
    /// # Parâmetros
    /// - `id`: ID do usuário a ser recuperado
    ///
    /// # Retorno
    /// - `Ok(Some(User))`: se o usuário for encontrado
    /// - `Ok(None)`: se o ID não estiver presente no banco
    /// - `Err(AppError::InternalError)`: erro técnico (ex: SQL malformado, conexão falhou)
    pub async fn get_user(&self, id: i32) -> Result<Option<User>, AppError> {
        let row = sqlx::query("SELECT id, name, email, birth_date FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| AppError::InternalError(format!("Erro ao acessar o banco: {}", err)))?;

        let user = row.map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            birth_date: row.get("birth_date"),
        });

        Ok(user)
    }

    /// Busca um usuário na base de dados utilizando seu endereço de email.
    ///
    /// Essa função consulta a tabela `users` em busca de um registro com o campo `email` igual ao valor informado.
    /// O resultado é encapsulado em um `Result<Option<User>, AppError>`, permitindo três possibilidades:
    ///
    /// - `Ok(Some(User))`: usuário encontrado com sucesso.
    /// - `Ok(None)`: nenhum usuário com esse email foi encontrado.
    /// - `Err(AppError)`: ocorreu uma falha técnica ao acessar o banco (ex: falha de conexão, erro de SQL).
    ///
    /// # Parâmetros
    /// - `email`: string de referência para busca, passada como fatia (`&str`).
    ///
    /// # Retorno
    /// - `Result<Option<User>, AppError>`:
    ///     - `Some(user)` → usuário com esse email foi encontrado.
    ///     - `None` → não existe usuário com esse email.
    ///     - `Err(AppError::InternalError)` → erro técnico na query.
    ///
    /// # Exemplo
    /// ```
    /// let resultado = repo.get_by_email("usuario@email.com").await?;
    /// match resultado {
    ///     Some(user) => println!("Usuário encontrado: {}", user.name),
    ///     None => println!("Usuário não encontrado"),
    /// }
    /// ```
    pub async fn get_by_email(&self, email: &str) -> Result<Option<User>, AppError> {
        // Prepara a query SQL parametrizada para evitar SQL Injection.
        // A função `fetch_optional` retorna Ok(Some(row)) se encontrou um, Ok(None) se não encontrou.
        let row = sqlx::query("SELECT id, name, email, birth_date FROM users WHERE email = ?")
            .bind(email) // Substitui o `?` na query pelo valor de `email`, com segurança.
            .fetch_optional(&self.pool) // Executa a query e retorna uma linha opcional.
            .await
            // Se ocorrer erro técnico (conexão, sintaxe SQL etc), mapeia para AppError::InternalError com mensagem descritiva.
            .map_err(|err| AppError::InternalError(format!("Erro ao buscar email: {}", err)))?;

        // Se encontrou algum registro (`Some(row)`), mapeia para struct `User` manualmente
        // Caso contrário, retorna `None`.
        Ok(row.map(|row| User {
            id: row.get("id"),
            name: row.get("name"),
            email: row.get("email"),
            birth_date: row.get("birth_date"),
        }))
    }
}
