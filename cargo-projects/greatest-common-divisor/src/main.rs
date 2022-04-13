use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  
  if args.len() == 1 {
    println!("Usage: gcd 10 90");
    return;
  }
  
  let first = &args[1].to_string();
  let first_number: u64 = first.parse().unwrap_or(0);
  
  let second = &args[2].to_string();
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

#[test]
fn test_gcd() {
  assert_eq!(greatest_common_divisor(10, 90), 10);
  
  assert_eq!(greatest_common_divisor(1, 2), 1);
}
