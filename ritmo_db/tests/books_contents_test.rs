use ritmo_db::models::books_contents::BookContent;
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_books_contents_crud() {
    // Setup in-memory DB and create necessary tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE contents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE books_contents (
            book_id INTEGER NOT NULL,
            content_id INTEGER NOT NULL,
            page_start INTEGER,
            page_end INTEGER,
            PRIMARY KEY (book_id, content_id),
            FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE
        );
    "#).await.unwrap();

    // Insert required references
    let book_id = sqlx::query("INSERT INTO books (name) VALUES ('Libro A')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let content_id = sqlx::query("INSERT INTO contents (name) VALUES ('Capitolo 1')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    // --- CREATE ---
    let new_link = BookContent {
        book_id,
        content_id,
        page_start: Some(5),
        page_end: Some(35),
    };
    BookContent::create(&pool, &new_link).await.unwrap();

    // --- GET ---
    let link = BookContent::get(&pool, book_id, content_id).await.unwrap();
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.book_id, book_id);
    assert_eq!(link.content_id, content_id);
    assert_eq!(link.page_start, Some(5));
    assert_eq!(link.page_end, Some(35));

    // --- LIST BY BOOK ---
    let by_book = BookContent::list_by_book(&pool, book_id).await.unwrap();
    assert_eq!(by_book.len(), 1);

    // --- LIST BY CONTENT ---
    let by_content = BookContent::list_by_content(&pool, content_id).await.unwrap();
    assert_eq!(by_content.len(), 1);

    // --- DELETE ---
    let deleted = BookContent::delete(&pool, book_id, content_id).await.unwrap();
    assert_eq!(deleted, 1);

    let should_be_none = BookContent::get(&pool, book_id, content_id).await.unwrap();
    assert!(should_be_none.is_none());
}