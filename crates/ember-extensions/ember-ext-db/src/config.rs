#![forbid(unsafe_code)]

//! Database configuration types.

use serde::{Deserialize, Serialize};

/// Database configuration for repositories.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DbConfig {
    /// Database connection URL.
    pub url: String,
    /// Username for the database.
    pub username: String,
    /// Password for the database.
    pub password: String,
}

/// Trait for types that expose a database configuration.
pub trait HasDbConfig {
    /// Return the database configuration for this type.
    fn db_config(&self) -> &DbConfig;
}

impl DbConfig {
    /// Create a new configuration instance.
    pub fn new(url: impl Into<String>, username: impl Into<String>, password: impl Into<String>) -> Self {
        Self {
            url: url.into(),
            username: username.into(),
            password: password.into(),
        }
    }
}
