#[derive(Debug, PartialEq)]
enum TokenType {
    // Operators
    Equal,
    Semi,

    // Literals
    Identifier,
    String,

    // Keywords
    Let,

    // End Of File
    Eof,
}

#[derive(Debug, PartialEq)]
struct Token {
    token_type: TokenType,
    lexeme: String,
}

struct Position(usize, usize);

impl Scanner {
    fn is_end(&self, current: usize) -> bool {
        current == self.source_code.len()
    }

    fn push_token(&mut self, token_type: TokenType, pos: Position) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source_code[pos.0..pos.1].to_owned(),
        });
    }

    fn peek(&self, current: usize) -> Option<char> {
        self.source_code.chars().nth(current + 1)
    }

    fn scan(&mut self) {
        let mut current_start = None;

        for (pos, char) in self.source_code.chars().enumerate() {
            match char {
                ';' => {
                    // There's most likely a value before the semicolon,
                    // so push the value into the tokens vector.
                    // current_start + 2 and pos - 1 is to ignore the quotes
                    // that surround the value.
                    self.tokens.push(Token {
                        token_type: TokenType::String,
                        lexeme: self.source_code[current_start.unwrap() + 2..pos - 1].to_owned(),
                    });

                    // Push semicolon into the tokens vector.
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
                        "print" => {
                            // peek - is the next ( or something else
                            // if (
                            //   then start reading inside then ()
                            //     )
                            if self.peek(pos) == Some('(') {}
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
    use crate::scanner::{self, Scanner, Token, TokenType};

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

    #[test]
    fn peek_works() {
        let scanner = Scanner::from("let x = \"hi\";");
        let actual = scanner.peek(1);
        assert_eq!(actual, Some('t'));
    }

    #[test]
    fn incorrect_grammar() {
        let mut scanner = Scanner::from("let = \"hi\";");
        todo!()
    }
}
