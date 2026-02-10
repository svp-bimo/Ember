#![forbid(unsafe_code)]

//! Health check extension for Ember.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, OnceLock};

use ember_core::App;

/// Register health routes for an Ember app.
///
/// This initializes the global health registry.
pub fn add_health_routes(_app: &mut App) {
    let _ = global_registry();
}

/// Global health registry initializer.
pub fn global_registry() -> &'static HealthRegistry {
    static REGISTRY: OnceLock<HealthRegistry> = OnceLock::new();
    REGISTRY.get_or_init(HealthRegistry::new)
}

/// Status of a health check.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HealthStatus {
    /// The check passed.
    Healthy,
    /// The check failed.
    Unhealthy,
}

impl HealthStatus {
    /// Return `true` if the status is healthy.
    pub fn is_healthy(self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }
}

/// Result of a health check.
#[derive(Debug, Clone)]
pub struct HealthReport {
    /// Name of the check.
    pub name: String,
    /// Status of the check.
    pub status: HealthStatus,
    /// Optional detail message.
    pub detail: Option<String>,
}

impl HealthReport {
    /// Create a healthy report.
    pub fn healthy(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Healthy,
            detail: None,
        }
    }

    /// Create an unhealthy report with detail.
    pub fn unhealthy(name: impl Into<String>, detail: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            status: HealthStatus::Unhealthy,
            detail: Some(detail.into()),
        }
    }
}

/// Health check hook.
pub trait HealthCheck: Send + Sync {
    /// Run the health check and return a report.
    fn check(&self) -> HealthReport;
}

/// Registry of health checks.
#[derive(Default)]
pub struct HealthRegistry {
    checks: Mutex<HashMap<String, Arc<dyn HealthCheck>>>,
}

impl HealthRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            checks: Mutex::new(HashMap::new()),
        }
    }

    /// Register a health check by name.
    pub fn register_check(&self, name: impl Into<String>, check: Arc<dyn HealthCheck>) {
        let mut checks = self.checks.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        checks.insert(name.into(), check);
    }

    /// Execute all checks and return their reports.
    pub fn snapshot(&self) -> Vec<HealthReport> {
        let checks = self.checks.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        checks.values().map(|check| check.check()).collect()
    }
}
