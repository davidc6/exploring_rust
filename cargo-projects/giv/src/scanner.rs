#[derive(Debug, PartialEq)]
enum TokenType {
    LeftParen,
    RightParen,

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

// Extract into error reporter
fn report_error(line: usize, location: String, message: String) {
    let e = format!("[line {line} ] Error {location}: {message}");
    println!("{e}");
}

struct Position(usize, usize);

impl Scanner {
    /// Checks whether at the end of the source or not.
    fn is_end(&self, current: usize) -> bool {
        current == self.source_code.len()
    }

    fn push_token(&mut self, token_type: TokenType, current_position: usize) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source_code[current_position..current_position + 1].to_owned(),
        });
    }

    fn push_token_end(
        &mut self,
        token_type: TokenType,
        current_position: usize,
        end_position: usize,
    ) {
        let lexeme = self.source_code[current_position..end_position].to_owned();

        if lexeme == "let".to_owned() {
            self.tokens.push(Token {
                token_type: TokenType::Let,
                lexeme,
            });
        } else {
            self.tokens.push(Token { token_type, lexeme });
        }
    }

    fn peek(&self, current: usize) -> Option<&str> {
        let a = &self.source_code.get(current + 1..current + 2);
        a.to_owned()
    }

    fn go(&mut self) {
        let mut source = self.source_code.chars().enumerate().peekable();
        let current_start = 0;

        // let mut count = 0;
        // let w = source.peek();

        while let Some(x) = source.next() {
            match x.1 {
                ' ' => {
                    let y = &self.source_code[current_start..1];

                    match y {
                        "print" => if source.peek() == Some(&(1, '(')) {},
                        _ => {}
                    };
                }
                _ => {}
            }
            // count += 1;
        }
    }

    fn scan(&mut self) {
        let mut pos = 0;

        loop {
            if pos >= self.source_code.len() {
                break;
            }

            let char = &self.source_code[pos..pos + 1].chars().next();

            match char.unwrap() {
                '(' => {
                    self.push_token(TokenType::LeftParen, pos);
                }
                ')' => {
                    self.push_token(TokenType::RightParen, pos);
                }
                ';' => {
                    self.push_token(TokenType::Semi, pos);
                    pos += 1;
                }
                ' ' | '\r' | '\t' => {
                    // These are ignored
                    pos += 1;
                    continue;
                }
                '\n' => {
                    // TODO: increment line count?
                }
                '=' => {
                    self.push_token(TokenType::Equal, pos);
                    pos += 1;
                }
                '"' => {
                    let mut cur = pos + 1;

                    while self.peek(cur) != Some("\"") {
                        cur += 1;
                        if cur >= self.source_code.len() {
                            break;
                        }
                    }

                    if cur >= self.source_code.len() {
                        self.push_token(TokenType::Semi, self.source_code.len() - 1);
                        return;
                    }

                    // TODO check if keyword
                    self.push_token_end(TokenType::String, pos + 1, cur + 1);

                    pos = cur + 1;
                }
                '=' => {
                    if &self.source_code[pos..pos + 1] != "=" {
                        self.push_token(TokenType::Equal, pos);
                    }

                    pos += 1;
                }
                _ => {
                    let mut count = pos;

                    if count >= self.source_code.len() {
                        return;
                    }

                    loop {
                        let b = &self.source_code[count..count + 1].chars().next().unwrap();
                        if b != &' ' && b.is_ascii_alphabetic() {
                            if b.is_alphanumeric() {
                                count += 1;
                                continue;
                            }

                            self.push_token_end(TokenType::Identifier, pos, count);

                            pos = count;
                            break;
                        } else {
                            self.push_token_end(TokenType::Identifier, pos, count);
                            pos = count;
                            break;
                        }
                    }

                    // We handle identifiers here.
                    //
                    // We handle numbers here.
                    //
                    // This case should catch unexpected syntax
                    //
                    // Log it
                    // if current_start.is_none() {
                    //     current_start = Some(pos);
                    // }
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

    #[ignore]
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

    #[ignore]
    #[test]
    fn peek_works() {
        let scanner = Scanner::from("let x = \"hi\";");
        let actual = scanner.peek(1);
        assert_eq!(actual, Some("t"));
    }

    #[ignore]
    #[test]
    fn incorrect_grammar() {
        let mut scanner = Scanner::from("let x = \"wow\";print(x);");
        scanner.scan();
        // todo!()
    }

    #[test]
    fn scanner_works() {
        let mut scanner = Scanner::from("let x = \"abc\";");
        scanner.scan();

        let actual = scanner.tokens;

        let expected = vec![
            Token {
                token_type: TokenType::Let,
                lexeme: "let".to_owned(),
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "x".to_owned(),
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_owned(),
            },
            Token {
                token_type: TokenType::String,
                lexeme: "abc".to_owned(),
            },
            Token {
                token_type: TokenType::Semi,
                lexeme: ";".to_owned(),
            },
        ];

        assert_eq!(actual, expected);
    }
}
