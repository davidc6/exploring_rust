use std::fs;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

struct Differ<'a, 'b> {
    file_a: &'a str,
    file_b: &'b str,
}

impl<'a, 'b> Differ<'a, 'b> {
    fn new(file_a: &'a str, file_b: &'b str) -> Self {
        Self { file_a, file_b }
    }

    fn compare(&self) {
        let a = self.file_a.lines();
        let mut b = self.file_b.lines();

        for line_a in a {
            // for line_b in b {
            let n = b.next().unwrap();

            let len_a = line_a.len();
            let len_b = n.len();

            // a b c d e
            // f g h i j

            if line_a != n {
                println!("{line_a}");
                println!("{n}");

                continue;
            }
            // }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let a = "hello, this is it\nby name is one";
        let b = "hello, this is it\nby name is two";

        let d = Differ::new(a, b);
        let actual = d.compare();

        assert_eq!(4, 4);
    }
}
