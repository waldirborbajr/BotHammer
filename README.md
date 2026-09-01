<p align="center">
<img width="256" height="256" src="./assets/logo.png" />
</p>
<h1 align="center">

  BanHammer - detects and bans users who post ilicity content
</h1>

**Automatic moderation bot for Telegram** — detects and bans users who post pornographic content, gambling/betting promotions, commercial spam, child exploitation (CSAM) content, and other illicit material in groups.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)
![Telegram Bot](https://img.shields.io/badge/Telegram-Bot-2CA5E0?style=flat&logo=telegram&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-green)

---

## ✨ Overview

**BanHammer** monitors messages in a Telegram group in real time. When a message (text or media caption) matches patterns associated with prohibited content, the bot:

1. **Deletes** the message immediately;
2. **Bans** the message's author from the group;
3. **Notifies** the group about the action taken.

The goal is to keep communities free of pornography, gambling/betting promotion, scam/commercial spam, and child exploitation material, without relying on constant manual moderation.

## 🚨 Detected categories

The current filter is based on case-insensitive regular expressions organized by category:

| Category | Trigger examples |
|---|---|
| Pornography / sexual content | `porn`, `nudes`, `onlyfans`, `xxx`, explicit sexual terms (in Portuguese) |
| Gambling / Betting | `bet`, `aposta`, `cassino`, `roleta`, `odds`, `bet365` |
| Commercial spam / sales | `compre agora`, `promoção`, `desconto`, `revenda`, `frete` — plus a Naive Bayes classifier (`src/moderation/bayes/`) as a probabilistic backup for spam that doesn't match any keyword |
| Child exploitation / CSAM | terms associated with child sexual exploitation |
| Suspicious links | `pornhub`, `xvideos`, link shorteners (`bit.ly`, `tinyurl`) |

> ⚠️ The keyword and domain lists live in `src/main.rs` and should be expanded as the bot is used in production. They are currently written in Portuguese — add English (or other language) patterns as needed for your audience.

## 🛠️ Tech stack

- **[Rust](https://www.rust-lang.org/)** (edition 2024)
- **[teloxide](https://github.com/teloxide/teloxide)** — Telegram bot framework
- **[tokio](https://tokio.rs/)** — async runtime
- **[linfa](https://github.com/rust-ml/linfa) / [linfa-bayes](https://crates.io/crates/linfa-bayes)** — Multinomial Naive Bayes classifier used as a probabilistic spam signal
- **regex** + **lazy_static** — pattern-matching engine
- **pretty_env_logger** / **log** — structured logging

## 📋 Requirements

- Rust (via [rustup](https://rustup.rs/)) — the exact version used by the project is pinned in [`rust-toolchain.toml`](./rust-toolchain.toml)
- A **Telegram bot token**, obtained from [@BotFather](https://t.me/BotFather)
- The bot must be an **administrator** of the group, with permission to:
  - Delete messages
  - Ban/remove members

## 🚀 Installation and usage

### 1. Clone the repository

```bash
git clone https://github.com/waldirborbajr/BanHammer.git
cd BanHammer
```

### 2. Configure the bot token

BanHammer reads the token from the `TELOXIDE_TOKEN` environment variable:

```bash
export TELOXIDE_TOKEN="123456789:AAYourTokenExampleHere"
```

Or create a `.env` file at the project root (not committed to version control):

```env
TELOXIDE_TOKEN=123456789:AAYourTokenExampleHere
RUST_LOG=info
```

### 3. Build and run

```bash
cargo run --release
```

### 4. Add the bot to your group

1. Add the bot to the Telegram group.
2. Promote it to **administrator**, enabling at least:
   - "Delete messages"
   - "Ban users"
3. Send `/status` in the group to confirm the bot is active.

## 💬 Available commands

| Command | Description |
|---|---|
| `/help` | Shows information about the bot |
| `/status` | Confirms the bot is online and monitoring |

## 🐳 Development environment (Dev Container)

The project includes a configuration in [`.devcontainer/`](./.devcontainer/devcontainer.json), ready to use with VS Code / GitHub Codespaces, ensuring a consistent Rust environment for contributors.

## 🗺️ Roadmap

- [x] Externally configurable keyword/domain list (YAML/TOML file), without needing to recompile
- [ ] Whitelist for users/admins exempt from the filter
- [ ] Warning system before banning, configurable per group
- [ ] Moderation action log in a separate channel
- [ ] Support for multiple groups with independent configurations
- [ ] Image detection via computer vision (beyond text/captions)

Contributions and suggestions are welcome — see the section below.

## 🤝 Contributing

1. Fork the project
2. Create a branch for your feature (`git checkout -b feature/new-rule`)
3. Run `cargo fmt` and `cargo clippy` before committing (settings in [`rustfmt.toml`](./rustfmt.toml) and [`clippy.toml`](./clippy.toml))
4. Open a Pull Request describing the change

## ⚖️ Legal notice

This bot is a **moderation aid** based on pattern (regex) matching and is **not infallible** — it can produce false positives and false negatives. It does not replace:

- Telegram's official reporting and moderation tools;
- The legal obligation to report child sexual abuse material (CSAM) to the relevant authorities and/or to the [Internet Watch Foundation](https://www.iwf.org.uk/) / [NCMEC CyberTipline](https://report.cybertip.org/).

Use at your own risk, and adapt the detection rules to the laws and policies applicable to your group/community.

## 📄 License

Distributed under the [MIT](./LICENSE) license.
