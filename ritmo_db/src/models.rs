use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Book {
    pub id: i64,
    pub name: String,
    pub publication_date: Option<String>,
    pub publisher_id: Option<i64>,
    pub format_id: Option<i64>,
    pub series_id: Option<i64>,
    pub series_index: Option<i64>,
    // Relazioni molti-a-molti
    pub contents: Vec<BookContent>,
    pub people_roles: Vec<BookPersonRole>,
    pub tags: Vec<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookContent {
    pub book_id: i64,
    pub content_id: i64,
    pub page_start: Option<i64>,
    pub page_end: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookPersonRole {
    pub book_id: i64,
    pub person_id: i64,
    pub role_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Content {
    pub id: i64,
    pub name: String,
    pub original_title: Option<String>,
    pub type_id: Option<i64>,
    pub publication_date: Option<String>,
    pub notes: Option<String>,
    // Relazioni molti-a-molti
    pub people_roles: Vec<ContentPersonRole>,
    pub tags: Vec<i64>,
    pub languages: Vec<RunningLanguage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPersonRole {
    pub content_id: i64,
    pub person_id: i64,
    pub role_id: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningLanguage {
    pub id: i64,
    pub lang_iso: i64,
    pub lang_role: i64,
}
