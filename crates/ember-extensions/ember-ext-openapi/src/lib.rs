#![forbid(unsafe_code)]

//! OpenAPI extension for Ember.

use serde_json::Value;

/// Generate an OpenAPI document.
pub fn generate_openapi() -> Value {
    serde_json::json!({
        "openapi": "3.0.0",
        "info": {
            "title": "Ember API",
            "version": "0.1.0"
        },
        "paths": {}
    })
}
