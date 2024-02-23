use std::io;
use thiserror::Error;

pub type BankResult<T> = Result<T, BankError>;

#[derive(Error, Debug)]
pub enum BankError {
    #[error("IO Error")]
    IoError(#[from] io::Error),
    #[error("IO Error")]
    Other(String),
}
