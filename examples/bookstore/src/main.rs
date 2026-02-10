#![forbid(unsafe_code)]

mod config;
mod controllers;
mod domain;
mod mappers;
mod repository;
mod services;

use anyhow::Result;
use controllers::book_controller::BookController;
use services::book_service::BookService;

#[tokio::main]
async fn main() -> Result<()> {
    let options = ember_core::RunOptions::new(std::path::Path::new(env!("CARGO_MANIFEST_DIR")));
    ember_core::run_with_db_and_controller::<config::AppConfig, _, _>(options, |_config| {
        let service = BookService::new();
        BookController::new(service)
    })
    .await
    .map_err(anyhow::Error::new)?;
    Ok(())
}
