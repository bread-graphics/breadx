// MIT/Apache2 License

use core::{fmt, ops::Deref};
#[cfg(feature = "std")]
use std::io::Error as IoError;

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
    /// BadReadError
    BadObjectRead(Option<&'static str>),
    /// Required extension was not present.
    ExtensionNotPresent(&'static str),
    /// An error propogated by the X11 server.
    XProtocol {
        error_code: ErrorCode,
        minor_code: u8,
        major_code: u8,
        sequence: u16,
    },
    /// The X connection closed without telling us.
    ClosedConnection,
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
            Self::BadObjectRead(name) => write!(
                f,
                "Unable to read object of type from bytes: {}",
                name.unwrap_or("Unknown")
            ),
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

pub type Result<Success = ()> = core::result::Result<Success, BreadError>;
