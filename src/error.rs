// MIT/Apache2 License

use core::fmt;
use std::io::Error as IoError;

#[derive(Debug)]
pub enum Error {
    /// Unable to parse connection name.
    UnableToParseConnection,
    /// Unable to open connection to X11 server.
    UnableToOpenConnection,
    /// IO Error
    Io(IoError),
    /// Unable to open connection to the X11 server.
    FailedToConnect,
    /// BadReadError
    BadObjectRead,
}

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
            Self::UnableToParseConnection => f.write_str("Unable to parse X11 connection name"),
            Self::UnableToOpenConnection => f.write_str("Unable to open connection to X11 server"),
            Self::FailedToConnect => f.write_str("Unable to connect to the X11 server"),
            Self::BadObjectRead => write!(f, "Unable to read object from bytes"),
            Self::Io(i) => write!(f, "{}", i),
        }
    }
}

pub type Result<Success = ()> = std::result::Result<Success, Error>;
