#![forbid(unsafe_code)]

mod config;
mod controllers;
mod services;

use anyhow::Result;
#[cfg(not(feature = "standalone"))]
use controllers::health_controller::HealthController;
use services::system_info_service::SystemInfoService;
use ember_logging::{init as init_logging, log_startup};
#[cfg(feature = "standalone")]
use std::sync::Arc;

#[cfg(feature = "standalone")]
use controllers::health_controller::HealthResponse;

#[cfg(feature = "standalone")]
use axum::{
    extract::State,
    http::StatusCode,
    routing::get,
    Json as AxumJson, Router,
};
#[cfg(feature = "standalone")]
use ember_core::ProblemDetails;
#[cfg(feature = "standalone")]
use ember_ext_health::{add_health_routes, global_registry as health_registry, HealthCheck, HealthReport};
#[cfg(feature = "standalone")]
use ember_ext_metrics::{install_metrics, MetricsHandle, MetricSample};

#[cfg(feature = "standalone")]
#[derive(Clone)]
struct AppState {
    system_info: SystemInfoService,
    metrics: MetricsHandle,
}

#[cfg(feature = "standalone")]
struct SystemHealthCheck {
    system_info: SystemInfoService,
}

#[cfg(feature = "standalone")]
impl HealthCheck for SystemHealthCheck {
    fn check(&self) -> HealthReport {
        let status = self.system_info.status();
        HealthReport::healthy(format!("system:{status}"))
    }
}

#[cfg(feature = "standalone")]
#[tokio::main]
async fn main() -> Result<()> {
    run_standalone().await
}

#[cfg(not(feature = "standalone"))]
fn main() -> Result<()> {
    run_ember()
}

#[cfg(not(feature = "standalone"))]
fn run_ember() -> Result<()> {
    init_logging().map_err(anyhow::Error::new)?;
    let config = config::AppConfig::default();
    let _service_name = config.service_name.clone();

    let system_info = SystemInfoService::default();
    let controller = HealthController::new(system_info);
    let _ = controller.health();

    let mut app = ember_core::App::new();
    app.register_controller(controller);
    log_startup("ember-example-service", "ember-runtime");
    app.run().map_err(anyhow::Error::new)?;

    Ok(())
}

#[cfg(feature = "standalone")]
async fn run_standalone() -> Result<()> {
    init_logging().map_err(anyhow::Error::new)?;
    install_metrics(&mut ember_core::App::new());
    add_health_routes(&mut ember_core::App::new());

    let metrics_handle = MetricsHandle::global();
    let registry = health_registry();
    let system_info = SystemInfoService::default();
    registry.register_check(
        "system",
        Arc::new(SystemHealthCheck {
            system_info: system_info.clone(),
        }),
    );

    let state = AppState {
        system_info,
        metrics: metrics_handle,
    };

    let app = Router::new()
        .route("/health", get(health))
        .route("/metrics", get(metrics_endpoint))
        .route("/fail", get(fail))
        .with_state(state);

    let addr = "0.0.0.0:8080";
    log_startup("ember-example-service", addr);
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;
    Ok(())
}

#[cfg(feature = "standalone")]
async fn health(State(state): State<AppState>) -> AxumJson<HealthResponse> {
    let counter = state.metrics.counter("http.health");
    counter.inc();
    let response = HealthResponse {
        status: "ok".to_owned(),
        system: state.system_info.status(),
    };
    AxumJson(response)
}

#[cfg(feature = "standalone")]
async fn metrics_endpoint(State(state): State<AppState>) -> AxumJson<Vec<MetricSample>> {
    let counter = state.metrics.counter("http.metrics");
    counter.inc();
    AxumJson(state.metrics.snapshot())
}

#[cfg(feature = "standalone")]
async fn fail() -> (StatusCode, AxumJson<ProblemDetails>) {
    let metrics = MetricsHandle::global();
    let counter = metrics.counter("http.fail");
    counter.inc();
    let problem = ProblemDetails::internal_error("simulated failure");
    (StatusCode::INTERNAL_SERVER_ERROR, AxumJson(problem))
}
