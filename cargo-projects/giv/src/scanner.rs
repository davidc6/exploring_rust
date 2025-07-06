#[derive(Debug, PartialEq)]
enum TokenType {
    Equal,
    Semi,
    Let,
    Identifier,
    String,
    Eof,
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

impl Scanner {
    fn push_token(&mut self, token_type: TokenType) {}

    fn scan(&mut self) {
        let mut current_start = None;

        for (pos, char) in self.source_code.chars().enumerate() {
            match char {
                ';' => {
                    self.tokens.push(Token {
                        token_type: TokenType::String,
                        lexeme: self.source_code[current_start.unwrap() + 2..pos - 1].to_owned(),
                    });

                    self.tokens.push(Token {
                        token_type: TokenType::Semi,
                        lexeme: self.source_code[pos..pos + 1].to_owned(),
                    });
                    current_start = Some(pos + 1);
                }
                ' ' => {
                    let current = &self.source_code[current_start.unwrap()..pos];

                    match current {
                        "let" => {
                            self.tokens.push(Token {
                                token_type: TokenType::Let,
                                lexeme: self.source_code[current_start.unwrap()..pos].to_owned(),
                            });
                            current_start = None;
                        }
                        "=" => {
                            self.tokens.push(Token {
                                token_type: TokenType::Equal,
                                lexeme: self.source_code[current_start.unwrap()..pos].to_owned(),
                            });
                            current_start = Some(pos);
                        }
                        _ => {
                            self.tokens.push(Token {
                                token_type: TokenType::Identifier,
                                lexeme: self.source_code[current_start.unwrap()..pos].to_owned(),
                            });
                            current_start = None;
                        }
                    }
                }
                _ => {
                    if current_start.is_none() {
                        current_start = Some(pos);
                    }
                }
            }
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_owned(),
        });
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
        let scanner = Scanner::from("let x = \"hey\";");
        assert!(scanner.tokens == vec![]);
        assert!(scanner.source_code == *"let x = \"hey\";");
    }

    #[test]
    fn push_works() {
        let mut scanner = Scanner::from("let x = \"hey\";let y = \"hello\";");
        scanner.scan();

        assert!(
            scanner.tokens
                == vec![
                    Token {
                        token_type: TokenType::Let,
                        lexeme: "let".to_owned()
                    },
                    Token {
                        token_type: TokenType::Identifier,
                        lexeme: "x".to_owned()
                    },
                    Token {
                        token_type: TokenType::Equal,
                        lexeme: "=".to_owned()
                    },
                    Token {
                        token_type: TokenType::String,
                        lexeme: "hey".to_owned()
                    },
                    Token {
                        token_type: TokenType::Semi,
                        lexeme: ";".to_owned()
                    },
                    Token {
                        token_type: TokenType::Let,
                        lexeme: "let".to_owned()
                    },
                    Token {
                        token_type: TokenType::Identifier,
                        lexeme: "y".to_owned()
                    },
                    Token {
                        token_type: TokenType::Equal,
                        lexeme: "=".to_owned()
                    },
                    Token {
                        token_type: TokenType::String,
                        lexeme: "hello".to_owned()
                    },
                    Token {
                        token_type: TokenType::Semi,
                        lexeme: ";".to_owned()
                    },
                    Token {
                        token_type: TokenType::Eof,
                        lexeme: "".to_owned()
                    },
                ]
        );
    }
}
