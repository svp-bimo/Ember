# ember-ext-mq

Message queue abstraction for Ember services.

## What it provides

- `MessageBus` trait for publishing messages.

## Example

```rust
use ember_ext_mq::MessageBus;

struct InMemoryBus;
impl MessageBus for InMemoryBus {
    fn publish(&self, topic: &str, payload: &[u8]) -> Result<(), ember_core::EmberError> {
        println!("topic={topic}, bytes={}", payload.len());
        Ok(())
    }
}
```

## Diagram

```mermaid
flowchart LR
    Service --> Bus[MessageBus]
    Bus --> Topic[(Topic)]
```

## Status

Early preview; adapters planned for real MQ backends.
