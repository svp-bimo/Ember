#![forbid(unsafe_code)]

//! Configuration extension for Ember.

use std::{env, fs};

use ember_ext_exceptions::EmberError;
use serde::de::DeserializeOwned;

/// Default environment variable for JSON configuration.
pub const DEFAULT_CONFIG_ENV: &str = "EMBER_CONFIG_JSON";

/// Load configuration for the given type from the default environment variable.
pub fn load_config<T>() -> Result<T, EmberError>
where
    T: DeserializeOwned,
{
    load_config_from_env(DEFAULT_CONFIG_ENV)
}

/// Load configuration for the given type from a JSON string.
pub fn load_config_json<T>(json: &str) -> Result<T, EmberError>
where
    T: DeserializeOwned,
{
    serde_json::from_str(json)
        .map_err(|err| EmberError::msg(format!("failed to parse config JSON: {err}")))
}

/// Load configuration for the given type from an environment variable.
pub fn load_config_from_env<T>(var_name: &str) -> Result<T, EmberError>
where
    T: DeserializeOwned,
{
    let value = env::var(var_name)
        .map_err(|_| EmberError::msg(format!("missing env var: {var_name}")))?;
    load_config_json(&value)
}

/// Load configuration for the given type from a YAML file path.
pub fn load_config_from_yaml_file<T>(path: &str) -> Result<T, EmberError>
where
    T: DeserializeOwned,
{
    let contents = fs::read_to_string(path)
        .map_err(|err| EmberError::msg(format!("failed to read config file: {err}")))?;
    serde_yaml::from_str(&contents)
        .map_err(|err| EmberError::msg(format!("failed to parse config YAML: {err}")))
}

/// Load configuration from YAML file if present, otherwise fall back to env.
pub fn load_config_yaml_or_env<T>(path: &str, var_name: &str) -> Result<T, EmberError>
where
    T: DeserializeOwned,
{
    if std::path::Path::new(path).exists() {
        load_config_from_yaml_file(path)
    } else {
        load_config_from_env(var_name)
    }
}
