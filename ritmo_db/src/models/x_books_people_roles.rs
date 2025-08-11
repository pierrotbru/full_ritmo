use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, PartialEq, Eq)]
pub struct BookPersonRole {
    pub book_id: i64,
    pub person_id: i64,
    pub role_id: i64,
}

impl BookPersonRole {
    pub async fn create(pool: &sqlx::SqlitePool, new_link: &BookPersonRole) -> Result<(), sqlx::Error> {
        sqlx::query(
            "INSERT INTO books_people_roles (book_id, person_id, role_id) VALUES (?, ?, ?)"
        )
        .bind(new_link.book_id)
        .bind(new_link.person_id)
        .bind(new_link.role_id)
        .execute(pool)
        .await?;
        Ok(())
    }

    pub async fn get(pool: &sqlx::SqlitePool, book_id: i64, person_id: i64, role_id: i64) -> Result<Option<BookPersonRole>, sqlx::Error> {
        let link = sqlx::query_as::<_, BookPersonRole>(
            "SELECT * FROM books_people_roles WHERE book_id = ? AND person_id = ? AND role_id = ?"
        )
        .bind(book_id)
        .bind(person_id)
        .bind(role_id)
        .fetch_optional(pool)
        .await?;
        Ok(link)
    }

    pub async fn delete(pool: &sqlx::SqlitePool, book_id: i64, person_id: i64, role_id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM books_people_roles WHERE book_id = ? AND person_id = ? AND role_id = ?"
        )
        .bind(book_id)
        .bind(person_id)
        .bind(role_id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_book(pool: &sqlx::SqlitePool, book_id: i64) -> Result<Vec<BookPersonRole>, sqlx::Error> {
        let links = sqlx::query_as::<_, BookPersonRole>(
            "SELECT * FROM books_people_roles WHERE book_id = ?"
        )
        .bind(book_id)
        .fetch_all(pool)
        .await?;
        Ok(links)
    }

    pub async fn list_by_person(pool: &sqlx::SqlitePool, person_id: i64) -> Result<Vec<BookPersonRole>, sqlx::Error> {
        let links = sqlx::query_as::<_, BookPersonRole>(
            "SELECT * FROM books_people_roles WHERE person_id = ?"
        )
        .bind(person_id)
        .fetch_all(pool)
        .await?;
        Ok(links)
    }

    pub async fn list_by_role(pool: &sqlx::SqlitePool, role_id: i64) -> Result<Vec<BookPersonRole>, sqlx::Error> {
        let links = sqlx::query_as::<_, BookPersonRole>(
            "SELECT * FROM books_people_roles WHERE role_id = ?"
        )
        .bind(role_id)
        .fetch_all(pool)
        .await?;
        Ok(links)
    }
}