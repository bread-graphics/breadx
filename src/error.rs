// MIT/Apache2 License

use core::{fmt, ops::Deref};
#[cfg(feature = "std")]
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
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
    /// BadReadError
    BadObjectRead,
    /// Required extension was not present.
    ExtensionNotPresent(&'static str),
    /// An errorp propogated by the X11 server.
    XProtocol {
        error_code: ErrorCode,
        minor_code: u8,
        major_code: u8,
        sequence: u16,
    },
}

impl Error {
    #[inline]
    pub(crate) fn from_x_error<T: Deref<Target = [u8]>>(bytes: T) -> Self {
        let b = &*bytes;
        Self::XProtocol {
            error_code: ErrorCode(b[1]),
            major_code: b[2],
            minor_code: b[3],
            sequence: 0,
        }
    }
}

#[cfg(feature = "std")]
impl From<IoError> for Error {
    #[inline]
    fn from(io: IoError) -> Self {
        Self::Io(io)
    }
}

impl fmt::Display for Error {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::StaticMsg(m) => f.write_str(m),
            Self::UnableToParseConnection => f.write_str("Unable to parse X11 connection name"),
            Self::UnableToOpenConnection => f.write_str("Unable to open connection to X11 server"),
            Self::FailedToConnect => f.write_str("Unable to connect to the X11 server"),
            Self::BadObjectRead => write!(f, "Unable to read object of type from bytes"),
            Self::ExtensionNotPresent(ext) => write!(f, "Unable to load extension {}", ext),
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

pub type Result<Success = ()> = core::result::Result<Success, Error>;
