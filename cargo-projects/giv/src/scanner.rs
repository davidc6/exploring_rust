use std::{collections::HashMap, fmt, sync::OnceLock};

static KEYWORDS: OnceLock<HashMap<&'static str, TokenType>> = OnceLock::new();

/// A map of keywords, used when parsing the source code.
fn keywords() -> &'static HashMap<&'static str, TokenType> {
    KEYWORDS.get_or_init(|| HashMap::from([("let", TokenType::Let)]))
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TokenType {
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

impl fmt::Display for TokenType  {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", 
            match self {
                TokenType::LeftParen => "L_PAREN",
                TokenType::RightParen => "R_PAREN",
                TokenType::Equal => "EQUAL",
                TokenType::Semi => "SEMI",
                TokenType::Identifier => "IDENTIFIER",
                TokenType::String => "STRING",
                TokenType::Let => "LET",
                TokenType::Eof => "EOF"
            }
        )
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Token {
    token_type: TokenType,
    lexeme: String,
    line_number: usize,
}

// Extract into error reporter
fn _log_error(line: usize, location: String, message: String) {
    let e = format!("[line {line} ] Error {location}: {message}");
    println!("{e}");
}

pub struct Scanner {
    source_code: String,
    tokens: Vec<Token>,
    current_line_number: usize,
}

impl From<&str> for Scanner {
    fn from(value: &str) -> Self {
        Scanner {
            source_code: value.to_owned(),
            tokens: vec![],
            current_line_number: 1,
        }
    }
}

impl Scanner {
    /// Checks whether the current (position) end of source code.
    fn is_end(&self, current: usize) -> bool {
        current >= self.source_code.len()
    }

    /// Creates a new token and pushes it to tokens vector.
    fn push_token(&mut self, token_type: TokenType, current_position: usize) {
        self.tokens.push(Token {
            token_type,
            lexeme: self.source_code[current_position..current_position + 1].to_owned(),
            line_number: self.current_line_number,
        });
    }

    /// Creates and pushes a token by looking at start and end positions in the source
    /// code. This is particularly useful for data types such as strings where we can extract 
    /// value out of the source code since we know the start and end positions.
    fn push_token_end(&mut self, token_type: TokenType, start: usize, end: usize) {
        match self.source_code.get(start..end) {
            Some(lexeme) => {
                // If it's a keyword, we push it into the vector.
                let Some(keyword) = keywords().get(&lexeme).copied() else {
                    // If not a lexeme, check for type and process accordingly.
                    return match token_type {
                        // start + 1 => to skip the opening quote.
                        // end + 1   => for range operator to not include it.
                        TokenType::String => self.tokens.push(Token {
                            token_type,
                            lexeme: self.source_code.get(start + 1..end + 1).unwrap().to_owned(),
                            line_number: self.current_line_number,
                        }),
                        _ => self.tokens.push(Token {
                            token_type,
                            lexeme: lexeme.to_owned(),
                            line_number: self.current_line_number,
                        }),
                    };
                };

                self.tokens.push(Token {
                    token_type: keyword,
                    lexeme: lexeme.to_owned(),
                    line_number: self.current_line_number,
                });
            }
            None => panic!("No lexeme found"),
        }
    }

    /// Peek at the next lexeme.
    fn peek(&self, current: usize) -> Option<&str> {
        self.source_code.get(current + 1..current + 2).to_owned()
    }

    /// Scan the source and build a vector of tokens.
    pub fn scan(&mut self) {
        let mut current_position = 0;

        loop {
            if self.is_end(current_position) {
                break;
            }

            let char = &self
                .source_code
                .get(current_position..current_position + 1)
                .unwrap_or("")
                .chars()
                .next();

            let Some(char) = char else {
                return;
            };

            match char {
                '(' => {
                    self.push_token(TokenType::LeftParen, current_position);
                }
                ')' => {
                    self.push_token(TokenType::RightParen, current_position);
                }
                ';' => {
                    self.push_token(TokenType::Semi, current_position);
                }
                ' ' | '\r' | '\t' => {
                    // These are ignored
                }
                '\n' => {
                    self.current_line_number += 1;
                }
                '=' => {
                    self.push_token(TokenType::Equal, current_position);
                }
                '"' => {
                    let mut current_end = current_position + 1;

                    while self.peek(current_end) != Some("\"") {
                        current_end += 1;

                        if self.is_end(current_end) {
                            break;
                        }
                    }

                    self.push_token_end(TokenType::String, current_position, current_end);

                    // Current_position will be the opening quote (").
                    // Why +2 here? We want to move from last char and closing quote (") to the next char:
                    //
                    // "hello";
                    //      ^
                    //      |
                    //      current_end is here
                    //
                    // "hello";
                    //        ^
                    //        |
                    //        current_position is now here (current_end + 2)
                    current_position = current_end + 2;
                    continue;
                }
                _ => {
                    let mut current_end = current_position;

                    if self.is_end(current_end) {
                        self.push_token(TokenType::Semi, self.source_code.len() - 1);
                        return;
                    }

                    loop {
                        let b = &self.source_code[current_end..current_end + 1]
                            .chars()
                            .next()
                            .unwrap();

                        if (b != &' ' || b != &';') && b.is_ascii_alphabetic() {
                            if b.is_alphanumeric() {
                                current_end += 1;
                                continue;
                            }

                            self.push_token_end(
                                TokenType::Identifier,
                                current_position,
                                current_end,
                            );

                            current_position = current_end;
                            break;
                        } else {
                            self.push_token_end(
                                TokenType::Identifier,
                                current_position,
                                current_end,
                            );
                            current_position = current_end;
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
                    //     current_start = Some(current_position);
                    // }
                }
            }

            current_position += 1;
        }

        self.tokens.push(Token {
            token_type: TokenType::Eof,
            lexeme: "".to_owned(),
            line_number: self.current_line_number,
        });
    }
}

#[cfg(test)]
mod scanner_tests {
    use crate::scanner::{Scanner, Token, TokenType};

    fn expected_part_1() -> Vec<Token> {
        vec![
            Token {
                token_type: TokenType::Let,
                lexeme: "let".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "x".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "hey".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Semi,
                lexeme: ";".to_owned(),
                line_number: 1,
            },
        ]
    }

    fn expected_part_2() -> Vec<Token> {
        vec![
            Token {
                token_type: TokenType::Let,
                lexeme: "let".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "y".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "hello".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Semi,
                lexeme: ";".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_owned(),
                line_number: 1,
            },
        ]
    }

    #[test]
    fn from_sets_up_source_code() {
        let scanner = Scanner::from("let x = \"hey\";");
        assert!(scanner.tokens == vec![]);
        assert!(scanner.source_code == *"let x = \"hey\";");
    }

    #[test]
    fn scan_creates_tokens_from_source_code() {
        let mut scanner = Scanner::from("let x = \"hey\";let y = \"hello\";");
        scanner.scan();

        let mut expected = expected_part_1();
        expected.extend(expected_part_2());

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn line_count_works_with_newline() {
        let mut scanner = Scanner::from("let x = \"hey\";\nlet y = \"hello\";");
        scanner.scan();

        let mut expected = expected_part_1();
        let to_modify = expected_part_2()
            .into_iter()
            .map(|mut val| {
                val.line_number = 2;
                val
            })
            .collect::<Vec<Token>>();
        expected.extend(to_modify);

        assert_eq!(scanner.tokens, expected);
    }

    #[test]
    fn peek_functionality_works() {
        let scanner = Scanner::from("let x = \"hi\";");
        let actual = scanner.peek(1);
        assert_eq!(actual, Some("t"));
    }

    #[test]
    fn scanner_keyword_parsing_works() {
        let mut scanner = Scanner::from("let;");
        scanner.scan();

        assert_eq!(
            scanner.tokens,
            vec![
                Token {
                    token_type: TokenType::Let,
                    lexeme: "let".to_owned(),
                    line_number: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_owned(),
                    line_number: 1
                }
            ]
        );
    }

    #[test]
    fn scanner_identifier_parsing_works() {
        let mut scanner = Scanner::from("var;");
        scanner.scan();

        assert_eq!(
            scanner.tokens,
            vec![
                Token {
                    token_type: TokenType::Identifier,
                    lexeme: "var".to_owned(),
                    line_number: 1
                },
                Token {
                    token_type: TokenType::Eof,
                    lexeme: "".to_owned(),
                    line_number: 1
                }
            ]
        );
    }

    #[test]
    fn scanner_assign_string_to_variable_works() {
        let mut scanner = Scanner::from("let x = \"abc\";");
        scanner.scan();

        let actual = scanner.tokens;

        let expected = vec![
            Token {
                token_type: TokenType::Let,
                lexeme: "let".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Identifier,
                lexeme: "x".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Equal,
                lexeme: "=".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::String,
                lexeme: "abc".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Semi,
                lexeme: ";".to_owned(),
                line_number: 1,
            },
            Token {
                token_type: TokenType::Eof,
                lexeme: "".to_owned(),
                line_number: 1,
            },
        ];

        assert_eq!(actual, expected);
    }
}
