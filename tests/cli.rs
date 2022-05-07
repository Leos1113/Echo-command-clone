use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn dies_if_no_args() {
    let mut cmd = Command::cargo_bin("echo-command").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn run() {
    let mut cmd = Command::cargo_bin("echo-command").unwrap();
    cmd.arg("hello").assert().success();
}
