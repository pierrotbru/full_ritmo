use ritmo_db::models::books_tags::BookTag;
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_books_tags_crud() {
    // Setup in-memory DB and create necessary tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE books_tags (
            book_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (book_id, tag_id),
            FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
        );
    "#).await.unwrap();

    // Insert required references
    let book_id = sqlx::query("INSERT INTO books (name) VALUES ('Libro A')")
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
    let new_link = BookTag {
        book_id,
        tag_id,
    };
    BookTag::create(&pool, &new_link).await.unwrap();

    // --- GET ---
    let link = BookTag::get(&pool, book_id, tag_id).await.unwrap();
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.book_id, book_id);
    assert_eq!(link.tag_id, tag_id);

    // --- LIST BY BOOK ---
    let by_book = BookTag::list_by_book(&pool, book_id).await.unwrap();
    assert_eq!(by_book.len(), 1);

    // --- LIST BY TAG ---
    let by_tag = BookTag::list_by_tag(&pool, tag_id).await.unwrap();
    assert_eq!(by_tag.len(), 1);

    // --- DELETE ---
    let deleted = BookTag::delete(&pool, book_id, tag_id).await.unwrap();
    assert_eq!(deleted, 1);

    let should_be_none = BookTag::get(&pool, book_id, tag_id).await.unwrap();
    assert!(should_be_none.is_none());
}