use crate::verify_database_path;
use sqlx::{migrate, migrate::Migrator, query};
use sqlx::sqlite::{SqliteConnectOptions, SqliteJournalMode, SqliteSynchronous, SqlitePool};
use std::{path::PathBuf, fs, str::FromStr};
use ritmo_errors::RitmoErr;
// use ritmo_core::path::verify_database_path;

static MIGRATOR: Migrator = migrate!();

/// Creates a new database at the specified path and initializes it with the latest schema.
/// 
/// # Arguments
/// * `path` - Path where the database file should be created
/// 
/// # Returns
/// Returns `Ok(())` if successful, or an error if the database could not be created or initialized.
pub async fn initialize_database(path: &PathBuf) -> Result<(), RitmoErr> {
    // Verify and prepare the database path
    let db_path = verify_database_path(path, true)?;
    
    // Create parent directories if they don't exist
    if let Some(parent) = db_path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| 
                RitmoErr::IoError(format!("Failed to create database directory: {}", e))
            )?;
        }
    }
    
    // Create an empty database file if it doesn't exist
    if !db_path.exists() {
        fs::File::create(&db_path).map_err(|e| 
            RitmoErr::IoError(format!("Failed to create database file: {}", e))
        )?;
    }
    
    // Create a connection pool and run migrations
    let pool = create_pool(path, true).await?;
    
    // Verify the database was properly initialized
    let db_version: (i64,) = sqlx::query_as("PRAGMA user_version")
        .fetch_one(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;
    
    println!("Database initialized successfully at {}", db_path.display());
    println!("Database version: {}", db_version.0);
    
    Ok(())
}

/// Creates a connection pool to the database at the specified path.
/// 
/// # Arguments
/// * `path` - Path to the database file
/// * `create` - If true, creates the database file if it doesn't exist
/// 
/// # Returns
/// Returns a `SqlitePool` if successful, or an error if the connection could not be established.
pub async fn create_pool(path: &PathBuf, create: bool) -> Result<SqlitePool, RitmoErr> {
    let db_path = verify_database_path(path, create)?;

    if create && !db_path.exists() {
        fs::File::create(db_path.clone()).map_err(|e| 
            RitmoErr::IoError(format!("Failed to create database file: {}", e))
        )?;
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

    // Enable foreign key support
    query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .map_err(|e| RitmoErr::DatabaseQueryFailed(e.to_string()))?;

    // Run migrations if this is a new database
    if create {
        MIGRATOR.run(&pool).await.map_err(|e| 
            RitmoErr::DatabaseMigrationFailed(e.to_string())
        )?;
    }

    // Run optimization
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

/// Verifies if the given path points to a valid database file.
/// 
/// # Arguments
/// * `path` - Path to check
/// 
/// # Returns
/// Returns `true` if the path points to a valid database, `false` otherwise.
pub async fn is_valid_database(path: &PathBuf) -> bool {
    // Check if path exists and is a file
    if !path.is_file() {
        return false;
    }
    
    // Check file size is not empty (SQLite requires at least 1 page = 512 bytes)
    if let Ok(metadata) = fs::metadata(path) {
        if metadata.len() > 0 && metadata.len() < 512 {
            return false;  // File is too small to be a valid SQLite database
        }
    } else {
        return false;  // Can't read file metadata
    }
    
    // Try to open the database and run a simple query
    match create_pool(path, false).await {
        Ok(pool) => {
            // Check if the required system tables exist
            let result: Result<(), _> = sqlx::query(
                r#"
                SELECT 1 FROM sqlite_master 
                WHERE type = 'table' 
                AND name IN ('system_config', 'sqlite_schema')
                "#
            )
            .fetch_optional(&pool)
            .await
            .map(|_| ());
            
            // Also verify we can read the user_version (checks if the file is not corrupted)
            let version_check: Result<(), _> = sqlx::query_scalar::<_, i32>("PRAGMA user_version")
                .fetch_one(&pool)
                .await
                .map(|_| ());
                
            result.is_ok() && version_check.is_ok()
        }
        Err(_) => false,  // Failed to create pool or execute queries
    }
}