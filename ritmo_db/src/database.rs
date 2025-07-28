use crate::books::Book;
use chrono::Utc;
use ritmo_errors::{RitmoErr, RitmoResult};
use sqlx::{sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous}, Pool, Sqlite};
use sqlx::{migrate, migrate::Migrator, query};
use std::{fs, path::PathBuf, str::FromStr};

// Importazione dei modelli

static MIGRATOR: Migrator = migrate!();

/// Struttura centralizzata per gestire tutte le interazioni con il database
#[derive(Debug)]
pub struct Database {
    /// Pool di connessioni al database
    pool: Pool<Sqlite>,
    /// Percorso al file del database
    path: PathBuf,
    /// Informazioni sul database
    metadata: DatabaseMetadata,
}

/// Metadati del database 
#[derive(Debug, Clone)]
pub struct DatabaseMetadata {
    /// Versione del database
    pub version: i64,
    /// Utente che ha creato/aperto il database
    pub user: String,
    /// Timestamp dell'ultima apertura
    pub last_access: i64,
//    /// Numero di record nel database (cache)
//    pub record_counts: RecordCounts,
}

/// Conteggi dei record per tipo
#[derive(Debug, Clone, Default)]
pub struct RecordCounts {
    pub books: i64,
    pub authors: i64,
    pub tags: i64,
    // Altri conteggi...
}

impl Database {
    /// Crea una nuova istanza di Database aprendo un database esistente
    pub async fn open(path: &PathBuf) -> RitmoResult<Self> {
        if !path.exists() {
            return Err(RitmoErr::FileNotFound(
                format!("Il database specificato '{}' non esiste", path.display())
            ));
        }

        let pool = Self::create_connection_pool(path, false).await?;
        
        // Verifica l'integrità del database
        if !Self::is_valid_database(&pool).await? {
            return Err(RitmoErr::DatabaseError(
                "Il database non è valido o è corrotto".to_string()
            ));
        }
        
        // Recupera i metadati del database
        let version = Self::get_database_version(&pool).await?;
//        let record_counts = Self::count_records(&pool).await?;
        
        let metadata = DatabaseMetadata {
            version,
            user: "pierrotbru".to_string(), // In produzione, ottenere dall'ambiente
            last_access: Utc::now().timestamp(),
//            record_counts,
        };
        
        Ok(Self {
            pool,
            path: path.clone(),
            metadata,
        })
    }
    
    /// Crea un nuovo database nel percorso specificato
    pub async fn create(path: &PathBuf) -> RitmoResult<Self> {
        // Crea le directory necessarie
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Crea il database
        let pool = Self::create_connection_pool(path, true).await?;
        
        // Esegui le migrazioni
        Self::run_migrations(&pool).await?;
        
        // Imposta i metadati iniziali
        let version = Self::get_database_version(&pool).await?;
        
        let metadata = DatabaseMetadata {
            version,
            user: "pierrotbru".to_string(),
            last_access: Utc::now().timestamp(),
//            record_counts: RecordCounts::default(),
        };
        
        Ok(Self {
            pool,
            path: path.clone(),
            metadata,
        })
    }
    
    /// Crea una copia del database
    pub async fn backup_to(&self, destination: &PathBuf) -> RitmoResult<()> {
        // Verifica che il database sorgente sia aperto
        if !self.path.exists() {
            return Err(RitmoErr::DatabaseError(
                "Il database sorgente non esiste".to_string()
            ));
        }
        
        // Assicurati che la directory di destinazione esista
        if let Some(parent) = destination.parent() {
            fs::create_dir_all(parent)?;
        }
        
        // Eseguiamo un checkpoint WAL completo per assicurarci che tutti i dati siano scritti nel file principale
        match query("PRAGMA wal_checkpoint(FULL);")
            .execute(&self.pool)
            .await {
            Ok(_) => {},
            Err(e) => return Err(RitmoErr::DatabaseQueryFailed(format!("Errore nel checkpoint WAL: {}", e)))
        }
        
        // Proviamo a usare VACUUM INTO (disponibile in SQLite 3.27.0+)
        let vacuum_result = sqlx::query("VACUUM INTO ?")
            .bind(destination.to_string_lossy().to_string())
            .execute(&self.pool)
            .await;
        
        // Se VACUUM INTO fallisce, utilizziamo il metodo di copia diretta
        if vacuum_result.is_err() {
            // Copia il file direttamente
            match fs::copy(&self.path, destination) {
                Ok(_) => {},
                Err(e) => return Err(RitmoErr::IoError(format!("Errore nella copia del file: {}", e)))
            }
            
            // Gestione dei file WAL e SHM (se esistono)
            let wal_path = self.path.with_extension("db-wal");
            let shm_path = self.path.with_extension("db-shm");
            
            if wal_path.exists() {
                let dest_wal = destination.with_extension("db-wal");
                // Ignoriamo eventuali errori nella copia dei file ancillari
                let _ = fs::copy(&wal_path, &dest_wal);
            }
            
            if shm_path.exists() {
                let dest_shm = destination.with_extension("db-shm");
                let _ = fs::copy(&shm_path, &dest_shm);
            }
        }
        
        Ok(())
    }
    
    /// Chiude esplicitamente il database
    pub fn close(&self) {
        // Drop del pool di connessioni (avviene automaticamente alla fine dello scope)
        // Ma potrebbe essere esteso per eseguire operazioni di pulizia aggiuntive
        println!("Chiusura del database: {}", self.path.display());
        println!("Ultimo accesso: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
    }
    
    /// Ottiene una connessione dal pool
    pub async fn get_connection(&self) -> RitmoResult<sqlx::pool::PoolConnection<Sqlite>> {
        self.pool.acquire().await.map_err(|e| 
            RitmoErr::DatabaseConnectionFailed(format!("Errore nell'ottenere una connessione: {}", e))
        )
    }
    
    /// Restituisce un riferimento al pool di connessioni
    pub fn pool(&self) -> &Pool<Sqlite> {
        &self.pool
    }
    
    /// Restituisce i metadati del database
    pub fn metadata(&self) -> &DatabaseMetadata {
        &self.metadata
    }
    
    /// Restituisce il percorso del database
    pub fn path(&self) -> &PathBuf {
        &self.path
    }
    
    /// Verifica se il database è valido
    async fn is_valid_database(pool: &Pool<Sqlite>) -> RitmoResult<bool> {
        // Verifica l'integrità del database
        let integrity_check: (String,) = sqlx::query_as("PRAGMA integrity_check(1)")
            .fetch_one(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
            
        if integrity_check.0 != "ok" {
            return Ok(false);
        }
        
        // Verifica la presenza delle tabelle principali
        let _tables_exist = sqlx::query(
            "SELECT count(*) FROM sqlite_master WHERE type='table' AND 
             name IN ('books', 'authors', 'tags')"
        )
        .fetch_one(pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
        
        Ok(true)
    }
    
    /// Ottiene la versione del database
    async fn get_database_version(pool: &Pool<Sqlite>) -> RitmoResult<i64> {
        let version: (i64,) = sqlx::query_as("PRAGMA user_version")
            .fetch_one(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
            
        Ok(version.0)
    }
    
    /// Conta i record nel database
    async fn count_records(pool: &Pool<Sqlite>) -> RitmoResult<RecordCounts> {
        let mut counts = RecordCounts::default();
        
        // Conta i libri
        let books_count: (i64,) = sqlx::query_as("SELECT count(*) FROM books")
            .fetch_one(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(format!("Errore nel conteggio dei libri: {}", e)))?;
        counts.books = books_count.0;
        
        // Conta gli autori
        let authors_count: (i64,) = sqlx::query_as("SELECT count(*) FROM authors")
            .fetch_one(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(format!("Errore nel conteggio degli autori: {}", e)))?;
        counts.authors = authors_count.0;
        
        // Conta i tag
        let tags_count: (i64,) = sqlx::query_as("SELECT count(*) FROM tags")
            .fetch_one(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(format!("Errore nel conteggio dei tag: {}", e)))?;
        counts.tags = tags_count.0;
        
        Ok(counts)
    }
    
    /// Crea un pool di connessioni al database
    async fn create_connection_pool(db_path: &PathBuf, create: bool) -> RitmoResult<Pool<Sqlite>> {
        if create && !db_path.exists() {
            if let Some(parent) = db_path.parent() {
                fs::create_dir_all(parent)?;
            }
            fs::File::create(db_path.clone())
                .map_err(|e| RitmoErr::IoError(format!("Failed to create database file: {}", e)))?;
        }

        let database_url = format!("sqlite:///{}", db_path.to_string_lossy());
        let mut options = SqliteConnectOptions::from_str(&database_url)
            .map_err(|e| RitmoErr::SqlxError(e))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        options = options
            .pragma("cache_size", "-64000")
            .pragma("temp_store", "MEMORY")
            .pragma("foreign_keys", "ON");
            
        let pool = SqlitePool::connect_with(options)
            .await
            .map_err(|e| RitmoErr::SqlxError(e))?;

        Ok(pool)
    }
    
    /// Esegue le migrazioni sul database
    async fn run_migrations(pool: &Pool<Sqlite>) -> RitmoResult<()> {
        MIGRATOR
            .run(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseMigrationFailed(e.to_string()))?;
            
        // Ottimizzazioni post-migrazione
        query("ANALYZE;")
            .execute(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

        query("PRAGMA optimize;")
            .execute(pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
            
        Ok(())
    }
    
    // --- OPERAZIONI CRUD SUI LIBRI ---
    
    /// Aggiunge un nuovo libro al database
    pub async fn add_book(&self, book: &Book) -> RitmoResult<i64> {
        let book_id = Book::create(&self.pool, book)
            .await
            .map_err(|e| RitmoErr::DatabaseError(format!("Errore nell'inserimento del libro: {}", e)))?;
            
        Ok(book_id)
    }
    
    /// Ottiene un libro dal database tramite ID
    pub async fn get_book(&self, id: i64) -> RitmoResult<Option<Book>> {
        let book = Book::get(&self.pool, id)
            .await
            .map_err(|e| RitmoErr::DatabaseError(format!("Errore nel recupero del libro: {}", e)))?;
            
        Ok(book)
    }
    
    /// Aggiorna un libro nel database
    pub async fn update_book(&self, book: &Book) -> RitmoResult<u64> {
        let rows_affected = book.update(&self.pool)
            .await
            .map_err(|e| RitmoErr::DatabaseError(format!("Errore nell'aggiornamento del libro: {}", e)))?;
            
        Ok(rows_affected)
    }
    
    /// Elimina un libro dal database
    pub async fn delete_book(&self, id: i64) -> RitmoResult<u64> {
        let rows_affected = Book::delete(&self.pool, id)
            .await
            .map_err(|e| RitmoErr::DatabaseError(format!("Errore nell'eliminazione del libro: {}", e)))?;
            
        Ok(rows_affected)
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        // Operazioni di pulizia quando il database viene eliminato
        println!("Database chiuso: {}", self.path.display());
        println!("Data e ora: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
    }
}