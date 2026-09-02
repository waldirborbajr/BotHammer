//! Classificador Naive Bayes (Multinomial) para spam.
//!
//! Complementa a detecção por palavra-chave já existente
//! (`spam::is_spam`) com um sinal probabilístico: mensagens que
//! nunca batem literalmente com uma keyword de `moderation.toml` mas
//! têm o "jeito" estatístico de spam (mesma distribuição de termos
//! do dataset de treino) também podem ser pegas.
//!
//! Construído sobre os pacotes nativos `linfa`/`linfa-bayes` (ver
//! `classifier::SpamClassifier`), não uma implementação artesanal de
//! Naive Bayes.
//!
//! O dataset de treino é a combinação de duas fontes:
//! - `seed_examples()`: um corpus semente pequeno, embutido no
//!   binário (`dataset::TRAINING_DATA`) — garante que o classificador
//!   funcione desde o primeiro boot, sem configuração nenhuma.
//! - Exemplos ensinados por admins via `/trainspam`/`/trainham`,
//!   persistidos em SQLite (`storage::sqlite::bayes_training_examples`)
//!   — é assim que o modelo melhora com o uso real de cada grupo,
//!   sem precisar editar código nem recompilar.
//!
//! Quem junta as duas fontes e treina o modelo é
//! `core::state::AppState::train_bayes` — no boot, em todo `/reload`
//! e a cada exemplo novo ensinado. Liga/desliga, limiar de confiança
//! e hiperparâmetros (`alpha`, `max_features`) ficam em `[bayes]` no
//! `moderation.toml`.

mod classifier;
mod dataset;
mod vectorizer;

pub use classifier::SpamClassifier;
pub use vectorizer::token_count;

/// Cópia (owned) do dataset semente embutido no binário — ponto de
/// partida sempre presente no treino, complementado pelos exemplos
/// persistidos em SQLite. Ver documentação do módulo.
pub fn seed_examples() -> Vec<(String, bool)> {
    dataset::TRAINING_DATA
        .iter()
        .map(|(text, is_spam)| (text.to_string(), *is_spam))
        .collect()
}
