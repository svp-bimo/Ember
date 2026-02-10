#![forbid(unsafe_code)]

//! System information service for the example service.

use ember_macros::service;

/// Provides system status information.
#[service]
#[derive(Clone, Debug, Default)]
pub struct SystemInfoService;

impl SystemInfoService {
    /// Return a simple status string.
    pub fn status(&self) -> String {
        "ok".to_owned()
    }
}
