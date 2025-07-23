// ritmo_core/src/lib.rs
use std::path::{Path, PathBuf};
use std::fs;
use ritmo_errors::RitmoErr;
use ritmo_errors::RitmoResult;
use sha2::{Sha256, Digest};

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

    /// Inizializza la struttura delle cartelle se non esistono
    pub fn initialize(&self) -> Result<(), RitmoErr> {
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

    /// Crea la struttura di cartelle per un hash
    pub fn create_hash_directory(&self, content_hash: &str) -> RitmoResult<PathBuf> {
        let path = self.calculate_path(content_hash);
        fs::create_dir_all(&path)?;
        
        // Crea anche la cartella thumbnails
        fs::create_dir_all(path.join("thumbnails"))?;
        
        Ok(path)
    }

    /// Restituisce i percorsi standard per un contenuto
    pub fn get_content_paths(&self, content_hash: &str) -> ContentPaths {
        let base_path = self.calculate_path(content_hash);
        
        ContentPaths {
            base_directory: base_path.clone(),
            content_file: base_path.join("content.dat"),
            metadata_file: base_path.join("metadata.opf"),
            cover_file: base_path.join("cover.jpg"),
            thumbnails_dir: base_path.join("thumbnails"),
        }
    }
}

/// Percorsi standard per un contenuto nella libreria
#[derive(Debug, Clone)]
pub struct ContentPaths {
    pub base_directory: PathBuf,
    pub content_file: PathBuf,
    pub metadata_file: PathBuf,
    pub cover_file: PathBuf,
    pub thumbnails_dir: PathBuf,
}

/// Manager principale della libreria
pub struct LibraryManager {
    pub config: LibraryConfig,
    pub storage: HashStorage,
}

impl LibraryManager {
    /// Crea un nuovo manager della libreria
    pub fn new<P: AsRef<Path>>(root_path: P) -> Self {
        let config = LibraryConfig::new(root_path);
        let storage = HashStorage::new(config.storage_path.clone());
        
        Self { config, storage }
    }

    /// Inizializza una nuova libreria
    pub fn initialize_library(&self) -> RitmoResult<()> {
        self.config.initialize()?;
        
        // Qui potresti chiamare ritmo_db per inizializzare il database
        // e creare i file di configurazione base
        
        Ok(())
    }

    /// Verifica che la libreria sia valida e accessibile
    pub fn validate_library(&self) -> RitmoResult<bool> {
        self.config.validate()
    }

    /// Aggiunge un nuovo contenuto alla libreria
    pub fn add_content<P: AsRef<Path>>(
        &self, 
        source_file: P, 
        metadata: Option<ContentMetadata>
    ) -> RitmoResult<String> {
        // 1. Calcola l'hash del file
        let content_hash = HashStorage::calculate_file_hash(&source_file)?;
        
        // 2. Crea la struttura di cartelle
        let content_dir = self.storage.create_hash_directory(&content_hash)?;
        
        // 3. Copia il file nel storage
        let paths = self.storage.get_content_paths(&content_hash);
        fs::copy(&source_file, &paths.content_file)?;
        
        // 4. Se abbiamo metadati, salviamoli
        if let Some(meta) = metadata {
            self.save_metadata(&content_hash, &meta)?;
        }
        
        Ok(content_hash)
    }

    /// Salva i metadati di un contenuto
    fn save_metadata(&self, content_hash: &str, metadata: &ContentMetadata) -> RitmoResult<()> {
        let paths = self.storage.get_content_paths(content_hash);
        
        // Qui chiameresti ritmo_formats per creare il file OPF
        // Per ora un placeholder
        let opf_content = format!(
            r#"<?xml version="1.0" encoding="UTF-8"?>
<package xmlns="http://www.idpf.org/2007/opf" unique-identifier="uuid_id" version="2.0">
  <metadata>
    <dc:title>{}</dc:title>
    <dc:creator>{}</dc:creator>
  </metadata>
</package>"#,
            metadata.title, 
            metadata.author.as_deref().unwrap_or("Unknown")
        );
        
        fs::write(&paths.metadata_file, opf_content)?;
        Ok(())
    }
}

/// Metadati base per un contenuto
#[derive(Debug, Clone)]
pub struct ContentMetadata {
    pub title: String,
    pub author: Option<String>,
    pub publisher: Option<String>,
    pub isbn: Option<String>,
    pub language: Option<String>,
    pub format: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_library_config_creation() {
        let temp_dir = tempdir().unwrap();
        let config = LibraryConfig::new(temp_dir.path());
        
        assert_eq!(config.root_path, temp_dir.path());
        assert_eq!(config.database_path, temp_dir.path().join("database"));
        assert_eq!(config.storage_path, temp_dir.path().join("storage"));
    }

    #[test]
    fn test_hash_storage_path_calculation() {
        let temp_dir = tempdir().unwrap();
        let storage = HashStorage::new(temp_dir.path().to_path_buf());
        
        let hash = "abcd123456789";
        let expected_path = temp_dir.path().join("ab").join("cd").join(hash);
        let calculated_path = storage.calculate_path(hash);
        
        assert_eq!(calculated_path, expected_path);
    }
}