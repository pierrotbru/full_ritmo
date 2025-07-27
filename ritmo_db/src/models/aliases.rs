use ritmo_errors::RitmoResult;
use sqlx::{query, FromRow, SqlitePool};

#[derive(Debug, Clone, FromRow)]
pub struct Aliases {
    pub id: i64,
    pub name: Option<String>,
    pub person_id: Option<i64>,
    pub alias_normalized: String,
    pub confidence: f64,
    pub created_at: i64,
}

impl Aliases {
    pub async fn create(pool: &SqlitePool, new_alias: &Aliases) -> RitmoResult<i64> {
        let result = query(
            r#"
            INSERT INTO aliases (
                name, person_id, alias_normalized, confidence, created_at
            ) VALUES (?, ?, ?, ?, strftime('%s', 'now'))
            "#,
        )
        .bind(&new_alias.name)
        .bind(&new_alias.person_id)
        .bind(&new_alias.alias_normalized)
        .bind(&new_alias.confidence)
        .execute(pool)
        .await?;

        Ok(result.last_insert_rowid())
    }

    /// Restituisce un libro dato il suo id, oppure None se non esiste.
    pub async fn get(pool: &SqlitePool, id: i64) -> RitmoResult<Option<Aliases>> {
        let alias = sqlx::query_as::<_, Aliases>(r#"SELECT * FROM aliases WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(alias)
    }

    pub async fn update(pool: &sqlx::SqlitePool, id: i64, name: &str) -> Result<(), sqlx::Error> {
        sqlx::query!("UPDATE aliases SET name = ? WHERE id = ?", name, id)
            .execute(pool)
            .await?;
        Ok(())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<(), sqlx::Error> {
        sqlx::query!("DELETE FROM aliases WHERE id = ?", id)
            .execute(pool)
            .await?;
        Ok(())
    }
}
