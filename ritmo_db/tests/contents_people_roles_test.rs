//use ritmo_db::models::contents_people_roles::{ContentPersonRole, NewContentPersonRole};
use ritmo_db::models::contents_people_roles::ContentPersonRole;
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_contents_people_roles_crud() {
    // Setup in-memory DB and create necessary tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE contents (
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
        CREATE TABLE contents_people_roles (
            content_id INTEGER NOT NULL,
            person_id INTEGER NOT NULL,
            role_id INTEGER NOT NULL,
            PRIMARY KEY (content_id, person_id, role_id),
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
            FOREIGN KEY (person_id) REFERENCES people(id) ON DELETE CASCADE,
            FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE
        );
    "#).await.unwrap();

    // Insert required references
    let content_id = sqlx::query("INSERT INTO contents (name) VALUES ('Libro A')")
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
    let new_link = ContentPersonRole {
        content_id,
        person_id,
        role_id,
    };
    ContentPersonRole::create(&pool, &new_link).await.unwrap();

    // --- GET ---
    let link = ContentPersonRole::get(&pool, content_id, person_id, role_id).await.unwrap();
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.content_id, content_id);
    assert_eq!(link.person_id, person_id);
    assert_eq!(link.role_id, role_id);

    // --- LIST BY CONTENT ---
    let by_content = ContentPersonRole::list_by_content(&pool, content_id).await.unwrap();
    assert_eq!(by_content.len(), 1);

    // --- LIST BY PERSON ---
    let by_person = ContentPersonRole::list_by_person(&pool, person_id).await.unwrap();
    assert_eq!(by_person.len(), 1);

    // --- LIST BY ROLE ---
    let by_role = ContentPersonRole::list_by_role(&pool, role_id).await.unwrap();
    assert_eq!(by_role.len(), 1);

    // --- DELETE ---
    let deleted = ContentPersonRole::delete(&pool, content_id, person_id, role_id).await.unwrap();
    assert_eq!(deleted, 1);

    let should_be_none = ContentPersonRole::get(&pool, content_id, person_id, role_id).await.unwrap();
    assert!(should_be_none.is_none());
}