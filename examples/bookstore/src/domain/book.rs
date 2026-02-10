#![forbid(unsafe_code)]

//! Domain model for books.

/// Book identifier type.
pub type BookId = u64;

/// Book domain object.
#[derive(Debug, Clone)]
pub struct Book {
    /// Book identifier.
    pub id: BookId,
    /// Book title.
    pub title: String,
    /// Book author.
    pub author: String,
    /// Publication year.
    pub year: u16,
}

/// Command for creating a new book.
#[derive(Debug, Clone)]
pub struct NewBook {
    /// Book title.
    pub title: String,
    /// Book author.
    pub author: String,
    /// Publication year.
    pub year: u16,
}

/// Command for updating a book.
#[derive(Debug, Clone)]
pub struct BookUpdate {
    /// Identifier of the book to update.
    pub id: BookId,
    /// Optional title update.
    pub title: Option<String>,
    /// Optional author update.
    pub author: Option<String>,
    /// Optional year update.
    pub year: Option<u16>,
}
