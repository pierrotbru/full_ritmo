use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct RunningLanguage {
    pub id: i64,
    pub iso_code_2char: String,
    pub iso_code_3char: String,
    pub official_name: String,
    pub language_role: String,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct NewRunningLanguage {
    pub iso_code_2char: String,
    pub iso_code_3char: String,
    pub official_name: String,
    pub language_role: String,
}

impl RunningLanguage {
    pub async fn create(pool: &sqlx::SqlitePool, new_language: &NewRunningLanguage) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "INSERT INTO running_languages (
                iso_code_2char, iso_code_3char, official_name, language_role, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&new_language.iso_code_2char)
        .bind(&new_language.iso_code_3char)
        .bind(&new_language.official_name)
        .bind(&new_language.language_role)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<RunningLanguage>, sqlx::Error> {
        let language = sqlx::query_as::<_, RunningLanguage>(
            "SELECT * FROM running_languages WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(language)
    }

    pub async fn update(&self, pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "UPDATE running_languages SET
                iso_code_2char = ?, iso_code_3char = ?, official_name = ?, language_role = ?, updated_at = ?
            WHERE id = ?"
        )
        .bind(&self.iso_code_2char)
        .bind(&self.iso_code_3char)
        .bind(&self.official_name)
        .bind(&self.language_role)
        .bind(now)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM running_languages WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_all(pool: &sqlx::SqlitePool) -> Result<Vec<RunningLanguage>, sqlx::Error> {
        let all = sqlx::query_as::<_, RunningLanguage>(
            "SELECT * FROM running_languages ORDER BY official_name"
        )
        .fetch_all(pool)
        .await?;
        Ok(all)
    }

    pub async fn search(pool: &sqlx::SqlitePool, pattern: &str) -> Result<Vec<RunningLanguage>, sqlx::Error> {
        let search_pattern = format!("%{}%", pattern);
        let found = sqlx::query_as::<_, RunningLanguage>(
            "SELECT * FROM running_languages WHERE official_name LIKE ? OR iso_code_2char LIKE ? OR iso_code_3char LIKE ? ORDER BY official_name"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(found)
    }
}