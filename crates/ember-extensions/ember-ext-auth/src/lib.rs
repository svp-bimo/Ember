#![forbid(unsafe_code)]

//! Authentication extension for Ember.

use ember_core::EmberError;

/// Authenticate incoming requests.
pub trait Authenticator {
    /// Validate a token and return success or an error.
    fn authenticate(&self, token: &str) -> Result<(), EmberError>;
}

/// Security context produced after successful authentication.
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Subject identifier (user or service).
    pub subject: String,
    /// Optional roles for authorization.
    pub roles: Vec<String>,
}

impl SecurityContext {
    /// Create a new security context.
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            roles: Vec::new(),
        }
    }

    /// Attach roles to the context.
    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = roles;
        self
    }
}

/// A minimal request view used by security filters.
#[derive(Debug, Clone)]
pub struct SecurityRequest {
    /// Request path.
    pub path: String,
    /// Authorization header value, if present.
    pub authorization: Option<String>,
}

/// Security filter hook that can validate tokens and build a security context.
pub trait SecurityFilter {
    /// Validate the request and return a security context.
    fn filter(&self, request: &SecurityRequest) -> Result<SecurityContext, EmberError>;
}

/// Validate an authorization header for a Bearer token.
pub fn parse_bearer_token(header_value: &str) -> Option<String> {
    let trimmed = header_value.trim();
    let prefix = "Bearer ";
    if trimmed.starts_with(prefix) {
        let token = trimmed[prefix.len()..].trim();
        if token.is_empty() {
            None
        } else {
            Some(token.to_owned())
        }
    } else {
        None
    }
}
