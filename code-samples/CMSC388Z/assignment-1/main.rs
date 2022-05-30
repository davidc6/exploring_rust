fn main() {
  let num = double(&2);
  println!("double: {}", num);

  let mut number = 2;
  double_in_place(&mut number);
  println!("double in place: {:?}", number);

  let num = double_to_i64(&2);
  println!("double i32 to i64: {}", num);

  let tup = (1,5);
  let slice = slice_arr(&[1,2,3,4], tup);
  println!("slice: {:?}", slice);

  let res = sqrt(124);
  println!("square root: {}", res);

  let result1 = fib((2, 3));
  println!("{:?}", result1);
}

fn double(x: &i32) -> i32 {
  return (*x * 2) as i32;
}

fn double_in_place(x: &mut i32) {
  *x *= 2
}

fn double_to_i64(x: &i32) -> i64 {
  return (x * 2) as i64;
}

fn sqrt(n: usize) -> usize {
  if n == 0 || n == 1 {
    return n;
  }

  let mut i = 1;
  let mut result = 1;

  while result <= n {
    i += 1;
    result = i * i;
  }

  return i - 1;
}

const OUTSIZE: usize = 5;
fn fib(ns: (i32, i32)) -> [i32; OUTSIZE] {
  let ( mut prev , mut next ) = ns;

  let mut arr = [0;5];
  let mut count = 0;

  while count < OUTSIZE {
    let current = prev + next;
    arr[count] = current;
    prev = next;
    next = current;
    count += 1;
  }

  return arr;
}

// Slice an array if in range
fn slice_arr(arr: &[i32], range: (usize, usize)) -> Result<&[i32], &'static str> {
  if arr.len() >= range.0 && arr.len() >= range.1 {
    return Ok(&arr[(range.0 - 1)..(range.1 - 1)]);
  }
  Err("OOB!")
}
