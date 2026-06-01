pub const SELF_INTRO_MSG: &str = "Hello! This is a PDF Convertor Bot. Please send me a PDF file to convert it to another format.";
pub const FAILED_WITH_NO_MIME: &str = "Sorry, I couldn't determine the file type of the document you sent. Please make sure to send a valid PDF file.";
pub const FAILED_WITH_INVALID_MIME: &str =
    "Sorry, I can only process PDF files. Please send a valid PDF file.";
pub const FAILED_TO_PROCESS_PDF: &str =
    "Failed to process file, we are investigating what is happening.";

pub fn get_stage_1_message(filename: &str) -> String {
    format!("Received PDF file: {}. Starting conversion...", filename)
}
