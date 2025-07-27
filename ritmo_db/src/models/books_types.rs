use ritmo_errors::RitmoResult;
use sqlx::{query, FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow, Default)]
pub struct BooksTypes {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub created_at: i64,
}

impl BooksTypes {
    pub async fn create(pool: &SqlitePool, new_book_type: &BooksTypes) -> RitmoResult<i64> {
        let result = query(
            r#"
            INSERT INTO books_types (
                name, description, created_at
            ) VALUES (?, ?, strftime('%s', 'now'))
            "#,
        )
        .bind(&new_book_type.name)
        .bind(&new_book_type.description)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Restituisce un BooksTypes dato il suo id, oppure None se non esiste.
    pub async fn get(pool: &SqlitePool, id: i64) -> RitmoResult<Option<BooksTypes>> {
        let alias = sqlx::query_as::<_, BooksTypes>(r#"SELECT * FROM books_types WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(alias)
    }

    /// Aggiorna i dati di questo BooksTypes nel database.
    pub async fn update(&self, pool: &SqlitePool) -> RitmoResult<u64> {
        let result = sqlx::query(
            r#"
            UPDATE books_types SET
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
        let result = sqlx::query("DELETE FROM books_types WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }
}
