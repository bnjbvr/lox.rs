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
    tokens: Vec<Token>,
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

    pub fn scan_tokens(mut self) -> LoxDiag<Vec<Token>> {
        let mut errors = vec![];

        loop {
            self.start = self.current;
            if let Err(err) = self.scan_token() {
                errors.push(err);
                break;
            }
        }

        self.tokens
            .push(Token::new(TokenType::EOF, "".to_string(), self.line));
        if errors.len() == 0 {
            Ok(self.tokens)
        } else {
            Err(errors)
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

    fn add_token(&mut self, which: TokenType) {
        self.tokens
            .push(Token::new(which, "".to_string(), self.line));
    }

    fn add_token_str(&mut self, which: TokenType, s: &'source str) {
        self.tokens
            .push(Token::new(which, s.to_string(), self.line));
    }

    fn scan_token(&mut self) -> LoxResult<bool> {
        let c = self.advance();
        if c.is_none() {
            return Ok(false);
        }
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
