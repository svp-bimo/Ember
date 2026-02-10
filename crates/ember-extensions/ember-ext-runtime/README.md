# ember-ext-runtime

Runtime facade types for Ember. Provides the `App` builder and controller metadata integration.

## What it provides

- `App` builder for route registration.
- `ControllerMetadata` trait for macro-generated route metadata.

## Example

```rust
use ember_ext_runtime::App;

let mut app = App::new();
app.register_route("GET", "/health");
app.run()?;
```

## Diagram

```mermaid
flowchart LR
    Controller --> Metadata[ControllerMetadata]
    Metadata --> App
    App --> Routes[Route Registry]
```

## Status

Early preview. The runtime will grow as Ember matures.
