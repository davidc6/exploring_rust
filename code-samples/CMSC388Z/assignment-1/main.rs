fn main() {
  let tup = (1,5);
  let slice = slice_arr(&[1,2,3,4], tup);
  println!("{:?}", slice);


}

// Slice an array if in range
fn slice_arr(arr: &[i32], range: (usize, usize)) -> Result<&[i32], &'static str> {
  println!("{}", arr.len() >= range.1);
  if arr.len() >= range.0 && arr.len() >= range.1 {
      return Ok(&arr[(range.0 - 1)..(range.1 - 1)]);
  }
  Err("OOB!")
}
