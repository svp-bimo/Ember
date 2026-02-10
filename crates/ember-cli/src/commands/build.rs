#![forbid(unsafe_code)]

use anyhow::Result;

use crate::commands::common::run_command;

pub fn run() -> Result<()> {
    run_command("cargo", &["build", "--release"])
}
