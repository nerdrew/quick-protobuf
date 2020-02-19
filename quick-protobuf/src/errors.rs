//! A module to handle all errors via error-chain crate

use std::io;

/// An error enum which derives `std::error::Error`
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Io error
    #[error("{0}")]
    Io(#[source] io::Error),
    /// Utf8 Error
    #[error("{0}")]
    Utf8(#[source] ::std::str::Utf8Error),

    /// Deprecated feature (in protocol buffer specification)
    #[error("Feature '{0}' has been deprecated")]
    Deprecated(&'static str),
    /// Unknown wire type
    #[error("Unknown wire type '{0}', must be less than 6")]
    UnknownWireType(u8),
    /// Varint decoding error
    #[error("Cannot decode varint")]
    Varint,
    /// Error while parsing protocol buffer message
    #[error("Error while parsing message: {0}")]
    Message(String),
    /// Unexpected map tag
    #[error("Unexpected map tag: '{0}', expecting 1 or 2")]
    Map(u8),
}

/// A wrapper for `Result<T, Error>`
pub type Result<T> = ::std::result::Result<T, Error>;

impl Into<io::Error> for Error {
    fn into(self) -> ::std::io::Error {
        match self {
            Error::Io(x) => x,
            Error::Utf8(x) => io::Error::new(io::ErrorKind::InvalidData, x),
            x => io::Error::new(io::ErrorKind::Other, x),
        }
    }
}

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}

impl From<::std::str::Utf8Error> for Error {
    fn from(e: ::std::str::Utf8Error) -> Error {
        Error::Utf8(e)
    }
}
