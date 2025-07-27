use ritmo_db::models::types::Types;

#[tokio::test]
async fn test_formats_crud() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query(
        "CREATE TABLE types (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            description TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now'))
        );",
    )
    .execute(&pool)
    .await
    .unwrap();

    // CREATE
    let id = Types::create(&pool, "autore", Some("autore di stocazzo"))
        .await
        .unwrap();
    assert!(id > 0);

    // READ
    let format = Types::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(format.name, "autore");
    assert_eq!(format.description.as_deref(), Some("autore di stocazzo"));

    // UPDATE
    Types::update(&pool, id, "traduttore", Some("traduci stocazzo"))
        .await
        .unwrap();
    let format = Types::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(format.name, "traduttore");
    assert_eq!(format.description.as_deref(), Some("traduci stocazzo"));

    // DELETE
    Types::delete(&pool, id).await.unwrap();
    let res = Types::get(&pool, id).await.unwrap();
    assert!(res.is_none());
}
