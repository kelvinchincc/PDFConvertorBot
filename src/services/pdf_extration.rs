use teloxide::{net::Download, prelude::*, types::Document};
use tokio::fs;

pub async fn prepare_working_dir(file_id: &str) -> anyhow::Result<()> {
    let dir_path = format!("./.downloads/{}", file_id);
    if fs::metadata(&dir_path).await.is_ok() {
        fs::remove_dir_all(&dir_path).await?;
    }
    fs::create_dir_all(&dir_path).await?;
    Ok(())
}

pub async fn download_pdf(bot: &Bot, document: &Document) -> anyhow::Result<()> {
    let file_id = &document.file.id;
    let file = bot.get_file(file_id.clone()).await?;
    let mut file_handle =
        fs::File::create(format!("./.downloads/{}/process.pdf", document.file.id)).await?;
    bot.download_file(&file.path, &mut file_handle).await?;
    Ok(())
}
