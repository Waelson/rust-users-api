// Importa o controlador de usuários, que será gerenciado dentro do contexto da aplicação.
use crate::controllers::user_controller::UserController;

/// `AppContext` é uma estrutura central que carrega as dependências compartilhadas da aplicação,
/// como controladores, serviços, caches, ou qualquer outro recurso que precise ser injetado
/// em múltiplas partes do sistema.
///
/// Ele atua como um **"container de dependências"**, fornecido globalmente ao Rocket via `.manage(...)`
/// para permitir **injeção de dependências explícita** nos handlers.
///
/// Como ele implementa `Clone`, pode ser clonado com baixo custo (usualmente só ponteiros internos).
#[derive(Clone)]
pub struct AppContext {
    /// Instância do `UserController`, responsável pela lógica de roteamento e coordenação
    /// das requisições relacionadas ao recurso "usuário".
    ///
    /// Esse campo pode ser acessado nos handlers por meio do tipo `&State<AppContext>`,
    /// exemplo:
    /// ```rust
    /// #[get("/users/<id>")]
    /// async fn get_user(ctx: &State<AppContext>, id: i32) -> ... {
    ///     ctx.user_controller.get_user(id).await
    /// }
    /// ```
    pub user_controller: UserController,
}
