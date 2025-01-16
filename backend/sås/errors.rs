// settings/errors.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SettingsError {
    #[error("Database error: {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Settings not found")]
    NotFound,
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Connection error: {0}")]
    ConnectionError(String),
    #[error("Database error: {0}")]
    DatabaseError(String),

}