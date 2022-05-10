use std::fs;
use std::error::Error;

pub struct Configuration {
  needle: String,
  haystack: String
}

impl Configuration {
  pub fn new(args: &[String]) -> Result<Configuration, &'static str> {
    if args.len() < 3 {
      return Err("Not enough arguments supplied");
    }
    
    // clone is not the most optimal solution but acceptable for now
    let needle = args[1].clone();
    let haystack = args[2].clone();
    
    Ok(Configuration { needle, haystack })
  }
}

// we return unit type as Ok case
// we return trait object Box<dyn Error> as Error case
pub fn execute(config: Configuration) -> Result<(), Box<dyn Error>> {
  // we propagate error here by using ? mark
  let file_contents = fs::read_to_string(config.haystack)?;

  for line in search(&config.needle, &file_contents) {
    println!("{}", line);
  }

  Ok(())
}

// we define lifetype explicitly here and it needs to be used in the return value
pub fn search<'a>(needle: &str, haystack: &'a str) -> Vec<&'a str> {
  let mut results = Vec::new();

  for line in haystack.lines() {
    if line.contains(needle) {
      results.push(line);
    }
  }
  
  results
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn it_works() {
    let needle = "hello";
    let haystack = "\n
hello from the\n
other side";
    
    assert_eq!(vec!["hello from the"], search(needle, haystack));
  }
}
