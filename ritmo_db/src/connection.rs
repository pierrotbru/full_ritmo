use sqlx::migrate::Migrator;
use sqlx::SqlitePool;
use std::path::Path;
use ritmo_errors::RitmoErr;

// Macro per le migrazioni
static MIGRATOR: Migrator = sqlx::migrate!();

/// Crea un pool di connessione a partire dal path del database sqlite
pub async fn create_pool(db_path: &Path) -> Result<SqlitePool, RitmoErr> {
    let db_url = format!("sqlite://{}", db_path.display());
    SqlitePool::connect(&db_url)
        .await
        .map_err(|e| RitmoErr::DatabaseConnectionFailed(e.to_string()))
}

/// Inizializza il database ed esegue le migrazioni
pub async fn initialize_database(db_path: &Path) -> Result<(), RitmoErr> {
    let pool = create_pool(db_path).await?;

    // Esegui le migrazioni
    MIGRATOR
        .run(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseMigrationFailed(e.to_string()))?;

    // Verifica la versione del database
    let db_version: (i64,) = sqlx::query_as("PRAGMA user_version")
        .fetch_one(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    println!("Database inizializzato con successo in {}", db_path.display());
    println!("Versione del database: {}", db_version.0);

    Ok(())
}