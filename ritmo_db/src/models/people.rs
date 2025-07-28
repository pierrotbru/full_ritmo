use sqlx::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<i64>,
    pub death_date: Option<i64>,
    pub biography: Option<String>,
    pub normalized_key: Option<String>,
    pub confidence: f64,
    pub source: String,
    pub verified: i64,
    pub created_at: i64,
    pub updated_at: i64,
}

#[derive(Debug)]
pub struct NewPerson {
    pub name: String,
    pub display_name: Option<String>,
    pub given_name: Option<String>,
    pub surname: Option<String>,
    pub middle_names: Option<String>,
    pub title: Option<String>,
    pub suffix: Option<String>,
    pub nationality: Option<String>,
    pub birth_date: Option<i64>,
    pub death_date: Option<i64>,
    pub biography: Option<String>,
    pub normalized_key: Option<String>,
    pub confidence: Option<f64>,
    pub source: Option<String>,
    pub verified: Option<i64>,
}

impl Person {
    pub async fn create(pool: &sqlx::SqlitePool, new_person: &NewPerson) -> Result<i64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let confidence = new_person.confidence.unwrap_or(1.0);
        let source = new_person.source.clone().unwrap_or_else(|| "biblioteca".to_string());
        let verified = new_person.verified.unwrap_or(0);
        let result = sqlx::query(
            "INSERT INTO people (
                name, display_name, given_name, surname, middle_names, title, suffix, nationality,
                birth_date, death_date, biography, normalized_key, confidence, source, verified,
                created_at, updated_at
            ) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&new_person.name)
        .bind(&new_person.display_name)
        .bind(&new_person.given_name)
        .bind(&new_person.surname)
        .bind(&new_person.middle_names)
        .bind(&new_person.title)
        .bind(&new_person.suffix)
        .bind(&new_person.nationality)
        .bind(&new_person.birth_date)
        .bind(&new_person.death_date)
        .bind(&new_person.biography)
        .bind(&new_person.normalized_key)
        .bind(confidence)
        .bind(source)
        .bind(verified)
        .bind(now)
        .bind(now)
        .execute(pool)
        .await?;
        Ok(result.last_insert_rowid())
    }

    pub async fn get(pool: &sqlx::SqlitePool, id: i64) -> Result<Option<Person>, sqlx::Error> {
        let person = sqlx::query_as::<_, Person>(
            "SELECT * FROM people WHERE id = ?"
        )
        .bind(id)
        .fetch_optional(pool)
        .await?;
        Ok(person)
    }

    pub async fn update(&self, pool: &sqlx::SqlitePool) -> Result<u64, sqlx::Error> {
        let now = chrono::Utc::now().timestamp();
        let result = sqlx::query(
            "UPDATE people SET
                name = ?, display_name = ?, given_name = ?, surname = ?, middle_names = ?, title = ?, suffix = ?,
                nationality = ?, birth_date = ?, death_date = ?, biography = ?, normalized_key = ?, confidence = ?,
                source = ?, verified = ?, updated_at = ?
            WHERE id = ?"
        )
        .bind(&self.name)
        .bind(&self.display_name)
        .bind(&self.given_name)
        .bind(&self.surname)
        .bind(&self.middle_names)
        .bind(&self.title)
        .bind(&self.suffix)
        .bind(&self.nationality)
        .bind(&self.birth_date)
        .bind(&self.death_date)
        .bind(&self.biography)
        .bind(&self.normalized_key)
        .bind(self.confidence)
        .bind(&self.source)
        .bind(self.verified)
        .bind(now)
        .bind(self.id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn delete(pool: &sqlx::SqlitePool, id: i64) -> Result<u64, sqlx::Error> {
        let result = sqlx::query(
            "DELETE FROM people WHERE id = ?"
        )
        .bind(id)
        .execute(pool)
        .await?;
        Ok(result.rows_affected())
    }

    pub async fn list_all(pool: &sqlx::SqlitePool) -> Result<Vec<Person>, sqlx::Error> {
        let all = sqlx::query_as::<_, Person>(
            "SELECT * FROM people ORDER BY name"
        )
        .fetch_all(pool)
        .await?;
        Ok(all)
    }

    pub async fn search(pool: &sqlx::SqlitePool, pattern: &str) -> Result<Vec<Person>, sqlx::Error> {
        let search_pattern = format!("%{}%", pattern);
        let found = sqlx::query_as::<_, Person>(
            "SELECT * FROM people WHERE name LIKE ? OR display_name LIKE ? OR given_name LIKE ? OR surname LIKE ? OR biography LIKE ? ORDER BY name"
        )
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await?;
        Ok(found)
    }
}