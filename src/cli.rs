use std::fs;

use anyhow::{Context, Ok, Result};
use clap::Parser;

use crate::alphabet::{encondestr, Frequency};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    #[arg(value_enum)]
    mode: Frequency,
    #[arg(short, long)]
    filename: String,
}

fn read_full_file(path: &str) -> Result<String> {
    let data = fs::read_to_string(path).context("must be a valid path to file")?;
    Ok(data)
}

pub fn execute_cli() -> Result<()> {
    let cli = Cli::parse();

    let data = read_full_file(&cli.filename)?;

    let result = match cli.mode {
        Frequency::Low => encondestr(Frequency::Low, data.as_str()),
        Frequency::Full => encondestr(Frequency::Full, data.as_str()),
    };
    print!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use anyhow::{Ok, Result};

    use super::read_full_file;

    #[test]
    fn it_works() {}

    #[test]
    fn test_read_full_file() -> Result<()> {
        let text = "the quick brown fox jumps over the lazy dog";
        let data = read_full_file("tests/text.txt")?;
        assert_eq!(text, data);
        Ok(())
    }
}
