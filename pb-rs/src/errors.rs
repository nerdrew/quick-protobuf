//! A module to handle all errors via error-chain crate

use std::io;

/// An error enum which derives `std::error::Error`
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Io error
    #[error("{0}")]
    Io(#[source] io::Error),
    /// Nom Error
    #[error("{0}")]
    Nom(#[source] ::nom::simple_errors::Err),

    // No .proto file provided
    #[error("No .proto file provided")]
    NoProto,
    /// Input file
    #[error("Cannot read input file '{0}'")]
    InputFile(String),
    /// Output file
    #[error("Cannot read output file '{0}'")]
    OutputFile(String),
    /// Output file
    #[error("Cannot read output directory '{0}'")]
    OutputDirectory(String),
    /// Multiple input files with --output argument
    #[error("--output only allowed for single input file")]
    OutputMultipleInputs,
    /// Invalid message
    #[error(
        "Message checks errored: {0}\r\n\
                   Proto definition might be invalid or something got wrong in the parsing"
    )]
    InvalidMessage(String),
    /// Varint decoding error
    #[error(
        "Cannot convert protobuf import into module import:: {0}\r\n\
                   Import definition might be invalid, some characters may not be supported"
    )]
    InvalidImport(String),
    /// Empty read
    #[error("No message or enum were read;\
                   either definition might be invalid or there were only unsupported structures")]
    EmptyRead,
    /// enum or message not found
    #[error("Could not find message or enum {0}")]
    MessageOrEnumNotFound(String),
    #[error(
        "Enum field cannot be set to '{0}', this variant does not exist"
    )]
    InvalidDefaultEnum(String),
    /// read_fn implementation for Maps
    #[error("There should be a special case for maps")]
    ReadFnMap,
    #[error("Messages {0:?} are cyclic (missing an optional field)")]
    Cycle(Vec<String>),
    /// --output and --output_directory both used
    #[error("only one of --output or --output_directory allowed")]
    OutputAndOutputDir,
}

/// A wrapper for `Result<T, Error>`
pub type Result<T> = ::std::result::Result<T, Error>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Error {
        Error::Io(e)
    }
}
