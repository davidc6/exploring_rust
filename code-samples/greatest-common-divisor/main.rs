use std::env;

fn main() {
  // read from command line and save to a vector
  let args: Vec<String>  = env::args().collect();
  
  if args.len() == 1 {
    println!("Usage: gcd 10 90");
    return;
  }
  
  let my_string = &args[1].to_string(); // turn first cli arg to string
  let mut n: u64 = my_string.parse().unwrap_or(0); // convert to 64 bit unsigned int

  let my_string_2 = &args[2].to_string(); // turn second cli arg to string
  let mut m: u64 = my_string_2.parse().unwrap_or(0); // convert to 64 bit unsigned int
    
  if n == 0 || m == 0 {
    println!("One of the arguments is 0");
    return;
  }
  
  while m != 0 {
    if m < n {
      let t = m;
      m = n;
      n = t;
    }
    m = m % n;
  }

  println!("{:?}", n);
}
