//! Error type used within crate with From for commonly used crate errors
use std::error;
use std::{fmt, io};

/// Result type used within crate
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, PartialEq, Clone)]
/// Kind of error produced
pub enum ErrorKind {
    /// Error running macOS `system_profiler` command
    SystemProfiler,
    /// Unsupported system for command being run; system_profiler not on macOS for example, libusb feature not installed
    Unsupported,
    /// Unable to find USB device on bus
    NotFound,
    /// Unable to open device to query device descriptors - check permissions
    Opening,
    /// Error parsing a string into a value - used for u32 to json deserialization
    Parsing,
    /// Error decoding an encoded value into a type
    Decoding,
    /// Error parsing config file
    Config,
    /// [`std::io::Error`] probably not found when reading file to parse
    Io,
    /// [`libusb::Error`] error
    LibUSB,
    /// Error calling udev
    Udev,
    /// Invalid arg for method or cli
    InvalidArg,
    /// Error From other crate without enum variant
    Other(&'static str),
}

#[derive(Debug, PartialEq)]
/// Cyme error which impl [`std::error`]
pub struct Error {
    /// The [`ErrorKind`]
    pub kind: ErrorKind,
    /// String description
    pub message: String,
}

impl Error {
    /// New error helper
    pub fn new(kind: ErrorKind, message: &str) -> Error {
        Error { kind, message: message.to_string() }
    }

    /// The [`ErrorKind`]
    pub fn kind(&self) -> ErrorKind {
        self.kind.to_owned()
    }

    /// The description
    pub fn message(&self) -> &String {
        &self.message
    }
}

impl error::Error for Error {}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if f.alternate() {
            return write!(f, "{}", self.message);
        } else {
            write!(f, "{:?} Error: {}", self.kind, self.message)
        }
    }
}

impl From<io::Error> for Error {
    fn from(error: io::Error) -> Self {
        Error {
            kind: ErrorKind::Io,
            message: error.to_string(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(error: serde_json::Error) -> Self {
        Error {
            kind: ErrorKind::Parsing,
            message: error.to_string(),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    fn from(error: std::string::FromUtf8Error) -> Self {
        Error {
            kind: ErrorKind::Other("FromUtf8Error"),
            message: error.to_string(),
        }
    }
}

impl Into<io::Error> for Error {
    fn into(self) -> io::Error {
        io::Error::new(io::ErrorKind::Other, self.message)
    }
}
