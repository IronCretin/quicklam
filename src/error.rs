use std::fmt::{self, Display};
use std::num::ParseIntError;

/// An error that occured during parsing
#[derive(Debug, Clone)]
pub struct Error {
    pub start: usize,
    pub kind: ErrorKind,
    pub end: usize,
}
impl Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} at {}:{}", self.kind, self.start, self.end)
    }
}
impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.kind.source()
    }
}

#[derive(Debug, Clone)]
pub enum ErrorKind {
    /// An error occured while parsing an integer
    BadNum(ParseIntError),
    /// An invalid variable name was encountered
    BadVar(String),
}
impl std::error::Error for ErrorKind {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            ErrorKind::BadNum(err) => Some(err),
            ErrorKind::BadVar(_) => None,
        }
    }
}
impl Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ErrorKind::BadNum(err) => write!(f, "Invalid number ({})", err),
            ErrorKind::BadVar(name) => write!(f, "Invalid variable name '{}'", name),
        }
    }
}
