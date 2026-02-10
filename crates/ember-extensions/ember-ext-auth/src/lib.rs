#![forbid(unsafe_code)]

//! Authentication extension for Ember.

use ember_core::EmberError;
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Authenticate incoming requests.
pub trait Authenticator {
    /// Validate a token and return success or an error.
    fn authenticate(&self, token: &str) -> Result<(), EmberError>;
}

/// Security context produced after successful authentication.
#[derive(Debug, Clone)]
pub struct SecurityContext {
    /// Subject identifier (user or service).
    pub subject: String,
    /// Optional roles for authorization.
    pub roles: Vec<String>,
}

impl SecurityContext {
    /// Create a new security context.
    pub fn new(subject: impl Into<String>) -> Self {
        Self {
            subject: subject.into(),
            roles: Vec::new(),
        }
    }

    /// Attach roles to the context.
    pub fn with_roles(mut self, roles: Vec<String>) -> Self {
        self.roles = roles;
        self
    }
}

/// A minimal request view used by security filters.
#[derive(Debug, Clone)]
pub struct SecurityRequest {
    /// Request path.
    pub path: String,
    /// Request method.
    pub method: String,
    /// Authorization header value, if present.
    pub authorization: Option<String>,
}

/// Security filter hook that can validate tokens and build a security context.
pub trait SecurityFilter {
    /// Validate the request and return a security context.
    fn filter(&self, request: &SecurityRequest) -> Result<SecurityContext, EmberError>;
}

/// Validate an authorization header for a Bearer token.
pub fn parse_bearer_token(header_value: &str) -> Option<String> {
    let trimmed = header_value.trim();
    let lower = trimmed.to_ascii_lowercase();
    let prefix = "bearer ";
    if lower.starts_with(prefix) {
        let token = trimmed[prefix.len()..].trim();
        if token.is_empty() {
            None
        } else {
            Some(token.to_owned())
        }
    } else {
        None
    }
}

/// JWT claims used by Ember auth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JwtClaims {
    /// Subject (user id).
    pub sub: String,
    /// Expiration timestamp (seconds since epoch).
    pub exp: usize,
    /// Optional roles.
    pub roles: Vec<String>,
    /// Optional issuer.
    pub iss: Option<String>,
    /// Optional audience.
    pub aud: Option<String>,
}

/// Configuration for JWT auth.
#[derive(Debug, Clone)]
pub struct JwtConfig {
    /// Secret for signing/verification.
    pub secret: String,
    /// Optional issuer.
    pub issuer: Option<String>,
    /// Optional audience.
    pub audience: Option<String>,
    /// Token expiry seconds.
    pub expires_in_seconds: u64,
    /// Public paths that do not require auth.
    pub allow_paths: Vec<String>,
}

impl JwtConfig {
    /// Create a new JWT config with defaults.
    pub fn new(secret: impl Into<String>) -> Self {
        Self {
            secret: secret.into(),
            issuer: None,
            audience: None,
            expires_in_seconds: 3600,
            allow_paths: Vec::new(),
        }
    }
}

/// JWT issuer helper.
#[derive(Debug, Clone)]
pub struct JwtIssuer {
    config: JwtConfig,
}

impl JwtIssuer {
    /// Create a new issuer from config.
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    /// Issue a JWT for the given subject and roles.
    pub fn issue_token(&self, subject: impl Into<String>, roles: Vec<String>) -> Result<String, EmberError> {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|err| EmberError::msg(format!("time error: {err}")))?
            .as_secs();
        let exp = now.saturating_add(self.config.expires_in_seconds) as usize;
        let claims = JwtClaims {
            sub: subject.into(),
            exp,
            roles,
            iss: self.config.issuer.clone(),
            aud: self.config.audience.clone(),
        };

        jsonwebtoken::encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.config.secret.as_bytes()),
        )
        .map_err(|err| EmberError::msg(format!("token encode failed: {err}")))
    }
}

/// JWT validator + security filter.
#[derive(Debug, Clone)]
pub struct JwtAuthFilter {
    config: JwtConfig,
}

impl JwtAuthFilter {
    /// Create a new auth filter from config.
    pub fn new(config: JwtConfig) -> Self {
        Self { config }
    }

    fn is_public_path(&self, path: &str) -> bool {
        self.config.allow_paths.iter().any(|p| path.starts_with(p))
    }

    fn validate_token(&self, token: &str) -> Result<SecurityContext, EmberError> {
        let mut validation = Validation::default();
        if let Some(issuer) = &self.config.issuer {
            validation.set_issuer(&[issuer.as_str()]);
        }
        if let Some(audience) = &self.config.audience {
            validation.set_audience(&[audience.as_str()]);
        }

        let decoded = jsonwebtoken::decode::<JwtClaims>(
            token,
            &DecodingKey::from_secret(self.config.secret.as_bytes()),
            &validation,
        )
        .map_err(|err| EmberError::msg(format!("token decode failed: {err}")))?;

        Ok(SecurityContext::new(decoded.claims.sub).with_roles(decoded.claims.roles))
    }
}

impl SecurityFilter for JwtAuthFilter {
    fn filter(&self, request: &SecurityRequest) -> Result<SecurityContext, EmberError> {
        if self.is_public_path(&request.path) {
            return Ok(SecurityContext::new("anonymous"));
        }
        let token = request
            .authorization
            .as_deref()
            .and_then(parse_bearer_token)
            .ok_or_else(|| EmberError::msg("missing bearer token"))?;
        self.validate_token(&token)
    }
}
