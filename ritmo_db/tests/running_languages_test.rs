use ritmo_db::models::running_languages::{RunningLanguage, NewRunningLanguage};
use sqlx::{SqlitePool, Executor};

#[tokio::test]
async fn test_running_languages_crud() {
    // Setup in-memory DB and create running_languages table
    let pool = SqlitePool::connect(":memory:").await.unwrap();
    pool.execute(r#"
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
    "#).await.unwrap();

    // --- CREATE ---
    let new_language = NewRunningLanguage {
        iso_code_2char: "it".to_string(),
        iso_code_3char: "ita".to_string(),
        official_name: "Italiano".to_string(),
        language_role: "Original".to_string(),
    };
    let id = RunningLanguage::create(&pool, &new_language).await.unwrap();
    assert!(id > 0);

    // --- READ ---
    let language = RunningLanguage::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(language.iso_code_2char, "it");
    assert_eq!(language.iso_code_3char, "ita");
    assert_eq!(language.official_name, "Italiano");
    assert_eq!(language.language_role, "Original");

    // --- UPDATE ---
    let mut language = language;
    language.official_name = "Italian".to_string();
    language.language_role = "Actual".to_string();
    let updated = language.update(&pool).await.unwrap();
    assert_eq!(updated, 1);

    let language = RunningLanguage::get(&pool, id).await.unwrap().unwrap();
    assert_eq!(language.official_name, "Italian");
    assert_eq!(language.language_role, "Actual");

    // --- LIST ALL ---
    let all = RunningLanguage::list_all(&pool).await.unwrap();
    assert_eq!(all.len(), 1);

    // --- SEARCH ---
    let found = RunningLanguage::search(&pool, "Ital").await.unwrap();
    assert_eq!(found.len(), 1);
    assert_eq!(found[0].official_name, "Italian");

    // --- DELETE ---
    let deleted = RunningLanguage::delete(&pool, id).await.unwrap();
    assert_eq!(deleted, 1);

    let language = RunningLanguage::get(&pool, id).await.unwrap();
    assert!(language.is_none());
}