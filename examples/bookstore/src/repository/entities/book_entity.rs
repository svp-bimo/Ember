#![forbid(unsafe_code)]

//! Book entity definition.

use ember_macros::entity;
use serde::{Deserialize, Serialize};

use crate::domain::book::BookId;

/// Book entity stored in the repository.
#[entity(id = "id", table = "books")]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookEntity {
    /// Book identifier.
    pub id: BookId,
    /// Book title.
    pub title: String,
    /// Book author.
    pub author: String,
    /// Publication year.
    pub year: u16,
}

/// Update command for book entities.
#[derive(Debug, Clone)]
pub struct BookEntityUpdate {
    /// Identifier of the book to update.
    pub id: BookId,
    /// Optional title update.
    pub title: Option<String>,
    /// Optional author update.
    pub author: Option<String>,
    /// Optional year update.
    pub year: Option<u16>,
}
