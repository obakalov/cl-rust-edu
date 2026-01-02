use assert_cmd::cargo;
use predicates::prelude::*;

#[test]
fn dies_no_args() {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));
}

#[test]
fn echoes_text() {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd.arg("hello")
        .assert()
        .success()
        .stdout("hello\n");
}

#[test]
fn echoes_text_without_newline() {
    let mut cmd = cargo::cargo_bin_cmd!("echor");
    cmd
        .arg("-n")
        .arg("hello")
        .assert()
        .success()
        .stdout("hello");
}