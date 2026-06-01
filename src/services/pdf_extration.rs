use pdf2image::{PDF, RenderOptionsBuilder};
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

pub fn extract_pdf_pages(document: &Document) -> anyhow::Result<()> {
    let file_id = &document.file.id;
    let pdf = PDF::from_file(format!("./.downloads/{}/process.pdf", file_id))?;
    let page_count = pdf.page_count();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=page_count),
        RenderOptionsBuilder::default().build()?,
    )?;

    let output_dir = format!("./downloads/{}/out", file_id);
    std::fs::create_dir(&output_dir)?;
    for (i, page) in pages.iter().enumerate() {
        page.save_with_format(
            format!("{}/page_{}.jpg", &output_dir, i),
            pdf2image::image::ImageFormat::Jpeg,
        )?
    }

    Ok(())
}
