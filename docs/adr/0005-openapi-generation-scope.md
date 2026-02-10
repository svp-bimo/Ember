# ADR 0005: OpenAPI Generation Scope

## Status
Proposed

## Context
Ember intends to support OpenAPI generation based on controller routes and DTOs. We need a clear scope for what is generated automatically versus manually provided.

## Decision
- Initial OpenAPI generation provides:
  - API metadata (title, version).
  - Paths based on controller routes.
  - Basic request/response schemas derived from DTOs where possible.
- Advanced features (auth schemes, tags, examples, pagination) will be **opt-in annotations** or config-driven.
- OpenAPI generation remains a separate extension (`ember-ext-openapi`).

## Consequences
- Developers get a usable baseline OpenAPI document with minimal setup.
- More complex APIs can enrich the document incrementally.
- The OpenAPI extension can evolve without impacting core runtime stability.
