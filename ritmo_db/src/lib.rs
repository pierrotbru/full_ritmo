// ritmo_db/src/lib.rs
pub mod connection;
pub mod models;
pub mod dto;
pub mod importers;

// Re-export delle funzioni più comuni per comodità
pub use connection::{create_pool, initialize_database, is_valid_database};
pub use models::*;
