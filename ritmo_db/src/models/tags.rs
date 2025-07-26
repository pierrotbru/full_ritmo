use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Tag {
    pub id: i64,
    pub name: String,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewTag {
    pub name: String,
}

impl Tag {
    /// Crea un nuovo record Tag e restituisce il suo id.
    pub async fn create(pool: &SqlitePool, new_tag: &NewTag) -> Result<i64> {
        let now = chrono::Utc::now().timestamp();
        
        let result = sqlx::query(
            r#"
            INSERT INTO tags (name, created_at, updated_at)
            VALUES (?, ?, ?)
            "#
        )
        .bind(&new_tag.name)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }

    /// Restituisce un record Tag dato il suo id, oppure None se non esiste.
    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Tag>> {
        let tag = sqlx::query_as::<_, Tag>(r#"SELECT * FROM tags WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(tag)
    }

    /// Restituisce un record Tag dato il suo nome, oppure None se non esiste.
    pub async fn get_by_name(pool: &SqlitePool, name: &str) -> Result<Option<Tag>> {
        let tag = sqlx::query_as::<_, Tag>(r#"SELECT * FROM tags WHERE name = ?"#)
            .bind(name)
            .fetch_optional(pool)
            .await?;
        Ok(tag)
    }

    /// Aggiorna i dati di questo record Tag nel database.
    pub async fn update(&self, pool: &SqlitePool) -> Result<u64> {
        let now = chrono::Utc::now().timestamp();
        
        let result = sqlx::query(
            r#"
            UPDATE tags
            SET name = ?, updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&self.name)
        .bind(now)
        .bind(self.id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Cancella un record Tag dal database per id. Restituisce il numero di righe eliminate.
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<u64> {
        let result = sqlx::query(
            r#"DELETE FROM tags WHERE id = ?"#
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Elenca tutti i tag.
    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Tag>> {
        let tags = sqlx::query_as::<_, Tag>(r#"SELECT * FROM tags ORDER BY name"#)
            .fetch_all(pool)
            .await?;
        Ok(tags)
    }

    /// Cerca tag in base a un pattern di ricerca.
    pub async fn search(pool: &SqlitePool, pattern: &str) -> Result<Vec<Tag>> {
        let search_pattern = format!("%{}%", pattern);
        let tags = sqlx::query_as::<_, Tag>(
            r#"SELECT * FROM tags WHERE name LIKE ? ORDER BY name"#
        )
        .bind(search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(tags)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use sqlx::sqlite::SqlitePoolOptions;

    async fn setup_test_db() -> Result<SqlitePool> {
        // Utilizziamo SQLite in memoria per i test
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect("sqlite::memory:")
            .await?;
        
        // Crea la tabella tags per i test
        sqlx::query(
            r#"
            CREATE TABLE tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                created_at INTEGER,
                updated_at INTEGER
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        Ok(pool)
    }
    
    #[tokio::test]
    async fn test_tag_crud_operations() -> Result<()> {
        let pool = setup_test_db().await?;
        
        // Test: Creazione di un tag
        let new_tag = NewTag {
            name: "Fantascienza".to_string(),
        };
        
        let tag_id = Tag::create(&pool, &new_tag).await?;
        assert!(tag_id > 0, "L'ID del tag dovrebbe essere positivo");
        
        // Test: Recupero di un tag per ID
        let tag_opt = Tag::get(&pool, tag_id).await?;
        assert!(tag_opt.is_some(), "Il tag dovrebbe esistere");
        
        let tag = tag_opt.unwrap();
        assert_eq!(tag.name, "Fantascienza", "Il nome del tag dovrebbe corrispondere");
        
        // Test: Recupero di un tag per nome
        let tag_by_name = Tag::get_by_name(&pool, "Fantascienza").await?;
        assert!(tag_by_name.is_some(), "Il tag dovrebbe essere trovato per nome");
        
        // Test: Aggiornamento di un tag
        let mut updated_tag = tag;
        updated_tag.name = "Science Fiction".to_string();
        
        let rows_affected = updated_tag.update(&pool).await?;
        assert_eq!(rows_affected, 1, "Una riga dovrebbe essere stata aggiornata");
        
        // Verifica che l'aggiornamento sia avvenuto
        let updated_tag_opt = Tag::get(&pool, tag_id).await?;
        assert!(updated_tag_opt.is_some(), "Il tag dovrebbe esistere dopo l'aggiornamento");
        assert_eq!(updated_tag_opt.unwrap().name, "Science Fiction", "Il nome dovrebbe essere aggiornato");
        
        // Test: Ricerca di tag
        let tag2 = NewTag {
            name: "Fantasy".to_string(),
        };
        Tag::create(&pool, &tag2).await?;
        
        let search_results = Tag::search(&pool, "Sci").await?;
        assert_eq!(search_results.len(), 1, "Dovrebbe trovare un tag con 'Sci'");
        
        let all_tags = Tag::list_all(&pool).await?;
        assert_eq!(all_tags.len(), 2, "Dovrebbero esserci due tag in totale");
        
        // Test: Eliminazione di un tag
        let rows_deleted = Tag::delete(&pool, tag_id).await?;
        assert_eq!(rows_deleted, 1, "Una riga dovrebbe essere stata eliminata");
        
        // Verifica che il tag sia stato eliminato
        let deleted_tag = Tag::get(&pool, tag_id).await?;
        assert!(deleted_tag.is_none(), "Il tag non dovrebbe esistere dopo l'eliminazione");
        
        Ok(())
    }
}