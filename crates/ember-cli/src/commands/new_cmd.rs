#![forbid(unsafe_code)]

use anyhow::Result;
use std::path::Path;

use crate::scaffold::create_project;

pub fn run(name: &str, path: Option<&Path>, entities: &[String]) -> Result<()> {
    create_project(name, path, entities)
}
