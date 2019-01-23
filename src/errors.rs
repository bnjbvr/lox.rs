use std::error::Error;
use std::fmt;
use std::io;
use std::ops::Deref;

#[derive(Debug)]
pub struct OwnError {
    line: u32,
    context: String,
    msg: String,
}

impl OwnError {
    fn new(line: u32, context: String, msg: String) -> Self {
        Self { line, context, msg }
    }
}

impl fmt::Display for OwnError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        writeln!(
            formatter,
            "[{}] Error {}: {}",
            self.line, self.context, &self.msg
        )
    }
}

#[derive(Debug)]
pub enum LoxError {
    Own(OwnError),
    Other(Box<Error>),
}

impl From<io::Error> for LoxError {
    fn from(other: io::Error) -> LoxError {
        LoxError::Other(Box::new(other))
    }
}

impl fmt::Display for LoxError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LoxError::Own(err) => err.fmt(formatter),
            LoxError::Other(err) => err.fmt(formatter),
        }
    }
}

impl Error for LoxError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            LoxError::Other(boxed) => Some(boxed.deref()),
            _ => None,
        }
    }
}

pub type LoxResult = Result<(), LoxError>;

pub fn report_error(line: u32, context: String, msg: String) -> LoxResult {
    Err(LoxError::Own(OwnError::new(line, context, msg)))
}
