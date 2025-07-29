use std::str::FromStr;
use ritmo_errors::RitmoErr;
use ritmo_errors::RitmoResult;
use sha2::{Digest, Sha256};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqlitePool, SqliteSynchronous};
use sqlx::query;
use std::fs;
use std::path::{Path, PathBuf};
pub use utils::normalize_path;

mod utils;

/// Configurazione principale della libreria MyLibrary
#[derive(Debug, Clone)]
pub struct LibraryConfig {
    /// Percorso root della libreria (MyLibrary/)
    pub root_path: PathBuf,
    /// Percorso del database (MyLibrary/database/)
    pub database_path: PathBuf,
    /// Percorso dello storage hash-based (MyLibrary/storage/)
    pub storage_path: PathBuf,
    /// Percorso delle configurazioni (MyLibrary/config/)
    pub config_path: PathBuf,
    /// Percorso del bootstrap (MyLibrary/bootstrap/)
    pub bootstrap_path: PathBuf,
}

impl LibraryConfig {
    // Metodi esistenti...
    
    /// Crea una nuova configurazione della libreria
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        let root = root_path.as_ref().to_path_buf();

        Self {
            database_path: root.join("database"),
            storage_path: root.join("storage"),
            config_path: root.join("config"),
            bootstrap_path: root.join("bootstrap"),
            root_path: root,
        }
    }

    /// Inizializza la struttura delle cartelle se non esistono.
    /// Se la root esiste ed è NON vuota, non crea la struttura.
    pub fn initialize(&self) -> Result<(), RitmoErr> {
        // Implementazione esistente...
        if self.root_path.exists() {
            // Controlla se è una directory e se è vuota
            if self.root_path.is_dir() {
                let mut entries = fs::read_dir(&self.root_path)?;
                if entries.next().is_some() {
                    // La cartella esiste e NON è vuota: non crea la struttura
                    return Ok(());
                    // In alternativa, per segnalare errore:
                    // return Err(RitmoErr::Msg("La cartella principale esiste ed è già popolata".to_string()));
                }
            }
        }
        // Crea le cartelle principali
        fs::create_dir_all(&self.root_path)?;
        fs::create_dir_all(&self.database_path)?;
        fs::create_dir_all(&self.storage_path)?;
        fs::create_dir_all(&self.config_path)?;
        fs::create_dir_all(&self.bootstrap_path)?;

        // Crea sottocartelle del bootstrap
        fs::create_dir_all(self.bootstrap_path.join("portable_app"))?;

        Ok(())
    }

    /// Verifica che la struttura della libreria sia valida
    pub fn validate(&self) -> Result<bool, RitmoErr> {
        Ok(self.root_path.exists()
            && self.database_path.exists()
            && self.storage_path.exists()
            && self.config_path.exists())
    }
    
    // NUOVI METODI PER LA GESTIONE DEL DATABASE
    
    /// Inizializza il database nel percorso specificato.
    /// 
    /// Crea un nuovo database e applica le migrazioni necessarie.
    pub async fn initialize_database(&self) -> Result<(), RitmoErr> {
        let db_path = self.database_path.join("ritmo.db");
        // Crea un pool di connessioni e esegui le migrazioni
        let pool = self.create_pool(&db_path, true).await?;

        // Verifica che il database sia stato inizializzato correttamente
        let db_version: (i64,) = sqlx::query_as("PRAGMA user_version")
            .fetch_one(&pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

        println!("Database inizializzato con successo in {}", db_path.display());
        println!("Versione del database: {}", db_version.0);

        Ok(())
    }

    /// Crea un pool di connessioni al database.
    pub async fn create_pool(&self, db_path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
        if create && !db_path.exists() {
            fs::File::create(db_path.clone())
                .map_err(|e| RitmoErr::IoError(format!("Impossibile creare il file del database: {}", e)))?;
        }

        let database_url = format!("sqlite:///{}", db_path.to_string_lossy());
        let mut options = SqliteConnectOptions::from_str(&database_url)
            .map_err(|e| RitmoErr::SqlxError(e))?
            .create_if_missing(true)
            .journal_mode(SqliteJournalMode::Wal)
            .synchronous(SqliteSynchronous::Normal);

        options = options
            .pragma("cache_size", "-64000")
            .pragma("temp_store", "MEMORY");
        let pool = SqlitePool::connect_with(options)
            .await
            .map_err(RitmoErr::SqlxError)?;

        // Abilita il supporto alle chiavi esterne
        query("PRAGMA foreign_keys = ON;")
            .execute(&pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

        query("ANALYZE;")
            .execute(&pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

        query("PRAGMA optimize;")
            .execute(&pool)
            .await
            .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

        Ok(pool)
    }
    
    /// Verifica il percorso del database e assicura che sia valido
    pub fn verify_database_path(&self) -> Result<PathBuf, RitmoErr> {
        if !self.database_path.exists() {
            return Err(RitmoErr::IoError("Il percorso del database non esiste".to_string()));
        }
        
        Ok(self.database_path.clone())
    }
}

/// Gestione dello storage hash-based per i contenuti
pub struct HashStorage {
    storage_root: PathBuf,
}

impl HashStorage {
    pub fn new(storage_path: PathBuf) -> Self {
        Self {
            storage_root: storage_path,
        }
    }

    /// Calcola il percorso hash-based per un contenuto
    /// Formato: storage/ab/cd/abcd123456...
    pub fn calculate_path(&self, content_hash: &str) -> PathBuf {
        if content_hash.len() < 4 {
            panic!("Hash troppo corto per il path hash-based");
        }

        let first_two = &content_hash[0..2];
        let second_two = &content_hash[2..4];

        self.storage_root
            .join(first_two)
            .join(second_two)
            .join(content_hash)
    }

    /// Calcola l'hash SHA256 di un file
    pub fn calculate_file_hash<P: AsRef<Path>>(file_path: P) -> RitmoResult<String> {
        let content = fs::read(file_path)?;
        let mut hasher = Sha256::new();
        hasher.update(&content);
        let result = hasher.finalize();
        Ok(format!("{:x}", result))
    }
}
