use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result, Sqlite, SqlitePool, Transaction};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Content {
    pub id: i64,
    pub name: String,
    pub parent_id: Option<i64>,
    pub description: Option<String>,
    pub content_type: Option<String>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

impl Content {
    /// Crea un nuovo record Content e restituisce il suo id.
    pub async fn create(pool: &SqlitePool, new_content: &Content) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO contents (
                name, parent_id, description, content_type, created_at, updated_at
            ) VALUES (?, ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'))
            "#,
        )
        .bind(&new_content.name)
        .bind(new_content.parent_id)
        .bind(&new_content.description)
        .bind(&new_content.content_type)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Restituisce un record Content dato il suo id, oppure None se non esiste.
    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Content>> {
        let content = sqlx::query_as::<_, Content>(r#"SELECT * FROM contents WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(content)
    }

    /// Aggiorna i dati di questo record Content nel database.
    pub async fn update(&self, pool: &SqlitePool) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE contents SET
                name = ?,
                parent_id = ?,
                description = ?,
                content_type = ?,
                updated_at = strftime('%s', 'now')
            WHERE id = ?
            "#,
        )
        .bind(&self.name)
        .bind(self.parent_id)
        .bind(&self.description)
        .bind(&self.content_type)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Cancella un record Content dal database per id. Restituisce il numero di righe eliminate.
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<u64> {
        let result = sqlx::query("DELETE FROM contents WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Inserisce un batch di contents in una singola transazione.
    pub async fn insert_batch(pool: &SqlitePool, contents: &[Content]) -> Result<()> {
        let mut tx: Transaction<'_, Sqlite> = pool.begin().await?;
        for content in contents {
            sqlx::query(
                r#"
                INSERT INTO contents (
                    name, parent_id, description, content_type, created_at, updated_at
                ) VALUES (?, ?, ?, ?, strftime('%s', 'now'), strftime('%s', 'now'))
                "#,
            )
            .bind(&content.name)
            .bind(content.parent_id)
            .bind(&content.description)
            .bind(&content.content_type)
            .execute(&mut tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Aggiorna un batch di contents in una singola transazione.
    pub async fn update_batch(pool: &SqlitePool, contents: &[Content]) -> Result<()> {
        let mut tx: Transaction<'_, Sqlite> = pool.begin().await?;
        for content in contents {
            sqlx::query(
                r#"
                UPDATE contents SET
                    name = ?,
                    parent_id = ?,
                    description = ?,
                    content_type = ?,
                    updated_at = strftime('%s', 'now')
                WHERE id = ?
                "#,
            )
            .bind(&content.name)
            .bind(content.parent_id)
            .bind(&content.description)
            .bind(&content.content_type)
            .bind(content.id)
            .execute(&mut tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Cancella un batch di contents dati i loro id.
    pub async fn delete_batch(pool: &SqlitePool, ids: &[i64]) -> Result<()> {
        let mut tx: Transaction<'_, Sqlite> = pool.begin().await?;
        for id in ids {
            sqlx::query("DELETE FROM contents WHERE id = ?")
                .bind(id)
                .execute(&mut tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
