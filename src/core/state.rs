use std::sync::Arc;

use sqlx::SqlitePool;
use tokio::sync::RwLock;

use crate::{
    core::config::Config,
    moderation::{
        bayes::{self, SpamClassifier},
        rules::{self, BayesConfig, ModerationRules},
    },
    storage::{memory::MemoryStorage, sqlite},
};

/// Estado compartilhado da aplicação.
///
/// Esta estrutura é clonada pelos handlers do Telegram.
/// Recursos compartilhados ficam protegidos por `Arc` para
/// evitar cópias desnecessárias e permitir acesso concorrente.
#[derive(Clone)]
pub struct AppState {
    /// Configuração carregada na inicialização.
    pub config: Config,

    /// Estado temporário em memória.
    pub memory: Arc<MemoryStorage>,

    /// Regras de moderação carregadas de
    /// `config/moderation.toml`.
    ///
    /// Podem ser recarregadas em runtime através
    /// do comando `/reload`.
    pub moderation: Arc<RwLock<ModerationRules>>,

    /// Classificador Naive Bayes (Multinomial) de spam
    /// (`moderation::bayes`).
    ///
    /// Treinado a partir de `bayes::seed_examples()` (dataset
    /// semente embutido no binário) somado aos exemplos ensinados
    /// por admins via `/trainspam`/`/trainham`, persistidos em
    /// SQLite. Fica atrás de um `RwLock` porque, diferente da
    /// versão anterior, o modelo *é* retreinado em runtime: no
    /// boot, em todo `/reload` (que também recarrega `alpha`/
    /// `max_features` de `moderation.toml`) e a cada exemplo novo
    /// ensinado (`add_training_example`). Ver `train_bayes`.
    pub bayes: Arc<RwLock<SpamClassifier>>,

    /// Lista de domínios bloqueados.
    ///
    /// É carregada do SQLite durante o boot e mantida
    /// em memória para evitar consultas ao banco a
    /// cada mensagem processada.
    pub blocked_domains: Arc<RwLock<Vec<String>>>,

    /// Pool de conexões SQLite.
    pub db: SqlitePool,
}

impl AppState {
    /// Inicializa todo o estado da aplicação.
    pub async fn new(config: Config) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let moderation = ModerationRules::load(rules::CONFIG_PATH)?;

        let db = sqlite::init_database(&config.database_url).await?;

        // Treina o classificador bayesiano de spam a partir do
        // dataset semente embutido no binário + qualquer exemplo já
        // ensinado via /trainspam//trainham em execuções anteriores
        // (persistido em SQLite, sobrevive a reinícios). Falha no
        // boot (igual a uma moderation.toml inválida) se o dataset
        // estiver vazio ou o treino do linfa-bayes falhar — um
        // classificador quebrado não deveria subir silenciosamente.
        let bayes = Self::train_bayes(&db, &moderation.bayes).await?;

        let blocked_domains = sqlite::get_blocked_domains(&db).await?;

        Ok(Self {
            config,

            memory: Arc::new(MemoryStorage::new()),

            moderation: Arc::new(RwLock::new(moderation)),

            bayes: Arc::new(RwLock::new(bayes)),

            blocked_domains: Arc::new(RwLock::new(blocked_domains)),

            db,
        })
    }

    /// Treina um classificador bayesiano de spam a partir da união
    /// de `bayes::seed_examples()` (dataset semente embutido no
    /// binário) com os exemplos ensinados via `/trainspam`/
    /// `/trainham` e persistidos em SQLite.
    ///
    /// Função associada (não depende de `&self`) porque é chamada
    /// tanto em `new` — antes de `Self` existir — quanto em
    /// `reload_moderation`/`add_training_example`.
    async fn train_bayes(
        db: &SqlitePool,
        config: &BayesConfig,
    ) -> Result<SpamClassifier, Box<dyn std::error::Error + Send + Sync>> {
        let mut examples = bayes::seed_examples();

        let learned = sqlite::get_training_examples(db).await?;

        examples.extend(learned);

        SpamClassifier::train(&examples, config.alpha, config.max_features)
    }

    /// Recarrega `config/moderation.toml` sem reiniciar o processo,
    /// e retreina o classificador bayesiano de spam com os
    /// hiperparâmetros (`alpha`/`max_features`) e o dataset
    /// (semente + exemplos ensinados) atuais.
    ///
    /// Em caso de erro, as regras e o classificador atualmente
    /// carregados permanecem válidos — a troca só acontece depois
    /// que o retreino dá certo.
    pub async fn reload_moderation(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let fresh = ModerationRules::load(rules::CONFIG_PATH)?;

        let retrained = Self::train_bayes(&self.db, &fresh.bayes).await?;

        let mut moderation_guard = self.moderation.write().await;

        *moderation_guard = fresh;

        drop(moderation_guard);

        let mut bayes_guard = self.bayes.write().await;

        *bayes_guard = retrained;

        Ok(())
    }

    /// Ensina o classificador bayesiano de spam com mais um exemplo
    /// rotulado (`/trainspam`/`/trainham`, em reply a uma mensagem
    /// real do grupo): persiste em SQLite e retreina o modelo na
    /// hora, sem precisar de um `/reload` manual depois.
    ///
    /// Como o retreino é rápido nessa escala de dataset (dezenas a
    /// poucos milhares de exemplos), fazer isso a cada exemplo novo
    /// é mais simples — e dá feedback imediato pro admin — do que
    /// enfileirar um retreino em lote.
    pub async fn add_training_example(
        &self,
        text: &str,
        is_spam: bool,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        sqlite::insert_training_example(&self.db, text, is_spam).await?;

        let config = self.moderation.read().await.bayes.clone();

        let retrained = Self::train_bayes(&self.db, &config).await?;

        let mut guard = self.bayes.write().await;

        *guard = retrained;

        Ok(())
    }

    /// Adiciona um domínio à lista de bloqueio.
    ///
    /// A alteração é persistida no SQLite e refletida
    /// imediatamente na cópia mantida em memória.
    pub async fn add_blocked_domain(&self, domain: &str) -> Result<(), sqlx::Error> {
        let normalized = domain.trim().to_lowercase();

        sqlite::add_blocked_domain(&self.db, &normalized).await?;

        let mut guard = self.blocked_domains.write().await;

        if !guard.iter().any(|existing| existing == &normalized) {
            guard.push(normalized);
        }

        Ok(())
    }
}
