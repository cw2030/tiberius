extern crate byteorder;
extern crate chrono;
extern crate encoding;
extern crate net2;

use std::borrow::Cow;
use std::convert::From;
use std::io;

mod protocol;
mod client;
pub use client::*;

pub static LIB_NAME: &'static str = "tiberius";

/// An error returned by the SQL-server
pub type ServerError = protocol::TokenStreamError;

#[derive(Debug)]
pub enum TdsProtocolError {
    InvalidValue(String)
}

#[derive(Debug)]
pub enum TdsError {
    ProtocolError(TdsProtocolError),
    UnexpectedEOF,
    IoError(io::Error),
    /// An error returned by the SQL-server
    ServerError(ServerError),
    Other(String)
}

pub type TdsResult<T> = std::result::Result<T, TdsError>;

impl From<io::Error> for TdsError {
    fn from(err: io::Error) -> TdsError {
        TdsError::IoError(err)
    }
}

impl From<byteorder::Error> for TdsError {
    fn from(err: byteorder::Error) -> TdsError {
        match err {
            byteorder::Error::Io(x) => TdsError::IoError(x),
            byteorder::Error::UnexpectedEOF => TdsError::UnexpectedEOF
        }
    }
}

impl From<Cow<'static, str>> for TdsError {
    fn from(err: Cow<'static, str>) -> TdsError {
        TdsError::Other(err.into_owned())
    }
}

impl From<TdsProtocolError> for TdsError {
    fn from(err: TdsProtocolError) -> TdsError {
        TdsError::ProtocolError(err)
    }
}
