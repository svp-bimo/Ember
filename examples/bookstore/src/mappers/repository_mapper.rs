#![forbid(unsafe_code)]

//! Mappers between repository entities and domain models.

use crate::domain::book::{Book, BookUpdate, NewBook};
use crate::repository::entities::book_entity::{BookEntity, BookEntityUpdate};

/// Map entity to domain.
pub fn entity_to_domain(entity: BookEntity) -> Book {
    Book {
        id: entity.id,
        title: entity.title,
        author: entity.author,
        year: entity.year,
    }
}

/// Map domain to entity.
pub fn domain_to_entity(book: Book) -> BookEntity {
    BookEntity {
        id: book.id,
        title: book.title,
        author: book.author,
        year: book.year,
    }
}

/// Map new book to entity.
pub fn new_to_entity(book: NewBook) -> BookEntity {
    BookEntity {
        id: 0,
        title: book.title,
        author: book.author,
        year: book.year,
    }
}

/// Map domain update to entity update.
pub fn update_to_entity(update: BookUpdate) -> BookEntityUpdate {
    BookEntityUpdate {
        id: update.id,
        title: update.title,
        author: update.author,
        year: update.year,
    }
}
