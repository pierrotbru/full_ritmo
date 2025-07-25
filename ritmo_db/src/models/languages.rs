use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunningLanguage {
    pub id: i64,
    pub lang_iso: i64,
    pub lang_role: i64,
}
