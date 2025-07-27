use ritmo_db::models::formats::Formats;

#[tokio::test]
async fn test_formats_crud() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query(
        "CREATE TABLE formats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    // CREATE
    let id = Formats::create(&pool, "PDF", Some("Portable Document Format")).await.unwrap();
    assert!(id > 0);

    // READ
    let format = Formats::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(format.name, "PDF");
    assert_eq!(format.description.as_deref(), Some("Portable Document Format"));

    // UPDATE
    Formats::update(&pool, id, "EPUB", Some("Electronic Publication")).await.unwrap();
    let format = Formats::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(format.name, "EPUB");
    assert_eq!(format.description.as_deref(), Some("Electronic Publication"));

    // DELETE
    Formats::delete(&pool, id).await.unwrap();
    let res = Formats::get(&pool, id).await.unwrap();
    assert!(res.is_none());
}