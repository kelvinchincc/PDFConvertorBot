mod constants;
use std::env;

use teloxide::{prelude::*, update_listeners::webhooks};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting PDF Convertor Bot...");

    let allowed_users = std::sync::Arc::new(constants::env::whitelisted_users());
    log::info!("Allowed users: {:?}", allowed_users);

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
        move |bot: Bot, msg: Message| {
            let allowed_users = allowed_users.clone();
            async move {
                if let Err(e) = handle_message(&allowed_users, bot, msg).await {
                    log::error!("Error handling message: {:?}", e);
                }
                Ok(())
            }
        },
        listener,
    )
    .await;

    Ok(())
}

async fn handle_message(allowed_users: &Vec<String>, bot: Bot, msg: Message) -> anyhow::Result<()> {
    if !allowed_users.contains(&msg.chat.id.to_string()) {
        return Ok(());
    }

    match msg.document() {
        None => {
            log::info!("Received message: {:?}", msg);
            bot.send_message(msg.chat.id, "Hello! This is a PDF Convertor Bot. Please send me a PDF file to convert it to another format.")
            .await?;
            return Ok(());
        }
        _ => {}
    }

    let document = msg.document().unwrap();
    let filename = match &document.file_name {
        Some(filename) => filename.clone(),
        None => "".to_string(),
    };
    let Some(mime_type) = &document.mime_type else {
        log::info!("Received document without MIME type: {:?}", document);
        bot.send_message(msg.chat.id, "Sorry, I couldn't determine the file type of the document you sent. Please make sure to send a valid PDF file.")
            .await?;
        return Ok(());
    };

    if *mime_type != mime::APPLICATION_PDF {
        log::info!(
            "Received non-PDF document: {:?} with MIME type: {}",
            document,
            mime_type
        );
        bot.send_message(
            msg.chat.id,
            "Sorry, I can only process PDF files. Please send a valid PDF file.",
        )
        .await?;
        return Ok(());
    }

    bot.send_message(
        msg.chat.id,
        format!("Received PDF file: {}. Starting conversion...", filename),
    )
    .await?;

    Ok(())
}
