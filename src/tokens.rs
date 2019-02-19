use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum TokenType<'source> {
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
    Identifier(&'source str),
    String(&'source str),
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
    pub which: TokenType<'source>, // TODO public?
    lexem: &'source str,
    line: usize,
}

impl<'source> Token<'source> {
    pub fn new(which: TokenType<'source>, lexem: &'source str, line: usize) -> Token<'source> {
        Token { which, lexem, line }
    }
}

impl<'source> fmt::Display for Token<'source> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{:?} at line {}", self.which, self.line)
    }
}
