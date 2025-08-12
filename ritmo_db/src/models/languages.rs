use ritmo_core::LanguageDto;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct RunningLanguages {
    pub id: Option<i64>,
    pub name: String,
    pub role: String,
    pub iso_code_2char: Option<String>,
    pub iso_code_3char: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl RunningLanguages {
    pub fn new() -> Self {
        Self {
            id: None,
            name: String::new(),
            role: String::new(),
            iso_code_2char: None,
            iso_code_3char: None,
            created_at: None,
            updated_at: None,
        }
    }

    pub async fn create(
        pool: &sqlx::SqlitePool,
        new_lang: &RunningLanguages,
    ) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result =
            sqlx::query!(
                "INSERT INTO running_languages (name, language_role, iso_code_2_char, iso_code_3char, created_at, updated_at) VALUES (?, ?, ?, ?, ?, ?)",
                new_lang.name,
                new_lang.role,
                new_lang.iso_code_2char,
                new_lang.iso_code_3char,
                now,
                now
                )
                .execute(pool)
                .await?;
        Ok(result.last_insert_rowid())
    }

    /// questi sono 2 placeholder
    pub fn from_dto(dto: &LanguageDto) -> Self {
        Self::set_language_data(dto)
    }

    pub fn set_language_data(_dto: &LanguageDto) -> RunningLanguages {
        RunningLanguages::new()
    }

    pub async fn get(
        pool: &sqlx::SqlitePool,
        id: i64,
    ) -> Result<Option<RunningLanguages>, sqlx::Error> {
        let result = sqlx::query_as::<_, RunningLanguages>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    pub async fn update(pool: &sqlx::SqlitePool, id: i64, name: &str) -> Result<(), sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        sqlx::query("UPDATE tags SET name = ?, updated_at = ? WHERE id = ?")
            .bind(name)
            .bind(now)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM tags WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
