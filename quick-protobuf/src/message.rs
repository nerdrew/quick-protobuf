//! A module to deserialize a `Message` as defined in a .proto file
//!
//! Creates the struct and implements a reader

#[cfg(feature = "std")]
use std::fs::File;
#[cfg(feature = "std")]
use std::io::BufWriter;
#[cfg(feature = "std")]
use std::path::Path;
#[cfg(not(feature = "std"))]
use alloc::vec::Vec;

use crate::errors::Result;
use crate::reader::BytesReader;
use crate::writer::{Writer, WriterBackend};

/// A trait to handle deserialization based on parsed `Field`s
pub trait MessageWrite: Sized {
    /// Writes `Self` into W writer
    fn write_message<W: WriterBackend>(&self, _: &mut Writer<W>) -> Result<()> {
        Ok(())
    }

    /// Computes necessary binary size of self once serialized in protobuf
    fn get_size(&self) -> usize {
        0
    }

    /// Writes self into a file
    #[cfg(feature = "std")]
    fn write_file<P: AsRef<Path>>(&self, p: P) -> Result<()> {
        let file = BufWriter::new(File::create(p)?);
        let mut writer = Writer::new(file);
        self.write_message(&mut writer)
    }
}

/// A trait to handle deserialization from protocol buffers.
pub trait MessageRead<'a>: Sized {
    /// Constructs an instance of `Self` by reading from the given bytes
    /// via the given reader.
    ///
    /// It does NOT read message length first. If you want to read a variable
    /// length message, use `BytesReader::read_message` directly
    fn from_reader(r: &mut BytesReader, bytes: &'a [u8]) -> Result<Self>;
}

/// A trait to provide basic information about a given message
pub trait MessageInfo {
    /// Full message path, in form of Module.Message
    const PATH: &'static str;
}

/// A trait for Owned messages
pub trait Owned {
    /// The proto message type
    type Inner<'a> where Self: 'a;

    /// Create an Owned message from a Vec and the inner proto message
    unsafe fn from_parts<'a>(buf: Vec<u8>, proto: Self::Inner<'a>) -> Self;

    /// The original serialized proto bytes. Warning: if the proto has been
    /// changed, those changes will not be included in these bytes
    fn buf(&self) -> &[u8];

    /// The inner proto message
    fn proto(&self) -> &Self::Inner<'_>;

    /// A mutable reference to the inner proto message
    fn proto_mut(&mut self) -> &mut Self::Inner<'_>;
}
