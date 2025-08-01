use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Alias {
    pub id: i64,
    pub name: String,
    pub person_id: i64,
    pub alias_normalized: Option<String>,
    pub confidence: f64,
    pub created_at: i64,
}

#[derive(Debug)]
pub struct NewAlias {
    pub name: String,
    pub person_id: i64,
    pub alias_normalized: Option<String>,
    pub confidence: Option<f64>, // if None, use 0.9 as default
}

impl Alias {
    pub async fn create(pool: &sqlx::SqlitePool, new_alias: &NewAlias) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let confidence = new_alias.confidence.unwrap_or(0.9);
        let result = sqlx::query(
            "INSERT INTO aliases (name, person_id, alias_normalized, confidence, created_at) VALUES (?, ?, ?, ?, ?)"
        )
        .bind(&new_alias.name)
        .bind(new_alias.person_id)
        .bind(&new_alias.alias_normalized)
        .bind(confidence)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Alias>, sqlx::Error> {
        let alias = sqlx::query_as::<_, Alias>(
            "SELECT * FROM aliases WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(alias)
    }

    pub async fn get_by_person_and_name(pool: &sqlx::SqlitePool, person_id: i64, name: &str) -> Result<Option<Alias>, sqlx::Error> {
        let alias = sqlx::query_as::<_, Alias>(
            "SELECT * FROM aliases WHERE person_id = ? AND name = ?"
        )
        .bind(person_id)
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(alias)
    }

    pub async fn update(&self, pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "UPDATE aliases SET name = ?, person_id = ?, alias_normalized = ?, confidence = ? WHERE id = ?"
        )
        .bind(&self.name)
        .bind(self.person_id)
        .bind(&self.alias_normalized)
        .bind(self.confidence)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM aliases WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_by_person(pool: &sqlx::SqlitePool, person_id: i64) -> Result<Vec<Alias>, sqlx::Error> {
        let aliases = sqlx::query_as::<_, Alias>(
            "SELECT * FROM aliases WHERE person_id = ? ORDER BY name"
        )
        .bind(person_id)
        .fetch_all(pool)
        .await?;
        Ok(aliases)
    }

    pub async fn search(pool: &sqlx::SqlitePool, pattern: &str) -> Result<Vec<Alias>, sqlx::Error> {
        let search_pattern = format!("%{}%", pattern);
        let aliases = sqlx::query_as::<_, Alias>(
            "SELECT * FROM aliases WHERE name LIKE ? OR alias_normalized LIKE ? ORDER BY name"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(aliases)
    }
}