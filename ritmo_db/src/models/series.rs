use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Series {
    pub id: i64,
    pub name: String,
    pub description: Option<String>,
    pub total_books: Option<i64>,
    pub completed: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct NewSeries {
    pub name: String,
    pub description: Option<String>,
    pub total_books: Option<i64>,
    pub completed: Option<i64>,
}

impl Series {
    pub async fn create(pool: &sqlx::SqlitePool, new_series: &NewSeries) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let completed = new_series.completed.unwrap_or(0);
        let result = sqlx::query(
            "INSERT INTO series (name, description, total_books, completed, created_at, updated_at)
             VALUES (?, ?, ?, ?, ?, ?)"
        )
        .bind(&new_series.name)
        .bind(&new_series.description)
        .bind(&new_series.total_books)
        .bind(completed)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Series>, sqlx::Error> {
        let series = sqlx::query_as::<_, Series>(
            "SELECT * FROM series WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(series)
    }

    pub async fn get_by_name(pool: &sqlx::SqlitePool, name: &str) -> Result<Option<Series>, sqlx::Error> {
        let series = sqlx::query_as::<_, Series>(
            "SELECT * FROM series WHERE name = ?"
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(series)
    }

    pub async fn update(&self, pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "UPDATE series SET name = ?, description = ?, total_books = ?, completed = ?, updated_at = ? WHERE id = ?"
        )
        .bind(&self.name)
        .bind(&self.description)
        .bind(&self.total_books)
        .bind(self.completed)
        .bind(now)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM series WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_all(pool: &sqlx::SqlitePool) -> Result<Vec<Series>, sqlx::Error> {
        let all = sqlx::query_as::<_, Series>(
            "SELECT * FROM series ORDER BY name"
        )
        .fetch_all(pool)
        .await?;
        Ok(all)
    }

    pub async fn search(pool: &sqlx::SqlitePool, pattern: &str) -> Result<Vec<Series>, sqlx::Error> {
        let search_pattern = format!("%{}%", pattern);
        let found = sqlx::query_as::<_, Series>(
            "SELECT * FROM series WHERE name LIKE ? OR description LIKE ? ORDER BY name"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(found)
    }
}