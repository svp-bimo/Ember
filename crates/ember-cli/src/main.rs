#![forbid(unsafe_code)]

use anyhow::Result;
use clap::{Parser, Subcommand};

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
    },
    /// Run the dev server with hot reload.
    Dev,
    /// Build the application and package artifacts.
    Build,
    /// Generate OpenAPI documentation.
    Openapi,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::New { name } => {
            println!("TODO: create new Ember project: {name}");
        }
        Commands::Dev => {
            println!("TODO: run Ember dev server");
        }
        Commands::Build => {
            println!("TODO: build Ember project");
        }
        Commands::Openapi => {
            println!("TODO: generate OpenAPI spec");
        }
    }
    Ok(())
}
