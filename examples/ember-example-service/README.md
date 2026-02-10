# Ember Example Service

This example demonstrates Ember-style controllers and a standalone fallback runtime.

## Run with Ember placeholders

```bash
cargo run -p ember-example-service
```

## Run in standalone mode (Axum)

```bash
cargo run -p ember-example-service --features standalone
```

Then open:
- `http://localhost:8080/health`
- `http://localhost:8080/fail`
