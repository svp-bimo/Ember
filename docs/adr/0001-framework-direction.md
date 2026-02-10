# ADR 0001: Framework Direction

## Status
Accepted

## Context
We want a Quarkus-inspired developer experience for Rust microservices. Runtime reflection is disallowed in favor of compile-time code generation for determinism and performance.

## Decision
- Ember is build-time first with proc-macro code generation.
- Runtime remains small and stable.
- Extensions remain opt-in, compiled separately from the core.

## Consequences
- Macros must stay thin and compile-first.
- Some features may require build-time registries instead of dynamic lookups.
- Testing strategy relies on compile tests for macro correctness.
