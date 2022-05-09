// const generics
fn sieve<const N: usize>() -> [bool; N] {
  let mut prime = [true; N];
  let mut p = 2;

  while p * p <= N {
    // if not changed then it is a prime
    if prime[p] == true {
      // update all multiples of p
      let mut y = p * p;

      while y <= N {
        if y == N {
          break;
        }
        prime[y] = false;
        y += p;
      }
    }

    p += 1;
  }

  prime
}

fn main() {
  const X: usize = 7;
  let array = sieve::<{X + 1}>();
    
  for (i, &val) in array.iter().enumerate() {
    if &val == &true && i > 2 {
      println!("{:?}", i);
    }
  }
}

#[cfg(test)]
mod tests {
  // since test is an inner module and out code is in the outer module,
  // we need to bring the outer module into the scope
  use super::*;

  #[test]
  fn it_works() {
    const N: usize = 5;
    let arr = [true, true, true, true, false];
    let res = sieve::<{ N }>();

    assert_eq!(arr, res);
  }
}
