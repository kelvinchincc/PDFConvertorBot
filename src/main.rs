mod constants;
mod services;

use constants::messages;
use teloxide::{prelude::*, types::Document, update_listeners::webhooks};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting PDF Convertor Bot...");

    let allowed_users = std::sync::Arc::new(constants::env::whitelisted_users());
    log::info!("Allowed users: {:?}", allowed_users);

    log::info!("Clearing old downloads...");
    services::pdf_extration::reset_work_folder()?;

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
        .send_message(msg.chat.id, messages::RECEIVED_PDF)
        .await?;

    match process_pdf(&bot, &msg, &msg_handle, document).await {
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

async fn process_pdf(
    bot: &Bot,
    msg: &Message,
    msg_handle: &Message,
    document: &Document,
) -> anyhow::Result<()> {
    use services::pdf_extration;

    let chat_id = msg.chat.id;
    let document_id = document.file.id.clone();
    let system_msg_id = msg_handle.id;

    pdf_extration::prepare_working_dir(&document.file.id.to_string()).await?;

    log::info!("Downloading file {} from Telegram...", document_id);
    bot.edit_message_text(chat_id, system_msg_id, messages::DOWNLOADING_PDF)
        .await?;
    pdf_extration::download_pdf(&bot, &document).await?;

    log::info!("Extracting pages from PDF {}...", document_id);
    bot.edit_message_text(chat_id, system_msg_id, messages::EXTRACTING_PDF)
        .await?;
    let page_size = pdf_extration::extract_pdf_pages(document)?;

    log::info!("Uploading pages of PDF {} to Telegram...", document_id);
    bot.edit_message_text(chat_id, system_msg_id, messages::UPLOADING_PAGES)
        .await?;
    pdf_extration::upload_pages_to_telegram(&bot, &msg.id, &chat_id, &document_id, page_size)
        .await?;

    log::info!("Cleaning up working dir of {}...", document_id);
    bot.delete_message(chat_id, msg_handle.id).await?;
    pdf_extration::clean_up_work_folder(&document_id)?;

    Ok(())
}
