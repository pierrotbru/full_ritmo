use anyhow::Result;
use sqlx::SqlitePool;

pub struct ContentTagRelation;
pub struct ContentPersonRoleRelation;
pub struct BookTagRelation;
pub struct BookPersonRoleRelation;

impl ContentTagRelation {
    /// Associa un tag a un contenuto
    pub async fn create(pool: &SqlitePool, content_id: i64, tag_id: i64) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO contents_tags (content_id, tag_id)
            VALUES (?, ?)
            "#
        )
        .bind(content_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }

    /// Rimuove l'associazione tra un tag e un contenuto
    pub async fn delete(pool: &SqlitePool, content_id: i64, tag_id: i64) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM contents_tags
            WHERE content_id = ? AND tag_id = ?
            "#
        )
        .bind(content_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Ottiene tutti i tag associati a un contenuto
    pub async fn get_tags_for_content(pool: &SqlitePool, content_id: i64) -> Result<Vec<i64>> {
        #[derive(sqlx::FromRow)]
        struct TagId {
            tag_id: i64,
        }
        
        let tag_ids = sqlx::query_as::<_, TagId>(
            r#"
            SELECT tag_id FROM contents_tags
            WHERE content_id = ?
            "#
        )
        .bind(content_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|record| record.tag_id)
        .collect();
        
        Ok(tag_ids)
    }

    /// Ottiene tutti i contenuti associati a un tag
    pub async fn get_contents_for_tag(pool: &SqlitePool, tag_id: i64) -> Result<Vec<i64>> {
        #[derive(sqlx::FromRow)]
        struct ContentId {
            content_id: i64,
        }
        
        let content_ids = sqlx::query_as::<_, ContentId>(
            r#"
            SELECT content_id FROM contents_tags
            WHERE tag_id = ?
            "#
        )
        .bind(tag_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|record| record.content_id)
        .collect();
        
        Ok(content_ids)
    }
}

impl ContentPersonRoleRelation {
    /// Associa una persona con un ruolo a un contenuto
    pub async fn create(
        pool: &SqlitePool, 
        content_id: i64, 
        person_id: i64, 
        role_id: i64
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO contents_people_roles (content_id, person_id, role_id)
            VALUES (?, ?, ?)
            "#
        )
        .bind(content_id)
        .bind(person_id)
        .bind(role_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }

    /// Rimuove l'associazione tra una persona con un ruolo e un contenuto
    pub async fn delete(
        pool: &SqlitePool, 
        content_id: i64, 
        person_id: i64, 
        role_id: i64
    ) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM contents_people_roles
            WHERE content_id = ? AND person_id = ? AND role_id = ?
            "#
        )
        .bind(content_id)
        .bind(person_id)
        .bind(role_id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Ottiene tutte le persone con ruolo associati a un contenuto
    pub async fn get_people_for_content(
        pool: &SqlitePool, 
        content_id: i64
    ) -> Result<Vec<(i64, i64)>> {
        #[derive(sqlx::FromRow)]
        struct PersonRole {
            person_id: i64,
            role_id: i64,
        }
        
        let relations = sqlx::query_as::<_, PersonRole>(
            r#"
            SELECT person_id, role_id FROM contents_people_roles
            WHERE content_id = ?
            "#
        )
        .bind(content_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|record| (record.person_id, record.role_id))
        .collect();
        
        Ok(relations)
    }
}

impl BookTagRelation {
    /// Associa un tag a un libro
    pub async fn create(pool: &SqlitePool, book_id: i64, tag_id: i64) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO books_tags (book_id, tag_id)
            VALUES (?, ?)
            "#
        )
        .bind(book_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }

    /// Rimuove l'associazione tra un tag e un libro
    pub async fn delete(pool: &SqlitePool, book_id: i64, tag_id: i64) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM books_tags
            WHERE book_id = ? AND tag_id = ?
            "#
        )
        .bind(book_id)
        .bind(tag_id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Ottiene tutti i tag associati a un libro
    pub async fn get_tags_for_book(pool: &SqlitePool, book_id: i64) -> Result<Vec<i64>> {
        #[derive(sqlx::FromRow)]
        struct TagId {
            tag_id: i64,
        }
        
        let tag_ids = sqlx::query_as::<_, TagId>(
            r#"
            SELECT tag_id FROM books_tags
            WHERE book_id = ?
            "#
        )
        .bind(book_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|record| record.tag_id)
        .collect();
        
        Ok(tag_ids)
    }
}

impl BookPersonRoleRelation {
    /// Associa una persona con un ruolo a un libro
    pub async fn create(
        pool: &SqlitePool, 
        book_id: i64, 
        person_id: i64, 
        role_id: i64
    ) -> Result<()> {
        sqlx::query(
            r#"
            INSERT OR IGNORE INTO books_people_roles (book_id, person_id, role_id)
            VALUES (?, ?, ?)
            "#
        )
        .bind(book_id)
        .bind(person_id)
        .bind(role_id)
        .execute(pool)
        .await?;
        
        Ok(())
    }

    /// Rimuove l'associazione tra una persona con un ruolo e un libro
    pub async fn delete(
        pool: &SqlitePool, 
        book_id: i64, 
        person_id: i64, 
        role_id: i64
    ) -> Result<u64> {
        let result = sqlx::query(
            r#"
            DELETE FROM books_people_roles
            WHERE book_id = ? AND person_id = ? AND role_id = ?
            "#
        )
        .bind(book_id)
        .bind(person_id)
        .bind(role_id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Ottiene tutte le persone con ruolo associati a un libro
    pub async fn get_people_for_book(
        pool: &SqlitePool, 
        book_id: i64
    ) -> Result<Vec<(i64, i64)>> {
        #[derive(sqlx::FromRow)]
        struct PersonRole {
            person_id: i64,
            role_id: i64,
        }
        
        let relations = sqlx::query_as::<_, PersonRole>(
            r#"
            SELECT person_id, role_id FROM books_people_roles
            WHERE book_id = ?
            "#
        )
        .bind(book_id)
        .fetch_all(pool)
        .await?
        .into_iter()
        .map(|record| (record.person_id, record.role_id))
        .collect();
        
        Ok(relations)
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
        
        // Crea le tabelle necessarie per i test
        sqlx::query(
            r#"
            CREATE TABLE contents (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                parent_id INTEGER,
                description TEXT,
                content_type TEXT,
                created_at INTEGER,
                updated_at INTEGER
            );

            CREATE TABLE tags (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                created_at INTEGER,
                updated_at INTEGER
            );

            CREATE TABLE people (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                display_name TEXT,
                given_name TEXT,
                surname TEXT,
                middle_names TEXT,
                normalized_key TEXT,
                confidence REAL,
                nationality TEXT,
                birth_date TEXT,
                death_date TEXT,
                source TEXT,
                verified INTEGER,
                created_at INTEGER,
                updated_at INTEGER
            );

            CREATE TABLE roles (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                created_at INTEGER,
                updated_at INTEGER
            );

            CREATE TABLE books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                created_at INTEGER,
                updated_at INTEGER
            );

            CREATE TABLE contents_tags (
                content_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                PRIMARY KEY (content_id, tag_id),
                FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            );

            CREATE TABLE contents_people_roles (
                content_id INTEGER NOT NULL,
                person_id INTEGER NOT NULL,
                role_id INTEGER NOT NULL,
                PRIMARY KEY (content_id, person_id, role_id),
                FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
                FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
                FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
            );

            CREATE TABLE books_tags (
                book_id INTEGER NOT NULL,
                tag_id INTEGER NOT NULL,
                PRIMARY KEY (book_id, tag_id),
                FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
                FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
            );

            CREATE TABLE books_people_roles (
                book_id INTEGER NOT NULL,
                person_id INTEGER NOT NULL,
                role_id INTEGER NOT NULL,
                PRIMARY KEY (book_id, person_id, role_id),
                FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
                FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
                FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
            );
            "#
        )
        .execute(&pool)
        .await?;
        
        // Inserisci dati di base per i test
        sqlx::query("INSERT INTO contents (id, name, created_at, updated_at) VALUES (1, 'Test Content', strftime('%s', 'now'), strftime('%s', 'now'))")
            .execute(&pool)
            .await?;
            
        sqlx::query("INSERT INTO books (id, name, created_at, updated_at) VALUES (1, 'Test Book', strftime('%s', 'now'), strftime('%s', 'now'))")
            .execute(&pool)
            .await?;
            
        sqlx::query("INSERT INTO tags (id, name, created_at, updated_at) VALUES (1, 'Test Tag', strftime('%s', 'now'), strftime('%s', 'now'))")
            .execute(&pool)
            .await?;
            
        sqlx::query("INSERT INTO people (id, name, created_at, updated_at) VALUES (1, 'Test Person', strftime('%s', 'now'), strftime('%s', 'now'))")
            .execute(&pool)
            .await?;
            
        sqlx::query("INSERT INTO roles (id, name, created_at, updated_at) VALUES (1, 'Author', strftime('%s', 'now'), strftime('%s', 'now'))")
            .execute(&pool)
            .await?;
        
        Ok(pool)
    }
    
    #[tokio::test]
    async fn test_content_tag_relation() -> Result<()> {
        let pool = setup_test_db().await?;
        
        // Test: Creazione di una relazione tra contenuto e tag
        ContentTagRelation::create(&pool, 1, 1).await?;
        
        // Test: Ottenere tag per un contenuto
        let tags = ContentTagRelation::get_tags_for_content(&pool, 1).await?;
        assert_eq!(tags.len(), 1, "Dovrebbe esserci un tag associato al contenuto");
        assert_eq!(tags[0], 1, "L'ID del tag dovrebbe essere 1");
        
        // Test: Ottenere contenuti per un tag
        let contents = ContentTagRelation::get_contents_for_tag(&pool, 1).await?;
        assert_eq!(contents.len(), 1, "Dovrebbe esserci un contenuto associato al tag");
        assert_eq!(contents[0], 1, "L'ID del contenuto dovrebbe essere 1");
        
        // Test: Rimozione di una relazione
        let rows_deleted = ContentTagRelation::delete(&pool, 1, 1).await?;
        assert_eq!(rows_deleted, 1, "Una relazione dovrebbe essere stata rimossa");
        
        // Verifica che la relazione sia stata rimossa
        let tags_after = ContentTagRelation::get_tags_for_content(&pool, 1).await?;
        assert_eq!(tags_after.len(), 0, "Non dovrebbero esserci tag associati al contenuto dopo la rimozione");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_content_person_role_relation() -> Result<()> {
        let pool = setup_test_db().await?;
        
        // Test: Creazione di una relazione tra contenuto, persona e ruolo
        ContentPersonRoleRelation::create(&pool, 1, 1, 1).await?;
        
        // Test: Ottenere persone con ruoli per un contenuto
        let people = ContentPersonRoleRelation::get_people_for_content(&pool, 1).await?;
        assert_eq!(people.len(), 1, "Dovrebbe esserci una persona associata al contenuto");
        assert_eq!(people[0], (1, 1), "L'ID della persona e del ruolo dovrebbero essere 1");
        
        // Test: Rimozione di una relazione
        let rows_deleted = ContentPersonRoleRelation::delete(&pool, 1, 1, 1).await?;
        assert_eq!(rows_deleted, 1, "Una relazione dovrebbe essere stata rimossa");
        
        // Verifica che la relazione sia stata rimossa
        let people_after = ContentPersonRoleRelation::get_people_for_content(&pool, 1).await?;
        assert_eq!(people_after.len(), 0, "Non dovrebbero esserci persone associate al contenuto dopo la rimozione");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_book_tag_relation() -> Result<()> {
        let pool = setup_test_db().await?;
        
        // Test: Creazione di una relazione tra libro e tag
        BookTagRelation::create(&pool, 1, 1).await?;
        
        // Test: Ottenere tag per un libro
        let tags = BookTagRelation::get_tags_for_book(&pool, 1).await?;
        assert_eq!(tags.len(), 1, "Dovrebbe esserci un tag associato al libro");
        assert_eq!(tags[0], 1, "L'ID del tag dovrebbe essere 1");
        
        // Test: Rimozione di una relazione
        let rows_deleted = BookTagRelation::delete(&pool, 1, 1).await?;
        assert_eq!(rows_deleted, 1, "Una relazione dovrebbe essere stata rimossa");
        
        // Verifica che la relazione sia stata rimossa
        let tags_after = BookTagRelation::get_tags_for_book(&pool, 1).await?;
        assert_eq!(tags_after.len(), 0, "Non dovrebbero esserci tag associati al libro dopo la rimozione");
        
        Ok(())
    }
    
    #[tokio::test]
    async fn test_book_person_role_relation() -> Result<()> {
        let pool = setup_test_db().await?;
        
        // Test: Creazione di una relazione tra libro, persona e ruolo
        BookPersonRoleRelation::create(&pool, 1, 1, 1).await?;
        
        // Test: Ottenere persone con ruoli per un libro
        let people = BookPersonRoleRelation::get_people_for_book(&pool, 1).await?;
        assert_eq!(people.len(), 1, "Dovrebbe esserci una persona associata al libro");
        assert_eq!(people[0], (1, 1), "L'ID della persona e del ruolo dovrebbero essere 1");
        
        // Test: Rimozione di una relazione
        let rows_deleted = BookPersonRoleRelation::delete(&pool, 1, 1, 1).await?;
        assert_eq!(rows_deleted, 1, "Una relazione dovrebbe essere stata rimossa");
        
        // Verifica che la relazione sia stata rimossa
        let people_after = BookPersonRoleRelation::get_people_for_book(&pool, 1).await?;
        assert_eq!(people_after.len(), 0, "Non dovrebbero esserci persone associate al libro dopo la rimozione");
        
        Ok(())
    }
}