//! Error types for the SAMP Query library.

use std::io;
use std::net::AddrParseError;
use std::string::FromUtf8Error;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse socket address: {0}")]
    AddrParse(#[from] AddrParseError),

    #[error("Failed to bind to socket: {0}")]
    Bind(#[source] io::Error),

    #[error("Failed to connect to server: {0}")]
    Connect(#[source] io::Error),

    #[error("Connection timed out")]
    Timeout,

    #[error("Failed to send packet: {0}")]
    Send(#[source] io::Error),

    #[error("Failed to receive packet: {0}")]
    Receive(#[source] io::Error),

    #[error("Invalid response: {0}")]
    InvalidResponse(String),

    #[error("Failed to parse UTF-8 string: {0}")]
    Utf8(#[from] FromUtf8Error),

    #[error("Server error: {0}")]
    ServerError(String),

    #[error("RCON authentication failed")]
    RconAuthFailed,

    #[error("Invalid query type: {0}")]
    InvalidQueryType(String),

    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Other(String),
}

impl Error {
    pub fn is_timeout(&self) -> bool {
        matches!(self, Error::Timeout)
    }

    pub fn is_auth_error(&self) -> bool {
        matches!(self, Error::RconAuthFailed)
    }

    pub fn is_server_error(&self) -> bool {
        matches!(self, Error::ServerError(_))
    }
}
