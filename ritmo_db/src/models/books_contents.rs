use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookContent {
    pub book_id: i64,
    pub content_id: i64,
    pub page_start: Option<i64>,
    pub page_end: Option<i64>,
}
