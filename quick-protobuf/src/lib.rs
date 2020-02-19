//! A library to read binary protobuf files
//!
//! This reader is developed similarly to a pull reader

#![deny(missing_docs)]
#![allow(dead_code)]

extern crate byteorder;

pub mod errors;
pub mod message;
pub mod reader;
pub mod sizeofs;
pub mod writer;

pub use crate::errors::{Error, Result};
pub use crate::message::{MessageRead, MessageWrite};
pub use crate::reader::{deserialize_from_slice, BytesReader, Reader};
pub use crate::writer::{serialize_into_vec, Writer};
