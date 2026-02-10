#![forbid(unsafe_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

use crate::commands;

/// Ember CLI entrypoint.
#[derive(Parser)]
#[command(name = "ember", version, about = "Ember CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new Ember project skeleton.
    New {
        /// Name of the project to create.
        name: String,
        /// Optional output directory (defaults to ./<name>).
        #[arg(long)]
        path: Option<PathBuf>,
        /// Entity definition(s) (e.g. --entity Book:id:i64,title:String,author:String).
        #[arg(long = "entity")]
        entity: Vec<String>,
    },
    /// Run the dev server with hot reload.
    Dev,
    /// Build the application and package artifacts.
    Build,
    /// Generate OpenAPI documentation.
    Openapi,
}

pub fn run() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name, path, entity } => {
            commands::new_cmd::run(&name, path.as_deref(), &entity)
        }
        Commands::Dev => commands::dev::run(),
        Commands::Build => commands::build::run(),
        Commands::Openapi => commands::openapi::run(),
    }
}
