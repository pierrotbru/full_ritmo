use ritmo_db::models::publishers::{Publisher, NewPublisher};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_publishers_crud() {
    // Setup in-memory DB and create publishers table
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE publishers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            country TEXT,
            website TEXT,
            notes TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );
    "#).await.unwrap();

    // --- CREATE ---
    let new_publisher = NewPublisher {
        name: "Einaudi".to_string(),
        country: Some("Italia".to_string()),
        website: Some("https://www.einaudi.it".to_string()),
        notes: Some("Casa editrice storica".to_string()),
    };
    let id = Publisher::create(&pool, &new_publisher).await.unwrap();
    assert!(id > 0);

    // --- READ ---
    let publisher = Publisher::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(publisher.name, "Einaudi");
    assert_eq!(publisher.country.as_deref(), Some("Italia"));
    assert_eq!(publisher.website.as_deref(), Some("https://www.einaudi.it"));
    assert_eq!(publisher.notes.as_deref(), Some("Casa editrice storica"));

    // --- UPDATE ---
    let mut publisher = publisher;
    publisher.name = "Mondadori".to_string();
    publisher.country = Some("Italia".to_string());
    publisher.website = Some("https://www.mondadori.it".to_string());
    publisher.notes = Some("Un altro editore importante".to_string());
    let updated = publisher.update(&pool).await.unwrap();
    assert_eq!(updated, 1);

    let publisher = Publisher::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(publisher.name, "Mondadori");
    assert_eq!(publisher.website.as_deref(), Some("https://www.mondadori.it"));

    // --- LIST ALL ---
    let all = Publisher::list_all(&pool).await.unwrap();
    assert_eq!(all.len(), 1);

    // --- SEARCH ---
    let found = Publisher::search(&pool, "Mondadori").await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].name, "Mondadori");

    // --- DELETE ---
    let deleted = Publisher::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let publisher = Publisher::get(&pool, id).await.unwrap();
    assert!(publisher.is_none());
}