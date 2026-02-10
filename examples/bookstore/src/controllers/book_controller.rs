#![forbid(unsafe_code)]

//! Book controller for the bookstore example.

use ember_core::Json;
use ember_macros::{controller, delete, get, post, put};

use crate::controllers::dto::{BookResponse, CreateBookRequest, UpdateBookRequest};
use crate::domain::book::BookId;
use crate::mappers::controller_mapper;
use crate::services::book_service::BookService;

/// Controller for book endpoints.
#[derive(Clone)]
pub struct BookController {
    service: BookService,
}

impl BookController {
    /// Create a new book controller.
    pub fn new(service: BookService) -> Self {
        Self { service }
    }
}

#[controller]
impl BookController {
    /// Get all books.
    #[get("/books")]
    pub fn list_books(&self) -> Json<Vec<BookResponse>> {
        let books = self.service.list_books();
        Json(books.into_iter().map(controller_mapper::to_response).collect())
    }

    /// Get a book by id.
    #[get("/books/{id}")]
    pub fn get_book(&self, id: BookId) -> Json<Option<BookResponse>> {
        let book = self
            .service
            .get_book(id)
            .into_option()
            .map(controller_mapper::to_response);
        Json(book)
    }

    /// Search for books by author.
    #[get("/books/search")]
    pub fn search_by_author(&self, author: String) -> Json<Vec<BookResponse>> {
        let books = self.service.search_by_author(&author);
        Json(books.into_iter().map(controller_mapper::to_response).collect())
    }

    /// Add a new book.
    #[post("/books")]
    pub fn add_book(&self, input: CreateBookRequest) -> Json<BookResponse> {
        let book = self.service.add_book(controller_mapper::to_new_book(input));
        Json(controller_mapper::to_response(book))
    }

    /// Update a book.
    #[put("/books/{id}")]
    pub fn update_book(&self, update: UpdateBookRequest) -> Json<Option<BookResponse>> {
        let book = self
            .service
            .update_book(controller_mapper::to_update(update))
            .into_option()
            .map(controller_mapper::to_response);
        Json(book)
    }

    /// Remove a book by id.
    #[delete("/books/{id}")]
    pub fn remove_book(&self, id: BookId) -> Json<bool> {
        Json(self.service.remove_book(id))
    }

    /// Remove all books.
    #[delete("/books")]
    pub fn remove_all_books(&self) -> Json<usize> {
        Json(self.service.remove_all())
    }
}
