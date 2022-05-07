// const generics
fn sieve<const N: usize>() -> [bool; N] {
  let mut prime = [true; N];
  let mut p = 2;

  while p * p <= N {
    // if not changed then it is a prime
    if prime[p] == true {
      // update all multiples of p
      let mut y = p * p;

      while y < N {
        prime[y] = false;
        y += p;
      }
    }

    p += 1;
  }

  prime
}

fn main() {
  const X: usize = 120;
  let array = sieve::<{X + 1}>();

  for (i, &val) in array.iter().enumerate() {
    if &val == &true && i > 2 {
      println!("{:?}", i);
    }
  }
}
