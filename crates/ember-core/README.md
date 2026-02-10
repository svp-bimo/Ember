# ember-core

Core types and minimal runtime API for Ember. This crate provides the smallest building blocks for running an Ember service, including the HTTP handler trait, run helpers, and re-exports of key extension types.

## What it solves

- A small, predictable runtime surface area.
- A single entrypoint to boot an Ember service with configuration + database wiring.
- Consistent HTTP handler types without reflection.

## Key concepts

- **`RunOptions`**: picks config sources, profiles, and service metadata.
- **`run_with_db_and_controller`**: bootstraps config, logging, DB migrations, and starts the service.
- **`HttpHandler` + `HttpResponse`**: minimal request/response contract.
- **Re-exports**: `Json`, `Route`, `Router`, `App`, `EmberError`, `ProblemDetails`.

## Example

```rust
use anyhow::Result;
use ember_core::RunOptions;
use my_service::controllers::MyController;
use my_service::services::MyService;

#[tokio::main]
async fn main() -> Result<()> {
    let options = RunOptions::new(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
    ember_core::run_with_db_and_controller::<my_service::config::AppConfig, _, _>(
        options,
        |_config| {
            let service = MyService::new();
            MyController::new(service)
        },
    )
    .await?;
    Ok(())
}
```

## Architecture (high level)

```mermaid
flowchart LR
    Config[Config (YAML/Env)] --> Core[ember-core]
    Core --> Logging[ember-logging]
    Core --> Db[ember-ext-db]
    Core --> Runtime[ember-ext-runtime]
    Runtime --> Http[HTTP Handler]
```

## Status

Early preview. APIs are evolving.

## Related crates

- `ember-ext-runtime` – App builder and route registration.
- `ember-ext-http` – `Json`, `Route`, `Router`.
- `ember-ext-db` – repository + migrations.
- `ember-logging` – logging initialization and startup banner.
