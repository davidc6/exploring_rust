#[derive(PartialEq)]
enum TokenType {
    Equal,
    Semi,
}

#[derive(PartialEq)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

struct Scanner {
    source_code: String,
    tokens: Vec<Token>,
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        Scanner {
            source_code: value.to_owned(),
            tokens: vec![],
        }
    }
}

#[cfg(test)]
mod scanner_tests {
    use crate::scanner::Scanner;

    #[test]
    fn from_works() {
        let scanner = Scanner::from("let x = \"a\";");
        assert!(scanner.tokens == vec![]);
        assert!(scanner.source_code == *"let x = \"a\";");
    }
}
