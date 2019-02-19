use crate::errors::{report_error, LoxDiag, LoxResult};
use crate::tokens::{Token, TokenType};

use std::collections::HashMap;
use std::iter::Peekable;
use std::str;
use std::str::CharIndices;

lazy_static! {
    static ref KEYWORDS: HashMap<&'static str, TokenType<'static>> = {
        let mut m = HashMap::new();
        m.insert("and", TokenType::And);
        m.insert("class", TokenType::Class);
        m.insert("else", TokenType::Else);
        m.insert("false", TokenType::False);
        m.insert("for", TokenType::For);
        m.insert("fun", TokenType::Fun);
        m.insert("if", TokenType::If);
        m.insert("nil", TokenType::Nil);
        m.insert("or", TokenType::Or);
        m.insert("print", TokenType::Print);
        m.insert("return", TokenType::Return);
        m.insert("super", TokenType::Super);
        m.insert("this", TokenType::This);
        m.insert("true", TokenType::True);
        m.insert("var", TokenType::Var);
        m.insert("while", TokenType::While);
        m
    };
}

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

    fn add_token(&mut self, which: TokenType<'source>) {
        self.tokens.push(Token::new(which, "", self.line));
    }

    fn matches(&mut self, expected: char) -> bool {
        if let Some(observed) = self.peek() {
            observed == expected
        } else {
            false
        }
    }

    /// Returns the character read at the current position (and set current to
    /// the new position).
    fn advance(&mut self) -> Option<char> {
        if let Some((i, ch)) = self.source_iter.next() {
            self.current = i;
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

    fn peek_next(&self) -> Option<char> {
        let mut clone = self.source_iter.clone();
        if let Some(_) = clone.next() {
            if let Some((_, c)) = clone.peek() {
                return Some(*c);
            }
        }
        None
    }

    fn scan_string(&mut self) -> LoxResult<()> {
        while let Some(c) = self.peek() {
            if c == '\n' {
                self.line += 1;
            }
            self.advance().unwrap();
            if c == '"' {
                let substr =
                    str::from_utf8(&self.source.as_bytes()[self.start + 1..self.current]).unwrap();
                self.add_token(TokenType::String(substr));
                return Ok(());
            }
        }
        report_error(
            self.line,
            "when reading a string".to_string(),
            "unterminated string".to_string(),
        )
    }

    fn scan_number(&mut self, first_digit: char) -> LoxResult<()> {
        let mut number = first_digit.to_digit(10).unwrap() as f64;
        let mut fractional_power_of_ten: Option<f64> = None;

        while let Some(c) = self.peek() {
            if is_digit(c) {
                self.advance().unwrap();
                let c_num = c.to_digit(10).unwrap() as f64;
                if let Some(decimal) = fractional_power_of_ten.as_mut() {
                    number += c_num * *decimal;
                    *decimal /= 10.0;
                } else {
                    number *= 10.0;
                    number += c_num;
                }
            } else if c == '.' {
                if let Some(d) = self.peek_next() {
                    if is_digit(d) {
                        self.advance().unwrap();
                        if fractional_power_of_ten.is_some() {
                            return report_error(
                                self.line,
                                "when parsing a number".to_string(),
                                "unexpected dot".to_string(),
                            )?;
                        }
                        fractional_power_of_ten = Some(0.1);
                        continue;
                    }
                }
                break;
            } else {
                break;
            }
        }

        self.add_token(TokenType::Number(number));
        Ok(())
    }

    fn scan_identifier(&mut self) -> LoxResult<()> {
        while let Some(c) = self.peek() {
            if !is_alpha_numeric(c) {
                break;
            }
            self.advance().unwrap();
        }

        let substr = str::from_utf8(&self.source.as_bytes()[self.start..self.current + 1]).unwrap();

        if let Some(token) = KEYWORDS.get(substr) {
            self.add_token((*token).clone());
        } else {
            self.add_token(TokenType::Identifier(substr));
        }
        Ok(())
    }

    fn scan_token(&mut self) -> LoxResult<bool> {
        let c = self.advance();
        if c.is_none() {
            return Ok(false);
        }

        self.start = self.current;
        let c = c.unwrap();
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

            '"' => {
                self.scan_string()?;
            }

            c if is_digit(c) => {
                self.scan_number(c)?;
            }

            c if is_alpha(c) => {
                self.scan_identifier()?;
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

fn is_digit(c: char) -> bool {
    c >= '0' && c <= '9'
}

fn is_alpha(c: char) -> bool {
    c >= 'a' && c <= 'z' || c >= 'A' && c <= 'Z' || c == '_'
}

fn is_alpha_numeric(c: char) -> bool {
    is_digit(c) || is_alpha(c)
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

#[test]
fn test_scan_string() {
    let s = Scanner::new(
        r#"
    "terminated 42 string"
"#,
    );
    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].which, TokenType::String("terminated 42 string"));
    assert_eq!(tokens[1].which, TokenType::EOF);

    let s = Scanner::new(
        r#"
    "unterminated 42 string
"#,
    );
    assert!(s.scan_tokens().is_err());
}

#[test]
fn test_scan_number() {
    let s = Scanner::new("423298");
    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].which, TokenType::Number(423298.0));
    assert_eq!(tokens[1].which, TokenType::EOF);

    let s = Scanner::new("423298.0");
    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].which, TokenType::Number(423298.0));
    assert_eq!(tokens[1].which, TokenType::EOF);

    let s = Scanner::new("423298.");
    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].which, TokenType::Number(423298.0));
    assert_eq!(tokens[1].which, TokenType::Dot);
    assert_eq!(tokens[2].which, TokenType::EOF);

    let s = Scanner::new(" 12.34  ");
    let tokens = s.scan_tokens().unwrap();
    assert_eq!(tokens.len(), 2);
    assert_eq!(tokens[0].which, TokenType::Number(12.34));
    assert_eq!(tokens[1].which, TokenType::EOF);
}
