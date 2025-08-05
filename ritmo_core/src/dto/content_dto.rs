use crate::dto::{LanguageDto, PersonRoleDto, TagDto};
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

    // Relazioni multiple
    pub authors: Vec<PersonRoleDto>,
    pub tags: Vec<TagDto>,
    pub languages: Vec<LanguageDto>,
}

impl Default for ContentDto {
    fn default() -> Self {
        Self {
            id: None,
            name: String::new(),
            original_title: None,
            type_name: String::new(),
            type_id: None,
            type_is_new: false,
            publication_date: None,
            notes: None,
            authors: Vec::new(),
            tags: Vec::new(),
            languages: Vec::new(),
        }
    }
}
