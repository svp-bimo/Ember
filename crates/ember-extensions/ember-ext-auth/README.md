# ember-ext-auth

Authentication primitives for Ember. Provides interfaces for authentication and security context building.

## What it provides

- `Authenticator` trait for token validation.
- `SecurityFilter` for request-level auth.
- `SecurityContext` with subject + roles.
- `parse_bearer_token()` helper.
- `JwtAuthFilter` + `JwtIssuer` for JWT-based auth.

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

    ## JWT example

    ```rust
    use ember_ext_auth::{JwtAuthFilter, JwtConfig, JwtIssuer};

    let mut config = JwtConfig::new("dev-secret");
    config.allow_paths = vec!["/login".to_owned(), "/health".to_owned()];

    let filter = JwtAuthFilter::new(config.clone());
    let issuer = JwtIssuer::new(config);
    let token = issuer.issue_token("user-1", vec!["admin".to_owned()])?;
    ```

## Diagram

```mermaid
flowchart LR
    Request --> Filter[SecurityFilter]
    Filter --> Context[SecurityContext]
```

## Status

Early preview; integration points will expand.
