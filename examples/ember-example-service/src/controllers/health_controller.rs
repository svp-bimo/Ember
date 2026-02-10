#![forbid(unsafe_code)]

//! Health controller for the example service.

use crate::services::system_info_service::SystemInfoService;
use ember_core::Json;
use ember_macros::{controller, get};
use serde::Serialize;

/// Health check response payload.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Service status string.
    pub status: String,
    /// System status string.
    pub system: String,
}

/// Controller for health endpoints.
pub struct HealthController {
    system_info: SystemInfoService,
}

impl HealthController {
    /// Create a new health controller.
    pub fn new(system_info: SystemInfoService) -> Self {
        Self { system_info }
    }
}

#[controller]
impl HealthController {
    /// Return health status.
    #[get("/health")]
    pub fn health(&self) -> Json<HealthResponse> {
        Json(HealthResponse {
            status: "ok".to_owned(),
            system: self.system_info.status(),
        })
    }
}
