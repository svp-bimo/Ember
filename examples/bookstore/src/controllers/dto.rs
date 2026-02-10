#![forbid(unsafe_code)]

//! DTOs for the book controller.

use serde::{Deserialize, Serialize};

use crate::domain::book::BookId;

/// Book response payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BookResponse {
    /// Book identifier.
    pub id: BookId,
    /// Book title.
    pub title: String,
    /// Book author.
    pub author: String,
    /// Publication year.
    pub year: u16,
}

/// Input payload for creating a book.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateBookRequest {
    /// Book title.
    pub title: String,
    /// Book author.
    pub author: String,
    /// Publication year.
    pub year: u16,
}

/// Input payload for updating a book.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateBookRequest {
    /// Identifier of the book to update.
    pub id: BookId,
    /// Optional title update.
    pub title: Option<String>,
    /// Optional author update.
    pub author: Option<String>,
    /// Optional year update.
    pub year: Option<u16>,
}
