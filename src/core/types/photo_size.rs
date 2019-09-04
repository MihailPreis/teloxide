#[derive(Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: i32,
    pub height: i32,
    pub file_size: Option<u32>,
}