#![forbid(unsafe_code)]

//! Query helpers for repositories.

use ember_ext_exceptions::EmberError;

use crate::repository::Entity;

/// Query value for a field.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QueryValue {
    /// String value.
    String(String),
    /// Signed integer value.
    I64(i64),
    /// Unsigned integer value.
    U64(u64),
    /// Boolean value.
    Bool(bool),
}

impl From<String> for QueryValue {
    fn from(value: String) -> Self {
        QueryValue::String(value)
    }
}

impl From<&str> for QueryValue {
    fn from(value: &str) -> Self {
        QueryValue::String(value.to_owned())
    }
}

impl From<i64> for QueryValue {
    fn from(value: i64) -> Self {
        QueryValue::I64(value)
    }
}

impl From<u64> for QueryValue {
    fn from(value: u64) -> Self {
        QueryValue::U64(value)
    }
}

impl From<bool> for QueryValue {
    fn from(value: bool) -> Self {
        QueryValue::Bool(value)
    }
}

/// Query specification.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Query {
    /// Field name.
    pub field: &'static str,
    /// Field value.
    pub value: QueryValue,
}

impl Query {
    /// Create a new query from field and value.
    pub fn new(field: &'static str, value: impl Into<QueryValue>) -> Self {
        Self {
            field,
            value: value.into(),
        }
    }
}

/// Repository extension supporting ad-hoc queries.
pub trait QueryRepository<E: Entity> {
    /// Find entities by query.
    fn find_by(&self, query: Query) -> Result<Vec<E>, EmberError>;
}
