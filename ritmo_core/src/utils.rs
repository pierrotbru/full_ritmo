use std::path::{Path, PathBuf};

/// Restituisce un path assoluto e normalizzato, anche se il file non esiste.
/// Se il path è già assoluto, normalizza i componenti ".." e ".".
/// Se è relativo, lo risolve rispetto alla directory corrente e normalizza.
pub fn normalize_path<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let p = path.as_ref();
    let absolute_path = if p.is_absolute() {
        p.to_path_buf()
    } else {
        std::env::current_dir()?.join(p)
    };
    
    // Normalizza i componenti del percorso
    let mut components = Vec::new();
    for component in absolute_path.components() {
        match component {
            std::path::Component::ParentDir => {
                if !components.is_empty() {
                    components.pop();
                }
            },
            std::path::Component::CurDir => {},  // Ignora "."
            _ => components.push(component),
        }
    }
    
    // Ricostruisci il percorso normalizzato
    let mut normalized = PathBuf::new();
    for component in components {
        normalized.push(component);
    }
    
    Ok(normalized)
}
