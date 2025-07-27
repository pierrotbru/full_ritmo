use ritmo_errors::RitmoResult;
use sqlx::{query, FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct Series {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
}

impl Series {
    pub async fn create(pool: &SqlitePool, new_series: &Series) -> RitmoResult<i64> {
        let result = query(
            r#"
            INSERT INTO series (
                name, description, created_at
            ) VALUES (?, ?, strftime('%s', 'now'))
            "#,
        )
        .bind(&new_series.name)
        .bind(&new_series.description)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Restituisce un BooksTypes dato il suo id, oppure None se non esiste.
    pub async fn get(pool: &SqlitePool, id: i64) -> RitmoResult<Option<Series>> {
        let alias = sqlx::query_as::<_, Series>(r#"SELECT * FROM books_types WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(alias)
    }

    /// Aggiorna i dati di questo BooksTypes nel database.
    pub async fn update(&self, pool: &SqlitePool) -> RitmoResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE series SET
                name = ?,
                description = ?,
                WHERE id = ?
                "#,
        )
        .bind(&self.name)
        .bind(&self.description)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Cancella un BooksTypes dal database per id. Restituisce il numero di righe eliminate.
    pub async fn delete(pool: &SqlitePool, id: i64) -> RitmoResult<u64> {
        let result = sqlx::query("DELETE FROM series WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
