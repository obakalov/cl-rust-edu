use anyhow::Result;
use assert_cmd::cargo;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use rand::Rng;
use rand::distr::Alphanumeric;
use std::fs;

const PRG: &str = "catr";
const EMPTY: &str = "tests/input/empty.txt";
const FOX: &str = "tests/input/fox.txt";
const SPIDERS: &str = "tests/input/spiders.txt";
const BUSTLE: &str = "tests/input/bustle.txt";

fn get_bad_file() -> String {
    loop {
        let filename: String = rand::rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        if fs::metadata(&filename).is_err() {
            return filename;
        }
    }
}

#[test]
fn skips_bad_file() -> Result<()> {
    let bad_file: String = get_bad_file();
    let expected: String = format!("{bad_file}: .* [(]os error 2[)]");
    cargo::cargo_bin_cmd!("catr")
        .arg(&bad_file)
        .assert()
        .success()
        .stderr(predicate::str::is_match(&expected)?);
    Ok(())
}

fn run(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = cargo::cargo_bin_cmd!("catr").args(args).output()?;
    assert!(output.status.success());

    let stdout = String::from_utf8(output.stdout)?;
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn bustle() -> Result<()> {
    run(&[BUSTLE], "tests/expected/bustle.txt")
}
