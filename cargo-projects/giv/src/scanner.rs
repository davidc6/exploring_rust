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

impl Scanner {
    fn push_token(&mut self, token_type: TokenType) {}

    fn scan(&mut self) {
        for (pos, char) in self.source_code.chars().enumerate() {
            match char {
                '=' => self.tokens.push(Token {
                    token_type: TokenType::Equal,
                    lexeme: self.source_code[pos..pos + 1].to_owned(),
                }),
                _ => (),
            }
        }
    }
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
    use crate::scanner::{Scanner, Token, TokenType};

    #[test]
    fn from_works() {
        let scanner = Scanner::from("let x = \"a\";");
        assert!(scanner.tokens == vec![]);
        assert!(scanner.source_code == *"let x = \"a\";");
    }

    #[test]
    fn push_works() {
        let mut scanner = Scanner::from("let x = \"a\";");
        scanner.scan();
        assert!(
            scanner.tokens
                == vec![Token {
                    token_type: TokenType::Equal,
                    lexeme: "=".to_owned()
                }]
        );
    }
}
