// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
use pdf2image::{PDF, RenderOptionsBuilder};
use teloxide::{
    net::Download,
    prelude::*,
    types::{Document, FileId, InputMedia, MessageId},
};
use tokio::fs;

const MEDIA_GROUP_SIZE: usize = 6;

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

pub fn extract_pdf_pages(document: &Document) -> anyhow::Result<usize> {
    let file_id = &document.file.id;
    let pdf = PDF::from_file(format!("./.downloads/{}/process.pdf", file_id))?;
    let page_count = pdf.page_count();
    let pages = pdf.render(
        pdf2image::Pages::Range(1..=page_count),
        RenderOptionsBuilder::default().build()?,
    )?;

    let output_dir = format!("./.downloads/{}/out", file_id);
    std::fs::create_dir(&output_dir)?;
    for (i, page) in pages.iter().enumerate() {
        page.save_with_format(
            format!("{}/page_{}.jpg", &output_dir, i),
            pdf2image::image::ImageFormat::Jpeg,
        )?
    }

    Ok(pages.len())
}

pub async fn upload_pages_to_telegram(
    bot: &Bot,
    _message_id: &MessageId,
    chat_id: &ChatId,
    document_id: &FileId,
    page_size: usize,
) -> anyhow::Result<()> {
    let output_dir = format!("./.downloads/{}/out", document_id);
    let mut media_group: Vec<InputMedia> = vec![];
    media_group.reserve(MEDIA_GROUP_SIZE);

    for i in 0..page_size {
        let img_path = format!("{}/page_{}.jpg", &output_dir, i);
        let input_file = teloxide::types::InputFile::file(std::path::PathBuf::from(img_path));

        let photo = teloxide::types::InputMediaPhoto {
            media: input_file,
            caption: None,
            parse_mode: None,
            caption_entities: None,
            show_caption_above_media: false,
            has_spoiler: false,
        };

        media_group.push(InputMedia::Photo(photo));

        if media_group.len() == MEDIA_GROUP_SIZE || i == page_size - 1 {
            bot.send_media_group(chat_id.clone(), media_group.clone())
                .await?;
            media_group.clear();
        }
    }

    Ok(())
}

pub fn clean_up_work_folder(document_id: &FileId) -> anyhow::Result<()> {
    let dir_path = format!("./.downloads/{}", document_id);
    if std::fs::metadata(&dir_path).is_ok() {
        std::fs::remove_dir_all(&dir_path)?;
    }
    Ok(())
}

pub fn reset_work_folder() -> anyhow::Result<()> {
    // Remove the contents in the working folder, do not remove the folder itself

    let dir_path = "./.downloads";
    let children = std::fs::read_dir(dir_path)?;
    for child in children {
        let child_path = child?.path();
        if child_path.is_dir() {
            std::fs::remove_dir_all(child_path)?;
        } else {
            std::fs::remove_file(child_path)?;
        }
    }

    Ok(())
}
