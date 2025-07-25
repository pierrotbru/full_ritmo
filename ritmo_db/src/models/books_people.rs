use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookPersonRole {
    pub book_id: i64,
    pub person_id: i64,
    pub role_id: i64,
}
