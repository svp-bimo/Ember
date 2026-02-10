# ADR 0002: Observability Standard

## Status
Proposed

## Context
Ember aims to provide production-ready microservices with a consistent developer experience. Observability (logging, metrics, tracing) should be predictable across services while remaining lightweight and opt-in.

## Decision
- **Logging** is the baseline default via `ember-logging`, using `tracing_subscriber` with `RUST_LOG` configuration.
- **Metrics** are opt-in via `ember-ext-metrics`, providing in-memory counters and snapshots.
- **Tracing** is opt-in via `ember-ext-tracing`, allowing integration with standard `tracing` ecosystems.
- Ember does not mandate a specific exporter backend; exporters are configured by the application.

## Consequences
- All Ember services can enable consistent logging with minimal setup.
- Metrics and tracing remain lightweight and only included when explicitly enabled.
- Exporter integrations (Prometheus/OpenTelemetry) will be provided as extensions or adapters, not core.
