use ritmo_errors::RitmoResult;
use std::path::Path;
use sqlx::{sqlite::SqlitePoolOptions, Sqlite, SqlitePool, migrate::MigrateDatabase};
use ritmo_errors::RitmoErr;
use tracing::log::{info, error};

/// Struttura principale che rappresenta un database RitmoDB
pub struct Database {
    pool: SqlitePool,
    metadata: DatabaseMetadata,
}

/// Metadati del database
pub struct DatabaseMetadata {
    pub version: String,
    pub created_at: i64,
    pub updated_at: i64,
}

impl Database {
    /// Crea un nuovo database con tutte le migrazioni necessarie
    /// 
    /// # Arguments
    /// 
    /// * `path` - Il percorso del file del database
    /// 
    /// # Returns
    /// 
    /// Un'istanza di `Database` configurata e pronta all'uso
    pub async fn create(path: &Path) -> RitmoResult<Self> {
        // Converti il path in un URL SQLite
        let db_url = format!("sqlite:{}", path.display());
        
        // Verifica se il database esiste già
        if Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            info!("Il database {} esiste già, lo elimino per crearlo da zero", path.display());
            Sqlite::drop_database(&db_url).await
                .map_err(|e| RitmoErr::DatabaseCreation(format!("Impossibile eliminare il database esistente: {}", e)))?;
        }
        
        // Crea il nuovo database
        info!("Creazione del nuovo database in {}", path.display());
        Sqlite::create_database(&db_url).await
            .map_err(|e| RitmoErr::DatabaseCreation(format!("Impossibile creare il database: {}", e)))?;
        
        // Configura il pool di connessioni
        let pool_options = SqlitePoolOptions::new();
//            .max_connections(10)
//            .connect_timeout(std::time::Duration::from_secs(30));
            
        let pool = pool_options.connect(&db_url).await
            .map_err(|e| RitmoErr::DatabaseConnection(format!("Impossibile connettersi al database: {}", e)))?;

// Modifica in:
//        let pool_options = SqlitePoolOptions::new();
//    
//        let pool = pool_options.connect(&db_url).await
//            .map_err(|e| RitmoErr::DatabaseConnection(format!("Impossibile connettersi al database: {}", e)))?;

        
        // Configura SQLite per prestazioni ottimali
        sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await
            .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile configurare il database in modalità WAL: {}", e)))?;
        sqlx::query("PRAGMA foreign_keys = ON").execute(&pool).await
            .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile abilitare le foreign keys: {}", e)))?;
        
        // Esegui le migrazioni
        info!("Esecuzione delle migrazioni...");
        
        // Utilizziamo la funzione embed_migrations! da refinery
        // Questa è una versione semplificata - adatta in base al sistema effettivo di migrazioni
        match Self::run_migrations(&pool).await {
            Ok(_) => info!("Migrazioni completate con successo"),
            Err(e) => {
                error!("Errore durante l'esecuzione delle migrazioni: {}", e);
                return Err(RitmoErr::DatabaseMigration(format!("Errore durante l'esecuzione delle migrazioni: {}", e)));
            }
        }
        
        // Inizializza i dati di base
        Self::initialize_base_data(&pool).await?;
        
        // Crea e imposta i metadati del database
        let now = chrono::Utc::now().timestamp();
        let metadata = DatabaseMetadata {
            version: env!("CARGO_PKG_VERSION").to_string(),
            created_at: now,
            updated_at: now,
        };
        
        // Salva i metadati nel database
        sqlx::query(
            "INSERT INTO metadata (key, value) VALUES (?, ?), (?, ?), (?, ?)"
        )
        .bind("version")
        .bind(&metadata.version)
        .bind("created_at")
        .bind(metadata.created_at.to_string())
        .bind("updated_at")
        .bind(metadata.updated_at.to_string())
        .execute(&pool).await
        .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile salvare i metadati del database: {}", e)))?;
        
        info!("Database creato con successo (versione: {})", metadata.version);
        
        Ok(Self { pool, metadata })
    }
    
    /// Esegue le migrazioni del database
    async fn run_migrations(pool: &SqlitePool) -> RitmoResult<()> {
        // Qui utilizzeremo il metodo di migrazione del tuo progetto
        // Per ora, creiamo la tabella metadata se non esiste
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS metadata (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )"
        )
        .execute(pool).await
        .map_err(|e| RitmoErr::DatabaseMigration(format!("Impossibile creare la tabella metadata: {}", e)))?;
        
        // Crea altre tabelle necessarie
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS formats (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )"
        )
        .execute(pool).await
        .map_err(|e| RitmoErr::DatabaseMigration(format!("Impossibile creare la tabella formats: {}", e)))?;
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS publishers (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )"
        )
        .execute(pool).await
        .map_err(|e| RitmoErr::DatabaseMigration(format!("Impossibile creare la tabella publishers: {}", e)))?;
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS series (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL UNIQUE,
                description TEXT,
                total_books INTEGER,
                completed INTEGER,
                created_at INTEGER NOT NULL,
                updated_at INTEGER NOT NULL
            )"
        )
        .execute(pool).await
        .map_err(|e| RitmoErr::DatabaseMigration(format!("Impossibile creare la tabella series: {}", e)))?;
        
        sqlx::query(
            "CREATE TABLE IF NOT EXISTS books (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                name TEXT NOT NULL,
                original_title TEXT,
                publisher_id INTEGER,
                format_id INTEGER,
                series_id INTEGER,
                series_index INTEGER,
                publication_date INTEGER,
                last_modified_date INTEGER NOT NULL,
                isbn TEXT,
                pages INTEGER,
                notes TEXT,
                has_cover INTEGER NOT NULL DEFAULT 0,
                has_paper INTEGER NOT NULL DEFAULT 0,
                file_link TEXT,
                file_size INTEGER,
                file_hash TEXT,
                created_at INTEGER NOT NULL,
                FOREIGN KEY(publisher_id) REFERENCES publishers(id) ON DELETE SET NULL,
                FOREIGN KEY(format_id) REFERENCES formats(id) ON DELETE SET NULL,
                FOREIGN KEY(series_id) REFERENCES series(id) ON DELETE SET NULL
            )"
        )
        .execute(pool).await
        .map_err(|e| RitmoErr::DatabaseMigration(format!("Impossibile creare la tabella books: {}", e)))?;
        
        Ok(())
    }
    
    /// Inizializza i dati di base nel database
    async fn initialize_base_data(pool: &SqlitePool) -> RitmoResult<()> {
        info!("Inizializzazione dei dati di base...");
        
        // Formati di libro predefiniti
        let formats = [
            ("Hardcover", "Libro con copertina rigida"),
            ("Paperback", "Libro con copertina flessibile"),
            ("Ebook", "Libro in formato digitale"),
            ("AudioBook", "Libro audio"),
            ("PDF", "Documento in formato PDF"),
            ("EPUB", "Ebook in formato EPUB"),
            ("MOBI", "Ebook in formato MOBI per Kindle"),
        ];
        
        for (name, description) in formats {
            let exists = sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM formats WHERE name = ?")
                .bind(name)
                .fetch_one(pool).await
                .map_err(|e| RitmoErr::DatabaseQuery(format!("Errore nel verificare l'esistenza del formato: {}", e)))?;
                
            if exists == 0 {
                let now = chrono::Utc::now().timestamp();
                sqlx::query(
                    "INSERT INTO formats (name, description, created_at, updated_at) VALUES (?, ?, ?, ?)"
                )
                .bind(name)
                .bind(description)
                .bind(now)
                .bind(now)
                .execute(pool).await
                .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile inserire il formato predefinito '{}': {}", name, e)))?;
                
                info!("Formato predefinito aggiunto: {}", name);
            }
        }
        
        Ok(())
    }
    
    /// Ottiene il pool di connessioni al database
    pub fn get_pool(&self) -> &SqlitePool {
        &self.pool
    }
    
    /// Ottiene i metadati del database
    pub fn get_metadata(&self) -> &DatabaseMetadata {
        &self.metadata
    }
    
    /// Apre un database esistente
    pub async fn open(path: &Path) -> RitmoResult<Self> {
        // Converti il path in un URL SQLite
        let db_url = format!("sqlite:{}", path.display());
        
        // Verifica che il database esista
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            return Err(RitmoErr::DatabaseNotFound(path.display().to_string()));
        }
        
        // Configura il pool di connessioni
        let pool_options = SqlitePoolOptions::new();
//            .max_connections(10)
//            .connect_timeout(std::time::Duration::from_secs(30));
            
        let pool = pool_options.connect(&db_url).await
            .map_err(|e| RitmoErr::DatabaseConnection(format!("Impossibile connettersi al database: {}", e)))?;
        
        // Configura SQLite per prestazioni ottimali
        sqlx::query("PRAGMA journal_mode = WAL").execute(&pool).await
            .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile configurare il database in modalità WAL: {}", e)))?;
        sqlx::query("PRAGMA foreign_keys = ON").execute(&pool).await
            .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile abilitare le foreign keys: {}", e)))?;
        
        // Carica i metadati dal database
        let version = match sqlx::query_scalar::<_, String>("SELECT value FROM metadata WHERE key = 'version'")
            .fetch_optional(&pool).await {
                Ok(Some(v)) => v,
                Ok(None) => "0.1.0".to_string(),
                Err(e) => return Err(RitmoErr::DatabaseQuery(format!("Errore nel leggere la versione del database: {}", e)))
            };
            
        let created_at = match sqlx::query_scalar::<_, String>("SELECT value FROM metadata WHERE key = 'created_at'")
            .fetch_optional(&pool).await {
                Ok(Some(v)) => v.parse::<i64>().unwrap_or_else(|_| chrono::Utc::now().timestamp()),
                Ok(None) => chrono::Utc::now().timestamp(),
                Err(e) => return Err(RitmoErr::DatabaseQuery(format!("Errore nel leggere la data di creazione del database: {}", e)))
            };
            
        let updated_at = match sqlx::query_scalar::<_, String>("SELECT value FROM metadata WHERE key = 'updated_at'")
            .fetch_optional(&pool).await {
                Ok(Some(v)) => v.parse::<i64>().unwrap_or_else(|_| chrono::Utc::now().timestamp()),
                Ok(None) => chrono::Utc::now().timestamp(),
                Err(e) => return Err(RitmoErr::DatabaseQuery(format!("Errore nel leggere la data di aggiornamento del database: {}", e)))
            };
        
        let metadata = DatabaseMetadata {
            version,
            created_at,
            updated_at,
        };
        
        info!("Database aperto con successo (versione: {})", metadata.version);
        
        // Esegui eventuali migrazioni pendenti
        info!("Verifica ed esecuzione di eventuali migrazioni pendenti...");
        Self::run_migrations(&pool).await?;
        
        // Aggiorna la versione e la data di aggiornamento
        let now = chrono::Utc::now().timestamp();
        let new_version = env!("CARGO_PKG_VERSION").to_string();
        
        if new_version != metadata.version {
            sqlx::query("UPDATE metadata SET value = ? WHERE key = 'version'")
                .bind(&new_version)
                .execute(&pool).await
                .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile aggiornare la versione del database: {}", e)))?;
                
            sqlx::query("UPDATE metadata SET value = ? WHERE key = 'updated_at'")
                .bind(now.to_string())
                .execute(&pool).await
                .map_err(|e| RitmoErr::DatabaseQuery(format!("Impossibile aggiornare la data di aggiornamento del database: {}", e)))?;
                
            info!("Database aggiornato alla versione: {}", new_version);
        }
        
        Ok(Self { 
            pool, 
            metadata: DatabaseMetadata {
                version: new_version,
                created_at: metadata.created_at,
                updated_at: now,
            }
        })
    }
}