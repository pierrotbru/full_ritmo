use ritmo_db::models::aliases::{Alias, NewAlias};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_aliases_crud() {
    // Setup in-memory DB and create minimal tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE people (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE aliases (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            person_id INTEGER NOT NULL,
            alias_normalized TEXT,
            confidence REAL NOT NULL DEFAULT 0.9 CHECK (confidence >= 0.0 AND confidence <= 1.0),
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
            UNIQUE(person_id, name)
        );
    "#).await.unwrap();

    // Insert a person to relate with the alias
    let person_id = sqlx::query("INSERT INTO people (name) VALUES ('Mario Rossi')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    // --- CREATE ---
    let new_alias = NewAlias {
        name: "M. Rossi".to_string(),
        person_id,
        alias_normalized: Some("mario rossi".to_string()),
        confidence: Some(0.95),
    };
    let id = Alias::create(&pool, &new_alias).await.unwrap();
    assert!(id > 0);

    // --- READ ---
    let alias = Alias::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(alias.name, "M. Rossi");
    assert_eq!(alias.person_id, person_id);
    assert_eq!(alias.alias_normalized.as_deref(), Some("mario rossi"));
    assert!((alias.confidence - 0.95).abs() < f64::EPSILON);

    // --- UPDATE ---
    let mut alias = alias;
    alias.name = "Mario R.".to_string();
    alias.confidence = 0.9;
    let updated = alias.update(&pool).await.unwrap();
    assert_eq!(updated, 1);

    let alias = Alias::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(alias.name, "Mario R.");

    // --- LIST BY PERSON ---
    let all = Alias::list_by_person(&pool, person_id).await.unwrap();
    assert_eq!(all.len(), 1);

    // --- SEARCH ---
    let found = Alias::search(&pool, "Mario").await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].name, "Mario R.");

    // --- DELETE ---
    let deleted = Alias::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let alias = Alias::get(&pool, id).await.unwrap();
    assert!(alias.is_none());
}