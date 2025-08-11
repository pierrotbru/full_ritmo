use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct BookTag {
    pub book_id: i64,
    pub tag_id: i64,
}

impl BookTag {
    pub async fn create(pool: &sqlx::SqlitePool, new_link: &BookTag) -> Result<(), sqlx::Error> {
        sqlx::query("INSERT INTO books_tags (book_id, tag_id) VALUES (?, ?)")
            .bind(new_link.book_id)
            .bind(new_link.tag_id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn get(
        pool: &sqlx::SqlitePool,
        book_id: i64,
        tag_id: i64,
    ) -> Result<Option<BookTag>, sqlx::Error> {
        let link = sqlx::query_as::<_, BookTag>(
            "SELECT * FROM books_tags WHERE book_id = ? AND tag_id = ?",
        )
        .bind(book_id)
        .bind(tag_id)
        .fetch_optional(pool)
        .await?;
        Ok(link)
    }

    pub async fn delete(
        pool: &sqlx::SqlitePool,
        book_id: i64,
        tag_id: i64,
    ) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM books_tags WHERE book_id = ? AND tag_id = ?")
            .bind(book_id)
            .bind(tag_id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_book(
        pool: &sqlx::SqlitePool,
        book_id: i64,
    ) -> Result<Vec<BookTag>, sqlx::Error> {
        let links = sqlx::query_as::<_, BookTag>("SELECT * FROM books_tags WHERE book_id = ?")
            .bind(book_id)
            .fetch_all(pool)
            .await?;
        Ok(links)
    }

    pub async fn list_by_tag(
        pool: &sqlx::SqlitePool,
        tag_id: i64,
    ) -> Result<Vec<BookTag>, sqlx::Error> {
        let links = sqlx::query_as::<_, BookTag>("SELECT * FROM books_tags WHERE tag_id = ?")
            .bind(tag_id)
            .fetch_all(pool)
            .await?;
        Ok(links)
    }
}
