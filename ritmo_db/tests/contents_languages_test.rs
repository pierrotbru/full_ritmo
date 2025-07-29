use ritmo_db::models::contents_languages::{ContentLanguage, NewContentLanguage};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_contents_languages_crud() {
    // Setup in-memory DB and create necessary tables
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
        CREATE TABLE contents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL
        );
        CREATE TABLE running_languages (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            iso_code_2char TEXT NOT NULL,
            iso_code_3char TEXT NOT NULL,
            official_name TEXT NOT NULL,
            language_role TEXT NOT NULL CHECK (language_role IN ('Original', 'Source', 'Actual')),
            created_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            updated_at INTEGER NOT NULL DEFAULT (strftime('%s', 'now')),
            UNIQUE(iso_code_2char, iso_code_3char, language_role)
        );
        CREATE TABLE contents_languages (
            content_id INTEGER NOT NULL,
            language_id INTEGER NOT NULL,
            PRIMARY KEY (content_id, language_id),
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
            FOREIGN KEY (language_id) REFERENCES running_languages(id) ON DELETE CASCADE
        );
    "#).await.unwrap();

    // Insert required references
    let content_id = sqlx::query("INSERT INTO contents (name) VALUES ('Capitolo 1')")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();
    let language_id = sqlx::query("INSERT INTO running_languages (iso_code_2char, iso_code_3char, official_name, language_role, created_at, updated_at) VALUES ('it', 'ita', 'Italiano', 'Original', strftime('%s','now'), strftime('%s','now'))")
        .execute(&pool)
        .await
        .unwrap()
        .last_insert_rowid();

    // --- CREATE ---
    let new_link = NewContentLanguage {
        content_id,
        language_id,
    };
    ContentLanguage::create(&pool, &new_link).await.unwrap();

    // --- GET ---
    let link = ContentLanguage::get(&pool, content_id, language_id).await.unwrap();
    assert!(link.is_some());
    let link = link.unwrap();
    assert_eq!(link.content_id, content_id);
    assert_eq!(link.language_id, language_id);

    // --- LIST BY CONTENT ---
    let by_content = ContentLanguage::list_by_content(&pool, content_id).await.unwrap();
    assert_eq!(by_content.len(), 1);

    // --- LIST BY LANGUAGE ---
    let by_language = ContentLanguage::list_by_language(&pool, language_id).await.unwrap();
    assert_eq!(by_language.len(), 1);

    // --- DELETE ---
    let deleted = ContentLanguage::delete(&pool, content_id, language_id).await.unwrap();
    assert_eq!(deleted, 1);

    let should_be_none = ContentLanguage::get(&pool, content_id, language_id).await.unwrap();
    assert!(should_be_none.is_none());
}