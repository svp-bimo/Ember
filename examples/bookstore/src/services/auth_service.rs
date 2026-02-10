#![forbid(unsafe_code)]

//! Auth service for the bookstore example.

use ember_ext_auth::{JwtConfig, JwtIssuer};

#[derive(Debug, Clone)]
struct User {
    username: String,
    password: String,
    roles: Vec<String>,
}

/// Simple auth service with in-memory users.
#[derive(Debug, Clone)]
pub struct AuthService {
    issuer: JwtIssuer,
    users: Vec<User>,
}

impl AuthService {
    pub fn new(config: JwtConfig) -> Self {
        Self {
            issuer: JwtIssuer::new(config),
            users: vec![User {
                username: "admin".to_owned(),
                password: "admin".to_owned(),
                roles: vec!["admin".to_owned()],
            }],
        }
    }

    pub fn login(&self, username: &str, password: &str) -> Option<String> {
        let user = self
            .users
            .iter()
            .find(|user| user.username == username && user.password == password)?;
        self.issuer.issue_token(&user.username, user.roles.clone()).ok()
    }
}
