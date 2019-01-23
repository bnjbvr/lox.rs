use std::fmt;

#[derive(Debug)]
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

pub struct Token {
    which: TokenType,
    lexem: String,
    line: u32,
}

impl Token {
    fn new(which: TokenType, lexem: String, line: u32) -> Token {
        Token { which, lexem, line }
    }
}

impl fmt::Display for Token {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        writeln!(fmt, "{:?} at line {}", self.which, self.line)
    }
}
