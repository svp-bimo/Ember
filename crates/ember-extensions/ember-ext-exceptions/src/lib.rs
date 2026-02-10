#![forbid(unsafe_code)]

//! Exception and error payloads for Ember.

use serde::{Deserialize, Serialize};

/// Ember error type for library crates.
#[derive(Debug, thiserror::Error)]
pub enum EmberError {
    /// A generic error message.
    #[error("{message}")]
    Message { message: String },
}

impl EmberError {
    /// Create a new error from a message.
    pub fn msg(message: impl Into<String>) -> Self {
        Self::Message {
            message: message.into(),
        }
    }
}

/// A consistent JSON error payload inspired by Problem Details.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProblemDetails {
    /// A URI reference that identifies the problem type.
    #[serde(rename = "type")]
    pub type_: String,
    /// A short, human-readable summary of the problem type.
    pub title: String,
    /// The HTTP status code for this occurrence of the problem.
    pub status: u16,
    /// A human-readable explanation specific to this occurrence.
    pub detail: String,
    /// A URI reference that identifies the specific occurrence.
    pub instance: String,
}

impl ProblemDetails {
    /// Create a new problem details payload.
    pub fn new(
        type_: impl Into<String>,
        title: impl Into<String>,
        status: u16,
        detail: impl Into<String>,
        instance: impl Into<String>,
    ) -> Self {
        Self {
            type_: type_.into(),
            title: title.into(),
            status,
            detail: detail.into(),
            instance: instance.into(),
        }
    }

    /// Create a 404 Not Found problem details payload.
    pub fn not_found(detail: impl Into<String>) -> Self {
        Self::new(
            "about:blank",
            "Not Found",
            404,
            detail,
            "urn:ember:problem:not-found",
        )
    }

    /// Create a 500 Internal Server Error problem details payload.
    pub fn internal_error(detail: impl Into<String>) -> Self {
        Self::new(
            "about:blank",
            "Internal Server Error",
            500,
            detail,
            "urn:ember:problem:internal-error",
        )
    }
}
