# ADR 0004: Extension Lifecycle

## Status
Proposed

## Context
Ember uses opt-in extensions to keep the core runtime minimal. Extensions should have a clear lifecycle so apps can register capabilities consistently.

## Decision
- Extensions are **compile-time linked** and opt-in via Cargo dependencies.
- Each extension exposes an **explicit initialization hook** (e.g., `install_metrics`, `add_health_routes`).
- Ember core does not auto-discover extensions; the application opts in and calls initialization explicitly.
- Future extension hooks should be deterministic and side-effect free outside initialization.

## Consequences
- Services remain lean by default; no hidden runtime scanning.
- Extensions are easy to reason about and test in isolation.
- App code controls extension activation order.
