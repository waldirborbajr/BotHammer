use crate::{core::state::AppState, telegram::events::TelegramEvent};

use super::{bayes, csam, gambling, links, pornography, regex::normalize_text, spam};

pub use super::violation::ViolationType;

/// Analisa uma mensagem usando:
///
/// - Detectores fixos
/// - Regras configuráveis
/// - Contexto do evento Telegram
///
/// Retorna o tipo da violação encontrada.
pub async fn analyze_message(
    text: &str,
    event: &TelegramEvent,
    state: &AppState,
) -> Option<ViolationType> {
    if text.trim().is_empty() {
        return None;
    }

    let normalized = normalize_text(text);

    //
    // Prioridade máxima:
    // sempre ativo e independente de configuração.
    //
    if csam::is_csam(&normalized) {
        return Some(ViolationType::Csam);
    }

    let rules = state.moderation.read().await;

    if pornography::is_pornography(&normalized, &rules.pornography.keywords) {
        return Some(ViolationType::Pornography);
    }

    if gambling::is_gambling(&normalized, &rules.gambling.keywords) {
        return Some(ViolationType::Gambling);
    }

    if links::is_suspicious_link(&normalized, &rules.links.domains) {
        return Some(ViolationType::SuspiciousLink);
    }

    if spam::is_spam(&normalized, &rules.spam.keywords) {
        return Some(ViolationType::Spam);
    }

    //
    // Sinal probabilístico (Naive Bayes), complementar às keywords
    // acima: pega spam que nunca bate literalmente com uma keyword
    // de moderation.toml mas tem a "cara" estatística do dataset de
    // treino. Mensagens curtas demais (`min_tokens`) são ignoradas —
    // pouco texto não dá sinal suficiente pro modelo.
    //
    if rules.bayes.enabled && bayes::token_count(&normalized) >= rules.bayes.min_tokens {
        let spam_probability = state.bayes.read().await.spam_probability(&normalized);

        if spam_probability >= rules.bayes.threshold {
            log::debug!(
                "classificador bayesiano marcou mensagem como spam (probabilidade {:.3})",
                spam_probability
            );

            return Some(ViolationType::Spam);
        }
    }

    drop(rules);

    //
    // Domínios adicionados dinamicamente.
    //
    let blocked_domains = state.blocked_domains.read().await;

    if links::is_suspicious_link(&normalized, &blocked_domains) {
        return Some(ViolationType::SuspiciousLink);
    }

    drop(blocked_domains);

    //
    // Heurística:
    // mensagens encaminhadas muito longas
    // possuem comportamento típico de spam.
    //
    if event.is_forwarded() && normalized.len() > 20 {
        return Some(ViolationType::Spam);
    }

    None
}

/// Apenas verifica existência de violação.
/// Apenas verifica existência de violação.
// TODO: usar em testes de integração do motor de moderação.
#[allow(dead_code)]
pub async fn is_violation(text: &str, event: &TelegramEvent, state: &AppState) -> bool {
    analyze_message(text, event, state).await.is_some()
}
