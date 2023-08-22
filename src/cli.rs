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
    use super::read_full_file;
    use anyhow::{Context, Ok, Result};
    use assert_cmd::Command;

    #[test]
    fn it_works() {}

    #[test]
    fn test_read_full_file() -> Result<()> {
        let text = "the quick brown fox jumps over the lazy dog";
        let data = read_full_file("tests/text.txt")?;
        assert_eq!(text, data);
        Ok(())
    }

    #[test]
    fn test_cmd_leetfy_low_frequency() {
        let mut cmd = Command::cargo_bin("leetfy")
            .context("we need assert that the binary leetfy is available")
            .unwrap();
        let test = cmd
            .arg("--filename")
            .arg("tests/text.txt")
            .arg("low")
            .assert();
        test.success().stdout(predicates::str::contains(
            "th3 qu1ck 8r0wn f0x jump5 0v3r th3 l4zy d0g",
        ));
    }

    #[test]
    fn test_cmd_leetfy_full_frequency() {
        let mut cmd = Command::cargo_bin("leetfy")
            .context("we need assert that the binary leetfy is available")
            .unwrap();
        let test = cmd
            .arg("--filename")
            .arg("tests/text.txt")
            .arg("full")
            .assert();
        test.success().stdout(predicates::str::contains(
            "+#3 9v1[x 820w~ f0* ]vm?5 0v32 +#3 142j )06",
        ));
    }
}
