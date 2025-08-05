use crate::dto::{BookContentDto, PersonRoleDto, TagDto};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookDto {
    pub id: Option<i64>,

    pub name: String,

    pub original_title: Option<String>,

    // Publisher con campi aggiuntivi
    pub publisher_name: String,
    pub publisher_id: Option<i64>,
    pub publisher_is_new: bool,

    // Format con campi aggiuntivi
    pub format_name: String,
    pub format_id: Option<i64>,
    pub format_is_new: bool,

    // Series con campi aggiuntivi
    pub series_name: String,
    pub series_id: Option<i64>,
    pub series_is_new: bool,

    pub series_index: Option<i64>,

    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub isbn: Option<String>,

    pub pages: Option<i64>,

    pub notes: Option<String>,
    pub has_cover: bool,
    pub has_paper: bool,
    pub file_link: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: Option<String>,

    // Relazioni multiple
    pub authors: Vec<PersonRoleDto>,
    pub tags: Vec<TagDto>,
    pub contents: Vec<BookContentDto>,
}

impl Default for BookDto {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            original_title: None,
            publisher_name: String::new(),
            publisher_id: None,
            publisher_is_new: false,
            format_name: String::new(),
            format_id: None,
            format_is_new: false,
            series_name: String::new(),
            series_id: None,
            series_is_new: false,
            series_index: None,
            publication_date: None,
            acquisition_date: None,
            isbn: None,
            pages: None,
            notes: None,
            has_cover: false,
            has_paper: false,
            file_link: None,
            file_size: None,
            file_hash: None,
            authors: Vec::new(),
            tags: Vec::new(),
            contents: Vec::new(),
        }
    }
}
