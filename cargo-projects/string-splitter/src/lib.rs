#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct StringSplitter<'a> {
    leftover: &'a str,
    delimiter: &'a str,
}

impl<'a> StringSplitter<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        StringSplitter {
            leftover: haystack,
            delimiter,
        }
    }
}

impl<'a> Iterator for StringSplitter<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        for (index, char) in self.leftover.chars().enumerate() {
            // Last word since it's length is the same as index + 1
            if index + 1 == self.leftover.len() {
                let word = &self.leftover[0..index + 1];
                self.leftover = &self.leftover[0..0];
                println!("AHA {word}");
                println!("WORDO {:?}", self.leftover);
                return Some(word);
            }

            // Check if at delimiter
            if char == self.delimiter.chars().next().unwrap() {
                let word = &self.leftover[0..index];
                self.leftover = &self.leftover[index + 1..self.leftover.len()];
                return Some(word);
            }
        }

        if self.leftover.is_empty() {
            return None;
        }

        Some(self.leftover)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let haystack = "hello world this is mars";
        let string_splitter: Vec<_> = StringSplitter::new(haystack, " ").collect();
        let expected = vec!["hello", "world", "this", "is", "mars"];

        assert_eq!(string_splitter, expected);
    }
}
