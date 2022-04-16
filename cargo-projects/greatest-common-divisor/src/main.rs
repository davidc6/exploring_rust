use std::env;

fn main() {
  let mut args: Vec<String> = env::args().collect();
  
  if args.len() == 1 {
    eprintln!("Usage: gcd <number> <number>");
    return;
  }
  
  args.remove(0);

  let first = &args[0].to_string();
  let first_number: u64 = first.parse().unwrap_or(0);
  
  let second = &args[1].to_string();
  let second_number: u64 = second.parse().unwrap_or(0);
  
  let number = greatest_common_divisor(first_number, second_number);
  
  println!("{}", number);
}

fn greatest_common_divisor(mut n: u64, mut m: u64) -> u64 {
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

#[cfg(test)]
mod tests {
  // import names from the outer scope
  use super::*;

  #[test]
  fn test_gcd_1() {
    assert_eq!(greatest_common_divisor(10, 90), 10);    
  }
  
  #[test]
  fn test_gcd_2() {
    assert_eq!(greatest_common_divisor(1, 2), 1);    
  }
}
