#![forbid(unsafe_code)]

//! Metrics extension for Ember.

use std::collections::HashMap;
use std::sync::{atomic::AtomicU64, atomic::Ordering, Arc, Mutex, OnceLock};

use serde::Serialize;

use ember_core::App;

/// Install metrics instrumentation for an Ember app.
///
/// This initializes the global metrics registry for use by the service.
pub fn install_metrics(_app: &mut App) {
    let _ = global_registry();
}

/// Global metrics registry initializer.
pub fn global_registry() -> &'static MetricsRegistry {
    static REGISTRY: OnceLock<MetricsRegistry> = OnceLock::new();
    REGISTRY.get_or_init(MetricsRegistry::new)
}

/// A lightweight handle to the metrics registry.
#[derive(Debug, Clone)]
pub struct MetricsHandle {
    registry: &'static MetricsRegistry,
}

impl MetricsHandle {
    /// Create a handle for the global registry.
    pub fn global() -> Self {
        Self {
            registry: global_registry(),
        }
    }

    /// Get or create a counter.
    pub fn counter(&self, name: impl Into<String>) -> Arc<Counter> {
        self.registry.counter(name)
    }

    /// Collect a snapshot of current counter values.
    pub fn snapshot(&self) -> Vec<MetricSample> {
        self.registry.snapshot()
    }
}

/// A registry of counters stored in-memory.
#[derive(Debug, Default)]
pub struct MetricsRegistry {
    counters: Mutex<HashMap<String, Arc<Counter>>>,
}

impl MetricsRegistry {
    /// Create a new empty registry.
    pub fn new() -> Self {
        Self {
            counters: Mutex::new(HashMap::new()),
        }
    }

    /// Get or create a counter with the given name.
    pub fn counter(&self, name: impl Into<String>) -> Arc<Counter> {
        let name = name.into();
        let mut counters = self.counters.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        counters
            .entry(name)
            .or_insert_with(|| Arc::new(Counter::new()))
            .clone()
    }

    /// Collect a snapshot of all counters.
    pub fn snapshot(&self) -> Vec<MetricSample> {
        let counters = self.counters.lock().unwrap_or_else(|poisoned| poisoned.into_inner());
        counters
            .iter()
            .map(|(name, counter)| MetricSample {
                name: name.clone(),
                value: counter.get(),
            })
            .collect()
    }
}

/// A monotonically increasing counter.
#[derive(Debug, Default)]
pub struct Counter {
    value: AtomicU64,
}

impl Counter {
    /// Create a new counter initialized to zero.
    pub fn new() -> Self {
        Self {
            value: AtomicU64::new(0),
        }
    }

    /// Increment the counter by one.
    pub fn inc(&self) {
        self.add(1);
    }

    /// Add an arbitrary value to the counter.
    pub fn add(&self, amount: u64) {
        self.value.fetch_add(amount, Ordering::Relaxed);
    }

    /// Get the current value.
    pub fn get(&self) -> u64 {
        self.value.load(Ordering::Relaxed)
    }
}

/// A sample of a metric value.
#[derive(Debug, Clone, Serialize)]
pub struct MetricSample {
    /// The metric name.
    pub name: String,
    /// The metric value.
    pub value: u64,
}
