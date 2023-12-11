use std::fmt;
use std::error::Error;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum PuzzleErrorKind {
    MissingInput,
    InputError,
    ParseError,
    IOError(std::io::Error)
}

#[derive(Debug)]
pub struct PuzzleError {
    kind: PuzzleErrorKind
}

impl PuzzleError {
    pub fn kind(&self) -> &PuzzleErrorKind {
        &self.kind
    }
}

impl From<PuzzleErrorKind> for PuzzleError {
    fn from(kind: PuzzleErrorKind) -> Self {
        Self { kind }
    }
}

impl From<ParseIntError> for PuzzleError {
    fn from(_: ParseIntError) -> Self {
        Self { kind: PuzzleErrorKind::ParseError }
    }
}

impl From<std::io::Error> for PuzzleError {
    fn from(io_error: std::io::Error) -> Self {
        Self {
            kind: PuzzleErrorKind::IOError(io_error)
        }
    }
}

impl fmt::Display for PuzzleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PuzzleError: {:?}", self.kind)
    }
}

impl Error for PuzzleError {}
