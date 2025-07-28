use ritmo_db::models::contents_tags::ContentTag;
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_contents_tags_crud() {
    // Setup in-memory DB and create necessary tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE contents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE contents_tags (
            content_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (content_id, tag_id),
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );
    "#).await.unwrap();

    // Insert required references
    let content_id = sqlx::query("INSERT INTO contents (name) VALUES ('Libro A')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let tag_id = sqlx::query("INSERT INTO tags (name) VALUES ('Fantasy')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    // --- CREATE ---
    let new_link = ContentTag {
        content_id,
        tag_id,
    };
    ContentTag::create(&pool, &new_link).await.unwrap();

    // --- GET ---
    let link = ContentTag::get(&pool, content_id, tag_id).await.unwrap();
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.content_id, content_id);
    assert_eq!(link.tag_id, tag_id);

    // --- LIST BY CONTENT ---
    let by_content = ContentTag::list_by_content(&pool, content_id).await.unwrap();
    assert_eq!(by_content.len(), 1);

    // --- LIST BY TAG ---
    let by_tag = ContentTag::list_by_tag(&pool, tag_id).await.unwrap();
    assert_eq!(by_tag.len(), 1);

    // --- DELETE ---
    let deleted = ContentTag::delete(&pool, content_id, tag_id).await.unwrap();
    assert_eq!(deleted, 1);

    let should_be_none = ContentTag::get(&pool, content_id, tag_id).await.unwrap();
    assert!(should_be_none.is_none());
}