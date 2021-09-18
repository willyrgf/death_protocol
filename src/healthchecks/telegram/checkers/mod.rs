use log::debug;
use teloxide::prelude::*;

use super::dialogue::Dialogue;

pub async fn send_health_check() {
    debug!("start bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::dialogues_repl(bot, |message, dialogue| async move {
        handle_message(message, dialogue)
            .await
            .expect("something wrong with the bot.")
    })
    .await
}

async fn handle_message(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    dialogue: Dialogue,
) -> TransitionOut<Dialogue> {
    match cx.update.text().map(ToOwned::to_owned) {
        Some(ans) => dialogue.react(cx, ans).await,
        None => {
            cx.answer("send me a text message").await?;
            next(dialogue)
        }
    }
}
