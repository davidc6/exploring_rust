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

  println!("{}", file_contents);

  Ok(())
}
