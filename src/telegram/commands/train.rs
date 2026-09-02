use teloxide::prelude::*;

use crate::{
    core::state::AppState,
    i18n::{Lang, messages},
    telegram::admin::is_chat_admin,
};

/// `/trainspam` e `/trainham` — usados em *reply* a uma mensagem de
/// texto do grupo, ensinam o classificador bayesiano de spam
/// (`moderation::bayes`) com um exemplo real. Apenas administradores.
///
/// `is_spam` decide o rótulo do exemplo: `true` para `/trainspam`,
/// `false` para `/trainham`. O classificador é retreinado na hora
/// (`AppState::add_training_example`) — o efeito vale já pra próxima
/// mensagem do grupo, sem precisar de `/reload`.
pub async fn handle(
    bot: &Bot,
    msg: &Message,
    state: &AppState,
    lang: Lang,
    is_spam: bool,
) -> ResponseResult<()> {
    let chat_id = msg.chat.id;

    let Some(user) = &msg.from else {
        return Ok(());
    };

    if !is_chat_admin(bot, chat_id, user.id).await {
        bot.send_message(chat_id, messages::train_no_permission(lang))
            .await?;

        return Ok(());
    }

    let text = msg
        .reply_to_message()
        .and_then(|replied| replied.text().or_else(|| replied.caption()));

    let Some(text) = text else {
        bot.send_message(chat_id, messages::train_missing_reply(lang))
            .await?;

        return Ok(());
    };

    match state.add_training_example(text, is_spam).await {
        Ok(_) => {
            bot.send_message(chat_id, messages::train_success(lang, is_spam))
                .await?;

            log::info!(
                "exemplo de treino bayesiano ({}) adicionado por {} no chat {}",
                if is_spam { "spam" } else { "ham" },
                user.id,
                chat_id
            );
        }

        Err(error) => {
            log::warn!("Falha ao adicionar exemplo de treino bayesiano: {}", error);

            bot.send_message(chat_id, messages::train_error(lang))
                .await?;
        }
    }

    Ok(())
}
