#![forbid(unsafe_code)]

//! Database extension for Ember (JPA-inspired).

mod config;
mod context;
mod optional;
mod pool;
mod query;
mod repository;

pub use config::{DbConfig, HasDbConfig};
pub use context::DbContext;
pub use optional::Optional;
pub use pool::DbPool;
pub use query::{Query, QueryRepository, QueryValue};
pub use repository::{Entity, InMemoryRepository, Repository};

/// Re-export inventory for macro-generated registrations.
pub use inventory;

/// Registered entity migration.
#[derive(Debug, Clone)]
pub struct EntityMigration {
	/// SQL statement to create or migrate the entity table.
	pub sql: &'static str,
}

inventory::collect!(EntityMigration);

/// Collect all registered entity migrations.
pub fn registered_entity_migrations() -> Vec<&'static str> {
	inventory::iter::<EntityMigration>
		.into_iter()
		.map(|entry| entry.sql)
		.collect()
}
