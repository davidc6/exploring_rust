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

fn dangle() -> &String {
  // s will go out of scope once the function is finished
  let x = String::from("Hi");
  
  // this essentially becomes dangling pointer
  &x
} // Rust won't let us do this

fn example1() {
  struct Person { name: String, birth: i32 }
    
  // returns a Vector (not a pointer)
  // ownership moves from Vec::new() to composers
  let mut composers = Vec::new();
  // to_string() returns a fresh instance of String
  // Person takes ownership of the string
  // The entire Person structure gets passed to composers via push method
  // composers gets ownership over Person and String as well
  composers.push(Person { name: "Palestrina".to_string(), birth: 1525 });
}

fn example2() {
  struct Person {
    name: Option<String>,
    age: String
  }
  // t owns Vector
  let mut t = vec![Person { name: Some("A".to_string()), age: "1".to_string() }];
  // get name and replace with None since "A" is wrapped in Some type
  // same as std::mem::replace(&mut t[0].name, None);
  let name = t[0].name.take();
  println!("{:?}", t[0].name);
}

// Arc and Rc
// moves and reference pointers are two ways to relax the ownership model
fn example3() {
    // Rc is a reference-counter pointer, Rc pointer is immutable
    // the value owned by Rc is immutable
    let s = Rc::new("hello".to_string());
    let t = s.clone(); // points to the original heap-allocated String, refers to the same block of memory
    let u = s.clone(); // points to the original heap-allocated String, refers to the same block of memory
}