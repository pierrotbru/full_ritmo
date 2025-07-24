use std::path::{Path, PathBuf};

/// Restituisce un path assoluto e normalizzato, anche se il file non esiste.
/// Se il path è già assoluto lo lascia invariato.
/// Se è relativo, lo risolve rispetto alla directory corrente.
/// Non fallisce se il file non esiste.
pub fn normalize_path<P: AsRef<Path>>(path: P) -> std::io::Result<PathBuf> {
    let p = path.as_ref();
    if p.is_absolute() {
        Ok(p.to_path_buf())
    } else {
        std::env::current_dir().map(|cwd| cwd.join(p))
    }
}
