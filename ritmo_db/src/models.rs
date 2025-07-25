// ritmo_db/src/modelsb.rs

#[derive(Debug, Default)]
pub struct Book {
    pub data: BookUserData,
    pub new: BookDbData,
}

#[derive(Debug, Default)]
pub struct BookUserData {
    pub name: String,
    pub publisher: Option<String>,
    pub format: Option<String>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub series: Option<String>,
    pub series_index: Option<i64>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
    pub has_cover: Option<i32>,
    pub has_paper: Option<i32>,
    pub file_link: Option<String>,
    pub tags: Vec<String>,
    pub people: Vec<(String, String)>,
    pub contents: Vec<Content>,
}

#[derive(Debug, Default)]
pub struct BookDbData {
    pub id: i64,
    pub name: String,
    pub publisher_id: Option<i64>,
    pub format_id: Option<i64>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub series_id: Option<i64>,
    pub series_index: Option<i64>,
    pub original_title: Option<String>,
    pub notes: Option<String>,
    pub has_cover: Option<i32>,
    pub has_paper: Option<i32>,
    pub file_link: Option<String>,
    pub pre_accepted: Option<i32>,
}

#[derive(Debug, Default)]
pub struct Content {
    pub data: ContentUserData,
    pub new: ContentDbData,
}

#[derive(Debug, Default)]
pub struct ContentUserData {
    pub name: String,
    pub original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub notes: Option<String>,
    pub type_id: Option<String>,
    pub lang: Vec<(String, String)>,
    pub people: Vec<(String, String)>,
    pub tags: Vec<String>,
    pub to_book: i64,
}

#[derive(Debug, Default)]
pub struct ContentDbData {
    pub id: i32,
    pub name: String,
    pub original_title: Option<String>,
    pub publication_date: Option<i64>,
    pub notes: Option<String>,
    pub type_id: Option<i64>,
}
