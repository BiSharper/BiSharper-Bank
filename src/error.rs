use std::fmt::{Display, Formatter};
use std::io;
use std::string;
use thiserror::Error;

pub type BankResult<T> = Result<T, BankError>;

#[derive(Error, Debug)]
pub enum BankError {
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("String formatting error.")]
    Utf8Error(#[from] string::FromUtf8Error),
    #[error("Encountered post mature version entry!")]
    PostMatureVersion,
    #[error("Encountered weird data offset")]
    InvalidDataOffset,
    #[error("Missing version entry!")]
    MissingVersionEntry,
    #[error("Missing {0} terminator!")]
    MissingNullTerminator(MissingTerminatorType),
}

#[derive(Debug, Eq, PartialEq)]
pub enum MissingTerminatorType {
    String,
    Checksum
}

impl Display for MissingTerminatorType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MissingTerminatorType::String => write!(f, "string"),
            MissingTerminatorType::Checksum => write!(f, "checksum"),
        }
    }
}
