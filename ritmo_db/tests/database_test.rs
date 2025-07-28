use ritmo_db::database::Database;
use ritmo_db::books::Book;
use tempfile::TempDir;

fn sample_book() -> Book {
    Book {
        name: "Test Book".to_string(),
        ..Default::default()
    }
}

#[tokio::test]
async fn test_create_and_open_database() {
    let temp_dir = TempDir::new().unwrap();
    dbg!(&temp_dir);
    let db_path = temp_dir.path().join("test_ritmo.db");
    dbg!(&db_path);
    // Test creazione
    let db = Database::create(&db_path).await.expect("Database creation failed");
    dbg!(&db);
    assert!(db_path.exists());

    // Test apertura
    let db2 = Database::open(&db_path).await.expect("Database open failed");
    dbg!(&db2);
    assert_eq!(db2.metadata().version, db.metadata().version);
}

#[tokio::test]
async fn test_add_and_get_book() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("test_books.db");
    let db = Database::create(&db_path).await.expect("Database creation failed");

    let new_book = sample_book();
    let book_id = db.add_book(&new_book).await.expect("Add book failed");
    dbg!(&book_id);

    let book = db.get_book(book_id).await.expect("Get book failed");
    assert!(book.is_some());
    let book = book.unwrap();
    assert_eq!(book.name, new_book.name);
}

#[tokio::test]
async fn test_delete_book() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("db_delete.db");
    let db = Database::create(&db_path).await.expect("Database creation failed");

    let new_book = sample_book();
    let book_id = db.add_book(&new_book).await.expect("Add book failed");

    let deleted = db.delete_book(book_id).await.expect("Delete failed");
    assert_eq!(deleted, 1);

    let book = db.get_book(book_id).await.expect("Get book failed");
    assert!(book.is_none());
}

#[tokio::test]
async fn test_backup_database() {
    let temp_dir = TempDir::new().unwrap();
    let db_path = temp_dir.path().join("db_original.db");
    let backup_path = temp_dir.path().join("db_backup.db");
    let db = Database::create(&db_path).await.expect("Database creation failed");

    db.backup_to(&backup_path).await.expect("Backup failed");
    assert!(backup_path.exists());
}