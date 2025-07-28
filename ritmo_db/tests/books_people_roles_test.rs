use ritmo_db::models::books_people_roles::BookPersonRole;
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_books_people_roles_crud() {
    // Setup in-memory DB and create necessary tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE people (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE books_people_roles (
            book_id INTEGER NOT NULL,
            person_id INTEGER NOT NULL,
            role_id INTEGER NOT NULL,
            PRIMARY KEY (book_id, person_id, role_id),
            FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
            FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
        );
    "#).await.unwrap();

    // Insert required references
    let book_id = sqlx::query("INSERT INTO books (name) VALUES ('Libro A')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let person_id = sqlx::query("INSERT INTO people (name) VALUES ('Mario Rossi')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let role_id = sqlx::query("INSERT INTO roles (name) VALUES ('Autore')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    // --- CREATE ---
    let new_link = BookPersonRole {
        book_id,
        person_id,
        role_id,
    };
    BookPersonRole::create(&pool, &new_link).await.unwrap();

    // --- GET ---
    let link = BookPersonRole::get(&pool, book_id, person_id, role_id).await.unwrap();
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.book_id, book_id);
    assert_eq!(link.person_id, person_id);
    assert_eq!(link.role_id, role_id);

    // --- LIST BY BOOK ---
    let by_book = BookPersonRole::list_by_book(&pool, book_id).await.unwrap();
    assert_eq!(by_book.len(), 1);

    // --- LIST BY PERSON ---
    let by_person = BookPersonRole::list_by_person(&pool, person_id).await.unwrap();
    assert_eq!(by_person.len(), 1);

    // --- LIST BY ROLE ---
    let by_role = BookPersonRole::list_by_role(&pool, role_id).await.unwrap();
    assert_eq!(by_role.len(), 1);

    // --- DELETE ---
    let deleted = BookPersonRole::delete(&pool, book_id, person_id, role_id).await.unwrap();
    assert_eq!(deleted, 1);

    let should_be_none = BookPersonRole::get(&pool, book_id, person_id, role_id).await.unwrap();
    assert!(should_be_none.is_none());
}