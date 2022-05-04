fn main() {
  let mut s = String::from("Some string");
  
  s.push_str(", is here!");
  
  { // we create new scope and scope starts
    let greeting = "Hello";
    println!("{}", greeting);
  } // scope ends, so greeting won't be accessible anymore
  
  // if this is passed as a value (without &)
  // then it will get cleaned up eventually as ownership will get passed to some_fn()
  let mut x = String::from("Hello from x!"); 
  
  // this won't work as we cannot modify a value that is not owned by the function
  // let y = some_format(x);
  
  // we refer the value with &
  // this is also known as borrowing
  // we can only have one mutable reference at a time
  // this can prevent data races at runtime
  some_fn(&mut x);
  
  println!("{}", x);  
  println!("{}", s);
  
  let s2 = s;
  // in order to actually deeply copy (heap gets copied), we need to:
  // let s2 = s.clone();
  
  println!("{}", s2);
  // s is no longer valid as it was assigned to s2 above
  // println!("{}", s);
} // main goes out of scope and memory is freed (heap)
// a function called drop() is called at this moment in time

// we can operate of the value without having ownership over it
fn some_fn(x: &mut String) {
  // let mut f = x.to_string();
  x.push_str(", reference updated.");
}
