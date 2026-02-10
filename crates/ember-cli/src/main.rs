#![forbid(unsafe_code)]

mod cli;
mod commands;
mod scaffold;

use anyhow::Result;

fn main() -> Result<()> {
    cli::run()
}
