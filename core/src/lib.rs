use std::fmt::Display;

pub mod actor;
pub mod core;
pub mod lang;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Type,
    Rank,
    Length,
    Name(String),
    MalformedProgram(String),
    NotYetImplemented,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Type => write!(f, "'type"),
            Error::Rank => write!(f, "'rank"),
            Error::Length => write!(f, "'length"),
            Error::Name(name) => write!(f, "'{name}"),
            Error::MalformedProgram(msg) => write!(f, "Malformed program: {msg}"),
            Error::NotYetImplemented => write!(f, "'nyi"),
        }
    }
}

impl std::error::Error for Error {}
