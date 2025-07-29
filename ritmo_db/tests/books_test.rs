use ritmo_db::models::books::{Book, NewBook};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_books_crud() {
    // Setup in-memory DB and create books table and reference tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE publishers (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE formats (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE series (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            original_title TEXT,
            publisher_id INTEGER,
            format_id INTEGER,
            series_id INTEGER,
            series_index INTEGER CHECK (series_index > 0),
            publication_date INTEGER,
            acquisition_date INTEGER,
            last_modified_date INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            isbn TEXT,
            pages INTEGER CHECK (pages > 0),
            notes TEXT,
            has_cover INTEGER NOT NULL DEFAULT 0 CHECK (has_cover IN (0, 1)),
            has_paper INTEGER NOT NULL DEFAULT 0 CHECK (has_paper IN (0, 1)),
            file_link TEXT UNIQUE,
            file_size INTEGER,
            file_hash TEXT,
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            FOREIGN KEY (format_id) REFERENCES formats(id) ON DELETE SET NULL,
            FOREIGN KEY (series_id) REFERENCES series(id) ON DELETE SET NULL,
            FOREIGN KEY (publisher_id) REFERENCES publishers(id) ON DELETE SET NULL
        );
    "#).await.unwrap();

    // Insert required references
    let publisher_id = sqlx::query("INSERT INTO publishers (name) VALUES ('Mondadori')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let format_id = sqlx::query("INSERT INTO formats (name) VALUES ('Cartaceo')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let series_id = sqlx::query("INSERT INTO series (name) VALUES ('Oscar Fantastica')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    // --- CREATE ---
    let new_book = NewBook {
        name: "Il Nome del Vento".to_string(),
        original_title: Some("The Name of the Wind".to_string()),
        publisher_id: Some(publisher_id),
        format_id: Some(format_id),
        series_id: Some(series_id),
        series_index: Some(1),
        publication_date: Some(2007),
        acquisition_date: Some(2021),
        isbn: Some("9788804658909".to_string()),
        pages: Some(662),
        notes: Some("Primo libro delle Cronache dell'Assassino del Re".to_string()),
        has_cover: Some(1),
        has_paper: Some(1),
        file_link: Some("file/path/il_nome_del_vento.epub".to_string()),
        file_size: Some(12345678),
        file_hash: Some("abcdef123456".to_string()),
    };
    let id = Book::create(&pool, &new_book).await.unwrap();
    assert!(id > 0);

    // --- READ ---
    let book = Book::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(book.name, "Il Nome del Vento");
    assert_eq!(book.original_title.as_deref(), Some("The Name of the Wind"));
    assert_eq!(book.publisher_id, Some(publisher_id));
    assert_eq!(book.format_id, Some(format_id));
    assert_eq!(book.series_id, Some(series_id));
    assert_eq!(book.series_index, Some(1));
    assert_eq!(book.publication_date, Some(2007));
    assert_eq!(book.acquisition_date, Some(2021));
    assert_eq!(book.isbn.as_deref(), Some("9788804658909"));
    assert_eq!(book.pages, Some(662));
    assert_eq!(book.notes.as_deref(), Some("Primo libro delle Cronache dell'Assassino del Re"));
    assert_eq!(book.has_cover, 1);
    assert_eq!(book.has_paper, 1);
    assert_eq!(book.file_link.as_deref(), Some("file/path/il_nome_del_vento.epub"));
    assert_eq!(book.file_size, Some(12345678));
    assert_eq!(book.file_hash.as_deref(), Some("abcdef123456"));

    // --- UPDATE ---
    let mut book = book;
    book.name = "Il Saggio del Re".to_string();
    book.series_index = Some(2);
    book.notes = Some("Secondo libro delle Cronache dell'Assassino del Re".to_string());
    book.has_cover = 0;
    let updated = book.update(&pool).await.unwrap();
    assert_eq!(updated, 1);

    let book = Book::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(book.name, "Il Saggio del Re");
    assert_eq!(book.series_index, Some(2));
    assert_eq!(book.notes.as_deref(), Some("Secondo libro delle Cronache dell'Assassino del Re"));
    assert_eq!(book.has_cover, 0);

    // --- LIST ALL ---
    let all = Book::list_all(&pool).await.unwrap();
    assert_eq!(all.len(), 1);

    // --- SEARCH ---
    let found = Book::search(&pool, "Saggio").await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].name, "Il Saggio del Re");

    // --- DELETE ---
    let deleted = Book::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let book = Book::get(&pool, id).await.unwrap();
    assert!(book.is_none());
}