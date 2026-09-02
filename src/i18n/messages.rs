use super::{en, es, lang::Lang, pt};

/// Mensagem de ajuda
pub fn help(lang: Lang) -> String {
    match lang {
        Lang::Pt => pt::help(),
        Lang::En => en::help(),
        Lang::Es => es::help(),
    }
}

/// Status do bot
pub fn status(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::STATUS,
        Lang::En => en::STATUS,
        Lang::Es => es::STATUS,
    }
}

/// Usuário banido
pub fn banned(lang: Lang, username: &str) -> String {
    match lang {
        Lang::Pt => pt::banned(username),
        Lang::En => en::banned(username),
        Lang::Es => es::banned(username),
    }
}

/// Usuário avisado (1ª violação de baixa severidade)
pub fn warned(lang: Lang, username: &str, count: i64) -> String {
    match lang {
        Lang::Pt => pt::warned(username, count),
        Lang::En => en::warned(username, count),
        Lang::Es => es::warned(username, count),
    }
}

/// Usuário silenciado (violação de baixa severidade recorrente)
pub fn muted(lang: Lang, username: &str, minutes: i64) -> String {
    match lang {
        Lang::Pt => pt::muted(username, minutes),
        Lang::En => en::muted(username, minutes),
        Lang::Es => es::muted(username, minutes),
    }
}

/// Usuário removido do grupo (kick, sem ban permanente)
pub fn kicked(lang: Lang, username: &str) -> String {
    match lang {
        Lang::Pt => pt::kicked(username),
        Lang::En => en::kicked(username),
        Lang::Es => es::kicked(username),
    }
}

/// Violação genérica
pub fn violation_generic(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::VIOLATION_GENERIC,
        Lang::En => en::VIOLATION_GENERIC,
        Lang::Es => es::VIOLATION_GENERIC,
    }
}

/// Idioma alterado
pub fn lang_set(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::LANG_SET,
        Lang::En => en::LANG_SET,
        Lang::Es => es::LANG_SET,
    }
}

/// Idioma inválido
pub fn lang_invalid(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::LANG_INVALID,
        Lang::En => en::LANG_INVALID,
        Lang::Es => es::LANG_INVALID,
    }
}

/// Sem permissão para alterar idioma
pub fn lang_no_permission(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::LANG_NO_PERMISSION,
        Lang::En => en::LANG_NO_PERMISSION,
        Lang::Es => es::LANG_NO_PERMISSION,
    }
}

/// Rótulos usados para montar a mensagem de /stats
pub struct StatsLabels {
    pub title: &'static str,
    pub total: &'static str,
    pub last_24h: &'static str,
    pub by_type: &'static str,
    pub top: &'static str,
    pub empty: &'static str,
    pub bayes_learned: &'static str,
}

/// Rótulos de estatísticas no idioma do chat
pub fn stats_labels(lang: Lang) -> StatsLabels {
    match lang {
        Lang::Pt => StatsLabels {
            title: pt::STATS_TITLE,
            total: pt::STATS_TOTAL,
            last_24h: pt::STATS_24H,
            by_type: pt::STATS_BY_TYPE,
            top: pt::STATS_TOP,
            empty: pt::STATS_EMPTY,
            bayes_learned: pt::STATS_BAYES_LEARNED,
        },

        Lang::En => StatsLabels {
            title: en::STATS_TITLE,
            total: en::STATS_TOTAL,
            last_24h: en::STATS_24H,
            by_type: en::STATS_BY_TYPE,
            top: en::STATS_TOP,
            empty: en::STATS_EMPTY,
            bayes_learned: en::STATS_BAYES_LEARNED,
        },

        Lang::Es => StatsLabels {
            title: es::STATS_TITLE,
            total: es::STATS_TOTAL,
            last_24h: es::STATS_24H,
            by_type: es::STATS_BY_TYPE,
            top: es::STATS_TOP,
            empty: es::STATS_EMPTY,
            bayes_learned: es::STATS_BAYES_LEARNED,
        },
    }
}

/// Config recarregada com sucesso
pub fn reload_success(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::RELOAD_SUCCESS,
        Lang::En => en::RELOAD_SUCCESS,
        Lang::Es => es::RELOAD_SUCCESS,
    }
}

/// Falha ao recarregar config
pub fn reload_error(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::RELOAD_ERROR,
        Lang::En => en::RELOAD_ERROR,
        Lang::Es => es::RELOAD_ERROR,
    }
}

/// Sem permissão para recarregar config
pub fn reload_no_permission(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::RELOAD_NO_PERMISSION,
        Lang::En => en::RELOAD_NO_PERMISSION,
        Lang::Es => es::RELOAD_NO_PERMISSION,
    }
}

/// Sem permissão para desbanir
pub fn unban_no_permission(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::UNBAN_NO_PERMISSION,
        Lang::En => en::UNBAN_NO_PERMISSION,
        Lang::Es => es::UNBAN_NO_PERMISSION,
    }
}

/// Argumento de /unban inválido (não é um user_id numérico)
pub fn unban_invalid_id(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::UNBAN_INVALID_ID,
        Lang::En => en::UNBAN_INVALID_ID,
        Lang::Es => es::UNBAN_INVALID_ID,
    }
}

/// Usuário desbanido com sucesso
pub fn unban_success(lang: Lang, user_id: u64) -> String {
    match lang {
        Lang::Pt => pt::unban_success(user_id),
        Lang::En => en::unban_success(user_id),
        Lang::Es => es::unban_success(user_id),
    }
}

/// Falha ao desbanir
pub fn unban_error(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::UNBAN_ERROR,
        Lang::En => en::UNBAN_ERROR,
        Lang::Es => es::UNBAN_ERROR,
    }
}

/// Sem permissão para bloquear domínio
pub fn blockdomain_no_permission(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::BLOCKDOMAIN_NO_PERMISSION,
        Lang::En => en::BLOCKDOMAIN_NO_PERMISSION,
        Lang::Es => es::BLOCKDOMAIN_NO_PERMISSION,
    }
}

/// Argumento de /blockdomain inválido (vazio)
pub fn blockdomain_invalid(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::BLOCKDOMAIN_INVALID,
        Lang::En => en::BLOCKDOMAIN_INVALID,
        Lang::Es => es::BLOCKDOMAIN_INVALID,
    }
}

/// Domínio bloqueado com sucesso
pub fn blockdomain_success(lang: Lang, domain: &str) -> String {
    match lang {
        Lang::Pt => pt::BLOCKDOMAIN_SUCCESS(domain),
        Lang::En => en::BLOCKDOMAIN_SUCCESS(domain),
        Lang::Es => es::BLOCKDOMAIN_SUCCESS(domain),
    }
}

/// Falha ao bloquear domínio
pub fn blockdomain_error(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::BLOCKDOMAIN_ERROR,
        Lang::En => en::BLOCKDOMAIN_ERROR,
        Lang::Es => es::BLOCKDOMAIN_ERROR,
    }
}

/// Sem permissão para ensinar o classificador bayesiano
/// (`/trainspam`/`/trainham`)
pub fn train_no_permission(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::TRAIN_NO_PERMISSION,
        Lang::En => en::TRAIN_NO_PERMISSION,
        Lang::Es => es::TRAIN_NO_PERMISSION,
    }
}

/// `/trainspam`/`/trainham` usado sem ser em reply a uma mensagem de
/// texto
pub fn train_missing_reply(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::TRAIN_MISSING_REPLY,
        Lang::En => en::TRAIN_MISSING_REPLY,
        Lang::Es => es::TRAIN_MISSING_REPLY,
    }
}

/// Exemplo de treino registrado com sucesso (`is_spam` decide a
/// mensagem: rotulado como spam ou como legítimo)
pub fn train_success(lang: Lang, is_spam: bool) -> String {
    match lang {
        Lang::Pt => pt::train_success(is_spam),
        Lang::En => en::train_success(is_spam),
        Lang::Es => es::train_success(is_spam),
    }
}

/// Falha ao registrar exemplo de treino
pub fn train_error(lang: Lang) -> &'static str {
    match lang {
        Lang::Pt => pt::TRAIN_ERROR,
        Lang::En => en::TRAIN_ERROR,
        Lang::Es => es::TRAIN_ERROR,
    }
}
