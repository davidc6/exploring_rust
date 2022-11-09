// # Notes
// 
// - lifetimes can be elided (omitted) in function item, function pointer, function closure trait signatures 
// - '_ the compiler infers/guesses the lifetime
// - static lifetime extends to end of the program
// - one lifetime that is longer than other can be used instead
// - can be thought of as "how long we need diff pointers to live for"

pub struct StrSplit<'a> {
    remainder: Option<&'a str>,
    delimiter: &'a str,
}

// - reads as StrSplit is generic over lifetime 'a
// - for every lifetime 'a define methods for the StrSplit<'a>
impl<'a> StrSplit<'a> {
    pub fn new(haystack: &'a str, delimiter: &'a str) -> Self {
        Self {
            remainder: Some(haystack),
            delimiter,
        }
    }
}

impl<'a> Iterator for StrSplit<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        // mut self - mutable reference to the value inside of self remainder - modify the existing one
        // can also be Some(next_delimiter) = &mut self.remainder
        if let Some(ref mut remainder /* &mut &'a str */) = self.remainder /* Option<&'a str> */ {
            if let Some(next_delimiter) = remainder.find(self.delimiter) {
                let until_delimiter = &remainder[..next_delimiter];
                *remainder = &remainder[(next_delimiter + self.delimiter.len())..];
                Some(until_delimiter)
            } else {
                self.remainder.take()
            }
        } else {
            None
        }
    }
}

#[cfg(test)]
fn should_work() {
    let haystack = "a b c d e";
    let letters: Vec<_> = StrSplit::new(haystack, " ").collect();
    assert_eq!(letters, vec!["a", "b", "c", "d", "e"]);
}