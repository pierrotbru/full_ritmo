use ritmo_db::{
    Database,
    models::{Book, Format, Publisher, Serie, NewSeries}
};
use std::path::{PathBuf, Path};
use std::fs;
use std::io::Write;

/// Inizializza la struttura di directory richiesta da ritmo_db_core
fn initialize_filesystem(base_path: &Path) -> std::io::Result<()> {
    println!("\nPreparazione della struttura filesystem per ritmo_db_core...");
    
    // Directory principali richieste
    let dirs = [
        "books",           // Directory principale per i libri
        "covers",          // Directory per le copertine
        "metadata",        // Directory per i metadati
        "cache",           // Directory per la cache
        "temp"             // Directory temporanea
    ];
    
    // Crea le directory principali
    for dir in dirs.iter() {
        let dir_path = base_path.join(dir);
        fs::create_dir_all(&dir_path)?;
        println!("  ✓ Directory creata: {}", dir_path.display());
    }
    
    // Crea un file di configurazione di esempio
    let config_path = base_path.join("ritmo_config.json");
    let config_content = r#"{
    "version": "1.0.0",
    "database": {
        "path": "library.db",
        "auto_backup": true,
        "backup_interval_days": 7
    },
    "filesystem": {
        "books_directory": "books",
        "covers_directory": "covers",
        "metadata_directory": "metadata",
        "cache_directory": "cache",
        "temp_directory": "temp"
    },
    "user_preferences": {
        "default_language": "it",
        "date_format": "YYYY-MM-DD",
        "theme": "light"
    }
}"#;
    
    let mut file = fs::File::create(&config_path)?;
    file.write_all(config_content.as_bytes())?;
    println!("  ✓ File di configurazione creato: {}", config_path.display());
    
    Ok(())
}

/// Crea un file di esempio per un libro
fn create_book_file(file_path: &Path, book_title: &str) -> std::io::Result<()> {
    // Assicuriamoci che esistano le directory necessarie
    if let Some(parent) = file_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // Creiamo un contenuto di esempio per il file EPUB
    // Qui stiamo creando un semplice file di testo, ma in un'applicazione reale
    // questo sarebbe un file EPUB valido
    let content = format!(
        "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
        <html xmlns=\"http://www.w3.org/1999/xhtml\">\n\
        <head>\n\
        <title>{}</title>\n\
        </head>\n\
        <body>\n\
        <h1>{}</h1>\n\
        <p>Questo è un file EPUB di esempio per il libro '{}'.</p>\n\
        <p>In un'applicazione reale, questo sarebbe un file EPUB completo con tutti i contenuti del libro.</p>\n\
        </body>\n\
        </html>", 
        book_title, book_title, book_title
    );
    
    // Scriviamo il contenuto nel file
    let mut file = fs::File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    
    // Otteniamo la dimensione del file creato
    let metadata = file.metadata()?;
    let file_size = metadata.len();
    
    println!("  ✓ File del libro creato: {} ({} bytes)", file_path.display(), file_size);
    
    // Restituiamo la dimensione del file
    Ok(())
}

/// Crea un file di copertina di esempio per il libro
fn create_cover_image(base_path: &Path, book_hash: &str, book_title: &str) -> std::io::Result<()> {
    // Creiamo un percorso per la copertina usando lo stesso schema di directory dell'hash
    let cover_path = base_path.join("covers")
                             .join(&book_hash[0..2])
                             .join(&book_hash[2..4])
                             .join(format!("{}.jpg", book_hash));
    
    // Assicuriamoci che esistano le directory necessarie
    if let Some(parent) = cover_path.parent() {
        fs::create_dir_all(parent)?;
    }
    
    // In un'applicazione reale, qui genereresti un'immagine di copertina vera
    // Per questo esempio, creiamo un file di testo che simula un'immagine
    let content = format!("Placeholder per la copertina di: {}", book_title);
    
    let mut file = fs::File::create(&cover_path)?;
    file.write_all(content.as_bytes())?;
    
    println!("  ✓ File di copertina creato: {}", cover_path.display());
    
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Esempio: Configurazione completa del database e aggiunta di un libro ===\n");
    
    // Directory base per il nostro esempio
    let base_path = PathBuf::from("./example_data");
    
    // Percorso al database
    let db_path = base_path.join("library.db");
    
    // Creiamo la directory base se non esiste
    fs::create_dir_all(&base_path)?;
    
    // Inizializziamo la struttura del filesystem
    initialize_filesystem(&base_path)?;
    
    // Se il database esiste già, lo rimuoviamo per iniziare da zero
    if db_path.exists() {
        fs::remove_file(&db_path)?;
        println!("\nDatabase esistente rimosso per iniziare da zero.");
    }
    
    // Creiamo un nuovo database con le migrazioni
    println!("Creazione nuovo database in: {}", db_path.display());
    let database = Database::create(&db_path).await?;
    
    // Verifichiamo che il database sia stato inizializzato correttamente
    println!("Database creato con successo (versione: {})", database.metadata().version);
    
    // Otteniamo il pool di connessioni
    let pool = database.get_pool();
    
    // === STEP 1: Verifica/Creazione di entità correlate necessarie ===
    
    println!("\n1. Preparazione entità correlate...");
    
    // Casa editrice
    let publisher_name = "Adelphi";
    let publisher = match Publisher::get_by_name(pool, publisher_name).await? {
        Some(p) => {
            println!("  ✓ Casa editrice trovata: {}", p.name);
            p
        },
        None => {
            let new_publisher = Publisher {
                id: None,
                name: publisher_name.to_string(),
                notes: Some("Casa editrice italiana di qualità".to_string()),
                created_at: chrono::Utc::now().timestamp(),
                updated_at: chrono::Utc::now().timestamp(),
                ..Default::default()
            };
            let publisher_id = Publisher::create(pool, &new_publisher).await?;
            let p = Publisher::get(pool, publisher_id).await?.unwrap();
            println!("  ✓ Nuova casa editrice creata: {}", p.name);
            p
        }
    };
    
    // Serie
    let series_name = "Il cimitero dei libri dimenticati";
    let series = match Serie::get_by_name(pool, series_name).await? {
        Some(s) => {
            println!("  ✓ Serie trovata: {}", s.name);
            s
        },
        None => {
            let new_series = NewSeries {
                name: series_name.to_string(),
                description: Some("Tetralogia di Carlos Ruiz Zafón".to_string()),
                total_books: Some(4),  // Sono 4 libri in totale
                completed: Some(1),    // La serie è completa
            };
            let series_id = Serie::create(pool, &new_series).await?;
            let s = Serie::get(pool, series_id).await?.unwrap();
            println!("  ✓ Nuova serie creata: {}", s.name);
            s
        }
    };
    
    // === STEP 2: Creazione del libro ===
    
    println!("\n2. Creazione dell'oggetto libro...");
    
    // Data di pubblicazione
    let publication_date = chrono::NaiveDate::from_ymd_opt(2001, 5, 17)
        .unwrap()
        .and_hms_opt(0, 0, 0)
        .unwrap()
        .timestamp();
    
    let book_title = "L'ombra del vento";
    
    // Creazione dell'oggetto libro
    let mut book = Book {
        id: None,
        name: book_title.to_string(),
        original_title: Some("La sombra del viento".to_string()),
        publisher_id: publisher.id,
        format_id: Some(0),
        series_id: series.id,
        series_index: Some(1),  // Primo libro della serie
        publication_date: Some(publication_date),
        last_modified_date: chrono::Utc::now().timestamp(),
        isbn: Some("9788845929526".to_string()),
        pages: Some(560),
        notes: Some("Primo libro della serie 'Il cimitero dei libri dimenticati'".to_string()),
        has_cover: 1,  // Ha una copertina
        has_paper: 1,  // È un libro fisico
        file_link: None,  // Sarà generato dalla set_book_persistence
        file_size: None,  // Sarà impostato dalla set_book_persistence
        file_hash: None,  // Sarà generato dalla set_book_persistence
        created_at: chrono::Utc::now().timestamp(),
    };
    
    println!("  ✓ Oggetto libro creato: \"{}\"", book.name);
    
    // === STEP 3: Generazione hash e file link ===
    
    println!("\n3. Generazione hash e file link...");
    book.set_book_persistence();
    
    let hash = book.file_hash.as_ref().unwrap();
    let file_link = book.file_link.as_ref().unwrap();
    
    println!("  ✓ Hash generato: {}", hash);
    println!("  ✓ File link: {}", file_link);
    
    // === STEP 4: Creazione del file del libro ===
    
    println!("\n4. Creazione del file del libro...");
    let file_path = base_path.join(file_link);
    create_book_file(&file_path, book_title)?;
    
    // Aggiorniamo la dimensione del file nel modello del libro
    let metadata = fs::metadata(&file_path)?;
    book.file_size = Some(metadata.len() as i64);
    println!("  ✓ Dimensione del file aggiornata: {} bytes", book.file_size.unwrap());
    
    // === STEP 5: Creazione della copertina ===
    
    println!("\n5. Creazione del file di copertina...");
    create_cover_image(&base_path, hash, book_title)?;
    
    // === STEP 6: Salvataggio nel database ===
    
    println!("\n6. Salvataggio nel database...");
    let book_id = Book::create(pool, &book).await?;
    println!("  ✓ Libro salvato con ID: {}", book_id);
    
    // === STEP 7: Recupero e verifica ===
    
    println!("\n7. Recupero e verifica del libro salvato...");
    let saved_book = Book::get(pool, book_id).await?.unwrap();
    
    println!("\nRiepilogo libro salvato:");
    println!("----------------------------------");
    println!("ID:              {}", saved_book.id.unwrap());
    println!("Titolo:          {}", saved_book.name);
    println!("Titolo originale:{:?}", saved_book.original_title);
    println!("Editore:         ID {:?} ({})", saved_book.publisher_id, publisher.name);
    println!("Serie:           ID {:?} ({}, #{:?})", 
             saved_book.series_id, series.name, saved_book.series_index);
    println!("ISBN:            {:?}", saved_book.isbn);
    println!("Pagine:          {:?}", saved_book.pages);
    println!("File hash:       {:?}", saved_book.file_hash);
    println!("File link:       {:?}", saved_book.file_link);
    println!("File size:       {:?} bytes", saved_book.file_size);
    println!("----------------------------------");
    
    // === STEP 8: Verifica della struttura del filesystem ===
    
    println!("\n8. Riepilogo struttura filesystem creata:");
    println!("  ✓ Database:         {}", db_path.display());
    println!("  ✓ File del libro:   {}", file_path.display());
    println!("  ✓ File di copertina: {}/covers/{}/{}/{}.jpg", 
             base_path.display(), &hash[0..2], &hash[2..4], hash);
    println!("  ✓ File di configurazione: {}/ritmo_config.json", base_path.display());
    
    println!("\n=== Esempio completato con successo! ===");
    println!("Hai ora una configurazione completa di ritmo_db_core con un libro di esempio.");
    
    Ok(())
}