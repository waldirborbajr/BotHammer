use teloxide::utils::command::BotCommands;

pub mod blockdomain;
pub mod help;
pub mod language;
pub mod reload;
pub mod stats;
pub mod status;
pub mod train;
pub mod unban;

/// Comandos disponíveis no BanHammer
#[derive(BotCommands, Clone, Debug)]
#[command(rename_rule = "lowercase", description = "BanHammer Commands")]
pub enum Command {
    /// Exibe ajuda do bot
    #[command(description = "Mostrar ajuda / Show help / Mostrar ayuda")]
    Help,

    /// Mostra o status do bot
    #[command(description = "Status do bot / Bot status / Estado del bot")]
    Status,

    /// Mostra estatísticas de moderação do grupo
    #[command(description = "Estatísticas / Stats / Estadísticas")]
    Stats,

    /// Altera o idioma do grupo
    #[command(description = "Idioma: pt|en|es / Language: pt|en|es")]
    Language(String),

    /// Recarrega config/moderation.toml sem reiniciar o bot
    #[command(description = "Recarrega moderation.toml (admin) / Reload config (admin)")]
    Reload,

    /// Remove o banimento de um usuário: /unban <user_id>
    #[command(description = "/unban <user_id> — remove banimento (admin)")]
    Unban(String),

    /// Bloqueia um domínio na hora: /blockdomain <dominio>
    #[command(description = "/blockdomain <dominio> — bloqueia domínio (admin)")]
    BlockDomain(String),

    /// Ensina o classificador bayesiano, em reply a uma mensagem,
    /// que ela é spam.
    #[command(description = "Reply a uma msg: ensina como spam (admin)")]
    TrainSpam,

    /// Ensina o classificador bayesiano, em reply a uma mensagem,
    /// que ela é legítima.
    #[command(description = "Reply a uma msg: ensina como legítima (admin)")]
    TrainHam,
}
