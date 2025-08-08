use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Format {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
}

impl Format {
    pub async fn create(
        pool: &sqlx::SqlitePool,
        name: &str,
        description: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        let rec = sqlx::query("INSERT INTO formats (name, description) VALUES (?, ?)")
            .bind(name)
            .bind(description)
            .execute(pool)
            .await?;
        // Recupera l'ID appena inserito
        let id = rec.last_insert_rowid();
        Ok(id)
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Format>, sqlx::Error> {
        let result = sqlx::query_as::<_, Format>(
            "SELECT id, name, description, created_at FROM formats WHERE id = ?",
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
        description: Option<&str>,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("UPDATE formats SET name = ?, description = ? WHERE id = ?")
            .bind(name)
            .bind(description)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM formats WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
