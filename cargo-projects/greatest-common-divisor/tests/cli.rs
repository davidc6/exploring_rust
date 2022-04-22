use assert_cmd::Command;

#[test]
fn it_works() -> Result<(), Box<dyn std::error::Error>> {
  let mut cmd = Command::cargo_bin("greatest-common-divisor")?;
  
  cmd
    .args(&["10","90"]);
  cmd
    .assert()
    .success()
    .stdout("10\n");
    
  Ok(())
}
