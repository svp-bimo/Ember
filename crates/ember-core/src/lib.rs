#![forbid(unsafe_code)]

//! Core types and minimal runtime API for Ember.

use std::net::ToSocketAddrs;
use std::path::{Path, PathBuf};

use ember_ext_config::load_config_yaml_or_env;
use ember_ext_db::{DbContext, HasDbConfig};
use ember_logging::log_startup;
use serde::de::DeserializeOwned;
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tracing::error;

/// Facade re-exports for Ember extensions.
pub use ember_ext_exceptions::{EmberError, ProblemDetails};
pub use ember_ext_http::{Json, Route, Router};
pub use ember_ext_runtime::App;

/// Commonly used Ember types.
pub mod prelude {
    pub use crate::{
        run_with_db_and_controller, App, EmberError, HasEmberService, HttpHandler, HttpResponse,
        Json, ProblemDetails, Route, Router, RunOptions,
    };
}

/// Trait for types that expose Ember service metadata.
pub trait HasEmberService {
    /// Service name for logging.
    fn service_name(&self) -> Option<&str>;
    /// Listen address for logging.
    fn listen_addr(&self) -> Option<&str>;
}

/// Minimal HTTP response returned by Ember handlers.
#[derive(Debug, Clone)]
pub struct HttpResponse {
    /// HTTP status code.
    pub status: u16,
    /// Content type header value.
    pub content_type: &'static str,
    /// Response body bytes.
    pub body: Vec<u8>,
}

impl HttpResponse {
    /// Create an empty response with the given status code.
    pub fn empty(status: u16) -> Self {
        Self {
            status,
            content_type: "text/plain",
            body: Vec::new(),
        }
    }

    /// Create a plain text response.
    pub fn text(status: u16, body: impl Into<String>) -> Self {
        Self {
            status,
            content_type: "text/plain",
            body: body.into().into_bytes(),
        }
    }
}

/// Trait for controllers that can handle HTTP requests.
pub trait HttpHandler {
    /// Handle an HTTP request and return a response.
    fn handle(&self, method: &str, path: &str, body: &[u8]) -> Result<HttpResponse, EmberError>;
}

/// Options for running an Ember application.
#[derive(Debug, Clone)]
pub struct RunOptions<'a> {
    /// Base directory for locating `application*.yaml`.
    pub base_dir: &'a Path,
    /// Environment variable that selects the profile.
    pub profile_env: &'a str,
    /// Environment variable with JSON configuration fallback.
    pub config_env: &'a str,
    /// Base name for YAML files (without extension).
    pub yaml_base_name: &'a str,
    /// Environment variable with the service name for logging.
    pub service_env: &'a str,
    /// Environment variable with the listen address for logging.
    pub listen_env: &'a str,
}

impl<'a> RunOptions<'a> {
    /// Create run options rooted at the provided base directory.
    pub fn new(base_dir: &'a Path) -> Self {
        Self {
            base_dir,
            profile_env: "EMBER_PROFILE",
            config_env: "EMBER_CONFIG_JSON",
            yaml_base_name: "application",
            service_env: "EMBER_SERVICE",
            listen_env: "EMBER_LISTEN",
        }
    }

    fn resolve_yaml_path(&self) -> PathBuf {
        let profile = std::env::var(self.profile_env)
            .ok()
            .filter(|value| !value.trim().is_empty());
        let mut path = match profile.as_deref() {
            Some(value) => self
                .base_dir
                .join(format!("{}-{}.yaml", self.yaml_base_name, value)),
            None => self.base_dir.join(format!("{}.yaml", self.yaml_base_name)),
        };
        if profile.is_some() && !path.exists() {
            path = self.base_dir.join(format!("{}.yaml", self.yaml_base_name));
        }
        path
    }
}

/// Run an Ember application that loads config and migrates entities before starting.
pub async fn run_with_db_and_controller<TConfig, TController, F>(
    options: RunOptions<'_>,
    build_controller: F,
) -> Result<(), EmberError>
where
    TConfig: DeserializeOwned + HasDbConfig + HasEmberService,
    TController: ember_ext_runtime::ControllerMetadata + HttpHandler + Clone + Send + Sync + 'static,
    F: FnOnce(TConfig) -> TController,
{
    let _ = dotenvy::dotenv();
    if let Err(err) = ember_logging::init() {
        return Err(EmberError::msg(format!("failed to initialize logging: {err}")));
    }
    let yaml_path = options.resolve_yaml_path();
    let config = load_config_yaml_or_env::<TConfig>(
        yaml_path.to_string_lossy().as_ref(),
        options.config_env,
    )?;
    let service_name = config
        .service_name()
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .or_else(|| {
            std::env::var(options.service_env)
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .unwrap_or_else(|| "ember-service".to_string());
    let listen = config
        .listen_addr()
        .filter(|value| !value.trim().is_empty())
        .map(str::to_string)
        .or_else(|| {
            std::env::var(options.listen_env)
                .ok()
                .filter(|value| !value.trim().is_empty())
        })
        .unwrap_or_else(|| "0.0.0.0:8080".to_string());
    log_startup(&service_name, &listen);
    let db = DbContext::new(config.db_config().clone());
    let _pool = db.connect_and_migrate_entities().await?;

    let controller = build_controller(config);
    let mut app = App::new();
    app.register_controller(controller.clone());
    app.run()?;
    run_basic_http(&listen, controller).await
}

async fn run_basic_http<T>(listen: &str, handler: T) -> Result<(), EmberError>
where
    T: HttpHandler + Send + Sync + 'static,
{
    let mut addrs = listen
        .to_socket_addrs()
        .map_err(|err| EmberError::msg(format!("invalid listen address: {err}")))?;
    let addr = addrs
        .next()
        .ok_or_else(|| EmberError::msg("listen address resolved to no sockets"))?;
    let listener = TcpListener::bind(addr).await.map_err(|err| {
        error!(error = %err, listen = %listen, "failed to bind listen address");
        EmberError::msg(format!("failed to bind listen address: {err}"))
    })?;
    let handler = Arc::new(handler);
    loop {
        let (mut socket, _) = listener
            .accept()
            .await
            .map_err(|err| EmberError::msg(format!("accept failed: {err}")))?;
        let handler = Arc::clone(&handler);
        tokio::spawn(async move {
            let response = match read_http_request(&mut socket).await {
                Ok((method, path, body)) => handler
                    .handle(&method, &path, &body)
                    .unwrap_or_else(|err| HttpResponse::text(500, err.to_string())),
                Err(err) => HttpResponse::text(400, err.to_string()),
            };
            let _ = write_http_response(&mut socket, response).await;
        });
    }
}

async fn read_http_request(
    socket: &mut tokio::net::TcpStream,
) -> Result<(String, String, Vec<u8>), EmberError> {
    let mut buffer = Vec::new();
    let mut temp = [0u8; 1024];
    let header_end;
    loop {
        let read = socket
            .read(&mut temp)
            .await
            .map_err(|err| EmberError::msg(format!("read failed: {err}")))?;
        if read == 0 {
            return Err(EmberError::msg("connection closed"));
        }
        buffer.extend_from_slice(&temp[..read]);
        if let Some(pos) = find_header_end(&buffer) {
            header_end = pos;
            break;
        }
        if buffer.len() > 64 * 1024 {
            return Err(EmberError::msg("request headers too large"));
        }
    }

    let header_bytes = &buffer[..header_end];
    let header_str = String::from_utf8_lossy(header_bytes);
    let mut lines = header_str.lines();
    let request_line = lines
        .next()
        .ok_or_else(|| EmberError::msg("missing request line"))?;
    let mut parts = request_line.split_whitespace();
    let method = parts
        .next()
        .ok_or_else(|| EmberError::msg("missing method"))?
        .to_string();
    let path = parts
        .next()
        .ok_or_else(|| EmberError::msg("missing path"))?
        .to_string();

    let mut content_length = 0usize;
    for line in lines {
        if let Some(value) = line.strip_prefix("Content-Length:") {
            content_length = value.trim().parse::<usize>().unwrap_or(0);
        }
    }

    let mut body = buffer[header_end + 4..].to_vec();
    while body.len() < content_length {
        let read = socket
            .read(&mut temp)
            .await
            .map_err(|err| EmberError::msg(format!("read failed: {err}")))?;
        if read == 0 {
            break;
        }
        body.extend_from_slice(&temp[..read]);
    }
    Ok((method, path, body))
}

fn find_header_end(buffer: &[u8]) -> Option<usize> {
    buffer
        .windows(4)
        .position(|window| window == b"\r\n\r\n")
}

async fn write_http_response(
    socket: &mut tokio::net::TcpStream,
    response: HttpResponse,
) -> Result<(), EmberError> {
    let status_text = match response.status {
        200 => "OK",
        201 => "Created",
        204 => "No Content",
        400 => "Bad Request",
        404 => "Not Found",
        500 => "Internal Server Error",
        _ => "OK",
    };
    let header = format!(
        "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n",
        response.status,
        status_text,
        response.body.len(),
        response.content_type
    );
    socket
        .write_all(header.as_bytes())
        .await
        .map_err(|err| EmberError::msg(format!("write failed: {err}")))?;
    if !response.body.is_empty() {
        socket
            .write_all(&response.body)
            .await
            .map_err(|err| EmberError::msg(format!("write failed: {err}")))?;
    }
    Ok(())
}
