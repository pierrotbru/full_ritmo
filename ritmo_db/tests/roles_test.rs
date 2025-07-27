use ritmo_db::models::roles::Roles;

#[tokio::test]
async fn test_formats_crud() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query(
        "CREATE TABLE roles (
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
    let id = Roles::create(&pool, "autore", Some("autore di stocazzo"))
        .await
        .unwrap();
    assert!(id > 0);

    // READ
    let format = Roles::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(format.name, "autore");
    assert_eq!(format.description.as_deref(), Some("autore di stocazzo"));

    // UPDATE
    Roles::update(&pool, id, "traduttore", Some("traduci stocazzo"))
        .await
        .unwrap();
    let format = Roles::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(format.name, "traduttore");
    assert_eq!(format.description.as_deref(), Some("traduci stocazzo"));

    // DELETE
    Roles::delete(&pool, id).await.unwrap();
    let res = Roles::get(&pool, id).await.unwrap();
    assert!(res.is_none());
}
