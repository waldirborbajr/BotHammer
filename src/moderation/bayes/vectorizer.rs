use std::collections::HashMap;

use ndarray::{Array1, Array2};

use crate::moderation::regex::normalize_text;

/// Vocabulário fixo (bag-of-words) usado para transformar texto em
/// vetores de contagem de termos, consumidos pelo classificador
/// Naive Bayes (`linfa_bayes::MultinomialNb`, em `bayes::classifier`).
///
/// É construído uma única vez a partir do dataset de treino embutido
/// no binário (`bayes::dataset::TRAINING_DATA`) e não muda em
/// runtime — não há re-treino via `/reload` nesta versão (ver
/// `TODO.md`).
#[derive(Debug, Clone)]
pub struct Vocabulary {
    index: HashMap<String, usize>,
}

impl Vocabulary {
    /// Constrói o vocabulário a partir de um corpus de treino,
    /// mantendo no máximo `max_features` termos — os mais frequentes
    /// no corpus, com empate resolvido em ordem alfabética para dar
    /// um resultado determinístico entre execuções (não depende da
    /// ordem de iteração do HashMap).
    pub fn build(corpus: &[&str], max_features: usize) -> Self {
        let mut frequency: HashMap<String, usize> = HashMap::new();

        for text in corpus {
            for token in tokenize(text) {
                *frequency.entry(token).or_insert(0) += 1;
            }
        }

        let mut terms: Vec<(String, usize)> = frequency.into_iter().collect();

        terms.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));

        terms.truncate(max_features);

        let index = terms
            .into_iter()
            .enumerate()
            .map(|(position, (term, _count))| (term, position))
            .collect();

        Self { index }
    }

    /// Número de termos no vocabulário — dimensão dos vetores
    /// gerados por `transform`/`transform_batch`.
    pub fn len(&self) -> usize {
        self.index.len()
    }

    pub fn is_empty(&self) -> bool {
        self.index.is_empty()
    }

    /// Transforma um único texto em um vetor de contagem de termos
    /// (bag-of-words), na dimensão do vocabulário. Termos fora do
    /// vocabulário (OOV) são ignorados — comportamento padrão de um
    /// `CountVectorizer` em produção, onde o texto de entrada nunca
    /// bate 100% com o corpus de treino.
    pub fn transform(&self, text: &str) -> Array1<f64> {
        let mut vector = Array1::<f64>::zeros(self.index.len());

        for token in tokenize(text) {
            if let Some(&position) = self.index.get(&token) {
                vector[position] += 1.0;
            }
        }

        vector
    }

    /// Transforma um lote de textos em uma matriz `(n_amostras,
    /// n_termos)`, usada para montar o `Dataset` de treino do
    /// `MultinomialNb`.
    pub fn transform_batch(&self, texts: &[&str]) -> Array2<f64> {
        let mut matrix = Array2::<f64>::zeros((texts.len(), self.index.len()));

        for (row, text) in texts.iter().enumerate() {
            for token in tokenize(text) {
                if let Some(&column) = self.index.get(&token) {
                    matrix[[row, column]] += 1.0;
                }
            }
        }

        matrix
    }
}

/// Tokeniza texto em palavras de pelo menos 2 caracteres.
///
/// Reaproveita `moderation::regex::normalize_text` — a mesma
/// normalização usada pelos demais detectores (lowercase, remoção de
/// caracteres especiais, espaços colapsados) — para que o vocabulário
/// do classificador enxergue o texto do mesmo jeito que
/// `spam::is_spam`/`gambling::is_gambling`/etc. `normalize_text` é
/// idempotente, então tokenizar um texto que `engine.rs` já
/// normalizou não muda o resultado.
fn tokenize(text: &str) -> Vec<String> {
    normalize_text(text)
        .split_whitespace()
        .filter(|token| token.chars().count() >= 2)
        .map(|token| token.to_string())
        .collect()
}

/// Conta quantos tokens um texto (já normalizado ou não) produziria
/// após a tokenização — usada por `engine.rs` para decidir se a
/// mensagem é longa o suficiente pra valer a pena passar pelo
/// classificador bayesiano (`[bayes].min_tokens` em
/// `moderation.toml`). Mensagens muito curtas ("oi", "kkkk") têm
/// poucochíssimo sinal estatístico e tendem a gerar falsos positivos.
pub fn token_count(text: &str) -> usize {
    tokenize(text).len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ignores_out_of_vocabulary_terms() {
        let vocabulary = Vocabulary::build(&["ganhe dinheiro fácil agora"], 100);

        let vector = vocabulary.transform("ganhe dinheiro fácil agora com palavras nunca vistas");

        assert_eq!(vector.sum(), 4.0);
    }

    #[test]
    fn respects_max_features_cap() {
        let vocabulary = Vocabulary::build(&["um dois tres quatro cinco"], 3);

        assert_eq!(vocabulary.len(), 3);
    }

    #[test]
    fn token_count_ignores_single_letter_noise() {
        assert_eq!(token_count("e a o"), 0);
        assert_eq!(token_count("bom dia pessoal"), 3);
    }
}
