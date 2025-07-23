use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use std::fs;
use ritmo_db::connection::initialize_database;
use ritmo_errors::{RitmoErr, RitmoResult};
use std::io::{self, Write};
use std::convert::From;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new database
    Create {
        /// Path where to create the database
        path: PathBuf,
    },
    /// Delete an existing database
    Delete {
        /// Path to the database to delete
        path: PathBuf,
        /// Skip confirmation prompt
        #[arg(short, long)]
        force: bool,
    },
    /// List all available databases in a directory
    List {
        /// Directory to search for databases (default: current directory)
        #[arg(default_value = ".")]
        directory: PathBuf,
        /// Show detailed information about each database
        #[arg(short, long)]
        verbose: bool,
    },
}

#[tokio::main]
async fn main() -> RitmoResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Create { path } => {
            initialize_database(&path).await?;
            println!("Database created successfully at {}", path.display());
        }
        Commands::Delete { path, force } => {
            // Normalizza il percorso fornito dall'utente all'inizio
            let canonical_path = path.canonicalize()
                .map_err(|e| RitmoErr::PathError(format!("Invalid delete path: {}. Error: {}", path.display(), e)))?;

            if !force {
                print!("WARNING: This will delete the database at {}\nAre you sure you want to continue? [y/N] ", canonical_path.display());
                io::stdout().flush().map_err(|e| RitmoErr::IoError(e.to_string()))?;
                
                let mut input = String::new();
                io::stdin().read_line(&mut input).map_err(|e| RitmoErr::IoError(e.to_string()))?;
                
                if !input.trim().eq_ignore_ascii_case("y") {
                    println!("Operation cancelled.");
                    return Ok(());
                }
            }
            
            delete_database(&canonical_path)?;
            println!("Database deleted successfully from {}", canonical_path.display());
        }
        Commands::List { directory, verbose } => {
            // Normalizza il percorso della directory all'inizio
            let canonical_directory = directory.canonicalize()
                .map_err(|e| RitmoErr::PathError(format!("Invalid list directory: {}. Error: {}", directory.display(), e)))?;

            list_databases(&canonical_directory, verbose).await?;
        }
    }

    Ok(())
}

async fn list_databases(directory: &Path, verbose: bool) -> RitmoResult<()> {
    // directory è già canonicalizzato dal main, quindi possiamo assumerne la validità assoluta
    // Non è necessario rifare directory.exists() o directory.is_dir() qui se l'errore
    // di canonicalize è gestito dal chiamante, a meno che non sia una condizione di race.

    if !directory.is_dir() { // Ricontrolla se non è una directory, dato che canonicalize() potrebbe risolvere a un file
        return Err(RitmoErr::PathError(format!("Path is not a directory: {}", directory.display())));
    }
    if !directory.exists() { // Se per qualche motivo il percorso canonico non esiste (es. race condition dopo canonicalize)
        return Err(RitmoErr::PathError(format!("Directory does not exist after canonicalization: {}", directory.display())));
    }

    println!("Databases in {}:", directory.display());
    println!("-------------------");

    let entries = fs::read_dir(directory)
        .map_err(|e| RitmoErr::IoError(format!("Failed to read directory: {}", e)))?;
    
    let mut found_any = false;

    for entry in entries {
        let entry = entry.map_err(|e| RitmoErr::IoError(format!("Failed to read directory entry: {}", e)))?;
        let path = entry.path();
        
        // Canonicalizza il percorso di ogni voce per garantire coerenza, specialmente se lo usi per confronti o ulteriori operazioni
        let canonical_entry_path = path.canonicalize()
            .map_err(|e| RitmoErr::PathError(format!("Failed to canonicalize entry path {}: {}", path.display(), e)))?;
        
        if is_database_directory(&canonical_entry_path)? { // Passa il percorso canonico
            found_any = true;
            
            if verbose {
                let metadata = fs::metadata(&canonical_entry_path) // Usa il percorso canonico
                    .map_err(|e| RitmoErr::IoError(format!("Failed to get metadata for {}: {}", canonical_entry_path.display(), e)))?;
                    
                let modified = metadata.modified()
                    .map_err(|e| RitmoErr::IoError(format!("Failed to get modification time: {}", e)))?
                    .elapsed()
                    .map(|d| format!("{} days ago", d.as_secs() / 86400))
                    .unwrap_or_else(|_| "unknown".to_string());
                
                println!("• {} (modified {})", 
                    canonical_entry_path.file_name().unwrap_or_default().to_string_lossy(),
                    modified
                );
            } else {
                println!("• {}", canonical_entry_path.file_name().unwrap_or_default().to_string_lossy());
            }
        }
    }

    if !found_any {
        println!("No databases found in this directory.");
    }

    Ok(())
}

fn is_database_directory(path: &Path) -> RitmoResult<bool> {
    // path dovrebbe essere già canonicalizzato quando arriva qui
    dbg!(path);
    if !path.is_dir() {
        return Ok(false);
    }
    
    let db_file = path.join("ritmo.db");
    Ok(db_file.exists())
}

async fn create_database(path: &Path) -> RitmoResult<()> {
    // path è già canonicalizzato

    dbg!(path);
    if path.exists() {
        return Err(RitmoErr::PathError(format!("Path already exists: {}", path.display())));
    }

    fs::create_dir_all(path).map_err(|e| RitmoErr::IoError(e.to_string()))?;
    initialize_database(&path.join("ritmo.db")).await?; // Crea il DB all'interno del percorso canonico
    Ok(())
}

fn delete_database(path: &Path) -> RitmoResult<()> {
    // path è già canonicalizzato
    if !path.exists() {
        return Err(RitmoErr::PathError(format!("Database does not exist: {}", path.display())));
    }

    if path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| RitmoErr::IoError(e.to_string()))?;
    } else {
        fs::remove_file(path).map_err(|e| RitmoErr::IoError(e.to_string()))?;
    }
    
    Ok(())
}
