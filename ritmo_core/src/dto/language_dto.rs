#[derive(Debug, Clone)]
pub struct LanguageDto {
    pub id: Option<i64>,
    pub name: String,
    pub role: String,
    pub is_new: bool,
}
