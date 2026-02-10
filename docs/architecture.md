# Ember Architecture

Ember is designed around build-time metadata and compile-time code generation. The runtime remains minimal and stable, while macros produce registries for routing, dependency injection, and OpenAPI documents.

## Goals
- Build-time first: no runtime reflection.
- Fast feedback loops and predictable behavior.
- Clear separation between core APIs and extensions.

## High-level flow
1. Developers annotate controllers, services, and config types with Ember macros.
2. Proc-macros emit metadata into build-time registries.
3. The runtime assembles routes and DI graphs from generated metadata.
4. Extensions plug into middleware and compilation hooks.
