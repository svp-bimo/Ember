#![forbid(unsafe_code)]

//! Ember runtime facade types.

use ember_ext_exceptions::EmberError;
use ember_ext_http::{Route, Router};

/// Metadata provided by controller macros.
pub trait ControllerMetadata {
    /// Return all routes for the controller.
    fn routes() -> &'static [Route];
}

/// An Ember application builder.
#[derive(Debug, Default)]
pub struct App {
    router: Router,
}

impl App {
    /// Create a new Ember application builder.
    pub fn new() -> Self {
        Self {
            router: Router::new(),
        }
    }

    /// Register a controller with the application.
    ///
    /// This is a placeholder that does not yet persist metadata.
    pub fn register_controller<T: ControllerMetadata>(&mut self, _controller: T) -> &mut Self {
        for route in T::routes() {
            self.router.register(route.method, route.path);
        }
        self
    }

    /// Register a route with the application.
    pub fn register_route(&mut self, method: &'static str, path: &'static str) -> &mut Self {
        self.router.register(method, path);
        self
    }

    /// Access the current route registry.
    pub fn routes(&self) -> &[Route] {
        self.router.routes()
    }

    /// Run the application.
    pub fn run(self) -> Result<(), EmberError> {
        let routes = self.router.routes().len();
        if routes > 0 {
            tracing::info!(routes, "ember-> app started");
        } else {
            tracing::info!("ember-> app started");
        }
        Ok(())
    }
}
