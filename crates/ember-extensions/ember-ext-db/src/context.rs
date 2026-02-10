#![forbid(unsafe_code)]

//! Database context handling.

use ember_ext_exceptions::EmberError;

use crate::config::DbConfig;
use crate::pool::DbPool;

/// Database context holding configuration.
#[derive(Debug, Clone)]
pub struct DbContext {
    /// Database configuration.
    pub config: DbConfig,
}

impl DbContext {
    /// Create a new database context from config.
    pub fn new(config: DbConfig) -> Self {
        Self { config }
    }

    /// Connect to the configured database.
    pub async fn connect(&self) -> Result<DbPool, EmberError> {
        DbPool::connect(&self.config).await
    }

    /// Connect and apply schema migrations.
    pub async fn connect_and_migrate(&self, schema_sql: &str) -> Result<DbPool, EmberError> {
        let pool = DbPool::connect(&self.config).await?;
        pool.migrate(schema_sql).await?;
        Ok(pool)
    }

    /// Connect and apply all registered entity migrations.
    pub async fn connect_and_migrate_entities(&self) -> Result<DbPool, EmberError> {
        let pool = DbPool::connect(&self.config).await?;
        pool.migrate_registered_entities().await?;
        Ok(pool)
    }
}
