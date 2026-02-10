# ember-ext-auth

Authentication primitives for Ember. Provides interfaces for authentication and security context building.

## What it provides

- `Authenticator` trait for token validation.
- `SecurityFilter` for request-level auth.
- `SecurityContext` with subject + roles.
- `parse_bearer_token()` helper.

## Example

```rust
use ember_ext_auth::{parse_bearer_token, SecurityContext, SecurityRequest, SecurityFilter};

struct MyAuth;
impl SecurityFilter for MyAuth {
    fn filter(&self, request: &SecurityRequest) -> Result<SecurityContext, ember_core::EmberError> {
        let token = request.authorization.as_deref().and_then(parse_bearer_token);
        if token.is_some() {
            Ok(SecurityContext::new("user-123"))
        } else {
            Err(ember_core::EmberError::msg("unauthorized"))
        }
    }
}
```

## Diagram

```mermaid
flowchart LR
    Request --> Filter[SecurityFilter]
    Filter --> Context[SecurityContext]
```

## Status

Early preview; integration points will expand.
