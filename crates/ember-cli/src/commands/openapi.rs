#![forbid(unsafe_code)]

use anyhow::{Context, Result};
use std::fs;

pub fn run() -> Result<()> {
    let doc = ember_ext_openapi::generate_openapi();
    let json = serde_json::to_string_pretty(&doc)?;
    fs::write("openapi.json", json).context("failed to write openapi.json")?;
    println!("OpenAPI document written to openapi.json");
    Ok(())
}
