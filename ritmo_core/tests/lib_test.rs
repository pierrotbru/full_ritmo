use ritmo_core::normalize_path;
use sha2::Digest;
use ritmo_core::{HashStorage, LibraryConfig};
use std::fs;
use std::path::{Path, PathBuf};
use tempfile::tempdir;

#[test]
fn test_library_config_new() {
    let root_path = Path::new("/test/path");
    let config = LibraryConfig::new(root_path);
    
    assert_eq!(config.root_path, PathBuf::from("/test/path"));
    assert_eq!(config.database_path, PathBuf::from("/test/path/database"));
    assert_eq!(config.storage_path, PathBuf::from("/test/path/storage"));
    assert_eq!(config.config_path, PathBuf::from("/test/path/config"));
    assert_eq!(config.bootstrap_path, PathBuf::from("/test/path/bootstrap"));
}

#[test]
fn test_library_config_initialize() {
    // Utilizziamo tempdir per creare una directory temporanea per i test
    let temp_dir = tempdir().expect("Impossibile creare directory temporanea");
    let root_path = temp_dir.path();
    
    let config = LibraryConfig::new(root_path);
    let result = config.initialize();
    
    assert!(result.is_ok(), "Inizializzazione fallita: {:?}", result);
    
    // Verifica che le directory siano state create
    assert!(config.root_path.exists());
    assert!(config.database_path.exists());
    assert!(config.storage_path.exists());
    assert!(config.config_path.exists());
    assert!(config.bootstrap_path.exists());
    assert!(config.bootstrap_path.join("portable_app").exists());
}

#[test]
fn test_library_config_initialize_existing_non_empty() {
    // Utilizziamo tempdir per creare una directory temporanea per i test
    let temp_dir = tempdir().expect("Impossibile creare directory temporanea");
    let root_path = temp_dir.path();
    
    // Creiamo un file per rendere la directory non vuota
    fs::write(root_path.join("some_file.txt"), b"test content").expect("Impossibile scrivere file di test");
    
    let config = LibraryConfig::new(root_path);
    let result = config.initialize();
    
    // Dovrebbe comunque restituire Ok perché la funzione è progettata per continuare
    // se la directory esiste già e non è vuota
    assert!(result.is_ok(), "Inizializzazione fallita: {:?}", result);
}

#[test]
fn test_library_config_validate() {
    // Utilizziamo tempdir per creare una directory temporanea per i test
    let temp_dir = tempdir().expect("Impossibile creare directory temporanea");
    let root_path = temp_dir.path();
    
    let config = LibraryConfig::new(root_path);
    
    // Prima dell'inizializzazione, la validazione dovrebbe fallire
    let validate_result = config.validate();
    assert!(validate_result.is_ok());
    assert_eq!(validate_result.unwrap(), false);
    
    // Dopo l'inizializzazione, la validazione dovrebbe riuscire
    config.initialize().expect("Inizializzazione fallita");
    let validate_result = config.validate();
    assert!(validate_result.is_ok());
    assert_eq!(validate_result.unwrap(), true);
}

#[test]
fn test_hash_storage_calculate_path() {
    let storage_path = PathBuf::from("/test/storage");
    let hash_storage = HashStorage::new(storage_path);
    
    let content_hash = "abcdef1234567890";
    let expected_path = PathBuf::from("/test/storage/ab/cd/abcdef1234567890");
    let calculated_path = hash_storage.calculate_path(content_hash);
    
    assert_eq!(calculated_path, expected_path);
}

#[test]
#[should_panic(expected = "Hash troppo corto per il path hash-based")]
fn test_hash_storage_calculate_path_short_hash() {
    let storage_path = PathBuf::from("/test/storage");
    let hash_storage = HashStorage::new(storage_path);
    
    // Questo hash è troppo corto e dovrebbe causare un panic
    let content_hash = "abc";
    hash_storage.calculate_path(content_hash);
}

#[test]
fn test_hash_storage_calculate_file_hash() {
    // Utilizziamo tempdir per creare una directory temporanea per i test
    let temp_dir = tempdir().expect("Impossibile creare directory temporanea");
    let file_path = temp_dir.path().join("test_file.txt");
    
    // Contenuto noto per verificare l'hash
    let content = b"test content for hashing";
    fs::write(&file_path, content).expect("Impossibile scrivere file di test");
    
    // Leggiamo il contenuto del file per verificare che sia stato scritto correttamente
    let read_content = fs::read(&file_path).expect("Impossibile leggere file di test");
    println!("Contenuto file scritto: {:?}", content);
    println!("Contenuto file letto: {:?}", read_content);
    
    // Calcoliamo l'hash direttamente dal contenuto in memoria
    let mut hasher = sha2::Sha256::new();
    hasher.update(content);
    let direct_hash = format!("{:x}", hasher.finalize());
    
    // Calcoliamo l'hash usando la funzione della libreria
    let calculated_hash = HashStorage::calculate_file_hash(&file_path).expect("Calcolo hash fallito");
    
    println!("Hash calcolato direttamente: {}", direct_hash);
    println!("Hash calcolato da file: {}", calculated_hash);
    
    // Invece di usare un hash predefinito, confrontiamo l'hash calcolato direttamente
    // con quello calcolato dalla funzione
    assert_eq!(calculated_hash, direct_hash);
}

use std::env;

#[test]
fn test_normalize_path() {
    // Salva la directory corrente per ripristinarla dopo il test
    let original_dir = env::current_dir().expect("Impossibile ottenere la directory corrente");
    
    // ---- Test con percorsi assoluti ----
    
    // Percorso assoluto semplice
    let absolute_path = if cfg!(windows) {
        PathBuf::from("C:\\Users\\pierrotbru\\Documents")
    } else {
        PathBuf::from("/home/pierrotbru/documents")
    };
    
    let result = normalize_path(&absolute_path).expect("Normalizzazione fallita");
    assert_eq!(result, absolute_path, "Un percorso assoluto senza elementi da normalizzare dovrebbe rimanere invariato");
    
    // Percorso assoluto con elementi da normalizzare
    let absolute_path_with_dots = if cfg!(windows) {
        PathBuf::from("C:\\Users\\pierrotbru\\..\\pierrotbru\\Documents\\.")
    } else {
        PathBuf::from("/home/pierrotbru/../pierrotbru/documents/.")
    };
    
    let expected_normalized_absolute = if cfg!(windows) {
        PathBuf::from("C:\\Users\\pierrotbru\\Documents")
    } else {
        PathBuf::from("/home/pierrotbru/documents")
    };
    
    let result = normalize_path(&absolute_path_with_dots).expect("Normalizzazione fallita");
    assert_eq!(
        result, 
        expected_normalized_absolute, 
        "Un percorso assoluto con '..' e '.' dovrebbe essere normalizzato correttamente"
    );
    
    // ---- Test con percorsi relativi ----
    
    // Percorso relativo semplice
    let relative_path = PathBuf::from("path/to/file.txt");
    let current_dir = env::current_dir().expect("Impossibile ottenere la directory corrente");
    let expected_absolute = current_dir.join("path").join("to").join("file.txt");
    
    let result = normalize_path(&relative_path).expect("Normalizzazione fallita");
    assert_eq!(
        result, 
        expected_absolute, 
        "Un percorso relativo semplice dovrebbe essere convertito in assoluto correttamente"
    );
    
    // Percorso relativo con elementi da normalizzare
    let complex_relative_path = PathBuf::from("./path/../to/./extra/../file.txt");
    let expected_complex_absolute = current_dir.join("to").join("file.txt");
    
    let result = normalize_path(&complex_relative_path).expect("Normalizzazione fallita");
    assert_eq!(
        result, 
        expected_complex_absolute, 
        "Un percorso relativo con '..' e '.' dovrebbe essere normalizzato correttamente"
    );
    
    // ---- Test con percorsi edge case ----
    
    // Percorso vuoto (dovrebbe restituire la directory corrente)
    let empty_path = PathBuf::from("");
    let expected_empty = env::current_dir().expect("Impossibile ottenere la directory corrente");
    
    let result = normalize_path(&empty_path).expect("Normalizzazione fallita");
    assert_eq!(
        result, 
        expected_empty, 
        "Un percorso vuoto dovrebbe essere risolto come la directory corrente"
    );

    // ---- Test con slashes multiple ----
    
    // Percorso con slashes consecutive
    let multiple_slashes = if cfg!(windows) {
        PathBuf::from("path\\\\to\\\\\\file.txt")
    } else {
        PathBuf::from("path//to///file.txt")
    };
    
    let expected_normalized_slashes = current_dir.join("path").join("to").join("file.txt");
    
    let result = normalize_path(&multiple_slashes).expect("Normalizzazione fallita");
    assert_eq!(
        result, 
        expected_normalized_slashes, 
        "Slashes multiple dovrebbero essere normalizzate"
    );
    
    // Ripristina la directory originale se il test l'ha cambiata
    env::set_current_dir(original_dir).expect("Impossibile ripristinare la directory originale");
}
