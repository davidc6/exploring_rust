fn main() {
  let num = double(&2);
  println!("{}", num);

  let mut number = 2;
  double_in_place(&mut number);
  println!("{:?}", number);

  let num = double_to_i64(&2);
  println!("{}", num);

  let tup = (1,5);
  let slice = slice_arr(&[1,2,3,4], tup);
  println!("{:?}", slice);
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

// Slice an array if in range
fn slice_arr(arr: &[i32], range: (usize, usize)) -> Result<&[i32], &'static str> {
  println!("{}", arr.len() >= range.1);
  if arr.len() >= range.0 && arr.len() >= range.1 {
      return Ok(&arr[(range.0 - 1)..(range.1 - 1)]);
  }
  Err("OOB!")
}
