use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentPersonRole {
    pub content_id: i64,
    pub person_id: i64,
    pub role_id: i64,
}
