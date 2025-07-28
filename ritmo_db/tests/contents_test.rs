use ritmo_db::models::contents::{Content, NewContent};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_contents_crud() {
    // Setup in-memory DB and create contents and types table
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE contents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            original_title TEXT,
            type_id INTEGER,
            publication_date INTEGER,
            pages INTEGER CHECK (pages > 0),
            notes TEXT,
            rating INTEGER CHECK (rating >= 1 AND rating <= 5),
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            FOREIGN KEY (type_id) REFERENCES types(id) ON DELETE SET NULL
        );
    "#).await.unwrap();

    // Insert a type to relate with content
    let type_id = sqlx::query("INSERT INTO types (name) VALUES ('Romanzo')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    // --- CREATE ---
    let new_content = NewContent {
        name: "Il Signore degli Anelli".to_string(),
        original_title: Some("The Lord of the Rings".to_string()),
        type_id: Some(type_id),
        publication_date: Some(1954),
        pages: Some(1200),
        notes: Some("Capolavoro fantasy".to_string()),
        rating: Some(5),
    };
    let id = Content::create(&pool, &new_content).await.unwrap();
    assert!(id > 0);

    // --- READ ---
    let content = Content::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(content.name, "Il Signore degli Anelli");
    assert_eq!(content.original_title.as_deref(), Some("The Lord of the Rings"));
    assert_eq!(content.type_id, Some(type_id));
    assert_eq!(content.pages, Some(1200));
    assert_eq!(content.rating, Some(5));

    // --- UPDATE ---
    let mut content = content;
    content.name = "Il Silmarillion".to_string();
    content.original_title = Some("The Silmarillion".to_string());
    content.pages = Some(800);
    content.rating = Some(4);
    let updated = content.update(&pool).await.unwrap();
    assert_eq!(updated, 1);

    let content = Content::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(content.name, "Il Silmarillion");
    assert_eq!(content.pages, Some(800));
    assert_eq!(content.rating, Some(4));

    // --- LIST ALL ---
    let all = Content::list_all(&pool).await.unwrap();
    assert_eq!(all.len(), 1);

    // --- SEARCH ---
    let found = Content::search(&pool, "Silmarillion").await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].name, "Il Silmarillion");

    // --- DELETE ---
    let deleted = Content::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let content = Content::get(&pool, id).await.unwrap();
    assert!(content.is_none());
}