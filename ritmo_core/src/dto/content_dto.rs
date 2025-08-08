use crate::dto::TagDto;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentDto {
    pub id: Option<i64>,

    pub name: String,

    pub original_title: Option<String>,

    // Type con campi aggiuntivi
    pub type_name: String,
    pub type_id: Option<i64>,
    pub type_is_new: bool,

    pub publication_date: Option<i64>,

    pub notes: Option<String>,

    pub tags: Vec<TagDto>,
    //    pub languages: Vec<LanguageDto>,
    pub people: Vec<String>,
}
