use chrono::Utc;
use ritmo_core::dto::BookDto;
use sqlx::FromRow;

#[derive(Debug, Clone, FromRow, Default)]
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
    pub last_modified_date: i64,
    pub isbn: Option<String>,
    pub pages: Option<i64>,
    pub notes: Option<String>,
    pub has_cover: i64,
    pub has_paper: i64,
    pub file_link: Option<String>,
    pub file_size: Option<i64>,
    pub file_hash: Option<String>,
    pub created_at: i64,
}

impl Book {
    // Metodo per la conversione da DTO al modello
    pub fn from_dto(dto: &BookDto) -> Self {
        let now = Utc::now().timestamp();

        Self {
            name: dto.name.clone(),
            original_title: dto.original_title.clone(),
            format_id: dto.format_id,
            series_id: dto.series_id,
            series_index: dto.series_index,
            publication_date: dto.publication_date,
            acquisition_date: dto.acquisition_date,
            last_modified_date: now,
            isbn: dto.isbn.clone(),
            notes: dto.notes.clone(),
            has_paper: if dto.has_paper { 1 } else { 0 },
            has_cover: if dto.has_cover { 1 } else { 0 },
            created_at: now,
            ..Default::default()
        }
    }

    pub async fn create(pool: &sqlx::SqlitePool, new_book: &Book) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "INSERT INTO books (
                name, original_title, publisher_id, format_id, series_id, series_index,
                publication_date, acquisition_date, last_modified_date, isbn, pages, notes,
                has_cover, has_paper, file_link, file_size, file_hash, created_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)",
        )
        .bind(&new_book.name)
        .bind(&new_book.original_title)
        .bind(&new_book.publisher_id)
        .bind(&new_book.format_id)
        .bind(&new_book.series_id)
        .bind(&new_book.series_index)
        .bind(&new_book.publication_date)
        .bind(&new_book.acquisition_date)
        .bind(now)
        .bind(&new_book.isbn)
        .bind(&new_book.pages)
        .bind(&new_book.notes)
        .bind(&new_book.has_cover)
        .bind(&new_book.has_paper)
        .bind(&new_book.file_link)
        .bind(&new_book.file_size)
        .bind(&new_book.file_hash)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Book>, sqlx::Error> {
        let book = sqlx::query_as::<_, Book>("SELECT * FROM books WHERE id = ?")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(book)
    }

    pub async fn update(&self, pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "UPDATE books SET
                name = ?, original_title = ?, publisher_id = ?, format_id = ?, series_id = ?, series_index = ?,
                publication_date = ?, acquisition_date = ?, last_modified_date = ?, isbn = ?, pages = ?, notes = ?,
                has_cover = ?, has_paper = ?, file_link = ?, file_size = ?, file_hash = ?
            WHERE id = ?"
        )
        .bind(&self.name)
        .bind(&self.original_title)
        .bind(&self.publisher_id)
        .bind(&self.format_id)
        .bind(&self.series_id)
        .bind(&self.series_index)
        .bind(&self.publication_date)
        .bind(&self.acquisition_date)
        .bind(now)
        .bind(&self.isbn)
        .bind(&self.pages)
        .bind(&self.notes)
        .bind(self.has_cover)
        .bind(self.has_paper)
        .bind(&self.file_link)
        .bind(&self.file_size)
        .bind(&self.file_hash)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query("DELETE FROM books WHERE id = ?")
            .bind(id)
            .execute(pool)
            .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_all(pool: &sqlx::SqlitePool) -> Result<Vec<Book>, sqlx::Error> {
        let all = sqlx::query_as::<_, Book>("SELECT * FROM books ORDER BY name")
            .fetch_all(pool)
            .await?;
        Ok(all)
    }

    pub async fn search(pool: &sqlx::SqlitePool, pattern: &str) -> Result<Vec<Book>, sqlx::Error> {
        let search_pattern = format!("%{}%", pattern);
        let found = sqlx::query_as::<_, Book>(
            "SELECT * FROM books WHERE name LIKE ? OR original_title LIKE ? OR notes LIKE ? OR isbn LIKE ? ORDER BY name"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(found)
    }
}
