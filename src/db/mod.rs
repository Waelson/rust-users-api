// Importa:
// - `Database`: derive macro do Rocket que integra com o sistema de fairings e gerencia o ciclo de vida do pool
// - `sqlx`: acesso direto ao driver MySQL do sqlx (MySqlPool, Query, etc)
use rocket_db_pools::{sqlx, Database};

/// Estrutura que representa o pool de conexões com o banco de dados MySQL,
/// integrada ao Rocket por meio da derive macro `#[derive(Database)]`.
///
/// A macro `#[database("mysql")]`:
/// - Diz ao Rocket para procurar uma configuração chamada `[databases.mysql]`
///   no arquivo `Rocket.toml` ou na configuração programática via `figment`.
///
/// Exemplo de configuração esperada:
/// ```toml
/// [default.databases.mysql]
/// url = "mysql://root:root@localhost:3306/rust_db"
/// ```
///
/// O Rocket injeta esse pool automaticamente nos seus handlers via `.attach(Db::init())`
/// e permite acessá-lo com `Db::fetch(&rocket)` após o ignite.
///
/// A estrutura é marcada com `Clone` para que possa ser clonada entre threads.
#[derive(Database, Clone)]
#[database("mysql")]
pub struct Db(sqlx::MySqlPool);

impl Db {
    /// Método auxiliar que expõe o pool interno do `sqlx::MySqlPool`,
    /// permitindo o uso direto da API do SQLx em repositórios e serviços.
    ///
    /// Exemplo de uso:
    /// ```rust
    /// let pool = db.inner();
    /// sqlx::query("SELECT ...").fetch_one(pool).await?;
    /// ```
    pub fn inner(&self) -> &sqlx::MySqlPool {
        &self.0
    }
}
