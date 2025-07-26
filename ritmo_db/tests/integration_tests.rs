use anyhow::Result;
use sqlx::{sqlite::SqlitePoolOptions, SqlitePool};

// Importa i moduli dal crate ritmo_db
use ritmo_db::models::tags::{Tag, NewTag};
use ritmo_db::models::people::{Person, NewPerson};
use ritmo_db::models::relations::{ContentTagRelation, BookTagRelation, ContentPersonRoleRelation, BookPersonRoleRelation};

async fn setup_test_db() -> Result<SqlitePool> {
    // Utilizziamo SQLite in memoria per i test
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite::memory:")
        .await?;
    
    // Crea le tabelle necessarie per i test
    sqlx::query(
        r#"
        CREATE TABLE contents (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            parent_id INTEGER,
            description TEXT,
            content_type TEXT,
            created_at INTEGER,
            updated_at INTEGER
        );

        CREATE TABLE tags (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at INTEGER,
            updated_at INTEGER
        );

        CREATE TABLE people (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            display_name TEXT,
            given_name TEXT,
            surname TEXT,
            middle_names TEXT,
            normalized_key TEXT,
            confidence REAL,
            nationality TEXT,
            birth_date TEXT,
            death_date TEXT,
            source TEXT,
            verified INTEGER,
            created_at INTEGER,
            updated_at INTEGER
        );

        CREATE TABLE roles (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            created_at INTEGER,
            updated_at INTEGER
        );

        CREATE TABLE books (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL,
            created_at INTEGER,
            updated_at INTEGER
        );

        CREATE TABLE contents_tags (
            content_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (content_id, tag_id),
            FOREIGN KEY (content_id) REFERENCES contents(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
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

        CREATE TABLE books_tags (
            book_id INTEGER NOT NULL,
            tag_id INTEGER NOT NULL,
            PRIMARY KEY (book_id, tag_id),
            FOREIGN KEY (book_id) REFERENCES books(id) ON DELETE CASCADE,
            FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
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
        "#
    )
    .execute(&pool)
    .await?;
    
    // Inserisci un contenuto e un libro per i test
    sqlx::query("INSERT INTO contents (id, name, created_at, updated_at) VALUES (1, 'Fondazione', strftime('%s', 'now'), strftime('%s', 'now'))")
        .execute(&pool)
        .await?;
        
    sqlx::query("INSERT INTO books (id, name, created_at, updated_at) VALUES (1, 'Fondazione e Terra', strftime('%s', 'now'), strftime('%s', 'now'))")
        .execute(&pool)
        .await?;
        
    // Inserisci un ruolo per i test
    sqlx::query("INSERT INTO roles (id, name, created_at, updated_at) VALUES (1, 'Autore', strftime('%s', 'now'), strftime('%s', 'now'))")
        .execute(&pool)
        .await?;
    
    Ok(pool)
}

#[tokio::test]
async fn test_full_workflow() -> Result<()> {
    let pool = setup_test_db().await?;
    
    // 1. Crea due tag
    let sci_fi_tag = NewTag {
        name: "Fantascienza".to_string(),
    };
    
    let classic_tag = NewTag {
        name: "Classico".to_string(),
    };
    
    let sci_fi_id = Tag::create(&pool, &sci_fi_tag).await?;
    let classic_id = Tag::create(&pool, &classic_tag).await?;
    
    // 2. Crea un autore
    let asimov = NewPerson {
        name: "Isaac Asimov".to_string(),
        display_name: Some("I. Asimov".to_string()),
        given_name: Some("Isaac".to_string()),
        surname: Some("Asimov".to_string()),
        middle_names: None,
        nationality: Some("American".to_string()),
        birth_date: Some("1920-01-02".to_string()),
        death_date: Some("1992-04-06".to_string()),
        source: Some("Wikipedia".to_string()),
        verified: Some(1),
    };
    
    let asimov_id = Person::create(&pool, &asimov).await?;
    
    // 3. Associa tag ai contenuti
    ContentTagRelation::create(&pool, 1, sci_fi_id).await?;
    ContentTagRelation::create(&pool, 1, classic_id).await?;
    
    // 4. Associa tag ai libri
    BookTagRelation::create(&pool, 1, sci_fi_id).await?;
    
    // 5. Associa l'autore ai contenuti
    ContentPersonRoleRelation::create(&pool, 1, asimov_id, 1).await?;
    
    // 6. Associa l'autore ai libri
    BookPersonRoleRelation::create(&pool, 1, asimov_id, 1).await?;
    
    // 7. Verifica le relazioni
    let content_tags = ContentTagRelation::get_tags_for_content(&pool, 1).await?;
    assert_eq!(content_tags.len(), 2, "Il contenuto dovrebbe avere due tag");
    
    let book_tags = BookTagRelation::get_tags_for_book(&pool, 1).await?;
    assert_eq!(book_tags.len(), 1, "Il libro dovrebbe avere un tag");
    
    let content_people = ContentPersonRoleRelation::get_people_for_content(&pool, 1).await?;
    assert_eq!(content_people.len(), 1, "Il contenuto dovrebbe avere un autore");
    
    let book_people = BookPersonRoleRelation::get_people_for_book(&pool, 1).await?;
    assert_eq!(book_people.len(), 1, "Il libro dovrebbe avere un autore");
    
    // 8. Verifica la ricerca di tag
    let sci_fi_tags = Tag::search(&pool, "Fanta").await?;
    assert_eq!(sci_fi_tags.len(), 1, "Dovrebbe trovare un tag con 'Fanta'");
    assert_eq!(sci_fi_tags[0].name, "Fantascienza", "Il nome del tag dovrebbe essere 'Fantascienza'");
    
    // 9. Verifica la ricerca di persone
    let asimov_search = Person::search(&pool, "Asimov").await?;
    assert_eq!(asimov_search.len(), 1, "Dovrebbe trovare una persona con 'Asimov'");
    assert_eq!(asimov_search[0].name, "Isaac Asimov", "Il nome della persona dovrebbe essere 'Isaac Asimov'");
    
    // 10. Aggiorna un tag
    let mut sci_fi_tag_obj = Tag::get(&pool, sci_fi_id).await?.unwrap();
    sci_fi_tag_obj.name = "Science Fiction".to_string();
    sci_fi_tag_obj.update(&pool).await?;
    
    let updated_tag = Tag::get(&pool, sci_fi_id).await?.unwrap();
    assert_eq!(updated_tag.name, "Science Fiction", "Il nome del tag dovrebbe essere aggiornato");
    
    // 11. Elimina le relazioni e poi i tag e le persone
    ContentTagRelation::delete(&pool, 1, sci_fi_id).await?;
    ContentTagRelation::delete(&pool, 1, classic_id).await?;
    BookTagRelation::delete(&pool, 1, sci_fi_id).await?;
    
    ContentPersonRoleRelation::delete(&pool, 1, asimov_id, 1).await?;
    BookPersonRoleRelation::delete(&pool, 1, asimov_id, 1).await?;
    
    Tag::delete(&pool, sci_fi_id).await?;
    Tag::delete(&pool, classic_id).await?;
    Person::delete(&pool, asimov_id).await?;
    
    // 12. Verifica che tutto sia stato eliminato
    assert!(Tag::get(&pool, sci_fi_id).await?.is_none(), "Il tag sci-fi dovrebbe essere eliminato");
    assert!(Tag::get(&pool, classic_id).await?.is_none(), "Il tag classic dovrebbe essere eliminato");
    assert!(Person::get(&pool, asimov_id).await?.is_none(), "La persona dovrebbe essere eliminata");
    
    Ok(())
}