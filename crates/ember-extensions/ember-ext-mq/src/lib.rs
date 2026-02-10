#![forbid(unsafe_code)]

//! Message queue extension for Ember.

use ember_core::EmberError;

/// A placeholder message bus abstraction.
pub trait MessageBus {
    /// Publish a message payload to a topic.
    fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), EmberError>;
}
