#![forbid(unsafe_code)]

//! Book service for the bookstore example.

use ember_ext_db::{Optional, Query, QueryRepository, Repository};
use ember_macros::service;

use crate::domain::book::{Book, BookId, BookUpdate, NewBook};
use crate::mappers::repository_mapper;
use crate::repository::repositories::book_repository::BookRepository;

/// Service for managing books.
#[service]
#[derive(Clone, Default)]
pub struct BookService {
    repository: BookRepository,
}

impl BookService {
    /// Create a new book service with initial data.
    pub fn new() -> Self {
        Self {
            repository: BookRepository::new(),
        }
    }

    /// List all books.
    pub fn list_books(&self) -> Vec<Book> {
        self.repository
            .list_all()
            .into_iter()
            .map(repository_mapper::entity_to_domain)
            .collect()
    }

    /// Get a specific book by id.
    pub fn get_book(&self, id: BookId) -> Optional<Book> {
        self.repository
            .find_by_id(id)
            .unwrap_or_else(|_| Optional::empty())
            .map(repository_mapper::entity_to_domain)
    }

    /// Search for books by author.
    pub fn search_by_author(&self, author: &str) -> Vec<Book> {
        self.repository
            .find_by(Query::new("author", author))
            .unwrap_or_else(|_| Vec::new())
            .into_iter()
            .map(repository_mapper::entity_to_domain)
            .collect()
    }

    /// Add a new book.
    pub fn add_book(&self, input: NewBook) -> Book {
        let entity = repository_mapper::new_to_entity(input);
        self.repository
            .save(entity)
            .map(repository_mapper::entity_to_domain)
            .unwrap_or_else(|_| Book {
                id: 0,
                title: "".to_owned(),
                author: "".to_owned(),
                year: 0,
            })
    }

    /// Update an existing book.
    pub fn update_book(&self, update: BookUpdate) -> Optional<Book> {
        let entity_update = repository_mapper::update_to_entity(update);
        self.repository
            .update(entity_update)
            .map(repository_mapper::entity_to_domain)
    }

    /// Remove a book by id.
    pub fn remove_book(&self, id: BookId) -> bool {
        self.repository.delete_by_id(id).is_ok()
    }

    /// Remove all books.
    pub fn remove_all(&self) -> usize {
        let entities = self.repository.list_all();
        let count = entities.len();
        for entity in entities {
            let _ = self.repository.delete_by_id(entity.id);
        }
        count
    }
}
