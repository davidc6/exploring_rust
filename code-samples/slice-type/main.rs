fn main() {
    let s = String::from("Hello world this is Rust");
    let a = find_str(&s, 2); // find the first word
    // uncommenting this will generate compiler error,
    // .clear() requires mutable references and then println immutable
    // and Rust does not allow this
    // s.clear(); 
    println!("{:?}", a);
}

fn find_str<'a>(s: &'a str, order: usize) -> &'a str {
    let space = b' ';
    let mut start = 0;
    let mut finish = 0;
    let mut count = 0;

    // as_bytes() - String contents to byte slice
    // iter() - iterator over the slice
    // enumerate() - iterator that gives iter count and value
    for (index, &value) in s.as_bytes().iter().enumerate() {
        if value == space && start == finish {
            start += 1;
            continue;
        }

        if value == space {
            count += 1;
            if count == order {
                return &s[start..finish]
            }
            start = index + 1;
            finish = index;
        }

        finish += 1;
    }

    &s[0..1]
}