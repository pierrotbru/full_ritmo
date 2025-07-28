use ritmo_db::models::people::{Person, NewPerson};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_people_crud() {
    // Setup in-memory DB and create people table
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE people (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            display_name TEXT,
            given_name TEXT,
            surname TEXT,
            middle_names TEXT,
            title TEXT,
            suffix TEXT,
            nationality TEXT,
            birth_date INTEGER,
            death_date INTEGER,
            biography TEXT,
            normalized_key TEXT,
            confidence REAL NOT NULL DEFAULT 1.0 CHECK (confidence >= 0.0 AND confidence <= 1.0),
            source TEXT NOT NULL DEFAULT 'biblioteca',
            verified INTEGER NOT NULL DEFAULT 0 CHECK (verified IN (0, 1)),
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );
    "#).await.unwrap();

    // --- CREATE ---
    let new_person = NewPerson {
        name: "J.R.R. Tolkien".to_string(),
        display_name: Some("J. Tolkien".to_string()),
        given_name: Some("John Ronald Reuel".to_string()),
        surname: Some("Tolkien".to_string()),
        middle_names: None,
        title: Some("Prof.".to_string()),
        suffix: None,
        nationality: Some("UK".to_string()),
        birth_date: Some(1892),
        death_date: Some(1973),
        biography: Some("Autore fantasy".to_string()),
        normalized_key: Some("tolkien_jrr".to_string()),
        confidence: Some(1.0),
        source: Some("biblioteca".to_string()),
        verified: Some(1),
    };
    let id = Person::create(&pool, &new_person).await.unwrap();
    assert!(id > 0);

    // --- READ ---
    let person = Person::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(person.name, "J.R.R. Tolkien");
    assert_eq!(person.display_name.as_deref(), Some("J. Tolkien"));
    assert_eq!(person.verified, 1);

    // --- UPDATE ---
    let mut person = person;
    person.verified = 0;
    person.surname = Some("Tolkien Jr.".to_string());
    let updated = person.update(&pool).await.unwrap();
    assert_eq!(updated, 1);

    let person = Person::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(person.surname.as_deref(), Some("Tolkien Jr."));
    assert_eq!(person.verified, 0);

    // --- LIST ALL ---
    let all = Person::list_all(&pool).await.unwrap();
    assert_eq!(all.len(), 1);

    // --- SEARCH ---
    let found = Person::search(&pool, "Tolkien").await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].name, "J.R.R. Tolkien");

    // --- DELETE ---
    let deleted = Person::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let person = Person::get(&pool, id).await.unwrap();
    assert!(person.is_none());
}