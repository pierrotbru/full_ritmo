use ritmo_db::models::tags::Tags;

#[tokio::test]
async fn test_tags_crud() {
    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();
    sqlx::query(
        "CREATE TABLE tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at INTEGER,
            updated_at INTEGER
        );"
    )
    .execute(&pool)
    .await
    .unwrap();

    // CREATE
    let id = Tags::create(&pool, "fiction").await.unwrap();
    assert!(id > 0);

    // READ
    let tag = Tags::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(tag.name, "fiction");

    // UPDATE
    Tags::update(&pool, id, "fantasy").await.unwrap();
    let tag = Tags::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(tag.name, "fantasy");

    // DELETE
    Tags::delete(&pool, id).await.unwrap();
    let res = Tags::get(&pool, id).await.unwrap();
    assert!(res.is_none());
}