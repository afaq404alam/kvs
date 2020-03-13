extern crate failure;
extern crate serde_json;

use failure::Fail;
use std::io;

/// Error enum for kvs
#[derive(Fail, Debug)]
pub enum KvsError {
    /// I/O error
    #[fail(display = "{}", _0)]
    Io(#[cause] io::Error),
    /// Serialization/Deserialization error
    #[fail(display = "{}", _0)]
    Serde(#[cause] serde_json::error::Error),
    /// Accessing non-existent key
    #[fail(display = "Key not found")]
    KeyNotFound,
    /// Unexpected command type error.
    /// It indicated a corrupt log or a program bug.
    #[fail(display = "Unexpected command type")]
    UnexpectedCommandType,
}

impl From<io::Error> for KvsError {
    fn from(err: io::Error) -> Self {
        KvsError::Io(err)
    }
}

impl From<serde_json::error::Error> for KvsError {
    fn from(err: serde_json::error::Error) -> Self {
        KvsError::Serde(err)
    }
}

/// Result type for kvs
pub type Result<T> = std::result::Result<T, KvsError>;
