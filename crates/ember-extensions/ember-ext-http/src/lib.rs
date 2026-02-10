#![forbid(unsafe_code)]

//! HTTP routing primitives for Ember.

use serde::{Deserialize, Serialize};

/// A lightweight JSON wrapper for Ember handlers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Json<T>(pub T);

impl<T> Json<T> {
    /// Create a new JSON wrapper.
    pub fn new(value: T) -> Self {
        Self(value)
    }
}

impl<T> From<T> for Json<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}

/// An HTTP route registered with Ember.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    /// HTTP method.
    pub method: &'static str,
    /// Route path.
    pub path: &'static str,
}

/// A simple route registry.
#[derive(Debug, Default)]
pub struct Router {
    routes: Vec<Route>,
}

impl Router {
    /// Create an empty router.
    pub fn new() -> Self {
        Self { routes: Vec::new() }
    }

    /// Register a route.
    pub fn register(&mut self, method: &'static str, path: &'static str) {
        self.routes.push(Route { method, path });
    }

    /// Read the registered routes.
    pub fn routes(&self) -> &[Route] {
        &self.routes
    }
}
