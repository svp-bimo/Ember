#![forbid(unsafe_code)]

//! Configuration for the bookstore example.

use ember_ext_db::DbConfig;
use ember_ext_db::HasDbConfig;
use ember_core::HasEmberService;
use serde::Deserialize;

/// Ember service configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct EmberServiceConfig {
    /// Service name for logging.
    pub name: String,
    /// Listen address for logging.
    pub listen: String,
}

/// Ember runtime configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct EmberConfig {
    /// Service configuration.
    pub service: EmberServiceConfig,
}

/// Bookstore application configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct AppConfig {
    /// Ember configuration.
    pub ember: EmberConfig,
    /// Auth configuration.
    pub auth: AuthConfig,
    /// Database configuration.
    pub database: DbConfig,
}

/// Authentication configuration.
#[derive(Debug, Clone, Deserialize)]
pub struct AuthConfig {
    /// JWT secret.
    pub jwt_secret: String,
    /// Optional issuer.
    pub jwt_issuer: Option<String>,
    /// Optional audience.
    pub jwt_audience: Option<String>,
    /// Token expiry in seconds.
    pub jwt_expires_in_seconds: u64,
    /// Paths that do not require auth.
    pub public_paths: Vec<String>,
}

impl HasDbConfig for AppConfig {
    fn db_config(&self) -> &DbConfig {
        &self.database
    }
}

impl HasEmberService for AppConfig {
    fn service_name(&self) -> Option<&str> {
        Some(self.ember.service.name.as_str())
    }

    fn listen_addr(&self) -> Option<&str> {
        Some(self.ember.service.listen.as_str())
    }
}
