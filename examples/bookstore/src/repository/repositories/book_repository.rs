#![forbid(unsafe_code)]

//! Book repository with custom queries.

use std::sync::{Arc, Mutex, MutexGuard};

use ember_ext_db::{Optional, Query, QueryRepository, QueryValue, Repository};

use crate::domain::book::BookId;
use crate::repository::entities::book_entity::{BookEntity, BookEntityUpdate};

#[derive(Debug, Default)]
struct BookStore {
    next_id: BookId,
    books: Vec<BookEntity>,
}

/// Repository for books.
#[derive(Clone, Default)]
pub struct BookRepository {
    state: Arc<Mutex<BookStore>>,
}

impl BookRepository {
    /// Create a new repository with seed data.
    pub fn new() -> Self {
        let mut store = BookStore::default();
        store.seed();
        Self {
            state: Arc::new(Mutex::new(store)),
        }
    }

    /// List all books.
    pub fn list_all(&self) -> Vec<BookEntity> {
        let store = self.lock_store();
        store.books.clone()
    }

    /// Find books by author name.
    pub fn find_by_author_name(&self, author: &str) -> Vec<BookEntity> {
        let store = self.lock_store();
        store
            .books
            .iter()
            .filter(|book| book.author.eq_ignore_ascii_case(author))
            .cloned()
            .collect()
    }

    /// Update a book using a partial update.
    pub fn update(&self, update: BookEntityUpdate) -> Optional<BookEntity> {
        let mut store = self.lock_store_mut();
        let book = store.books.iter_mut().find(|book| book.id == update.id);
        let Some(book) = book else {
            return Optional::empty();
        };

        if let Some(title) = update.title {
            book.title = title;
        }
        if let Some(author) = update.author {
            book.author = author;
        }
        if let Some(year) = update.year {
            book.year = year;
        }

        Optional::of(book.clone())
    }

    fn lock_store(&self) -> MutexGuard<'_, BookStore> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }

    fn lock_store_mut(&self) -> MutexGuard<'_, BookStore> {
        self.state
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
    }
}

impl Repository<BookEntity> for BookRepository {
    fn find_by_id(&self, id: BookId) -> Result<Optional<BookEntity>, ember_core::EmberError> {
        let store = self.lock_store();
        let book = store.books.iter().find(|book| book.id == id).cloned();
        Ok(Optional::from(book))
    }

    fn save(&self, mut entity: BookEntity) -> Result<BookEntity, ember_core::EmberError> {
        let mut store = self.lock_store_mut();
        if entity.id == 0 {
            entity.id = store.next_id;
            store.next_id = store.next_id.saturating_add(1);
        }
        store.books.push(entity.clone());
        Ok(entity)
    }

    fn delete_by_id(&self, id: BookId) -> Result<(), ember_core::EmberError> {
        let mut store = self.lock_store_mut();
        store.books.retain(|book| book.id != id);
        Ok(())
    }
}

impl QueryRepository<BookEntity> for BookRepository {
    fn find_by(&self, query: Query) -> Result<Vec<BookEntity>, ember_core::EmberError> {
        match (query.field, query.value) {
            ("author", QueryValue::String(author)) => Ok(self.find_by_author_name(&author)),
            _ => Ok(Vec::new()),
        }
    }
}

impl BookStore {
    fn seed(&mut self) {
        self.next_id = 1;
        self.books = vec![
            BookEntity {
                id: 1,
                title: "The Rust Book".to_owned(),
                author: "Steve Klabnik".to_owned(),
                year: 2019,
            },
            BookEntity {
                id: 2,
                title: "Programming Rust".to_owned(),
                author: "Jim Blandy".to_owned(),
                year: 2021,
            },
        ];
        self.next_id = 3;
    }
}
