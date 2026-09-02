use linfa::dataset::Dataset;
use linfa::traits::Fit;
use linfa_bayes::{MultinomialNb, NaiveBayes};
use ndarray::{Array1, Axis};

use super::vectorizer::Vocabulary;

/// Classificador Naive Bayes (Multinomial) para spam.
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
/// `config/moderation.toml`, recarregáveis via `/reload`.
///
/// O modelo em si é treinado a partir de `bayes::seed_examples()`
/// (dataset semente embutido no binário) somado aos exemplos
/// ensinados por admins via `/trainspam`/`/trainham`, persistidos em
/// `storage::sqlite` (tabela `bayes_training_examples`). `AppState`
/// retreina o modelo no boot, em todo `/reload` e a cada exemplo
/// novo — ver `core::state::AppState::train_bayes`.
pub struct SpamClassifier {
    vocabulary: Vocabulary,
    model: MultinomialNb<f64, bool>,
}

impl SpamClassifier {
    /// Treina o classificador a partir de um corpus rotulado.
    ///
    /// `examples` é uma lista de `(texto, é_spam)`. `alpha` é a
    /// suavização de Laplace (aditiva) do Multinomial Naive Bayes —
    /// `1.0` é o padrão usual (add-one smoothing). `max_features` é
    /// o número máximo de termos mantidos no vocabulário
    /// bag-of-words — os mais frequentes no corpus são priorizados.
    /// Ambos vêm de `[bayes]` em `moderation.toml`
    /// (`BayesConfig::alpha`/`BayesConfig::max_features`).
    pub fn train(
        examples: &[(String, bool)],
        alpha: f64,
        max_features: usize,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        if examples.is_empty() {
            return Err("dataset de treino do classificador bayesiano está vazio".into());
        }

        let texts: Vec<&str> = examples
            .iter()
            .map(|(text, _is_spam)| text.as_str())
            .collect();

        let vocabulary = Vocabulary::build(&texts, max_features);

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
            .alpha(alpha)
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

    fn owned_seed() -> Vec<(String, bool)> {
        TRAINING_DATA
            .iter()
            .map(|(text, is_spam)| (text.to_string(), *is_spam))
            .collect()
    }

    #[test]
    fn classifies_obvious_spam_above_obvious_ham() {
        let examples = owned_seed();

        let classifier =
            SpamClassifier::train(&examples, 1.0, 8_000).expect("treino do classificador");

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
        let result = SpamClassifier::train(&[], 1.0, 8_000);

        assert!(result.is_err());
    }

    #[test]
    fn learns_from_extra_examples() {
        let mut examples = owned_seed();

        // Termo que não existe em nenhum exemplo semente — sem essa
        // linha extra, "xylozorp" cairia fora do vocabulário e o
        // classificador não teria como reagir a ele.
        examples.push((
            "xylozorp promoção exclusiva clique agora mesmo".to_string(),
            true,
        ));

        let classifier =
            SpamClassifier::train(&examples, 1.0, 8_000).expect("treino do classificador");

        let score = classifier.spam_probability("xylozorp promoção exclusiva");

        assert!(score > 0.5, "score={score}");
    }
}
