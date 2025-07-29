use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Content {
    pub id: i64,
    pub name: String,
    pub original_title: Option<String>,
    pub type_id: Option<i64>,
    pub publication_date: Option<i64>,
    pub pages: Option<i64>,
    pub notes: Option<String>,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct NewContent {
    pub name: String,
    pub original_title: Option<String>,
    pub type_id: Option<i64>,
    pub publication_date: Option<i64>,
    pub pages: Option<i64>,
    pub notes: Option<String>,
}

impl Content {
    pub async fn create(pool: &sqlx::SqlitePool, new_content: &NewContent) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "INSERT INTO contents (
                name, original_title, type_id, publication_date, pages, notes, created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&new_content.name)
        .bind(&new_content.original_title)
        .bind(&new_content.type_id)
        .bind(&new_content.publication_date)
        .bind(&new_content.pages)
        .bind(&new_content.notes)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Content>, sqlx::Error> {
        let content = sqlx::query_as::<_, Content>(
            "SELECT * FROM contents WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(content)
    }

    pub async fn update(&self, pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "UPDATE contents SET
                name = ?, original_title = ?, type_id = ?, publication_date = ?, pages = ?, notes = ?, updated_at = ?
            WHERE id = ?"
        )
        .bind(&self.name)
        .bind(&self.original_title)
        .bind(&self.type_id)
        .bind(&self.publication_date)
        .bind(&self.pages)
        .bind(&self.notes)
        .bind(now)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM contents WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_all(pool: &sqlx::SqlitePool) -> Result<Vec<Content>, sqlx::Error> {
        let all = sqlx::query_as::<_, Content>(
            "SELECT * FROM contents ORDER BY name"
        )
        .fetch_all(pool)
        .await?;
        Ok(all)
    }

    pub async fn search(pool: &sqlx::SqlitePool, pattern: &str) -> Result<Vec<Content>, sqlx::Error> {
        let search_pattern = format!("%{}%", pattern);
        let found = sqlx::query_as::<_, Content>(
            "SELECT * FROM contents WHERE name LIKE ? OR original_title LIKE ? OR notes LIKE ? ORDER BY name"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(found)
    }
}