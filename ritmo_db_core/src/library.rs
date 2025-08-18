use std::path::Path;
use std::fs;
use crate::LibraryConfig;
use crate::connection::pool::create_connection_pool;
use crate::database::Database;
use ritmo_errors::RitmoErr;

/// Crea tutta la struttura di una nuova libreria/database atomica.
/// Usa LibraryConfig per ottenere tutti i path canonici!
pub async fn create_full_database_library<P: AsRef<Path>>(root: P) -> Result<Database, RitmoErr> {
    let config = LibraryConfig::new(&root);

    // Crea tutte le directory canoniche
    fs::create_dir_all(config.canonical_database_path())?;
    fs::create_dir_all(config.canonical_config_path())?;
    fs::create_dir_all(config.canonical_bootstrap_path())?;
    fs::create_dir_all(config.canonical_portable_bootstrap_path())?;

    // Crea il file fisico del database nel path canonico
    let db_path = config.db_file_path();
    let pool = create_connection_pool(&db_path, true).await?;
    let db = Database::from_pool(pool).await?;

    Ok(db)
}