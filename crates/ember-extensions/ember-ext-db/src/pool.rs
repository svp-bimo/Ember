#![forbid(unsafe_code)]

//! Postgres connection pool wrapper.

use ember_ext_exceptions::EmberError;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;

use crate::config::DbConfig;

/// A Postgres connection pool wrapper.
#[derive(Debug, Clone)]
pub struct DbPool {
    pool: PgPool,
}

impl DbPool {
    /// Connect to the database using the provided config.
    pub async fn connect(config: &DbConfig) -> Result<Self, EmberError> {
        let pool = PgPoolOptions::new()
            .connect(&config.url)
            .await
            .map_err(|err| EmberError::msg(format!("db connect failed: {err}")))?;
        Ok(Self { pool })
    }

    /// Execute a SQL statement.
    pub async fn execute(&self, sql: &str) -> Result<(), EmberError> {
        sqlx::query(sql)
            .execute(&self.pool)
            .await
            .map_err(|err| EmberError::msg(format!("db execute failed: {err}")))?;
        Ok(())
    }

    /// Apply a schema migration statement.
    pub async fn migrate(&self, sql: &str) -> Result<(), EmberError> {
        self.execute(sql).await
    }

    /// Apply all registered entity migrations.
    pub async fn migrate_registered_entities(&self) -> Result<(), EmberError> {
        for sql in crate::registered_entity_migrations() {
            self.migrate(sql).await?;
        }
        Ok(())
    }

    /// Access the inner pool.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
