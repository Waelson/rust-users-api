// Importa o trait necessário para encadear `.with(...)` nos layers do tracing.
use tracing_subscriber::prelude::*;

// Importa:
// - `EnvFilter` para configurar o nível de log via variável de ambiente (como RUST_LOG)
// - `fmt` para formatação dos logs (ex: timestamp, nível, mensagem, etc)
use tracing_subscriber::{fmt, EnvFilter};

/// Inicializa o sistema de logging estruturado com `tracing`.
///
/// Essa função configura dois layers:
/// 1. Um filtro baseado na variável de ambiente `RUST_LOG` (como em outras libs de log Rust).
/// 2. Um formatter de log (linha, nível, etc) para imprimir logs formatados no stdout.
///
/// Exemplo de uso:
/// ```bash
/// RUST_LOG=debug,user_api=info cargo run
/// ```
///
/// Se a variável `RUST_LOG` não estiver presente, o nível padrão será `info`.
pub fn init() {
    // Cria o filtro de nível de log a partir da variável `RUST_LOG`.
    // Ex: RUST_LOG=debug,user_api=warn
    // Se a variável não estiver definida, aplica o nível `info` como padrão.
    let filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info"));

    // Cria o formatter de logs para exibir logs no terminal com configurações detalhadas.
    let fmt_layer = fmt::layer()
        // Não exibe o "target" (ex: nome do módulo/fonte)
        .with_target(true)
        // Exibe o nível do log (info, warn, error, etc)
        .with_level(true)
        // Exibe o número da linha do código onde o log foi chamado
        .with_line_number(true)
        // Exibe o ID da thread que gerou o log
        .with_thread_ids(true)
        // Exibe o nome da thread (se configurado)
        .with_thread_names(true);

    // Registra o subscriber com os dois layers combinados:
    // - `filter_layer`: define o que será logado com base em `RUST_LOG`
    // - `fmt_layer`: define como será exibido
    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
