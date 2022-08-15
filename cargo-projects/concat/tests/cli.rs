use std::fs;
use assert_cmd::Command;
use rand::Rng;
use rand::distributions::Alphanumeric;

type TestResult = Result<(), Box<dyn std::error::Error>>;

const PROGRAM: &str = "concat";

fn bad_file_gen() -> String {
    loop {
        let filename: String = rand::thread_rng()
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
fn ignore_bad_file() -> TestResult {
    let bad_file = bad_file_gen();
    Command::cargo_bin(PROGRAM)?
        .arg(&bad_file)
        .assert()
        .success()
        .stderr(predicates::str::is_match("os error 2")?);
    Ok(())
}