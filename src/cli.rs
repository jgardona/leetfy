use std::io::Read;

use anyhow::{Ok, Result};
use clap::Parser;

use crate::alphabet::{encodestr, Frequency};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// The dictionary type
    #[arg(value_enum)]
    mode: Frequency,
    /// Read from a file
    #[arg(name = "filename", short, long)]
    filename: Option<String>,
    /// Read from stdin
    #[arg(short, long, conflicts_with = "filename")]
    stdin: bool,
}

fn read_data_from<R: Read>(reader: &mut R) -> Result<String> {
    let mut data = String::new();
    reader.read_to_string(&mut data)?;
    Ok(data)
}

pub fn execute_cli() -> Result<()> {
    let cli = Cli::parse();

    let data = if let Some(filename) = cli.filename {
        let mut file = std::fs::File::open(filename)?;
        read_data_from(&mut file)?
    } else {
        let mut stdin = std::io::stdin().lock();
        read_data_from(&mut stdin)?
    };

    let result = match cli.mode {
        Frequency::Low => encodestr(Frequency::Low, data.as_str()),
        Frequency::Full => encodestr(Frequency::Full, data.as_str()),
    };
    print!("{result}");

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::io::{Cursor, Write};

    use super::read_data_from;
    use anyhow::Context;
    use assert_cmd::Command;

    #[test]
    fn it_works() {}

    #[test]
    fn test_read_from_stdin() {
        let mut cursor = Cursor::new(Vec::<u8>::new());
        cursor.write_all(b"one\ntwo\nthree").unwrap();
        cursor.set_position(0);
        let result = read_data_from(&mut cursor).unwrap();
        let expected = "one\ntwo\nthree";
        assert_eq!(expected, result);
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
