#![forbid(unsafe_code)]

//! Logging utilities for Ember services.

use tracing_subscriber::{fmt, EnvFilter};

/// Errors that can occur while initializing logging.
#[derive(Debug, thiserror::Error)]
pub enum EmberLoggingError {
    /// Failed to install the global tracing subscriber.
    #[error("failed to initialize logging: {0}")]
    InitFailed(#[from] Box<dyn std::error::Error + Send + Sync>),
}

/// Initialize Ember logging with sensible defaults.
///
/// The log level can be controlled with the `RUST_LOG` environment variable.
pub fn init() -> Result<(), EmberLoggingError> {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info,sqlx=warn"));
    fmt()
        .with_env_filter(env_filter)
        .with_target(false)
        .with_level(true)
        .with_thread_ids(false)
        .with_thread_names(false)
        .try_init()?;
    Ok(())
}

/// Emit a standard startup log line.
pub fn log_startup(service_name: &str, address: &str) {
    tracing::info!("{}", STARTUP_BANNER);
    tracing::info!(service = service_name, listen = address, "service starting");
}

const STARTUP_BANNER: &str = r#"
███████╗███╗   ███╗██████╗ ███████╗██████╗          ██╗  
██╔════╝████╗ ████║██╔══██╗██╔════╝██╔══██╗         ╚██╗ 
█████╗  ██╔████╔██║██████╔╝█████╗  ██████╔╝    █████╗╚██╗
██╔══╝  ██║╚██╔╝██║██╔══██╗██╔══╝  ██╔══██╗    ╚════╝██╔╝
███████╗██║ ╚═╝ ██║██████╔╝███████╗██║  ██║         ██╔╝ 
╚══════╝╚═╝     ╚═╝╚═════╝ ╚══════╝╚═╝  ╚═╝         ╚═╝  
                                                            "#;
