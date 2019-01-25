use std::error::Error;
use std::fmt;
use std::io;
use std::ops::Deref;

#[derive(Debug)]
pub struct OwnError {
    line: usize,
    context: String,
    msg: String,
}

impl OwnError {
    fn new(line: usize, context: String, msg: String) -> Self {
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

pub type LoxResult<T> = Result<T, LoxError>;
pub type LoxDiag<T> = Result<T, Vec<LoxError>>;

pub enum DisplayableError {
    s(String),
    errors(Vec<LoxError>),
}

impl From<String> for DisplayableError {
    fn from(s: String) -> DisplayableError {
        DisplayableError::s(s)
    }
}

impl From<Vec<LoxError>> for DisplayableError {
    fn from(v: Vec<LoxError>) -> DisplayableError {
        DisplayableError::errors(v)
    }
}

pub fn report_error<T>(
    line: usize,
    context: String,
    msg: impl Into<DisplayableError>,
) -> LoxResult<T> {
    let msg = match msg.into() {
        DisplayableError::s(s) => s,
        DisplayableError::errors(errors) => errors
            .iter()
            .map(|err| match err {
                LoxError::Own(own) => format!("{}", own),
                LoxError::Other(other) => format!("{}", other),
            })
            .fold("".to_string(), |acc, x| format!("{}\n{}", acc, x)),
    };
    Err(LoxError::Own(OwnError::new(line, context, msg)))
}
