use crate::errors::{report_error, LoxDiag, LoxResult};
use crate::tokens::{Token, TokenType};

use std::iter::Peekable;
use std::str::CharIndices;

pub struct Scanner<'source> {
    source: &'source str,
    source_iter: Peekable<CharIndices<'source>>,
    start: usize,
    current: usize,
    line: usize,
    tokens: Vec<Token<'source>>,
}

impl<'source> Scanner<'source> {
    pub fn new(source: &'source str) -> Self {
        Self {
            source,
            source_iter: source.char_indices().peekable(),
            start: 0,
            current: 0,
            line: 1,
            tokens: vec![],
        }
    }

    pub fn scan_tokens(mut self) -> LoxDiag<Vec<Token<'source>>> {
        let mut errors = vec![];

        loop {
            self.start = self.current;
            match self.scan_token() {
                Ok(has_more) => {
                    if !has_more {
                        break;
                    }
                }
                Err(err) => {
                    errors.push(err);
                }
            }
        }

        self.tokens.push(Token::new(TokenType::EOF, "", self.line));
        if errors.len() == 0 {
            Ok(self.tokens)
        } else {
            Err(errors)
        }
    }

    fn add_token(&mut self, which: TokenType) {
        self.tokens.push(Token::new(which, "", self.line));
    }

    fn add_token_str(&mut self, which: TokenType, s: &'source str) {
        self.tokens.push(Token::new(which, s, self.line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if let Some(observed) = self.peek() {
            observed == expected
        } else {
            false
        }
    }

    fn advance(&mut self) -> Option<char> {
        if let Some((i, ch)) = self.source_iter.next() {
            self.current += i;
            Some(ch)
        } else {
            None
        }
    }

    fn peek(&mut self) -> Option<char> {
        if let Some((_, c)) = self.source_iter.peek() {
            Some(*c)
        } else {
            None
        }
    }

    fn scan_token(&mut self) -> LoxResult<bool> {
        let c = self.advance();
        if c.is_none() {
            return Ok(false);
        }
        let c = c.unwrap();

        println!("Read char is {}", c);

        match c {
            '(' => {
                self.add_token(TokenType::LeftParen);
            }
            ')' => {
                self.add_token(TokenType::RightParen);
            }
            '{' => {
                self.add_token(TokenType::LeftBrace);
            }
            '}' => {
                self.add_token(TokenType::RightBrace);
            }
            ',' => {
                self.add_token(TokenType::Comma);
            }
            '.' => {
                self.add_token(TokenType::Dot);
            }
            '-' => {
                self.add_token(TokenType::Minus);
            }
            '+' => {
                self.add_token(TokenType::Plus);
            }
            ';' => {
                self.add_token(TokenType::Semi);
            }
            '*' => {
                self.add_token(TokenType::Star);
            }
            '!' => {
                let token_type = if self.matches('=') {
                    TokenType::BangEqual
                } else {
                    TokenType::Bang
                };
                self.add_token(token_type);
            }
            '=' => {
                let token_type = if self.matches('=') {
                    TokenType::EqualEqual
                } else {
                    TokenType::Equal
                };
                self.add_token(token_type);
            }
            '<' => {
                let token_type = if self.matches('=') {
                    TokenType::LessEqual
                } else {
                    TokenType::Less
                };
                self.add_token(token_type);
            }
            '>' => {
                let token_type = if self.matches('=') {
                    TokenType::GreaterEqual
                } else {
                    TokenType::Greater
                };
                self.add_token(token_type);
            }
            '/' => {
                if self.matches('/') {
                    // A comment goes until the end of the line.
                    while let Some(c) = self.peek() {
                        if c != '\n' {
                            self.advance().unwrap();
                        } else {
                            break;
                        }
                    }
                } else {
                    self.add_token(TokenType::Slash);
                }
            }
            c if c == ' ' || c == '\t' || c == '\r' => {
                // Do nothing for whitespace.
            }
            '\n' => self.line += 1,
            _ => {
                report_error(
                    self.line,
                    "parsing".to_string(),
                    format!("unexpected character '{}'", c),
                )?;
            }
        }

        Ok(true)
    }
}

#[test]
fn test_empty() {
    let s = Scanner::new("");
    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].which, TokenType::EOF);
}

#[test]
fn test_comments() {
    let s = Scanner::new("// hello world");
    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].which, TokenType::EOF);
}

#[test]
fn test_empty_multilines() {
    let s = Scanner::new(
        r#"// hello world

    "#,
    );

    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 1);
    assert_eq!(tokens[0].which, TokenType::EOF);
}

#[test]
fn test_multines() {
    let s = Scanner::new(
        r#"
    {} // inline comment {}
    /;*
"#,
    );

    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 6);
    assert_eq!(tokens[0].which, TokenType::LeftBrace);
    assert_eq!(tokens[1].which, TokenType::RightBrace);
    assert_eq!(tokens[2].which, TokenType::Slash);
    assert_eq!(tokens[3].which, TokenType::Semi);
    assert_eq!(tokens[4].which, TokenType::Star);
    assert_eq!(tokens[5].which, TokenType::EOF);
}
