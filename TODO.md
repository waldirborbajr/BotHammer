# TODO — BanHammer

Roadmap de features sugeridas, organizadas por área. Não estão em ordem de prioridade fixa — cada item tem uma nota de esforço/impacto pra ajudar a decidir por onde começar.

---

## Governança e ações graduais

- [x] **Sistema de strikes configurável** (aviso → mute → kick → ban), em vez de ban direto para toda violação.
  Implementado: `[strikes]` em `moderation.toml` (`window_days`, `mute_at`, `kick_at`, `ban_at`, `mute_duration_minutes`, `kick_ban_seconds`, validado no boot e reload). Contagem via `sqlite::count_recent_violations` (persistida, escopada por chat+usuário — corrige de brinde o bug do contador antigo não ser por chat). `moderation::strikes::resolve_action` decide a ação; `handlers.rs` aplica via `restrict_chat_member`/`ban_chat_member(...).until_date(...)`.
  _Esforço: médio · Impacto: alto_
  → **Assumido:** `csam`, `pornography` e `suspicious_link` continuam com ban direto (zero tolerância) — só `gambling`/`spam` passam pela escada. `suspicious_link` foi incluído aí porque a lista de domínios mistura sites adultos com apostas; se quiser links suspeitos na escada gradual em vez de ban direto, é uma linha em `ViolationType::is_zero_tolerance`.
  → `MemoryStorage::violation_counter` (contador em memória por sessão) foi superado por essa mudança e ficou marcado `#[allow(dead_code)]`.

- [x] **Comando `/unban` ou `/appeal`** para admin reverter um banimento incorreto sem sair do Telegram.
  _Esforço: baixo · Impacto: médio_

- [x] **Whitelist de usuários confiáveis** — membros antigos sem histórico de violação recebem checagem mais branda.
  Implementado: `[trust]` em `moderation.toml` (`enabled`, `min_days_in_group`, `max_violations`, `strikes_multiplier`), validado no boot/reload junto das demais seções. `sqlite::is_trusted_user` checa `users.first_seen` + contagem total de violações do usuário no chat. `handlers::resolve_strikes_config` multiplica `mute_at`/`kick_at`/`ban_at` (via `StrikesConfig::scaled`) para usuários elegíveis, chamado **antes** de `record_violation` — senão a violação em curso contaminaria a checagem de "histórico limpo".
  _Esforço: médio · Impacto: médio_
  → **Assumido:** não afeta `csam`/`pornography`/`suspicious_link` (zero tolerância) — só relaxa a escada de strikes de `gambling`/`spam`. Não há comando manual `/trust`; é 100% automático a partir de tempo de casa + histórico. Se quiser um override manual de admin, é uma extensão natural (comando + coluna/flag em `users`).

---

## Persistência que já existe mas está pela metade

- [x] **Popular a tabela `users`** (schema já existe em `sqlite.rs`).
  `upsert_user` grava `user_id`+`username` a cada violação (`record_violation`); `get_chat_stats` resolve via `LEFT JOIN users` e `/stats` mostra `@username` quando disponível, com fallback pro `user_id` cru.
  _Esforço: baixo · Impacto: médio_ — já funcionando; só não compilava até a correção dos erros de build.
  → Sobrou solto: coluna `first_seen` é gravada mas nunca lida em lugar nenhum (possível item futuro: "membro desde" num comando de perfil).

- [ ] **Usar `blocked_domains` de verdade** (tabela e funções `add_blocked_domain`/`get_blocked_domains` já existem, nada chama).
  Comando `/blockdomain <dominio>` para admins adicionarem na hora, sem editar TOML e reiniciar.
  _Esforço: baixo · Impacto: médio_

- [x] **Comando `/reload`** — recarrega `moderation.toml` em runtime sem reiniciar o processo.
  Faz sentido agora que a config é externa (ver histórico de migração keyword/domain → TOML).
  _Esforço: baixo · Impacto: médio_

---

## Observabilidade / operação

- [ ] **Endpoint Prometheus (`/metrics`)** via um mini servidor HTTP (`axum`/`warp`) rodando junto do bot.
  Contagem de violações, latência de análise, mensagens processadas.
  _Esforço: médio · Impacto: médio_

- [ ] **Modo dry-run** via variável de ambiente — só loga o que baniria, sem agir.
  Ótimo para testar regra nova em produção sem risco.
  _Esforço: baixo · Impacto: alto (segurança operacional)_

- [ ] **Rate limiting de mensagens por usuário** (flood / repetição) — hoje só cobre palavra-chave e link, não volume.
  _Esforço: médio · Impacto: médio_

---

## Detecção mais robusta

- [x] **Classificador Naive Bayes (Multinomial) de spam**, complementar à detecção por palavra-chave.
  Implementado: `moderation::bayes` (`vectorizer.rs` — bag-of-words próprio; `dataset.rs` — corpus de treino pt/en/es embutido no binário; `classifier.rs` — `SpamClassifier` sobre `linfa`/`linfa-bayes`, os pacotes nativos do ecossistema Rust ML). Treinado uma única vez no boot (`AppState::new`) a partir de `dataset::TRAINING_DATA`; `engine::analyze_message` consulta `state.bayes.spam_probability(...)` como sinal extra, em paralelo a `spam::is_spam`, controlado por `[bayes]` em `moderation.toml` (`enabled`, `threshold`, `min_tokens` — recarregáveis via `/reload`).
  _Esforço: médio · Impacto: médio_
  → **Assumido:** o dataset de treino é um ponto de partida pequeno, escrito à mão — não há dado real de nenhum grupo. Sem re-treino em runtime: o modelo (pesos) só é gerado no boot; mudar o dataset exige reiniciar o processo. `threshold = 0.90` por padrão é conservador de propósito, justamente por causa do corpus pequeno.
  → **Não implementado ainda (próximo passo natural):** comando de admin pra rotular mensagens reais do grupo (`/trainspam`/`/trainham` em reply, por exemplo) e persistir esse corpus em SQLite, permitindo re-treino via `/reload` sem precisar editar `dataset.rs` e recompilar. Sem isso, a precisão do modelo não melhora sozinha com o uso do bot.
  → Não usei `linfa-preprocessing::CountVectorizer` (que também faria o bag-of-words) porque ele devolve uma matriz esparsa (`sprs::CsMat`) e puxaria mais uma dependência (`sprs`) só pra depois converter pra denso antes do `linfa-bayes::MultinomialNb` — preferi um `Vocabulary` bag-of-words próprio, pequeno e sem dependência extra, mantendo `linfa`/`linfa-bayes` (o algoritmo em si) como os pacotes nativos priorizados.

- [ ] **Normalização anti-evasão mais forte** em `normalize_text` (leetspeak, espaçamento entre letras, caracteres Unicode visualmente parecidos).
  Contorna o truque mais comum de burlar filtro de texto.
  _Esforço: médio · Impacto: alto_

- [ ] **Verificação de imagem via hash perceptual** contra bases conhecidas.
  Hoje o bot só analisa texto/legenda — imagem passa batido.
  _Esforço: alto · Impacto: alto_

- [ ] **Checagem de link malicioso/phishing** via Google Safe Browsing API (separada da lista estática de domínio suspeito).
  _Esforço: médio · Impacto: médio_

---

## Multi-tenant de verdade

- [ ] **Override de regras por grupo** (ex: `rules.chat_overrides[chat_id]`) — hoje `moderation.toml` é global para o bot inteiro.
  _Esforço: alto · Impacto: médio_

- [ ] **Severidade configurável por grupo** — ex: um grupo bane direto, outro só deleta e avisa.
  _Esforço: médio · Impacto: médio_

---

## Notas de manutenção (não-feature)

- [x] Migrar keywords/domínios de `gambling`, `pornography`, `spam`, `links` para `config/moderation.toml`.
- [x] Manter `csam.rs` fixo no binário, sem depender de config externa.
- [x] Validar no boot que nenhuma seção do `moderation.toml` está vazia (`ModerationRules::validate`).
- [x] Registrar violações no SQLite (`insert_violation`) e expor via `/stats`.
- [x] Corrigir `Cargo.toml` (`[alias]` movido para `.cargo/config.toml`, `rust-version` adicionado, feature `macros` do sqlx removida por estar sem uso).
- [x] Corrigir cadeia de erros de compilação da refatoração de i18n/moderation: `violation.rs` faltando (`ViolationType`), re-export de `messages::Messages` inexistente, `DEFAULT_LANG` removido de `lang.rs`, imports perdidos em `handlers.rs`/`engine.rs`, e `UpdateId` (`u32`) vs `i32` no dedupe de updates. Build limpo com `RUSTFLAGS=-D warnings`.
- [x] Deduplicar Update do Telegram (`MemoryStorage::processed_updates`, filtro `dedupe_update` no dispatcher) — evita reprocessar o mesmo Update em retries de long polling (ex.: banir duas vezes pelo mesmo evento).
  ⚠️ `processed_updates` é um `HashSet` que só cresce, sem expiração — ok por enquanto, mas vaza memória lentamente em execuções muito longas.
- [x] `Config::default_language` virou a fonte única do idioma padrão (usada por `LanguageManager::get`/`main.rs`), eliminando releitura duplicada de `BOT_DEFAULT_LANG` via `Lang::default_from_env()`.
- [x] Código do roadmap ainda não conectado (`Lang::display_name`, `LanguageManager::reset`/`exists`, `engine::is_violation`, `MemoryStorage::get_violation_count`) marcado com `#[allow(dead_code)]` + `TODO(roadmap)` apontando pra que serve, em vez de deixá-lo quebrar o build sob `-D warnings`.
- [ ] Rodar `cargo check` / `cargo clippy --all-targets --all-features -- -D warnings` localmente antes de cada release — o ambiente de review não tem toolchain compatível com edition 2024 para validar compilação.
