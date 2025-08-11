use chrono::Utc;
use ritmo_errors::{RitmoErr, RitmoResult};
use sqlx::{Pool, Sqlite};
use std::{fs, path::PathBuf};
use crate::{
    connection::pool::create_connection_pool,
    maintenance::{integrity::{check_integrity, get_database_version}, backup::backup_database},
    config::optimizations::optimize_database
};

/// Metadati del database
#[derive(Debug, Clone)]
pub struct DatabaseMetadata {
    /// Versione del database
    pub version: i64,
    /// Utente che ha creato/aperto il database
    pub user: String,
    /// Timestamp dell'ultima apertura
    pub last_access: i64,
}

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

impl Database {
    /// Crea una nuova istanza di Database aprendo un database esistente
    pub async fn open(path: &PathBuf) -> RitmoResult<Self> {
        if !path.exists() {
            return Err(RitmoErr::FileNotFound(format!(
                "Il database specificato '{}' non esiste",
                path.display()
            )));
        }

        let pool = create_connection_pool(path, false).await?;

        // Verifica l'integrità del database
        if !check_integrity(&pool).await? {
            return Err(RitmoErr::DatabaseError(
                "Il database non è valido o è corrotto".to_string(),
            ));
        }

        // Recupera i metadati del database
        let version = get_database_version(&pool).await?;

        let metadata = DatabaseMetadata {
            version,
            user: "pierrotbru".to_string(), // In produzione, ottenere dall'ambiente
            last_access: Utc::now().timestamp(),
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
        let pool = create_connection_pool(path, true).await?;

        // Imposta i metadati iniziali
        let version = get_database_version(&pool).await?;

        let metadata = DatabaseMetadata {
            version,
            user: "pierrotbru".to_string(),
            last_access: Utc::now().timestamp(),
        };

        Ok(Self {
            pool,
            path: path.clone(),
            metadata,
        })
    }

    /// Crea una copia del database
    pub async fn backup_to(&self, destination: &PathBuf) -> RitmoResult<()> {
        backup_database(&self.pool, &self.path, destination).await
    }

    /// Applica ottimizzazioni al database
    pub async fn optimize(&self) -> RitmoResult<()> {
        optimize_database(&self.pool).await
    }

    /// Ottiene una connessione dal pool
    pub async fn get_connection(&self) -> RitmoResult<sqlx::pool::PoolConnection<Sqlite>> {
        self.pool.acquire().await.map_err(|e| {
            RitmoErr::DatabaseConnectionFailed(format!(
                "Errore nell'ottenere una connessione: {}",
                e
            ))
        })
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
    
    /// Chiude esplicitamente il database
    pub fn close(&self) {
        // Esegue operazioni di pulizia aggiuntive se necessario
        println!("Chiusura del database: {}", self.path.display());
        println!("Ultimo accesso: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
    }
}

impl Drop for Database {
    fn drop(&mut self) {
        // Operazioni di pulizia quando il database viene eliminato
        println!("Database chiuso: {}", self.path.display());
        println!("Data e ora: {}", Utc::now().format("%Y-%m-%d %H:%M:%S"));
    }
}