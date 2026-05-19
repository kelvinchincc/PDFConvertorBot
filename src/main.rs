use std::env;

use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting PDF Convertor Bot...");

    let bot = Bot::from_env();
    let addr = ([127, 0, 0, 1], 3000).into();
    let url = (env::var("TELOXIDE_WEBHOOK_URL").expect("TELOXIDE_WEB_HOOK_URL must be set"))
        .parse()
        .expect("Failed to parse TELOXIDE_WEB_HOOK_URL");
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(addr, url).secret_token(
            env::var("TELOXIDE_SECRET_TOKEN").expect("TELOXIDE_SECRET_TOKEN must set"),
        ),
    )
    .await
    .expect("Falied to start telegram bot");

    teloxide::repl_with_listener(
        bot,
        |bot: Bot, msg: Message| async move {
            log::info!("h!");
            bot.send_message(msg.chat.id, "Pong!").await?;
            Ok(())
        },
        listener,
    )
    .await;

    Ok(())
}
