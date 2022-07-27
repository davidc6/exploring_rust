use assert_cmd::Command;
use predicates::prelude::*;

// method calls on std::error::Error are (dyn) dynamically dispatched
// and the error will live inside a poiner and its' memory is allocated 
// on the heap (Box).
// () - unit type is used here as no other meaningful type could be returned
type TestOutcome = Result<(), Box<dyn std::error::Error>>;

fn init(args: &[&str], expected_file: &str) -> TestOutcome {
    let expected = std::fs::read_to_string(expected_file)?;
    Command::cargo_bin("cliapp")?
        .args(args)
        .assert()
        .success()
        .stdout(expected);
    Ok(())
}

#[test]
fn no_args_bails_out() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp")?;
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("USAGE"));
    Ok(())
}

#[test]
fn runs_and_exist_successfully() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp")?;
    cmd.arg("hello").assert().success();
    Ok(())
}

#[test]
fn hello_arg_format1() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp")?;
    let expected = String::from("Hello world\n");
    cmd.arg("Hello world").assert().success().stdout(expected);
    Ok(())
}

#[test]
fn hello_arg_format2() -> TestOutcome {
    // let expected = String::from("Hello there\n"); // same as (two spaces between args) -> echo "Hello"  "there"
    // let expected = std::fs::read_to_string("tests/expectations/data2.txt")?;
    // let mut cmd = Command::cargo_bin("cliapp")?;
    // cmd.args(vec!["Hello", "World"]).assert().success().stdout(expected);
    // Ok(())

    init(&["Hello", "World"], "tests/expectations/data2.txt")
}

#[test]
fn hello_arg_format3() -> TestOutcome {
    let mut cmd = Command::cargo_bin("cliapp")?;
    let expected = String::from("Hello  world");
    cmd.args(vec!["Hello  world", "-n"]).assert().success().stdout(expected);
    Ok(())
}