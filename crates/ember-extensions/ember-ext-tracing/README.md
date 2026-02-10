# ember-ext-tracing

Tracing initialization hooks for Ember. This crate is currently a placeholder for future tracing configuration.

## Example

```rust
use ember_ext_tracing::init_tracing;

init_tracing();
```

## Diagram

```mermaid
flowchart LR
    Service --> Tracing[init_tracing]
    Tracing --> Subscriber[tracing subscriber]
```

## Status

Early preview; implementation is intentionally minimal right now.
