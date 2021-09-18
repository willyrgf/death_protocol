use log::debug;
use teloxide::prelude::*;

pub async fn send_health_check() {
    debug!("start bot...");

    let bot = Bot::from_env().auto_send();

    teloxide::repl(bot, |message| async move {
        message.answer("hi").await?;
        respond(())
    })
    .await
}
