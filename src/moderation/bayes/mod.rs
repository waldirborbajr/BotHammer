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
//! Liga/desliga e limiar de confiança ficam em `[bayes]` no
//! `moderation.toml`, recarregáveis via `/reload` — só o modelo em si
//! (pesos treinados) não é reciclado em runtime nesta versão, ver
//! `TODO.md`.

mod classifier;
mod dataset;
mod vectorizer;

pub use classifier::SpamClassifier;
pub use vectorizer::token_count;

/// Treina o classificador padrão a partir do dataset embutido no
/// binário (`dataset::TRAINING_DATA`). Chamado uma única vez em
/// `AppState::new`.
pub fn build_default() -> Result<SpamClassifier, Box<dyn std::error::Error + Send + Sync>> {
    SpamClassifier::train(dataset::TRAINING_DATA)
}
