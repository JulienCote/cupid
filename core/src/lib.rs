use std::fmt::Display;

pub mod actor;
pub mod sandbox;
pub mod type_system;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    Type,
    Name(String),
    MalformedProgram(String),
    NotYetImplemented,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Type => write!(f, "'type"),
            Error::Name(name) => write!(f, "'{name}"),
            Error::MalformedProgram(msg) => write!(f, "Malformed program: {msg}"),
            Error::NotYetImplemented => write!(f, "'nyi"),
        }
    }
}
