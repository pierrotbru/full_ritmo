use anyhow::Result;
use serde::{Deserialize, Serialize};
use sqlx::SqlitePool;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub normalized_key: Option<String>,
    pub confidence: Option<f64>,
    pub nationality: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub source: Option<String>,
    pub verified: Option<i64>,
    pub created_at: Option<i64>,
    pub updated_at: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewPerson {
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<String>,
    pub death_date: Option<String>,
    pub source: Option<String>,
    pub verified: Option<i64>,
}

impl Person {
    /// Crea un nuovo record Person e restituisce il suo id.
    pub async fn create(pool: &SqlitePool, new_person: &NewPerson) -> Result<i64> {
        let now = chrono::Utc::now().timestamp();
        
        let result = sqlx::query(
            r#"
            INSERT INTO people (
                name, display_name, given_name, surname, middle_names, 
                nationality, birth_date, death_date, source, verified, 
                created_at, updated_at
            )
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#
        )
        .bind(&new_person.name)
        .bind(&new_person.display_name)
        .bind(&new_person.given_name)
        .bind(&new_person.surname)
        .bind(&new_person.middle_names)
        .bind(&new_person.nationality)
        .bind(&new_person.birth_date)
        .bind(&new_person.death_date)
        .bind(&new_person.source)
        .bind(&new_person.verified)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        
        Ok(result.last_insert_rowid())
    }

    /// Restituisce un record Person dato il suo id, oppure None se non esiste.
    pub async fn get(pool: &SqlitePool, id: i64) -> Result<Option<Person>> {
        let person = sqlx::query_as::<_, Person>(r#"SELECT * FROM people WHERE id = ?"#)
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(person)
    }

    /// Restituisce un record Person dato il suo nome, oppure None se non esiste.
    pub async fn get_by_name(pool: &SqlitePool, name: &str) -> Result<Option<Person>> {
        let person = sqlx::query_as::<_, Person>(r#"SELECT * FROM people WHERE name = ?"#)
            .bind(name)
            .fetch_optional(pool)
            .await?;
        Ok(person)
    }

    /// Aggiorna i dati di questo record Person nel database.
    pub async fn update(&self, pool: &SqlitePool) -> Result<u64> {
        let now = chrono::Utc::now().timestamp();
        
        let result = sqlx::query(
            r#"
            UPDATE people
            SET name = ?, 
                display_name = ?, 
                given_name = ?, 
                surname = ?, 
                middle_names = ?,
                normalized_key = ?,
                confidence = ?, 
                nationality = ?, 
                birth_date = ?, 
                death_date = ?, 
                source = ?, 
                verified = ?, 
                updated_at = ?
            WHERE id = ?
            "#
        )
        .bind(&self.name)
        .bind(&self.display_name)
        .bind(&self.given_name)
        .bind(&self.surname)
        .bind(&self.middle_names)
        .bind(&self.normalized_key)
        .bind(self.confidence)
        .bind(&self.nationality)
        .bind(&self.birth_date)
        .bind(&self.death_date)
        .bind(&self.source)
        .bind(self.verified)
        .bind(now)
        .bind(self.id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Cancella un record Person dal database per id. Restituisce il numero di righe eliminate.
    pub async fn delete(pool: &SqlitePool, id: i64) -> Result<u64> {
        let result = sqlx::query(
            r#"DELETE FROM people WHERE id = ?"#
        )
        .bind(id)
        .execute(pool)
        .await?;
        
        Ok(result.rows_affected())
    }

    /// Elenca tutte le persone.
    pub async fn list_all(pool: &SqlitePool) -> Result<Vec<Person>> {
        let people = sqlx::query_as::<_, Person>(r#"SELECT * FROM people ORDER BY name"#)
            .fetch_all(pool)
            .await?;
        Ok(people)
    }

    /// Cerca persone in base a un pattern di ricerca.
    pub async fn search(pool: &SqlitePool, pattern: &str) -> Result<Vec<Person>> {
        let search_pattern = format!("%{}%", pattern);
        let people = sqlx::query_as::<_, Person>(
            r#"SELECT * FROM people WHERE name LIKE ? OR display_name LIKE ? ORDER BY name"#
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(people)
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
        
        // Crea la tabella people per i test
        sqlx::query(
            r#"
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
            )
            "#
        )
        .execute(&pool)
        .await?;
        
        Ok(pool)
    }
    
    #[tokio::test]
    async fn test_person_crud_operations() -> Result<()> {
        let pool = setup_test_db().await?;
        
        // Test: Creazione di una persona
        let new_person = NewPerson {
            name: "Isaac Asimov".to_string(),
            display_name: Some("I. Asimov".to_string()),
            given_name: Some("Isaac".to_string()),
            surname: Some("Asimov".to_string()),
            middle_names: None,
            nationality: Some("American".to_string()),
            birth_date: Some("1920-01-02".to_string()),
            death_date: Some("1992-04-06".to_string()),
            source: Some("Wikipedia".to_string()),
            verified: Some(1),
        };
        
        let person_id = Person::create(&pool, &new_person).await?;
        assert!(person_id > 0, "L'ID della persona dovrebbe essere positivo");
        
        // Test: Recupero di una persona per ID
        let person_opt = Person::get(&pool, person_id).await?;
        assert!(person_opt.is_some(), "La persona dovrebbe esistere");
        
        let person = person_opt.unwrap();
        assert_eq!(person.name, "Isaac Asimov", "Il nome della persona dovrebbe corrispondere");
        assert_eq!(person.nationality, Some("American".to_string()), "La nazionalità dovrebbe corrispondere");
        
        // Test: Recupero di una persona per nome
        let person_by_name = Person::get_by_name(&pool, "Isaac Asimov").await?;
        assert!(person_by_name.is_some(), "La persona dovrebbe essere trovata per nome");
        
        // Test: Aggiornamento di una persona
        let mut updated_person = person;
        updated_person.nationality = Some("Russian-American".to_string());
        updated_person.verified = Some(1);
        
        let rows_affected = updated_person.update(&pool).await?;
        assert_eq!(rows_affected, 1, "Una riga dovrebbe essere stata aggiornata");
        
        // Verifica che l'aggiornamento sia avvenuto
        let updated_person_opt = Person::get(&pool, person_id).await?;
        assert!(updated_person_opt.is_some(), "La persona dovrebbe esistere dopo l'aggiornamento");
        assert_eq!(
            updated_person_opt.unwrap().nationality, 
            Some("Russian-American".to_string()), 
            "La nazionalità dovrebbe essere aggiornata"
        );
        
        // Test: Aggiunta di una seconda persona
        let new_person2 = NewPerson {
            name: "Arthur C. Clarke".to_string(),
            display_name: Some("A. C. Clarke".to_string()),
            given_name: Some("Arthur".to_string()),
            surname: Some("Clarke".to_string()),
            middle_names: Some("Charles".to_string()),
            nationality: Some("British".to_string()),
            birth_date: Some("1917-12-16".to_string()),
            death_date: Some("2008-03-19".to_string()),
            source: Some("Wikipedia".to_string()),
            verified: Some(1),
        };
        
        Person::create(&pool, &new_person2).await?;
        
        // Test: Ricerca di persone
        let search_results = Person::search(&pool, "Arthur").await?;
        assert_eq!(search_results.len(), 1, "Dovrebbe trovare una persona con 'Arthur'");
        
        let all_people = Person::list_all(&pool).await?;
        assert_eq!(all_people.len(), 2, "Dovrebbero esserci due persone in totale");
        
        // Test: Eliminazione di una persona
        let rows_deleted = Person::delete(&pool, person_id).await?;
        assert_eq!(rows_deleted, 1, "Una riga dovrebbe essere stata eliminata");
        
        // Verifica che la persona sia stata eliminata
        let deleted_person = Person::get(&pool, person_id).await?;
        assert!(deleted_person.is_none(), "La persona non dovrebbe esistere dopo l'eliminazione");
        
        Ok(())
    }
}