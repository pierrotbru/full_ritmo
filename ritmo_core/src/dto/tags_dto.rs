use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TagDto {
    pub id: Option<i64>,
    pub name: String,
    pub is_new: bool,
    pub is_book_tag: bool,
    pub is_content_tag: bool,
}
