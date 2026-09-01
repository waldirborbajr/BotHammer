use linfa::dataset::Dataset;
use linfa::traits::Fit;
use linfa_bayes::{MultinomialNb, NaiveBayes};
use ndarray::{Array1, Axis};

use super::vectorizer::Vocabulary;

/// Quantidade máxima de termos mantidos no vocabulário do
/// bag-of-words. O dataset atual (`dataset::TRAINING_DATA`) gera bem
/// menos termos que isso — a cap existe para quando o corpus de
/// treino crescer (ver `TODO.md`).
const MAX_FEATURES: usize = 8_000;

/// Suavização de Laplace (aditiva) aplicada pelo Multinomial Naive
/// Bayes — evita probabilidade zero para termos que nunca apareceram
/// numa classe durante o treino. `1.0` é o padrão usual (add-one
/// smoothing).
const ALPHA: f64 = 1.0;

/// Classificador Naive Bayes (Multinomial) para spam, treinado uma
/// única vez no boot a partir do dataset embutido no binário.
///
/// Implementado sobre `linfa` + `linfa-bayes` — os pacotes nativos
/// do ecossistema Rust ML para esse algoritmo — em vez de um Naive
/// Bayes escrito à mão. `bayes::vectorizer::Vocabulary` cuida só da
/// etapa de bag-of-words (transformar texto em vetor de contagem),
/// que fica fora do escopo do `linfa-bayes` em si.
///
/// É só mais um SINAL para `engine::analyze_message` — o resultado
/// não vira violação sozinho: o corte de confiança (`[bayes].
/// threshold`) e o liga/desliga (`[bayes].enabled`) vivem em
/// `config/moderation.toml`, recarregáveis via `/reload` sem precisar
/// retreinar o modelo (o modelo em si só é treinado uma vez, no
/// boot — ver `AppState::new`).
pub struct SpamClassifier {
    vocabulary: Vocabulary,
    model: MultinomialNb<f64, bool>,
}

impl SpamClassifier {
    /// Treina o classificador a partir de um corpus rotulado.
    ///
    /// `examples` é uma lista de `(texto, é_spam)`. Chamado uma vez
    /// no boot (`AppState::new`) com `dataset::TRAINING_DATA`.
    pub fn train(
        examples: &[(&str, bool)],
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if examples.is_empty() {
            return Err("dataset de treino do classificador bayesiano está vazio".into());
        }

        let texts: Vec<&str> = examples.iter().map(|(text, _is_spam)| *text).collect();

        let vocabulary = Vocabulary::build(&texts, MAX_FEATURES);

        if vocabulary.is_empty() {
            return Err(
                "vocabulário do classificador bayesiano ficou vazio após tokenizar o dataset \
                 de treino"
                    .into(),
            );
        }

        let features = vocabulary.transform_batch(&texts);

        let targets: Array1<bool> = examples.iter().map(|(_text, is_spam)| *is_spam).collect();

        let dataset = Dataset::new(features, targets);

        let model = MultinomialNb::params()
            .alpha(ALPHA)
            .fit(&dataset)
            .map_err(|error| format!("falha ao treinar classificador bayesiano: {error}"))?;

        log::info!(
            "classificador bayesiano de spam treinado: {} exemplos, vocabulário de {} termos",
            examples.len(),
            vocabulary.len()
        );

        Ok(Self { vocabulary, model })
    }

    /// Probabilidade estimada (0.0–1.0) de `text` ser spam.
    ///
    /// Usa `predict_log_proba` (probabilidade posterior normalizada,
    /// em log) em vez do `predict` "cru" do `linfa` — assim dá pra
    /// aplicar um limiar de confiança (`[bayes].threshold` em
    /// `moderation.toml`) em vez de aceitar cegamente a classe de
    /// maior probabilidade, que numa mensagem curta e ambígua pode
    /// virar "spam" por uma margem mínima (ex.: 51% x 49%).
    pub fn spam_probability(&self, text: &str) -> f64 {
        let vector = self.vocabulary.transform(text);

        // predict_log_proba espera uma matriz (n_amostras, n_termos);
        // uma única mensagem vira uma matriz de 1 linha.
        let matrix = vector.insert_axis(Axis(0));

        let (log_proba, classes) = self.model.predict_log_proba(matrix.view());

        let Some(spam_column) = classes.iter().position(|is_spam| **is_spam) else {
            // Não deveria acontecer — a classe `true` (spam) sempre
            // existe no dataset de treino — mas não vale a pena
            // travar a moderação por isso; trata como "sem sinal".
            log::warn!("classe 'spam' não encontrada no modelo bayesiano treinado");

            return 0.0;
        };

        log_proba[[0, spam_column]].exp()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::moderation::bayes::dataset::TRAINING_DATA;

    #[test]
    fn classifies_obvious_spam_above_obvious_ham() {
        let classifier = SpamClassifier::train(TRAINING_DATA).expect("treino do classificador");

        let spam_score = classifier.spam_probability(
            "ganhe dinheiro fácil trabalhando de casa clique no link e comece hoje",
        );

        let ham_score = classifier
            .spam_probability("bom dia pessoal, alguém sabe que horas começa a reunião hoje");

        assert!(
            spam_score > ham_score,
            "esperava spam_score ({spam_score}) > ham_score ({ham_score})"
        );

        assert!(spam_score > 0.5, "spam_score={spam_score}");
        assert!(ham_score < 0.5, "ham_score={ham_score}");
    }

    #[test]
    fn rejects_empty_dataset() {
        let result = SpamClassifier::train(&[]);

        assert!(result.is_err());
    }
}
