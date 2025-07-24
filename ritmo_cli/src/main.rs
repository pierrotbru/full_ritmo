use clap::{Parser, Subcommand};
use ritmo_core::normalize_path;
use ritmo_core::LibraryConfig;
use ritmo_db::connection::initialize_database;
use ritmo_errors::RitmoErr;
use ritmo_errors::RitmoResult;
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "ritmo_cli")]
#[command(about = "Gestione libreria Ritmo", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Inizializza una nuova libreria e database vuoto nel percorso specificato
    Init {
        /// Percorso della root della nuova libreria
        raw_path: PathBuf,
    },
    // ... altri comandi ...
}

#[tokio::main]
async fn main() -> RitmoResult<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { raw_path } => {
            let path = normalize_path(raw_path)
                .map_err(|e| RitmoErr::PathError(format!("Path normalization failed: {}", e)))?;

            // 1. Crea la struttura delle cartelle
            let config = LibraryConfig::new(&path);
            if path.exists() && std::fs::read_dir(&path)?.next().is_some() {
                // Se la cartella esiste ed è non vuota, restituisci errore
                eprintln!(
                    "Errore: la cartella '{}' esiste ed è non vuota. Annullato.",
                    path.display()
                );
                std::process::exit(1);
            }
            config.initialize()?;

            // 2. Inizializza il database
            initialize_database(&config.database_path).await?;

            // 3. Messaggio di conferma
            println!(
                "Nuova libreria inizializzata in '{}'.\nDatabase creato in '{}'.",
                path.display(),
                config.database_path.display()
            );
        } // ... altri comandi ...
    }

    Ok(())
}
