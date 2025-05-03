// Importa os módulos necessários do framework Rocket para criar um fairing (middleware),
// manipular headers HTTP e interceptar requisições e respostas.
use rocket::{
    fairing::{Fairing, Info, Kind}, // `Fairing` define um interceptor de requisição/resposta
    http::Header,                   // Permite definir cabeçalhos HTTP
    Request,                        // Representa a requisição HTTP recebida
    Response,                       // Representa a resposta HTTP a ser enviada
};

/// Estrutura `CORS` que representa o middleware de CORS.
///
/// CORS (Cross-Origin Resource Sharing) é um mecanismo de segurança dos navegadores
/// que impede requisições feitas por domínios diferentes.
/// Ao adicionar esse middleware, sua API passa a permitir chamadas de outras origens (ex: frontend separado).
pub struct CORS;

/// Implementa o trait `Fairing` para a struct `CORS`.
/// Fairings em Rocket funcionam como middlewares que podem interceptar e modificar
/// requisições e respostas antes que elas cheguem ou saiam da aplicação.
///
/// O atributo `#[rocket::async_trait]` é necessário porque o trait exige métodos assíncronos.
#[rocket::async_trait]
impl Fairing for CORS {
    /// Define as informações básicas sobre o fairing.
    ///
    /// - `name`: nome do fairing, apenas para log e identificação.
    /// - `kind`: tipo do fairing, neste caso `Response`, indicando que será aplicado após
    /// a requisição ser processada, antes da resposta ser enviada.
    fn info(&self) -> Info {
        Info {
            name: "CORS Headers", // Nome do fairing (aparece nos logs do Rocket)
            kind: Kind::Response, // Define que esse fairing atua sobre respostas
        }
    }

    /// Método chamado em todas as respostas HTTP antes de serem enviadas ao cliente.
    ///
    /// Aqui adicionamos os cabeçalhos CORS necessários para permitir que
    /// clientes de outros domínios possam acessar a API com segurança.
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        // Permite acesso de qualquer origem (`*`). Pode ser trocado por uma origem específica em produção.
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));

        // Especifica quais métodos HTTP são aceitos para requisições cross-origin.
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "GET, POST, PUT, DELETE, OPTIONS",
        ));

        // Informa ao navegador quais cabeçalhos personalizados são permitidos na requisição.
        response.set_header(Header::new(
            "Access-Control-Allow-Headers",
            "Content-Type, Authorization",
        ));

        // Permite o envio de cookies e headers de autenticação na requisição (ex: Authorization: Bearer).
        // Importante: só funciona se o Allow-Origin **não for** `*`.
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
