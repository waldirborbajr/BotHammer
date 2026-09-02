use teloxide::prelude::*;

use crate::{
    core::state::AppState,
    i18n::LanguageManager,
    telegram::commands::{self, Command},
};

/// Handler principal dos comandos Telegram — só roteia; a lógica de
/// cada comando vive em `telegram::commands::<comando>`.
pub async fn command_handler(
    bot: Bot,
    msg: Message,
    cmd: Command,
    state: AppState,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let lang = LanguageManager::get(&state, chat_id).await;

    match cmd {
        Command::Help => commands::help::handle(&bot, chat_id, lang).await?,

        Command::Status => commands::status::handle(&bot, chat_id, lang).await?,

        Command::Stats => commands::stats::handle(&bot, &msg, &state, lang).await?,

        Command::Language(language) => {
            commands::language::handle(&bot, &msg, &state, lang, language.trim()).await?
        }

        Command::Reload => commands::reload::handle(&bot, &msg, &state, lang).await?,

        Command::Unban(argument) => {
            commands::unban::handle(&bot, &msg, lang, argument.trim()).await?
        }

        Command::BlockDomain(argument) => {
            commands::blockdomain::handle(&bot, &msg, &state, lang, argument.trim()).await?
        }

        Command::TrainSpam => commands::train::handle(&bot, &msg, &state, lang, true).await?,

        Command::TrainHam => commands::train::handle(&bot, &msg, &state, lang, false).await?,
    }

    Ok(())
}
