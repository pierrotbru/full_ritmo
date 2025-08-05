// ritmo_db/src/lib.rs
pub mod database;
pub mod models;

// Re-export delle funzioni più comuni per comodità
pub use database::*;
pub use models::*;
