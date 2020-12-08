// MIT/Apache2 License

//! This module provides structures used in error handling of `breadx` functions.

use core::{fmt, ops::Deref};
#[cfg(feature = "std")]
use std::{error::Error as StdError, io::Error as IoError};

/// The common error type returned by `breadx` functions.
#[derive(Debug)]
pub enum BreadError {
    StaticMsg(&'static str),
    /// Unable to parse connection name.
    UnableToParseConnection,
    /// Unable to open connection to X11 server.
    UnableToOpenConnection,
    /// IO Error
    #[cfg(feature = "std")]
    Io(IoError),
    /// Unable to open connection to the X11 server.
    FailedToConnect,
    /// X11 server rejected our authorization.
    FailedToAuthorize,
    /// BadReadError
    BadObjectRead(Option<&'static str>),
    /// Required extension was not present.
    ExtensionNotPresent,
    /// An error propogated by the X11 server.
    XProtocol {
        error_code: ErrorCode,
        minor_code: u8,
        major_code: u8,
        sequence: u16,
    },
    /// The X connection closed without telling us.
    ClosedConnection,
    /// Attempted to call an async function on a blocking object.
    WouldBlock,
}

impl BreadError {
    #[inline]
    pub(crate) fn from_x_error<T: Deref<Target = [u8]>>(bytes: T) -> Self {
        let b = &*bytes;
        let mut sequence: [u8; 2] = [0; 2];
        sequence.copy_from_slice(&bytes[2..=3]);
        let sequence = u16::from_ne_bytes(sequence);
        let mut minor_code: [u8; 2] = [0; 2];
        minor_code.copy_from_slice(&bytes[8..=9]);
        let minor_code = u16::from_ne_bytes(minor_code);
        Self::XProtocol {
            error_code: ErrorCode(b[1]),
            major_code: b[10],
            minor_code: minor_code as _,
            sequence,
        }
    }
}

#[cfg(feature = "std")]
impl From<IoError> for BreadError {
    #[inline]
    fn from(io: IoError) -> Self {
        Self::Io(io)
    }
}

impl fmt::Display for BreadError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaticMsg(m) => f.write_str(m),
            Self::UnableToParseConnection => f.write_str("Unable to parse X11 connection name"),
            Self::UnableToOpenConnection => f.write_str("Unable to open connection to X11 server"),
            Self::FailedToConnect => f.write_str("Unable to connect to the X11 server"),
            Self::FailedToAuthorize => f.write_str("Authorization was rejected by the X11 server"),
            Self::WouldBlock => f.write_str("Attempted to call async I/O on a blocking connection"),
            Self::BadObjectRead(name) => write!(
                f,
                "Unable to read object of type from bytes: {}",
                name.unwrap_or("Unknown")
            ),
            Self::ExtensionNotPresent => f.write_str("Extension was not found on X server"),
            Self::XProtocol {
                error_code,
                minor_code,
                major_code,
                sequence,
            } => write!(
                f,
                "An X11 error of type {} occurred on a request of opcode {}:{} and sequence {}",
                error_code, major_code, minor_code, sequence
            ),
            Self::ClosedConnection => f.write_str("The X connection closed without our end of the connection closing. Did you forget to listen for WM_DELTE_WINDOW?"),
            #[cfg(feature = "std")]
            Self::Io(i) => write!(f, "{}", i),
        }
    }
}

#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct ErrorCode(u8);

impl fmt::Display for ErrorCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self.0 {
            1 => "Request",
            2 => "Value",
            3 => "Window",
            4 => "Pixmap",
            5 => "Atom",
            6 => "Cursor",
            7 => "Font",
            8 => "Match",
            9 => "Drawable",
            10 => "Access",
            11 => "Alloc",
            12 => "Colormap",
            13 => "GConext",
            14 => "IDChoice",
            15 => "Name",
            16 => "Length",
            17 => "Implementation",
            id => return write!(f, "{}", id),
        })
    }
}

impl fmt::Debug for ErrorCode {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

#[cfg(feature = "std")]
impl StdError for BreadError {}

pub type Result<Success = ()> = core::result::Result<Success, BreadError>;
