use ritmo_db::models::series::{Series, NewSeries};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_series_crud() {
    // Setup in-memory DB and create series table
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            description TEXT,
            total_books INTEGER,
            completed INTEGER NOT NULL DEFAULT 0 CHECK (completed IN (0, 1)),
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );
    "#).await.unwrap();

    // --- CREATE ---
    let new_series = NewSeries {
        name: "Oscar Fantastica".to_string(),
        description: Some("Collana fantasy Mondadori".to_string()),
        total_books: Some(12),
        completed: Some(0),
    };
    let id = Series::create(&pool, &new_series).await.unwrap();
    assert!(id > 0);

    // --- READ ---
    let series = Series::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(series.name, "Oscar Fantastica");
    assert_eq!(series.description.as_deref(), Some("Collana fantasy Mondadori"));
    assert_eq!(series.total_books, Some(12));
    assert_eq!(series.completed, 0);

    // --- UPDATE ---
    let mut series = series;
    series.name = "Oscar Draghi".to_string();
    series.description = Some("Nuova collana fantasy".to_string());
    series.total_books = Some(20);
    series.completed = 1;
    let updated = series.update(&pool).await.unwrap();
    assert_eq!(updated, 1);

    let series = Series::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(series.name, "Oscar Draghi");
    assert_eq!(series.completed, 1);

    // --- LIST ALL ---
    let all = Series::list_all(&pool).await.unwrap();
    assert_eq!(all.len(), 1);

    // --- SEARCH ---
    let found = Series::search(&pool, "Draghi").await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].name, "Oscar Draghi");

    // --- DELETE ---
    let deleted = Series::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let series = Series::get(&pool, id).await.unwrap();
    assert!(series.is_none());
}