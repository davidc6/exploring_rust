use std::env;
use std::process;

use litegrep::Configuration;

fn main() {
    let args: Vec<String> = env::args().collect();
    // let config = parse_args(&args);
    let config = Configuration::new(&args).unwrap_or_else(|err| {
      println!("Problem with arguments: {}", err);
      process::exit(1);
    });
    
    println!()
    
    // Result<String> gets returned
    // expect() unwraps and allow to provide custom panic message
    if let Err(e) = litegrep::execute(config) {
      println!("Error: {}", e);
      process::exit(1);
    }
}
