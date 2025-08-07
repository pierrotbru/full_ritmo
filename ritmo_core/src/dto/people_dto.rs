use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonRoleDto {
    // Dati della persona
    pub person_name: String,
    pub person_id: Option<i64>,
    pub person_is_new: bool,

    // Dati del ruolo
    pub role_name: String,
    pub role_id: Option<i64>,
    pub role_is_new: bool,
}

impl PersonRoleDto {
    pub fn new(person_name: &str, role_name: &str) -> Self {
        Self {
            person_name: person_name.to_string(),
            person_id: None,
            person_is_new: false,
            role_name: role_name.to_string(),
            role_id: None,
            role_is_new: false,
        }
    }

    pub fn with_ids(person_name: &str, person_id: i64, role_name: &str, role_id: i64) -> Self {
        Self {
            person_name: person_name.to_string(),
            person_id: Some(person_id),
            person_is_new: false,
            role_name: role_name.to_string(),
            role_id: Some(role_id),
            role_is_new: false,
        }
    }
}
