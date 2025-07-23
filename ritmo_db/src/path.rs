// ritmo_db/src/path.rs
use std::path::{Path, PathBuf};
use std::fs;
use ritmo_errors::RitmoErr;

/// Verifica e prepara il path del database per l'uso
/// 
/// # Arguments
/// * `path` - Path del database da verificare
/// * `create` - Se true, permette la creazione del file se non esiste
/// 
/// # Returns
/// Returns il PathBuf normalizzato e verificato, o un errore se il path non è valido
pub fn verify_database_path(path: &PathBuf, create: bool) -> Result<PathBuf, RitmoErr> {
    // Normalizza il path (risolve ., .., symlinks, ecc.)
    let canonical_path = if path.exists() {
        path.canonicalize()
            .map_err(|e| RitmoErr::IoError(format!("Cannot canonicalize path {}: {}", path.display(), e)))?
    } else if create {
        // Se dobbiamo creare il file, canonicalizziamo almeno la directory parent
        let parent = path.parent()
            .ok_or_else(|| RitmoErr::PathError(format!("Path has no parent directory: {}", path.display())))?;
        
        if !parent.exists() {
            return Err(RitmoErr::PathError(format!("Parent directory does not exist: {}", parent.display())));
        }
        
        let canonical_parent = parent.canonicalize()
            .map_err(|e| RitmoErr::IoError(format!("Cannot canonicalize parent directory {}: {}", parent.display(), e)))?;
        
        canonical_parent.join(path.file_name().unwrap())
    } else {
        return Err(RitmoErr::FileNotFound(format!("Database file not found: {}", path.display())));
    };

    // Verifica che l'estensione sia appropriata per un database
    if let Some(extension) = canonical_path.extension() {
        let ext_str = extension.to_string_lossy().to_lowercase();
        if !["db", "sqlite", "sqlite3"].contains(&ext_str.as_str()) {
            return Err(RitmoErr::PathError(format!(
                "Invalid database file extension: {}. Expected .db, .sqlite, or .sqlite3", 
                ext_str
            )));
        }
    } else {
        return Err(RitmoErr::PathError(format!(
            "Database file must have an extension (.db, .sqlite, or .sqlite3): {}", 
            canonical_path.display()
        )));
    }

    // Se il file esiste, verifica che sia accessibile
    if canonical_path.exists() {
        // Verifica che sia un file regolare
        if !canonical_path.is_file() {
            return Err(RitmoErr::PathError(format!(
                "Path exists but is not a regular file: {}", 
                canonical_path.display()
            )));
        }

        // Verifica permessi di lettura
        match fs::File::open(&canonical_path) {
            Ok(_) => {},
            Err(e) => return Err(RitmoErr::IoError(format!(
                "Cannot open database file {}: {}", 
                canonical_path.display(), 
                e
            ))),
        }

        // Se dobbiamo scrivere, verifica anche i permessi di scrittura
        if create {
            match fs::OpenOptions::new().write(true).append(true).open(&canonical_path) {
                Ok(_) => {},
                Err(e) => return Err(RitmoErr::IoError(format!(
                    "Database file is not writable {}: {}", 
                    canonical_path.display(), 
                    e
                ))),
            }
        }
    }

    Ok(canonical_path)
}

/// Genera un path predefinito per il database in base al root della libreria
pub fn default_database_path(library_root: &Path) -> PathBuf {
    library_root.join("database").join("library.db")
}

/// Verifica se un path punta a un file di database valido (controllo superficiale)
pub fn looks_like_database(path: &Path) -> bool {
    if !path.is_file() {
        return false;
    }

    // Controlla estensione
    if let Some(extension) = path.extension() {
        let ext_str = extension.to_string_lossy().to_lowercase();
        if !["db", "sqlite", "sqlite3"].contains(&ext_str.as_str()) {
            return false;
        }
    } else {
        return false;
    }

    // Controlla dimensione minima (SQLite ha almeno 512 bytes)
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.len() > 0 && metadata.len() < 512 {
                return false;
            }
        }
        Err(_) => return false,
    }

    true
}

/// Crea un backup del database esistente
pub fn backup_database(db_path: &Path) -> Result<PathBuf, RitmoErr> {
    if !db_path.exists() {
        return Err(RitmoErr::FileNotFound(format!("Database file not found: {}", db_path.display())));
    }

    let timestamp = chrono::Utc::now().format("%Y%m%d_%H%M%S");
    let backup_name = format!(
        "{}_backup_{}.{}",
        db_path.file_stem().unwrap().to_string_lossy(),
        timestamp,
        db_path.extension().unwrap().to_string_lossy()
    );
    
    let backup_path = db_path.parent().unwrap().join(backup_name);
    
    fs::copy(db_path, &backup_path)
        .map_err(|e| RitmoErr::IoError(format!(
            "Failed to backup database from {} to {}: {}", 
            db_path.display(), 
            backup_path.display(), 
            e
        )))?;

    println!("✓ Database backed up to: {}", backup_path.display());
    Ok(backup_path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::{tempdir, NamedTempFile};
    use std::io::Write;

    #[test]
    fn test_verify_database_path_existing_file() {
        let mut temp_file = NamedTempFile::with_suffix(".db").unwrap();
        temp_file.write_all(b"test database content").unwrap();
        temp_file.flush().unwrap();
        
        let path = temp_file.path().to_path_buf();

        let verified = verify_database_path(&path, false).unwrap();
        
        assert!(verified.is_absolute(), "Il percorso verificato non è assoluto: {:?}", verified);

        assert_eq!(
            verified.extension().and_then(|s| s.to_str()),
            Some("db"),
            "Il percorso verificato non ha l'estensione '.db'. Percorso: {:?}",
            verified
        );
    }

    #[test]
    fn test_verify_database_path_create_new() {
        let temp_dir = tempdir().unwrap();
        let db_path = temp_dir.path().join("test.db");
        
        let verified = verify_database_path(&db_path, true).unwrap();
        assert!(verified.is_absolute());
        assert!(verified.ends_with("test.db"));
    }

    #[test]
    fn test_invalid_extension() {
        let temp_dir = tempdir().unwrap();
        let invalid_path = temp_dir.path().join("test.txt");
        
        let result = verify_database_path(&invalid_path, true);
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RitmoErr::PathError(_)));
    }

    #[test]
    fn test_default_database_path() {
        let temp_dir = tempdir().unwrap();
        let default_path = default_database_path(temp_dir.path());
        
        assert_eq!(default_path, temp_dir.path().join("database").join("library.db"));
    }

    #[test]
    fn test_looks_like_database() {
        // Test con file che sembra un database
        let mut temp_file = NamedTempFile::with_suffix(".db").unwrap();
        // Scrivi abbastanza dati per superare i 512 bytes
        temp_file.write_all(&[0u8; 1024]).unwrap();
        temp_file.flush().unwrap();
        
        assert!(looks_like_database(temp_file.path()));
        
        // Test con file troppo piccolo
        let mut small_file = NamedTempFile::with_suffix(".db").unwrap();
        small_file.write_all(b"small").unwrap();
        small_file.flush().unwrap();
        
        assert!(!looks_like_database(small_file.path()));
        
        // Test con estensione sbagliata
        let mut wrong_ext = NamedTempFile::with_suffix(".txt").unwrap();
        wrong_ext.write_all(&[0u8; 1024]).unwrap();
        wrong_ext.flush().unwrap();
        
        assert!(!looks_like_database(wrong_ext.path()));
    }

    #[test]
    fn test_backup_database() {
        let mut temp_file = NamedTempFile::with_suffix(".db").unwrap();
        temp_file.write_all(b"test database for backup").unwrap();
        temp_file.flush().unwrap();
        
        let backup_path = backup_database(temp_file.path()).unwrap();
        
        assert!(backup_path.exists());
        assert!(backup_path.to_string_lossy().contains("_backup_"));
        
        // Verifica che il contenuto sia lo stesso
        let original_content = fs::read(temp_file.path()).unwrap();
        let backup_content = fs::read(&backup_path).unwrap();
        assert_eq!(original_content, backup_content);
    }
}