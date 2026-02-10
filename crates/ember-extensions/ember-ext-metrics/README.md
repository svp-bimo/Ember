# ember-ext-metrics

Metrics registry and counters for Ember services.

## What it provides

- `MetricsRegistry` for counters.
- `MetricsHandle` to fetch counters and snapshots.
- `Counter` for monotonic increments.

## Example

```rust
use ember_ext_metrics::MetricsHandle;

let metrics = MetricsHandle::global();
let counter = metrics.counter("requests.total");
counter.inc();
let snapshot = metrics.snapshot();
```

## Diagram

```mermaid
flowchart LR
    Counter --> Registry[MetricsRegistry]
    Registry --> Snapshot[MetricSample]
```

## Status

Early preview; exporter integrations planned.
