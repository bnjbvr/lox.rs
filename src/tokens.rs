use std::fmt;

#[derive(Debug, PartialEq)]
pub enum TokenType {
    // Single char.
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,
    Minus,
    Plus,
    Semi,
    Slash,
    Star,

    // One or two.
    Bang,
    BangEqual,
    Equal,
    EqualEqual,
    Greater,
    GreaterEqual,
    Less,
    LessEqual,

    // Literals.
    Identifier(String),
    String(String),
    Number(f64),

    // Keywords.
    And,
    Class,
    Else,
    False,
    Fun,
    For,
    If,
    Nil,
    Or,
    Print,
    Return,
    Super,
    This,
    True,
    Var,
    While,

    EOF,
}

pub struct Token<'source> {
    pub which: TokenType, // TODO public?
    lexem: &'source str,
    line: usize,
}

impl<'source> Token<'source> {
    pub fn new(which: TokenType, lexem: &'source str, line: usize) -> Token {
        Token { which, lexem, line }
    }
}

impl<'source> fmt::Display for Token<'source> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{:?} at line {}", self.which, self.line)
    }
}
