use sqlx::SqlitePool;
use ritmo_db::models::books::Book;
//use sqlx::{FromRow, Result, Sqlite, SqlitePool, Transaction, Execute};
use sqlx::Executor;

fn get_memory_pool() -> SqlitePool {
    // Usa una connessione in-memory per i test
    SqlitePool::connect_lazy("sqlite::memory:").unwrap()
}

async fn setup_schema(pool: &SqlitePool) {
    pool.execute(
        r#"
        CREATE TABLE books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            original_title TEXT,
            publisher_id INTEGER,
            format_id INTEGER,
            series_id INTEGER,
            series_index INTEGER,
            publication_date INTEGER,
            acquisition_date INTEGER,
            last_modified_date INTEGER,
            isbn TEXT,
            pages INTEGER,
            notes TEXT,
            has_cover BOOLEAN NOT NULL,
            has_paper BOOLEAN NOT NULL,
            file_link TEXT,
            file_size INTEGER,
            file_hash TEXT,
            rating INTEGER,
            read_status TEXT,
            created_at INTEGER
        );
        "#,
    )
    .await
    .unwrap();
}

#[tokio::test]
async fn test_book_crud() {
    let pool = get_memory_pool();
    setup_schema(&pool).await;

    // Creo un nuovo libro
    let book = Book {
        id: 0,
        name: "Il Nome della Rosa".to_string(),
        original_title: Some("Der Name der Rose".to_string()),
        publisher_id: None,
        format_id: None,
        series_id: None,
        series_index: None,
        publication_date: Some(1980),
        acquisition_date: None,
        last_modified_date: 123,
        isbn: Some("9788806170343".to_string()),
        pages: Some(500),
        notes: Some("Romanzo storico".to_string()),
        has_cover: 1,
        has_paper: 0,
        file_link: None,
        file_size: None,
        file_hash: None,
        rating: Some(5),
        read_status: Some("letto".to_string()),
        created_at: 123,
    };

    // Insert
    let id = Book::create(&pool, &book).await.unwrap();

    // Get
    let book_db = Book::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(book_db.name, "Il Nome della Rosa");
    assert_eq!(book_db.rating, Some(5));

    // Update
    let mut book_mod = book_db.clone();
    book_mod.name = "Il Nome della Rosa - Edito".to_string();
    book_mod.rating = Some(4);
    let n = book_mod.update(&pool).await.unwrap();
    assert_eq!(n, 1);

    let book_upd = Book::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(book_upd.name, "Il Nome della Rosa - Edito");
    assert_eq!(book_upd.rating, Some(4));

    // Delete
    let deleted = Book::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let book_none = Book::get(&pool, id).await.unwrap();
    assert!(book_none.is_none());
}

// Puoi aggiungere altri test per insert_batch, update_batch, delete_batch...
