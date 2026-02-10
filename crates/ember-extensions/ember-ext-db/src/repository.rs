#![forbid(unsafe_code)]

//! Repository traits and basic implementations.

use std::marker::PhantomData;

use ember_ext_exceptions::EmberError;

use crate::optional::Optional;

/// Marker trait for Ember entities.
///
/// This is a compile-time marker with an associated identifier type.
pub trait Entity {
    /// The primary key type for the entity.
    type Id: Clone + Send + Sync + 'static;

    /// Return the entity identifier.
    fn id(&self) -> Self::Id;
}

/// A repository interface for working with entities.
///
/// Implementations can be backed by SQL, in-memory stores, or other systems.
pub trait Repository<E: Entity> {
    /// Find an entity by id.
    fn find_by_id(&self, id: E::Id) -> Result<Optional<E>, EmberError>;

    /// Persist an entity.
    fn save(&self, entity: E) -> Result<E, EmberError>;

    /// Delete an entity by id.
    fn delete_by_id(&self, id: E::Id) -> Result<(), EmberError>;
}

/// A minimal in-memory repository for testing or examples.
#[derive(Debug, Default)]
pub struct InMemoryRepository<E: Entity> {
    _marker: PhantomData<E>,
}

impl<E: Entity> InMemoryRepository<E> {
    /// Create a new in-memory repository.
    pub fn new() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<E: Entity> Repository<E> for InMemoryRepository<E> {
    fn find_by_id(&self, _id: E::Id) -> Result<Optional<E>, EmberError> {
        Ok(Optional::empty())
    }

    fn save(&self, entity: E) -> Result<E, EmberError> {
        Ok(entity)
    }

    fn delete_by_id(&self, _id: E::Id) -> Result<(), EmberError> {
        Ok(())
    }
}
