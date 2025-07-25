use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Result, SqlitePool};

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Book {
    pub id: i64,
    pub name: String,
    pub original_title: Option<String>,
    pub publisher_id: Option<i64>,
    pub format_id: Option<i64>,
    pub series_id: Option<i64>,
    pub series_index: Option<i64>,
    pub publication_date: Option<i64>,
    pub acquisition_date: Option<i64>,
    pub last_modified_date: Option<i64>,
    pub isbn: Option<String>,
    pub pages: Option<i64>,
    pub notes: Option<String>,
    pub has_cover: bool,
    pub has_paper: bool,
    pub file_link: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: Option<String>,
    pub rating: Option<i64>,
    pub read_status: Option<String>,
    pub created_at: Option<i64>,
}

impl Book {
    /// Crea un nuovo libro e restituisce il suo id.
    pub async fn create(pool: &SqlitePool, new_book: &Book) -> Result<i64> {
        let result = sqlx::query(
            r#"
            INSERT INTO books (
                name, original_title, publisher_id, format_id, series_id, series_index,
                publication_date, acquisition_date, last_modified_date, isbn, pages, notes,
                has_cover, has_paper, file_link, file_size, file_hash, rating, read_status, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'))
            "#
        )
        .bind(&new_book.name)
        .bind(&new_book.original_title)
        .bind(new_book.publisher_id)
        .bind(new_book.format_id)
        .bind(new_book.series_id)
        .bind(new_book.series_index)
        .bind(new_book.publication_date)
        .bind(new_book.acquisition_date)
        .bind(new_book.last_modified_date)
        .bind(&new_book.isbn)
        .bind(new_book.pages)
        .bind(&new_book.notes)
        .bind(new_book.has_cover)
        .bind(new_book.has_paper)
        .bind(&new_book.file_link)
        .bind(new_book.file_size)
        .bind(&new_book.file_hash)
        .bind(new_book.rating)
        .bind(&new_book.read_status)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Restituisce un libro dato il suo id, oppure None se non esiste.
    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Book>> {
        let book = sqlx::query_as::<_, Book>(r#"SELECT * FROM books WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(book)
    }

    /// Aggiorna i dati di questo libro nel database.
    pub async fn update(&self, pool: &SqlitePool) -> Result<u64> {
        let result = sqlx::query(
            r#"
            UPDATE books SET
                name = ?,
                original_title = ?,
                publisher_id = ?,
                format_id = ?,
                series_id = ?,
                series_index = ?,
                publication_date = ?,
                acquisition_date = ?,
                last_modified_date = strftime('%s', 'now'),
                isbn = ?,
                pages = ?,
                notes = ?,
                has_cover = ?,
                has_paper = ?,
                file_link = ?,
                file_size = ?,
                file_hash = ?,
                rating = ?,
                read_status = ?
            WHERE id = ?
            "#,
        )
        .bind(&self.name)
        .bind(&self.original_title)
        .bind(self.publisher_id)
        .bind(self.format_id)
        .bind(self.series_id)
        .bind(self.series_index)
        .bind(self.publication_date)
        .bind(self.acquisition_date)
        .bind(&self.isbn)
        .bind(self.pages)
        .bind(&self.notes)
        .bind(self.has_cover)
        .bind(self.has_paper)
        .bind(&self.file_link)
        .bind(self.file_size)
        .bind(&self.file_hash)
        .bind(self.rating)
        .bind(&self.read_status)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    /// Cancella un libro dal database per id. Restituisce il numero di righe eliminate.
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<u64> {
        let result = sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    /// Inserisce un batch di libri in una singola transazione.
    pub async fn insert_batch(pool: &SqlitePool, books: &[Book]) -> Result<()> {
        let mut tx = pool.begin().await?;
        for book in books {
            sqlx::query(
                r#"
                INSERT INTO books (
                    name, original_title, publisher_id, format_id, series_id, series_index,
                    publication_date, acquisition_date, last_modified_date, isbn, pages, notes,
                    has_cover, has_paper, file_link, file_size, file_hash, rating, read_status, created_at
                ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, strftime('%s', 'now'))
                "#
            )
            .bind(&book.name)
            .bind(&book.original_title)
            .bind(book.publisher_id)
            .bind(book.format_id)
            .bind(book.series_id)
            .bind(book.series_index)
            .bind(book.publication_date)
            .bind(book.acquisition_date)
            .bind(book.last_modified_date)
            .bind(&book.isbn)
            .bind(book.pages)
            .bind(&book.notes)
            .bind(book.has_cover)
            .bind(book.has_paper)
            .bind(&book.file_link)
            .bind(book.file_size)
            .bind(&book.file_hash)
            .bind(book.rating)
            .bind(&book.read_status)
            .execute(&mut tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Aggiorna un batch di libri (solo i campi principali, per esempio) in una transazione.
    pub async fn update_batch(pool: &SqlitePool, books: &[Book]) -> Result<()> {
        let mut tx = pool.begin().await?;
        for book in books {
            sqlx::query(
                r#"
                UPDATE books SET
                    name = ?,
                    original_title = ?,
                    publisher_id = ?,
                    format_id = ?,
                    series_id = ?,
                    series_index = ?,
                    publication_date = ?,
                    acquisition_date = ?,
                    last_modified_date = strftime('%s', 'now'),
                    isbn = ?,
                    pages = ?,
                    notes = ?,
                    has_cover = ?,
                    has_paper = ?,
                    file_link = ?,
                    file_size = ?,
                    file_hash = ?,
                    rating = ?,
                    read_status = ?
                WHERE id = ?
                "#,
            )
            .bind(&book.name)
            .bind(&book.original_title)
            .bind(book.publisher_id)
            .bind(book.format_id)
            .bind(book.series_id)
            .bind(book.series_index)
            .bind(book.publication_date)
            .bind(book.acquisition_date)
            .bind(&book.isbn)
            .bind(book.pages)
            .bind(&book.notes)
            .bind(book.has_cover)
            .bind(book.has_paper)
            .bind(&book.file_link)
            .bind(book.file_size)
            .bind(&book.file_hash)
            .bind(book.rating)
            .bind(&book.read_status)
            .bind(book.id)
            .execute(&mut tx)
            .await?;
        }
        tx.commit().await?;
        Ok(())
    }

    /// Cancella un batch di libri dati i loro id.
    pub async fn delete_batch(pool: &SqlitePool, ids: &[i64]) -> Result<()> {
        let mut tx = pool.begin().await?;
        for id in ids {
            sqlx::query("DELETE FROM books WHERE id = ?")
                .bind(id)
                .execute(&mut tx)
                .await?;
        }
        tx.commit().await?;
        Ok(())
    }
}
