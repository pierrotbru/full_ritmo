use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishersDto {
    pub id: Option<i64>,

    pub name: String,
    pub name_is_new: bool,

    pub country: Option<String>,

    pub website: Option<String>,
    pub notes: Option<String>,

    pub created_at: i64,
    pub updated_at: i64,
}

impl PublisherDto {}
