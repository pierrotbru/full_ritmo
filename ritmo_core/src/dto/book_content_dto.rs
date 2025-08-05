use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookContentDto {
    pub content_name: String,
    pub content_id: Option<i64>,
    pub content_is_new: bool,
}

impl BookContentDto {
    pub fn new(content_name: &str) -> Self {
        Self {
            content_name: content_name.to_string(),
            content_id: None,
            content_is_new: false,
        }
    }

    pub fn with_id(content_name: &str, content_id: i64) -> Self {
        Self {
            content_name: content_name.to_string(),
            content_id: Some(content_id),
            content_is_new: false,
        }
    }

    pub fn with_pages(content_name: &str, content_id: i64) -> Self {
        Self {
            content_name: content_name.to_string(),
            content_id: Some(content_id),
            content_is_new: false,
        }
    }
}
