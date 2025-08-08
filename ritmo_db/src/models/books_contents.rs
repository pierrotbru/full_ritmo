use ritmo_core::{BookDto, ContentDto};
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct BookContent {
    pub book_id: i64,
    pub content_id: i64,
}

impl BookContent {
    pub fn from_dto(bookdto: &mut BookDto, contentdto: &ContentDto) -> Option<Self> {
        let book_id = bookdto.id?;
        let content_id = contentdto.id?;

        Some(Self {
            book_id,
            content_id,
        })
    }

    pub async fn create(
        pool: &sqlx::SqlitePool,
        new_link: &BookContent,
    ) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO books_contents (book_id, content_id) VALUES (?, ?)")
            .bind(new_link.book_id)
            .bind(new_link.content_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get(
        pool: &sqlx::SqlitePool,
        book_id: i64,
        content_id: i64,
    ) -> Result<Option<BookContent>, sqlx::Error> {
        let link = sqlx::query_as::<_, BookContent>(
            "SELECT * FROM books_contents WHERE book_id = ? AND content_id = ?",
        )
        .bind(book_id)
        .bind(content_id)
        .fetch_optional(pool)
        .await?;
        Ok(link)
    }

    pub async fn delete(
        pool: &sqlx::SqlitePool,
        book_id: i64,
        content_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM books_contents WHERE book_id = ? AND content_id = ?")
            .bind(book_id)
            .bind(content_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_book(
        pool: &sqlx::SqlitePool,
        book_id: i64,
    ) -> Result<Vec<BookContent>, sqlx::Error> {
        let links =
            sqlx::query_as::<_, BookContent>("SELECT * FROM books_contents WHERE book_id = ?")
                .bind(book_id)
                .fetch_all(pool)
                .await?;
        Ok(links)
    }

    pub async fn list_by_content(
        pool: &sqlx::SqlitePool,
        content_id: i64,
    ) -> Result<Vec<BookContent>, sqlx::Error> {
        let links =
            sqlx::query_as::<_, BookContent>("SELECT * FROM books_contents WHERE content_id = ?")
                .bind(content_id)
                .fetch_all(pool)
                .await?;
        Ok(links)
    }
}
