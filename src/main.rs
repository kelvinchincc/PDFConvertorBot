mod constants;
mod services;

use constants::messages;
use teloxide::{
    dispatching::dialogue::GetChatId, prelude::*, types::Document, update_listeners::webhooks,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting PDF Convertor Bot...");

    let allowed_users = std::sync::Arc::new(constants::env::whitelisted_users());
    log::info!("Allowed users: {:?}", allowed_users);

    let bot = Bot::from_env();
    let addr = ([127, 0, 0, 1], 3000).into();
    let url = constants::env::teleoxide_webhook_url();
    let listener = webhooks::axum(
        bot.clone(),
        webhooks::Options::new(addr, url).secret_token(constants::env::teleoxide_secret_token()),
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

    let Some(document) = msg.document() else {
        log::info!("Received message without document: {:?}", msg);
        bot.send_message(msg.chat.id, messages::SELF_INTRO_MSG)
            .await?;
        return Ok(());
    };
    let filename = match &document.file_name {
        Some(filename) => filename.clone(),
        None => "".to_string(),
    };
    let Some(mime_type) = &document.mime_type else {
        log::info!("Received document without MIME type: {:?}", document);
        bot.send_message(msg.chat.id, messages::FAILED_WITH_NO_MIME)
            .await?;
        return Ok(());
    };

    if *mime_type != mime::APPLICATION_PDF {
        log::info!(
            "Received non-PDF document: {:?} with MIME type: {}",
            document,
            mime_type
        );
        bot.send_message(msg.chat.id, messages::FAILED_WITH_INVALID_MIME)
            .await?;
        return Ok(());
    }

    let msg_handle = bot
        .send_message(
            msg.chat.id,
            messages::get_stage_1_message(filename.as_str()),
        )
        .await?;

    match process_pdf(&bot, document).await {
        Ok(()) => log::info!("PDF of {} being processed!", document.file.id),
        Err(err) => {
            log::error!(
                "Failed to process PDF of {}, where {}",
                document.file.id,
                err.to_string()
            );
            bot.edit_message_text(msg.chat.id, msg_handle.id, messages::FAILED_TO_PROCESS_PDF)
                .await?;
        }
    }

    Ok(())
}

async fn process_pdf(bot: &Bot, document: &Document) -> anyhow::Result<()> {
    use services::pdf_extration;

    pdf_extration::prepare_working_dir(&document.file.id.to_string()).await?;
    pdf_extration::download_pdf(&bot, &document).await?;
    pdf_extration::extract_pdf_pages(document)?;

    Ok(())
}
