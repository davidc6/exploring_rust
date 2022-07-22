use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn no_args_bails_out() {
    let mut cmd = Command::cargo_bin("cliapp").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
}

#[test]
fn runs_and_exist_successfully() {
    let mut cmd = Command::cargo_bin("cliapp").unwrap();
    cmd.arg("hello").assert().success();
}