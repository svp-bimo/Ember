#![forbid(unsafe_code)]

//! Mappers between controller DTOs and domain models.

use crate::controllers::dto::{BookResponse, CreateBookRequest, UpdateBookRequest};
use crate::domain::book::{Book, BookUpdate, NewBook};

/// Map domain book to response.
pub fn to_response(book: Book) -> BookResponse {
    BookResponse {
        id: book.id,
        title: book.title,
        author: book.author,
        year: book.year,
    }
}

/// Map create request to domain new book.
pub fn to_new_book(request: CreateBookRequest) -> NewBook {
    NewBook {
        title: request.title,
        author: request.author,
        year: request.year,
    }
}

/// Map update request to domain update.
pub fn to_update(request: UpdateBookRequest) -> BookUpdate {
    BookUpdate {
        id: request.id,
        title: request.title,
        author: request.author,
        year: request.year,
    }
}
