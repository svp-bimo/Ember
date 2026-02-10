#![forbid(unsafe_code)]

use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn run_command(bin: &str, args: &[&str]) -> Result<()> {
    let status = Command::new(bin)
        .args(args)
        .status()
        .with_context(|| format!("failed to run {bin}"))?;
    if !status.success() {
        bail!("command failed with status: {status}");
    }
    Ok(())
}
