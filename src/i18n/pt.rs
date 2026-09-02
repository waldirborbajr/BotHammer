pub fn help() -> String {
    format!(
        r#"🤖 *BotHammer v{}*

Detecta automaticamente:
- Pornografia
- Vendas / spam
- Apostas
- Pedofilia / CSAM
- Links suspeitos

Comandos:
/help — esta mensagem
/status — status do bot
/stats — estatísticas de moderação do grupo
/language <pt|en|es> — define o idioma do bot neste grupo (apenas administradores)
/reload — recarrega moderation.toml sem reiniciar o bot (apenas administradores)
/unban <user_id> — remove o banimento de um usuário (apenas administradores)
/blockdomain <dominio> — bloqueia um domínio na hora (apenas administradores)
/trainspam — em reply a uma mensagem, ensina como spam o classificador bayesiano (apenas administradores)
/trainham — em reply a uma mensagem, ensina como legítima o classificador bayesiano (apenas administradores)"#,
        env!("CARGO_PKG_VERSION")
    )
}

pub const STATUS: &str = "✅ BotHammer está online e protegendo o grupo!";

pub const VIOLATION_GENERIC: &str = "🚫 Conteúdo proibido detectado e removido.";

pub fn banned(username: &str) -> String {
    format!("🚫 @{username} foi banido por violação das regras.")
}

pub fn warned(username: &str, count: i64) -> String {
    format!(
        "⚠️ @{username}, sua mensagem foi removida por violar as regras do grupo. \
         Aviso {count} — violações repetidas resultam em silenciamento e depois banimento."
    )
}

pub fn muted(username: &str, minutes: i64) -> String {
    format!(
        "🔇 @{username} foi silenciado por {minutes} minuto(s) após violações repetidas das regras."
    )
}

pub fn kicked(username: &str) -> String {
    format!(
        "👢 @{username} foi removido do grupo por violações repetidas. \
         Pode entrar novamente, mas a próxima violação resulta em banimento."
    )
}

pub const LANG_SET: &str = "✅ Idioma do bot definido para Português.";

pub const LANG_INVALID: &str =
    "⚠️ Idioma inválido. Use `/language pt`, `/language en` ou `/language es`.";

pub const LANG_NO_PERMISSION: &str = "⚠️ Apenas administradores podem alterar o idioma do bot.";

pub const STATS_TITLE: &str = "📊 *Estatísticas do grupo*";
pub const STATS_TOTAL: &str = "Violações totais";
pub const STATS_24H: &str = "Últimas 24h";
pub const STATS_BY_TYPE: &str = "Por categoria";
pub const STATS_TOP: &str = "Top infratores";
pub const STATS_EMPTY: &str = "✅ Nenhuma violação registrada neste grupo ainda.";
pub const STATS_BAYES_LEARNED: &str = "Exemplos ensinados ao classificador bayesiano";

pub const RELOAD_SUCCESS: &str = "✅ Configuração de moderação recarregada com sucesso.";
pub const RELOAD_ERROR: &str = "⚠️ Falha ao recarregar moderation.toml. As regras antigas continuam ativas. Veja os logs do bot.";
pub const RELOAD_NO_PERMISSION: &str = "⚠️ Apenas administradores podem recarregar a configuração.";

pub const UNBAN_NO_PERMISSION: &str = "⚠️ Apenas administradores podem remover banimentos.";
pub const UNBAN_INVALID_ID: &str = "⚠️ Uso: `/unban <user_id>` — o ID precisa ser numérico.";

pub fn unban_success(user_id: u64) -> String {
    format!("✅ Usuário `{user_id}` foi desbanido.")
}

pub const UNBAN_ERROR: &str = "⚠️ Falha ao desbanir o usuário. Verifique se o ID está correto e se o bot tem permissão de admin.";

#[allow(non_snake_case)]
pub fn BLOCKDOMAIN_SUCCESS(domain: &str) -> String {
    format!("✅ Domínio `{domain}` bloqueado com sucesso.")
}

pub const BLOCKDOMAIN_NO_PERMISSION: &str = "⚠️ Apenas administradores podem bloquear domínios.";
pub const BLOCKDOMAIN_INVALID: &str =
    "⚠️ Uso: `/blockdomain <dominio>` (ex: /blockdomain spam-site.com).";
pub const BLOCKDOMAIN_ERROR: &str = "⚠️ Falha ao bloquear o domínio. Veja os logs do bot.";

pub const TRAIN_NO_PERMISSION: &str =
    "⚠️ Apenas administradores podem ensinar o classificador de spam.";
pub const TRAIN_MISSING_REPLY: &str =
    "⚠️ Use `/trainspam` ou `/trainham` em *reply* a uma mensagem de texto.";
pub const TRAIN_ERROR: &str =
    "⚠️ Falha ao registrar o exemplo de treino. Veja os logs do bot.";

pub fn train_success(is_spam: bool) -> String {
    if is_spam {
        "✅ Mensagem registrada como *spam*. O classificador foi retreinado agora.".to_string()
    } else {
        "✅ Mensagem registrada como *legítima*. O classificador foi retreinado agora."
            .to_string()
    }
}
