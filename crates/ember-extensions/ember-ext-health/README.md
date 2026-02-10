# ember-ext-health

Health check registry for Ember services. Provides a global registry to register checks and produce health reports.

## What it provides

- `HealthRegistry` for registering checks.
- `HealthCheck` trait for custom health probes.
- `HealthReport` with status + detail.

## Example

```rust
use std::sync::Arc;
use ember_ext_health::{global_registry, HealthCheck, HealthReport};

struct DbHealth;
impl HealthCheck for DbHealth {
    fn check(&self) -> HealthReport {
        HealthReport::healthy("db")
    }
}

let registry = global_registry();
registry.register_check("db", Arc::new(DbHealth));
let snapshot = registry.snapshot();
```

## Diagram

```mermaid
flowchart LR
    Check[HealthCheck] --> Registry[HealthRegistry]
    Registry --> Report[HealthReport]
```

## Status

Stable lightweight registry.
