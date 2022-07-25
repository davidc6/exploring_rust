use assert_cmd::Command;
use predicates::prelude::*;

type TestOutcome = Result<(), Box<dyn std::error::Error>>;

#[test]
fn no_args_bails_out() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp").unwrap();
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs_and_exist_successfully() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp").unwrap();
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello_arg_format1() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp").unwrap();
    let expected = String::from("Hello world\n");
    cmd.arg("Hello world").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello_arg_format2() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp").unwrap();
    // let expected = String::from("Hello there\n"); // same as (two spaces between args) -> echo "Hello"  "there"
    let expected = std::fs::read_to_string("tests/expectations/data2.txt").unwrap();
    cmd.args(vec!["Hello", "World"]).assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello_arg_format3() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp").unwrap();
    let expected = String::from("Hello  world");
    cmd.args(vec!["Hello  world", "-n"]).assert().success().stdout(expected);
    Ok(())
}