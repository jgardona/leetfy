use anyhow::{Context, Ok, Result};
use cli::execute_cli;

mod alphabet;
mod cli;

fn main() -> Result<()> {
    execute_cli().context("the application didn't ran properly")?;
    Ok(())
}
