// ritmo_db/src/lib.rs

pub mod connection;
pub mod path;

// Re-export delle funzioni più comuni per comodità
pub use connection::{initialize_database, create_pool, is_valid_database};
pub use path::{verify_database_path, default_database_path, looks_like_database, backup_database};