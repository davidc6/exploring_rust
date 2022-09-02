fn main() {
    let s = String::from("Hello world this is Rust");
    let a = find_str(&s, 2); // find the first word
    println!("{:?}", a);
}

fn find_str<'a>(s: &'a String, order: usize) -> &'a str {
    let space = b' ';
    let mut start = 0;
    let mut finish = 0;
    let mut count = 0;

    for (index, value) in s.bytes().enumerate() {
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