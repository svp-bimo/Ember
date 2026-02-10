#![forbid(unsafe_code)]

mod config;
mod controllers;
mod domain;
mod mappers;
mod repository;
mod services;

use anyhow::Result;
use controllers::book_controller::BookController;
use ember_ext_auth::{JwtAuthFilter, JwtConfig};
use services::book_service::BookService;
use services::auth_service::AuthService;

#[tokio::main]
async fn main() -> Result<()> {
    let options = ember_core::RunOptions::new(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
    ember_core::run_with_db_and_controller_and_auth::<config::AppConfig, _, _, _>(
        options,
        |config| {
            let mut jwt_config = JwtConfig::new(config.auth.jwt_secret.clone());
            jwt_config.issuer = config.auth.jwt_issuer.clone();
            jwt_config.audience = config.auth.jwt_audience.clone();
            jwt_config.expires_in_seconds = config.auth.jwt_expires_in_seconds;
            jwt_config.allow_paths = config.auth.public_paths.clone();

            let auth_service = AuthService::new(jwt_config.clone());
            let service = BookService::new();
            let controller = BookController::new(service, auth_service);
            let filter = JwtAuthFilter::new(jwt_config);
            (controller, filter)
        },
    )
    .await
    .map_err(anyhow::Error::new)?;
    Ok(())
}
