// MIT/Apache2 License

use core::fmt;

cfg_std! {
    use std::error::Error as StdError;
    use std::io::Error as IoError;
}

cfg_std_unix! {
    use nix::errno::Errno;
}

/// An error that may occur during operation.
pub struct Error {
    inner: Inner,
}

enum Inner {
    /// We tried to run an operation that is not supported.
    Unsupported(Unsupported),
    /// We did something that has put the X11 connection into an invalid state.
    ///
    /// This is a signal to the connection to halt all operations.
    InvalidState(InvalidState),
    /// We could not parse the display.
    CouldntParseDisplay,
    /// An I/O error occurred.
    #[cfg(feature = "std")]
    Io(IoError),
}

#[derive(Clone, Copy)]
pub(crate) enum Unsupported {
    Fds,
    Socket,
}

#[derive(Clone, Copy)]
pub(crate) enum InvalidState {
    FdsNotWritten,
}

impl Error {
    pub(crate) fn invalid_state(is: InvalidState) -> Self {
        Error {
            inner: Inner::InvalidState(is),
        }
    }

    pub(crate) fn unsupported(us: Unsupported) -> Self {
        Error {
            inner: Inner::Unsupported(us),
        }
    }

    pub(crate) fn io(io: IoError) -> Self {
        Error {
            inner: Inner::Io(io),
        }
    }

    pub(crate) fn couldnt_parse_display() -> Self {
        Error {
            inner: Inner::CouldntParseDisplay,
        }
    }
}

cfg_std_unix! {
    impl Error {
        pub(crate) fn nix(errno: Errno) -> Self {
            Error::io(IoError::from_raw_os_error(errno as i32))
        }
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use alloc::format;

        let s = match self.inner {
            Inner::Unsupported(ref e) => format!("Unsupported: {}", e),
            Inner::InvalidState(ref e) => format!("InvalidState: {}", e),
            Inner::CouldntParseDisplay => "CouldntParseDisplay".into(),
            #[cfg(feature = "std")]
            Inner::Io(ref e) => format!("{:?}", e),
        };

        f.debug_tuple("Error").field(&s).finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self.inner {
            Inner::Unsupported(ref e) => write!(f, "attempted an unsupported operation: {}", e),
            Inner::InvalidState(ref e) => write!(f, "entered an invalid state: {}", e),
            Inner::CouldntParseDisplay => write!(f, "could not parse the DISPLAY string"),
            #[cfg(feature = "std")]
            Inner::Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for Unsupported {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Unsupported::Fds => write!(f, "file descriptors"),
            Unsupported::Socket => write!(f, "unix sockets"),
        }
    }
}

impl fmt::Display for InvalidState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            InvalidState::FdsNotWritten => write!(f, "file descriptors have not been written"),
        }
    }
}

cfg_std! {
    impl StdError for Error {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            match self.inner {
                #[cfg(feature = "std")]
                Inner::Io(ref e) => Some(e),
                _ => None,
            }
        }
    }
}

pub type Result<T = ()> = core::result::Result<T, Error>;
