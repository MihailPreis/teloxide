use crate::core::types::{InputMessageContent, InlineKeyboardMarkup, ParseMode};

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct InlineQueryResultCachedGif {
    pub id: String,
    pub gif_file_id: String,
    pub title: Option<String>,
    pub caption: Option<String>,
    pub parse_mode: Option<ParseMode>,
    pub reply_markup: Option<InlineKeyboardMarkup>,
    pub input_message_content: Option<InputMessageContent>,
}