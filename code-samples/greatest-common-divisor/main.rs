use std::env;

fn main() {
  // read from command line and save to a vector
  let args: Vec<String>  = env::args().collect();
  
  let my_string = &args[1].to_string(); // dereference to the first command line variable to string
  let mut n: i32 = my_string.parse().unwrap_or(0);

  let my_string_2 = &args[2].to_string();
  let mut m: i32 = my_string_2.parse().unwrap_or(0); // reference to the second command line variable
  
  assert!(n != 0 && m != 0); // 
  
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
