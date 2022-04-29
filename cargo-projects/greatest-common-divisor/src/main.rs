use std::env;
use std::str::FromStr;

fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
  assert!(n != 0 && m != 0);
  while m != 0 {
    if m < n { 
      let t = m;
      m = n;
      n = t;
    }
    m = m % n;
  }
  return n;
}

fn main() {
  // automatically gets freed when args go out of scope at the end of main
  let mut args = Vec::new();
  
  for arg in env::args().skip(1) {
    // u64 results Result and expect parses it
    args.push(u64::from_str(&arg)
              .expect("error parsing arg"));
  }
  
  if args.len() == 0 {
    eprintln!("No arguments passed in");
    std::process::exit(1);
  }
  
  let mut d = args[0];
  
  // vector ownership stays with args
  // & - borrow reference to vector elements staring with the second one
  // on each iteration we borrow reference to each element
  for m in &args[1..] {
    // * - dereference
    // *m - we yield the value of m that it refers to
    d = greatest_common_divisor(d, *m);
  }

  println!("{}", d);
}

#[cfg(test)]
mod tests {
  // import names from the outer scope
  use super::*;

  #[test]
  fn test_gcd_two_numbers() {
    assert_eq!(greatest_common_divisor(10, 90), 10);    
  }
  
  #[test]
  fn test_gcd_one_number() {
    assert_eq!(greatest_common_divisor(2, 1), 1);    
  }
}
