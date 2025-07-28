use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Tags {
    pub id: i64,
    pub name: String,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Tags {
    pub async fn create(
        pool: &sqlx::SqlitePool,
        name: &str,
    ) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query("INSERT INTO tags (name, created_at, updated_at) VALUES (?, ?, ?)")
            .bind(name)
            .bind(now)
            .bind(now)
            .execute(pool)
            .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Tags>, sqlx::Error> {
        let result = sqlx::query_as::<_, Tags>(
            "SELECT id, name, created_at, updated_at FROM tags WHERE id = ?",
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(result)
    }

    pub async fn update(
        pool: &sqlx::SqlitePool,
        id: i64,
        name: &str,
    ) -> Result<(), sqlx::Error> {
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