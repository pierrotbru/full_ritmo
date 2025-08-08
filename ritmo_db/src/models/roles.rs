use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Role {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
}

impl Role {
    pub async fn create(
        pool: &sqlx::SqlitePool,
        name: &str,
        description: Option<&str>,
    ) -> Result<i64, sqlx::Error> {
        let rec = sqlx::query("INSERT INTO roles (name, description) VALUES (?, ?)")
            .bind(name)
            .bind(description)
            .execute(pool)
            .await?;
        // Recupera l'ID appena inserito
        let id = rec.last_insert_rowid();
        Ok(id)
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Role>, sqlx::Error> {
        let result = sqlx::query_as::<_, Role>(
            "SELECT id, name, description, created_at FROM roles WHERE id = ?",
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
        sqlx::query("UPDATE roles SET name = ?, description = ? WHERE id = ?")
            .bind(name)
            .bind(description)
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query("DELETE FROM roles WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
