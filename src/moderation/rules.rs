use serde::Deserialize;
use std::fmt;
use std::fs;

/// Caminho padrão do arquivo de regras de moderação.
pub const CONFIG_PATH: &str = "config/moderation.toml";

#[derive(Debug, Deserialize, Clone)]
pub struct ModerationRules {
    pub pornography: KeywordGroup,

    pub gambling: KeywordGroup,

    pub spam: KeywordGroup,

    pub links: LinkGroup,

    pub strikes: StrikesConfig,

    pub trust: TrustConfig,

    pub bayes: BayesConfig,
}

#[derive(Debug, Deserialize, Clone)]
pub struct KeywordGroup {
    pub keywords: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct LinkGroup {
    pub domains: Vec<String>,
}

/// Escada de punições para violações de baixa severidade
/// (gambling, spam). Ver `ViolationType::is_zero_tolerance`
/// para as categorias que ignoram essa escada e banem direto.
#[derive(Debug, Deserialize, Clone)]
pub struct StrikesConfig {
    pub window_days: i64,
    pub mute_at: u32,
    pub kick_at: u32,
    pub ban_at: u32,
    pub mute_duration_minutes: i64,
    pub kick_ban_seconds: i64,
}

/// Configuração da whitelist de usuários confiáveis.
///
/// Não desativa nem afrouxa detecção de `csam`, `pornography` ou
/// `suspicious_link` — essas categorias são zero tolerância e banem
/// direto independente de confiança (ver `ViolationType::is_zero_tolerance`
/// e `handlers::message_handler`). O efeito de "checagem mais branda"
/// se limita a multiplicar os limiares da escada de strikes
/// (`mute_at`/`kick_at`/`ban_at`) para gambling/spam, dando mais
/// margem antes de escalar a punição de um membro antigo e limpo.
#[derive(Debug, Deserialize, Clone)]
pub struct TrustConfig {
    /// Liga/desliga a whitelist sem precisar remover a seção do TOML.
    pub enabled: bool,

    /// Dias mínimos desde `first_seen` para o usuário ser elegível.
    pub min_days_in_group: i64,

    /// Violações (de qualquer tipo, no chat) toleradas no histórico
    /// total do usuário para ainda ser considerado "sem histórico".
    /// Normalmente 0 — qualquer violação já registrada remove a
    /// elegibilidade até o histórico "prescrever" (não há prescrição
    /// automática hoje: uma violação antiga continua contando).
    pub max_violations: i64,

    /// Fator de multiplicação aplicado a `mute_at`/`kick_at`/`ban_at`
    /// da `StrikesConfig` quando o usuário é confiável. `2` significa
    /// que o usuário confiável precisa de o dobro de violações
    /// recentes para sofrer a mesma punição que um usuário comum.
    pub strikes_multiplier: u32,
}

/// Configuração do classificador Naive Bayes de spam
/// (`moderation::bayes`).
///
/// O modelo em si (pesos treinados a partir de
/// `bayes::dataset::TRAINING_DATA`) é treinado uma única vez no boot
/// e não é afetado por `/reload` — só estes dois parâmetros de
/// política são recarregáveis em runtime junto do resto do
/// `moderation.toml`.
#[derive(Debug, Deserialize, Clone)]
pub struct BayesConfig {
    /// Liga/desliga o classificador sem precisar remover a seção do
    /// TOML nem recompilar o bot.
    pub enabled: bool,

    /// Probabilidade mínima (0.0–1.0, exclusivo de zero) de "spam"
    /// estimada pelo modelo para a mensagem virar
    /// `ViolationType::Spam`. Mais alto = menos falsos positivos,
    /// porém mais spam sutil passa batido.
    pub threshold: f64,

    /// Número mínimo de tokens (após normalização) que uma mensagem
    /// precisa ter para ser avaliada pelo classificador. Mensagens
    /// muito curtas ("oi", "kkkk", "👍") carregam pouquíssimo sinal
    /// estatístico e tendem a gerar falsos positivos.
    pub min_tokens: usize,
}

impl BayesConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        if !(self.threshold > 0.0 && self.threshold <= 1.0) {
            return Err(ValidationError {
                section: "bayes",
                reason: Some("threshold precisa estar entre 0.0 (exclusivo) e 1.0 (inclusivo)"),
            });
        }

        if self.min_tokens < 1 {
            return Err(ValidationError {
                section: "bayes",
                reason: Some("min_tokens precisa ser >= 1"),
            });
        }

        Ok(())
    }
}

/// Erro de validação de configuração de moderação.
#[derive(Debug)]
pub struct ValidationError {
    pub section: &'static str,
    pub reason: Option<&'static str>,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.reason {
            Some(reason) => write!(
                f,
                "moderation.toml: seção [{}] inválida — {}",
                self.section, reason
            ),

            None => write!(
                f,
                "moderation.toml: seção [{}] está vazia — o bot não pode iniciar sem regras de moderação carregadas",
                self.section
            ),
        }
    }
}

impl std::error::Error for ValidationError {}

impl ModerationRules {
    /// `Send + Sync` no tipo de erro é necessário porque este método
    /// é chamado dentro do handler `/reload`, e o dptree (teloxide)
    /// exige que o Future de cada endpoint seja `Send`. Um
    /// `Box<dyn Error>` comum não é `Send` e quebra a injeção de
    /// dependência do dispatcher (erro `Injectable<...>` no build).
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let content = fs::read_to_string(path)?;

        let rules = toml::from_str::<ModerationRules>(&content)?;

        rules.validate()?;

        Ok(rules)
    }

    /// Garante que nenhuma categoria de moderação
    /// foi carregada vazia por engano (config incompleta,
    /// arquivo corrompido, edição malfeita, etc), e que a
    /// escada de strikes está configurada de forma coerente.
    fn validate(&self) -> Result<(), ValidationError> {
        if self.pornography.keywords.is_empty() {
            return Err(ValidationError {
                section: "pornography",
                reason: None,
            });
        }

        if self.gambling.keywords.is_empty() {
            return Err(ValidationError {
                section: "gambling",
                reason: None,
            });
        }

        if self.spam.keywords.is_empty() {
            return Err(ValidationError {
                section: "spam",
                reason: None,
            });
        }

        if self.links.domains.is_empty() {
            return Err(ValidationError {
                section: "links",
                reason: None,
            });
        }

        self.strikes.validate()?;

        self.trust.validate()?;

        self.bayes.validate()?;

        Ok(())
    }
}

impl TrustConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        if self.min_days_in_group < 0 {
            return Err(ValidationError {
                section: "trust",
                reason: Some("min_days_in_group precisa ser >= 0"),
            });
        }

        if self.max_violations < 0 {
            return Err(ValidationError {
                section: "trust",
                reason: Some("max_violations precisa ser >= 0"),
            });
        }

        if self.strikes_multiplier < 1 {
            return Err(ValidationError {
                section: "trust",
                reason: Some("strikes_multiplier precisa ser >= 1"),
            });
        }

        Ok(())
    }
}

impl StrikesConfig {
    fn validate(&self) -> Result<(), ValidationError> {
        if !(1 <= self.mute_at && self.mute_at < self.kick_at && self.kick_at < self.ban_at) {
            return Err(ValidationError {
                section: "strikes",
                reason: Some("mute_at, kick_at e ban_at precisam ser crescentes e >= 1"),
            });
        }

        if self.window_days < 1 {
            return Err(ValidationError {
                section: "strikes",
                reason: Some("window_days precisa ser >= 1"),
            });
        }

        if self.mute_duration_minutes < 1 {
            return Err(ValidationError {
                section: "strikes",
                reason: Some("mute_duration_minutes precisa ser >= 1"),
            });
        }

        // Telegram considera bans com until_date a menos de 30s no
        // futuro como permanentes — abaixo de 31 o "kick" viraria ban.
        if self.kick_ban_seconds < 31 {
            return Err(ValidationError {
                section: "strikes",
                reason: Some(
                    "kick_ban_seconds precisa ser >= 31 (o Telegram trata valores menores como ban permanente)",
                ),
            });
        }

        Ok(())
    }

    /// Retorna uma cópia com `mute_at`/`kick_at`/`ban_at` multiplicados
    /// por `multiplier` — usada para dar mais margem a usuários da
    /// whitelist de confiança antes de escalar a punição. As demais
    /// propriedades (janela, duração de mute/kick) permanecem iguais.
    pub fn scaled(&self, multiplier: u32) -> Self {
        Self {
            mute_at: self.mute_at.saturating_mul(multiplier),
            kick_at: self.kick_at.saturating_mul(multiplier),
            ban_at: self.ban_at.saturating_mul(multiplier),
            ..self.clone()
        }
    }
}
