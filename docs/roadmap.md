# Ember Roadmap (Draft)

## Top-3 priorities (next)
1. **Minimal HTTP runtime**
   - Use generated metadata to register routes into a lightweight router.
   - Provide a stable handler signature for controllers.

2. **Configuration binding**
   - Environment + file + default layering.
   - Typed config structs with compile-time validation hooks.

3. **DI registry**
   - Constructor injection with compile-time registries.
   - No runtime reflection or service locator.

## Near-term follow-ups
- Security filter chain with policy checks.
- OpenAPI generation and golden file tests.
- Observability: tracing + metrics + structured logs.
- CLI project templates + `ember dev`.

## Notes
This roadmap favors compile-first design and stable public APIs before runtime expansion.
