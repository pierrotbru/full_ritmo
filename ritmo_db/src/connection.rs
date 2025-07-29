use sqlx::migrate;
use ritmo_core::LibraryConfig;
use ritmo_errors::RitmoErr;
use sqlx::migrate::Migrator;

// La macro migrate!() resta qui dove può trovare la cartella migrations
static MIGRATOR: Migrator = migrate!();

/// Inizializza il database utilizzando la configurazione della libreria
pub async fn initialize_database(config: &LibraryConfig) -> Result<(), RitmoErr> {
    let db_path = config.database_path.join("ritmo.db");
    let pool = config.create_pool(&db_path, true).await?;
    
    // Esegui le migrazioni se questo è un nuovo database
    MIGRATOR
        .run(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseMigrationFailed(e.to_string()))?;
    
    // Verifica che il database sia stato inizializzato correttamente
    let db_version: (i64,) = sqlx::query_as("PRAGMA user_version")
        .fetch_one(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    println!("Database inizializzato con successo in {}", db_path.display());
    println!("Versione del database: {}", db_version.0);
    
    Ok(())
}

/// Crea un pool di connessioni al database utilizzando la configurazione della libreria
pub async fn create_pool(config: &LibraryConfig, db_name: &str) -> Result<sqlx::SqlitePool, RitmoErr> {
    let db_path = config.database_path.join(db_name);
    config.create_pool(&db_path, true).await
}