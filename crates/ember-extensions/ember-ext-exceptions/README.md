# ember-ext-exceptions

Error and problem details types for Ember services.

## What it provides

- `EmberError` for library errors.
- `ProblemDetails` for consistent JSON error payloads.

## Example

```rust
use ember_ext_exceptions::{EmberError, ProblemDetails};

let err = EmberError::msg("something went wrong");
let problem = ProblemDetails::internal_error("unexpected failure");
```

## Diagram

```mermaid
flowchart LR
    Error[EmberError] --> Handler[HTTP Handler]
    Handler --> Payload[ProblemDetails]
```

## Status

Stable foundational types used across Ember.
