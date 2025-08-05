use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LanguageDto {
    pub name: String,
    pub id: Option<i64>,
    pub is_new: bool,
}

impl LanguageDto {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            id: None,
            is_new: false,
        }
    }

    pub fn with_id(name: &str, id: i64) -> Self {
        Self {
            name: name.to_string(),
            id: Some(id),
            is_new: false,
        }
    }
}
