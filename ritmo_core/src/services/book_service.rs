use ritmo_db::models::books::{Book, NewBook};
use ritmo_db::models::books_contents::BookContent;
use ritmo_db::models::books_people_roles::BookPersonRole;
use ritmo_db::models::books_tags::BookTag;
use ritmo_db::models::contents::{Content, NewContent};
use ritmo_errors::RitmoErr;
use sqlx::{Pool, Sqlite, Transaction};

/// Servizio che fornisce operazioni di alto livello per i libri
pub struct BookService;

impl BookService {
    /// Salva un nuovo libro completo con tutte le sue relazioni (autori, contenuti, tag)
    /// in un'unica transazione.
    ///
    /// # Arguments
    /// * `pool` - Pool di connessione al database SQLite
    /// * `book_data` - Dati del nuovo libro
    /// * `authors` - Lista di tuple (person_id, role_id) che rappresentano gli autori e i loro ruoli
    /// * `contents` - Lista di tuple (content_id, page_start, page_end) che rappresentano i contenuti del libro
    /// * `tags` - Lista di tag_id da associare al libro
    ///
    /// # Returns
    /// * `Result<i64, RitmoErr>` - ID del libro creato o un errore
    pub async fn save_complete_book(
        pool: &Pool<Sqlite>,
        book_data: NewBook,
        authors: Vec<(i64, i64)>, // (person_id, role_id)
        contents: Vec<(i64, Option<i64>, Option<i64>)>, // (content_id, page_start, page_end)
        tags: Vec<i64>, // tag_ids
    ) -> Result<i64, RitmoErr> {
        // Inizia una transazione per garantire l'atomicità dell'operazione
        let mut tx = pool.begin().await?;

        // 1. Crea il libro
        let book_id = Self::create_book(&mut tx, &book_data).await?;

        // 2. Associa gli autori
        for (person_id, role_id) in authors {
            Self::add_person_role(&mut tx, book_id, person_id, role_id).await?;
        }

        // 3. Associa i contenuti
        for (content_id, page_start, page_end) in contents {
            Self::add_content(&mut tx, book_id, content_id, page_start, page_end).await?;
        }

        // 4. Associa i tag
        for tag_id in tags {
            Self::add_tag(&mut tx, book_id, tag_id).await?;
        }

        // Commit della transazione
        tx.commit().await?;

        Ok(book_id)
    }

    /// Crea un nuovo libro nel database
    async fn create_book(
        tx: &mut Transaction<'_, Sqlite>,
        new_book: &NewBook,
    ) -> Result<i64, RitmoErr> {
        let book_id = Book::create(tx, new_book).await?;
        Ok(book_id)
    }

    /// Associa una persona con un ruolo specifico al libro
    async fn add_person_role(
        tx: &mut Transaction<'_, Sqlite>,
        book_id: i64,
        person_id: i64,
        role_id: i64,
    ) -> Result<(), RitmoErr> {
        let book_person_role = BookPersonRole {
            book_id,
            person_id,
            role_id,
        };
        
        BookPersonRole::create(tx, &book_person_role).await?;
        Ok(())
    }

    /// Associa un contenuto al libro, specificando le pagine di inizio e fine
    async fn add_content(
        tx: &mut Transaction<'_, Sqlite>,
        book_id: i64,
        content_id: i64,
        page_start: Option<i64>,
        page_end: Option<i64>,
    ) -> Result<(), RitmoErr> {
        let book_content = BookContent {
            book_id,
            content_id,
            page_start,
            page_end,
        };
        
        BookContent::create(tx, &book_content).await?;
        Ok(())
    }

    /// Associa un tag al libro
    async fn add_tag(
        tx: &mut Transaction<'_, Sqlite>,
        book_id: i64,
        tag_id: i64,
    ) -> Result<(), RitmoErr> {
        let book_tag = BookTag {
            book_id,
            tag_id,
        };
        
        BookTag::create(tx, &book_tag).await?;
        Ok(())
    }

    /// Ottiene un libro completo con tutte le sue relazioni
    pub async fn get_complete_book(pool: &Pool<Sqlite>, book_id: i64) -> Result<Option<BookDetails>, RitmoErr> {
        let book_opt = Book::get(pool, book_id).await?;
        
        if let Some(book) = book_opt {
            // Recupera autori/persone e ruoli
            let people_roles = BookPersonRole::list_by_book(pool, book_id).await?;
            
            // Recupera contenuti
            let contents_links = BookContent::list_by_book(pool, book_id).await?;
            
            // Recupera i dettagli completi di ogni contenuto
            let mut contents_details = Vec::new();
            for link in contents_links {
                if let Some(content) = Content::get(pool, link.content_id).await? {
                    contents_details.push(ContentWithPages {
                        content,
                        page_start: link.page_start,
                        page_end: link.page_end,
                    });
                }
            }
            
            // Recupera tag
            let tags = BookTag::list_by_book(pool, book_id).await?;
            
            Ok(Some(BookDetails {
                book,
                people_roles,
                contents: contents_details,
                tags,
            }))
        } else {
            Ok(None)
        }
    }

    /// Aggiorna un libro completo e le sue relazioni
    pub async fn update_complete_book(
        pool: &Pool<Sqlite>,
        book_id: i64,
        book_data: Book,
        authors: Vec<(i64, i64)>, // (person_id, role_id)
        contents: Vec<(i64, Option<i64>, Option<i64>)>, // (content_id, page_start, page_end)
        tags: Vec<i64>, // tag_ids
    ) -> Result<(), RitmoErr> {
        // Inizia una transazione
        let mut tx = pool.begin().await?;
        
        // 1. Aggiorna i dati del libro
        book_data.update(&mut tx).await?;
        
        // 2. Elimina tutte le relazioni esistenti
        BookPersonRole::delete_all_by_book(&mut tx, book_id).await?;
        BookContent::delete_all_by_book(&mut tx, book_id).await?;
        BookTag::delete_all_by_book(&mut tx, book_id).await?;
        
        // 3. Ricrea le relazioni
        // Associa gli autori
        for (person_id, role_id) in authors {
            Self::add_person_role(&mut tx, book_id, person_id, role_id).await?;
        }

        // Associa i contenuti
        for (content_id, page_start, page_end) in contents {
            Self::add_content(&mut tx, book_id, content_id, page_start, page_end).await?;
        }

        // Associa i tag
        for tag_id in tags {
            Self::add_tag(&mut tx, book_id, tag_id).await?;
        }
        
        // Commit della transazione
        tx.commit().await?;
        
        Ok(())
    }

    /// Elimina un libro e tutte le sue relazioni
    pub async fn delete_complete_book(pool: &Pool<Sqlite>, book_id: i64) -> Result<(), RitmoErr> {
        // Per le relazioni con ON DELETE CASCADE, la cancellazione delle relazioni
        // avviene automaticamente quando si elimina il libro
        Book::delete(pool, book_id).await?;
        Ok(())
    }
}

/// Struttura che contiene un libro con tutte le sue relazioni
#[derive(Debug)]
pub struct BookDetails {
    pub book: Book,
    pub people_roles: Vec<BookPersonRole>,
    pub contents: Vec<ContentWithPages>,
    pub tags: Vec<BookTag>,
}

/// Struttura che rappresenta un contenuto associato a un libro con informazioni sulle pagine
#[derive(Debug)]
pub struct ContentWithPages {
    pub content: Content,
    pub page_start: Option<i64>,
    pub page_end: Option<i64>,
}