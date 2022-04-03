use sha1::{Digest};
use std::{
  env,
  error::Error, 
  fs::File,
  io::{BufRead,BufReader}
};

const LENGTH: usize = 40;

fn check_hash(hash: &str) -> Result<&str, Box<dyn Error>>  {
  let hash_to_check = hash.trim();

  if hash_to_check.len() != LENGTH {
    return Err("sha1 hash not valid".into());
  }

  Ok(hash_to_check)
}

fn reader(path: &str) -> Result<BufReader<File>, Box<dyn Error>> {
  match File::open(&path) {
    Ok(words) => Ok(BufReader::new(words)),
    Err(err) => Err(err.into())
  }
  
  // same as above
  // let words = File::open(&path);
  // let reader = BufReader::new(words);
  // return Ok(reader)
}

fn main() -> Result<(), Box<dyn Error>> {
  // put cli args to a collection
  let args: Vec<String> = env::args().collect();

  if args.len() != 3 {
    println!("Usage:");
    println!("sha1_tester: <file_with_words.txt> <sha1_hash>");
    return Ok(());
  }

  let hash_to_check = check_hash(&args[2])?;
  // let words = File::open(&args[1])?;
  // let reader = BufReader::new(&words);
  let reader = reader(&args[1])?;

  for line in reader.lines() {
    let line = line?;
    let common_password = line.trim();

    if hash_to_check == &hex::encode(sha1::Sha1::digest(common_password.as_bytes())) {
      println!("Password found: {}", &common_password);
      return Ok(());
    }
  }

  println!("Password not found");

  // Ok() is an expression and expressions evaluate to a value
  // Their opposites/statements are instructions that end with a ;
  Ok(())
}
