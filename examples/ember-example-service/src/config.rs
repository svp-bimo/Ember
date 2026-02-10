#![forbid(unsafe_code)]

//! Configuration for the example service.

use ember_macros::{config, EmberConfig};
use serde::Deserialize;

/// Application configuration placeholder.
#[config]
#[derive(Clone, Debug, Deserialize, EmberConfig)]
pub struct AppConfig {
    /// Service name.
    pub service_name: String,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            service_name: "ember-example-service".to_owned(),
        }
    }
}
