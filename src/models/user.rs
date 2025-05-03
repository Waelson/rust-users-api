// Importa `NaiveDate` da crate `chrono`.
// `NaiveDate` representa uma data sem fuso horário (ex: "2023-05-01").
// É útil para armazenar datas como data de nascimento, sem se preocupar com horas ou timezones.
use chrono::NaiveDate;

// Importa traits para serializar (converter em JSON) e deserializar (converter de JSON)
// via as crates `serde` e `rocket::serde`.
use serde::{Deserialize, Serialize};

/// Struct `User` representa um **usuário persistido no banco de dados**.
///
/// Esse modelo é usado como resposta da API, ou seja,
/// quando os dados do usuário já existem e incluem o campo `id`.
///
/// A estrutura implementa:
/// - `Serialize`: para ser transformada em JSON e enviada como resposta HTTP
/// - `Deserialize`: caso queira desserializar (não obrigatório para respostas)
#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    /// Identificador único do usuário (chave primária no banco de dados).
    pub id: i32,

    /// Nome completo do usuário.
    pub name: String,

    /// Endereço de email do usuário.
    pub email: String,

    /// Data de nascimento no formato `YYYY-MM-DD`.
    pub birth_date: NaiveDate,
}

/// Struct `NewUser` representa os **dados necessários para criar um novo usuário**.
///
/// Esse modelo é usado como entrada na API, vindo do corpo da requisição (JSON),
/// por isso não possui o campo `id`, já que este é gerado automaticamente.
///
/// A estrutura implementa:
/// - `Serialize`: pode ser usada para logs ou testes
/// - `Deserialize`: permite converter JSON da requisição em uma instância de `NewUser`
#[derive(Debug, Serialize, Deserialize)]
pub struct NewUser {
    /// Nome completo do usuário.
    pub name: String,

    /// Endereço de email do usuário.
    pub email: String,

    /// Data de nascimento no formato `YYYY-MM-DD`.
    pub birth_date: NaiveDate,
}
