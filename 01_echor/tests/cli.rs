use anyhow::Result;
use assert_cmd::cargo;
use predicates::prelude::*;
use pretty_assertions::assert_eq;
use std::fs;

#[test]
fn dies_no_args() -> Result<()> {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
    Ok(())
}

#[test]
fn echoes_text() {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.arg("hello").assert().success().stdout("hello\n");
}

#[test]
fn echoes_text_without_newline() {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.arg("-n")
        .arg("hello")
        .assert()
        .success()
        .stdout("hello");
}

fn run_from_file(args: &[&str], expected_file: &str) -> Result<()> {
    let expected = fs::read_to_string(expected_file)?;
    let output = cargo::cargo_bin_cmd!("echor")
        .args(args)
        .output()
        .expect("failed to execute a process");

    let stdout = String::from_utf8(output.stdout).expect("invalid UTF-8");
    assert_eq!(stdout, expected);
    Ok(())
}

#[test]
fn hello1() -> Result<()> {
    run_from_file(&["Hello there"], "tests/expected/hello1.txt")
}

#[test]
fn hello2() -> Result<()> {
    run_from_file(&["Hello", "there"], "tests/expected/hello2.txt")
}

#[test]
fn hello1_no_newline() -> Result<()> {
    run_from_file(&["-n", "Hello   there"], "tests/expected/hello1.n.txt")
}

#[test]
fn hello2_no_newline() -> Result<()> {
    run_from_file(&["-n", "Hello", "there"], "tests/expected/hello2.n.txt")
}
