use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn runs_single_arg() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("greatest-common-divisor")?;
  
  cmd
    .args(&["90"]);
  cmd
    .assert()
    .success()
    .stdout("90\n");
    
  Ok(())
}

#[test]
fn runs_two_args() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("greatest-common-divisor")?;
  
  cmd
    .args(&["10","90"]);
  cmd
    .assert()
    .success()
    .stdout("10\n");
    
  Ok(())
}

#[test]
fn errors_no_args() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("greatest-common-divisor")?;

  cmd
    .assert()
    .failure()
    .stderr(predicate::str::contains("No arguments passed in"));
    
  Ok(())
}
