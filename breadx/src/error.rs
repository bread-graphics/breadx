//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use core::any::type_name;
use core::fmt;
use core::str;

use alloc::string::String;
use alloc::string::ToString;
use x11rb_protocol::x11_utils::X11Error;
use x11rb_protocol::{
    errors::{ConnectError, ParseError},
    protocol::xproto::{SetupAuthenticate, SetupFailed},
};

cfg_std! {
    use std::error::Error as StdError;
    use std::io::Error as IoError;
    use std::io::ErrorKind;
}

cfg_std_unix! {
    use nix::errno::Errno;
}

/// An error that may occur during operation of this crate.
pub struct Error {
    /// The inner details of the error.
    inner: Inner,
    /// Whether or not this error occurred during connection
    /// initialization.
    initialization: bool,
}

/// The innards of the [`Error`] struct.
///
/// Making this type not public allows us to change it without it being
/// a breaking change.
enum Inner {
    /// We tried to run an operation that is not supported.
    #[allow(dead_code)]
    Unsupported(Unsupported),
    /// A custom message.
    Message(String),
    /// We did something that has put the X11 connection into an invalid state.
    ///
    /// This is a signal to the connection to halt all operations.
    InvalidState(InvalidState),
    /// We could not parse the display string.
    CouldntParseDisplay {
        /// True if we got the display string from an environment
        /// variable, false otherwise.
        from_env: bool,
    },
    /// This type was poisoned.
    Poisoned {
        /// The name of the involved type.
        type_name: &'static str,
    },
    /// We could not parse the given type.
    ParseError(ParseError),
    /// A failure occurred during the setup process.
    SetupFailed(SetupFailure),
    /// An X11 error occurred.
    X11Error(X11Error),
    /// The extension is missing.
    MissingExtension {
        /// The name of the extension.
        name: &'static str,
    },
    /// Request was too large to be sent.
    RequestTooLarge {
        /// Length of the request.
        x_len: usize,
        /// Maximum request length.
        max_len: usize,
    },
    /// Connection was closed.
    Disconnected,
    /// Attempted to send a request while another was in progress.
    #[cfg(feature = "async")]
    AsyncSendInProgress {
        /// The sequence number of the request that was in progress.
        seq: u64,
    },
    /// An I/O error occurred.
    #[cfg(feature = "std")]
    Io(IoError),
}

#[allow(dead_code)]
#[derive(Clone, Copy)]
pub(crate) enum Unsupported {
    Fds,
    Socket,
}

/// Reason why something entered an invalid state.
///
/// Keeping this crate private means that we can add more in the future
/// if we want to.
#[derive(Clone, Copy)]
pub(crate) enum InvalidState {
    UnexpectedFds,
    NotEnoughSetup,
    ScreenOutOfRange,
    ZeroIdMask,
    BadError,
    XidsExhausted,
}

/// The setup for the connection failed.
#[derive(Clone)]
pub enum SetupFailure {
    Failed(SetupFailed),
    Authenticate(SetupAuthenticate),
}

// crate-private API
impl Error {
    fn from_inner(inner: Inner) -> Self {
        Self {
            inner,
            initialization: false,
        }
    }

    pub(crate) fn make_invalid_state(is: InvalidState) -> Self {
        Error::from_inner(Inner::InvalidState(is))
    }

    #[allow(dead_code)]
    pub(crate) fn make_unsupported(us: Unsupported) -> Self {
        Error::from_inner(Inner::Unsupported(us))
    }

    pub(crate) fn couldnt_parse_display(from_env: bool) -> Self {
        Error::from_inner(Inner::CouldntParseDisplay { from_env })
    }

    pub(crate) fn make_poisoned<T>() -> Self {
        Error::from_inner(Inner::Poisoned {
            type_name: type_name::<T>(),
        })
    }

    pub(crate) fn make_large_request(x_len: usize, max_len: usize) -> Self {
        Error::from_inner(Inner::RequestTooLarge { x_len, max_len })
    }

    pub(crate) fn make_disconnected() -> Self {
        Error::from_inner(Inner::Disconnected)
    }

    pub(crate) fn make_setup_failure(sf: SetupFailure) -> Self {
        Error::from_inner(Inner::SetupFailed(sf))
    }

    pub(crate) fn make_connect_error(ce: ConnectError) -> Self {
        // convert the error to one of our other variants
        let mut err = match ce {
            ConnectError::ParseError(pe) => Error::make_parse_error(pe),
            #[cfg(feature = "std")]
            ConnectError::IoError(io) => Error::io(io),
            ConnectError::InvalidScreen => {
                Error::make_invalid_state(InvalidState::ScreenOutOfRange)
            }
            ConnectError::ZeroIdMask => Error::make_invalid_state(InvalidState::ZeroIdMask),
            ConnectError::SetupFailed(sf) => Error::make_setup_failure(SetupFailure::Failed(sf)),
            ConnectError::SetupAuthenticate(sa) => {
                Error::make_setup_failure(SetupFailure::Authenticate(sa))
            }
            ConnectError::DisplayParsingError => Error::couldnt_parse_display(false),
            ConnectError::Incomplete { .. } => {
                Error::make_invalid_state(InvalidState::NotEnoughSetup)
            }
            _ => unreachable!(),
        };

        err.initialization = true;
        err
    }

    /// Tell if this is a would-block I/O error.
    pub(crate) fn would_block(&self) -> bool {
        cfg_if::cfg_if! {
            if #[cfg(feature = "std")] {
                match self.as_io_error() {
                    Some(io) => io.kind() == ErrorKind::WouldBlock,
                    None => false,
                }
            } else {
                false
            }
        }
    }

    #[allow(dead_code)]
    pub(crate) fn is_protocol_error(&self) -> bool {
        matches!(
            self.inner,
            Inner::X11Error(..) | Inner::MissingExtension { .. }
        )
    }
}

cfg_std! {
    impl Error {
        pub(crate) fn io(io: IoError) -> Self {
            if !matches!(io.kind(), ErrorKind::WouldBlock) {
                tracing::error!("encountered I/O error: {io:?}", io = io);
            }
            Error::from_inner(Inner::Io(io))
        }
    }
}

// public API
impl Error {
    /// Create a new error from something that can be formatted into
    /// a message.
    #[must_use]
    pub fn make_msg<D: fmt::Display>(msg: D) -> Self {
        Self::from_inner(Inner::Message(msg.to_string()))
    }

    /// Did this error happen as the result of calling an
    /// unsupported operation?
    #[must_use]
    pub fn unsupported(&self) -> bool {
        matches!(self.inner, Inner::Unsupported(_))
    }

    /// Create an error from an X11 parse error.
    #[must_use]
    pub fn make_parse_error(pe: ParseError) -> Self {
        Error::from_inner(Inner::ParseError(pe))
    }

    /// Create an error from a missing extension.
    #[must_use]
    pub fn make_missing_extension(name: &'static str) -> Self {
        Error::from_inner(Inner::MissingExtension { name })
    }

    /// Did this error happen as a result of some state-based
    /// object entering an invalid state?
    #[must_use]
    pub fn invalid_state(&self) -> bool {
        #[allow(unused_mut)]
        let mut base = matches!(self.inner, Inner::InvalidState(_) | Inner::Poisoned { .. });

        cfg_if::cfg_if! {
            if #[cfg(feature = "std")] {
                base |= matches!(self.inner, Inner::Io(ref err) if err.kind() != ErrorKind::WouldBlock);
            }
        }

        base
    }

    /// Did this error occur during initialization of the X11
    /// connection?
    #[must_use]
    pub fn initialization(&self) -> bool {
        self.initialization
    }

    /// Get the inner setup failure that this error is a wrapper around.
    #[must_use]
    pub fn as_setup_failure(&self) -> Option<&SetupFailure> {
        match self.inner {
            Inner::SetupFailed(ref sf) => Some(sf),
            _ => None,
        }
    }

    /// Convert this error into a setup failure.
    pub fn into_setup_failure(self) -> Result<SetupFailure> {
        match self.inner {
            Inner::SetupFailed(sf) => Ok(sf),
            inner => Err(Self::from_inner(inner)),
        }
    }

    /// Did this error occur because the X11 connection was closed?
    #[must_use]
    pub fn disconnected(&self) -> bool {
        matches!(self.inner, Inner::Disconnected)
    }
}

// crate-private api
cfg_async! {
    impl Error {
        pub(crate) fn async_send_in_progress(seq: u64) -> Self {
            Error::from_inner(Inner::AsyncSendInProgress { seq })
        }
    }
}

// public API
cfg_std! {
    impl Error {
        /// Get the inner I/O error that this error is a wrapper around.
        #[must_use]
        pub fn as_io_error(&self) -> Option<&IoError> {
            match self.inner {
                Inner::Io(ref io) => Some(io),
                _ => None,
            }
        }

        /// Convert this error into an I/O error.
        pub fn into_io_error(self) -> core::result::Result<IoError, Self> {
            match self.inner {
                Inner::Io(io) => Ok(io),
                inner => Err(Self::from_inner(inner)),
            }
        }
    }
}

// crate-private API
cfg_std_unix! {
    impl Error {
        pub(crate) fn nix(errno: Errno) -> Self {
            Error::io(IoError::from_raw_os_error(errno as i32))
        }
    }
}

/* public trait impls */

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct InnerRepr<'a>(&'a Inner);

        impl<'a> fmt::Debug for InnerRepr<'a> {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self.0 {
                    Inner::Unsupported(ref e) => write!(f, "Unsupported: {}", e),
                    Inner::Message(ref msg) => f.write_str(msg),
                    Inner::InvalidState(ref e) => write!(f, "InvalidState: {}", e),
                    Inner::CouldntParseDisplay { .. } => f.write_str("CouldntParseDisplay"),
                    Inner::Poisoned { type_name } => write!(f, "Poisoned({})", type_name),
                    Inner::ParseError(err) => write!(f, "ParseError: {}", err),
                    Inner::SetupFailed(SetupFailure::Authenticate(_)) => {
                        f.write_str("SetupFailed: could not authenticate")
                    }
                    Inner::SetupFailed(SetupFailure::Failed(fail)) => {
                        let reason = str::from_utf8(&fail.reason).unwrap_or("bad utf-8");
                        write!(f, "SetupFailed: {}", reason)
                    }
                    Inner::X11Error(x11) => fmt::Debug::fmt(x11, f),
                    Inner::MissingExtension { name } => {
                        write!(f, "MissingExtension: {}", name)
                    }
                    Inner::RequestTooLarge { x_len, max_len } => {
                        write!(f, "RequestTooLarge: {} > {}", x_len, max_len)
                    }
                    Inner::Disconnected => write!(f, "Disconnected"),
                    #[cfg(feature = "async")]
                    Inner::AsyncSendInProgress { seq } => {
                        write!(f, "AsyncSendInProgress: seq {}", seq)
                    }
                    #[cfg(feature = "std")]
                    Inner::Io(ref e) => write!(f, "{:?}", e),
                }
            }
        }

        f.debug_tuple("Error")
            .field(&InnerRepr(&self.inner))
            .finish()
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.inner {
            Inner::Unsupported(ref e) => write!(f, "attempted an unsupported operation: {}", e),
            Inner::InvalidState(ref e) => write!(f, "entered an invalid state: {}", e),
            Inner::CouldntParseDisplay { from_env: true } => {
                f.write_str("could not parse the $DISPLAY environment variable")
            }
            Inner::Poisoned { type_name } => write!(f, "object of type {} poisoned", type_name),
            Inner::CouldntParseDisplay { from_env: false } => {
                f.write_str("could not parse the display string")
            }
            Inner::ParseError(ref err) => fmt::Display::fmt(err, f),
            Inner::SetupFailed(SetupFailure::Authenticate(ref e)) => {
                let reason = str::from_utf8(&e.reason).unwrap_or("<invalid utf8>");
                write!(f, "could not authenticate to the X server: {}", reason)
            }
            Inner::SetupFailed(SetupFailure::Failed(ref e)) => {
                let reason = str::from_utf8(&e.reason).unwrap_or("<invalid utf8>");
                write!(f, "could not setup the X connection: {}", reason)
            }
            Inner::X11Error(ref x11) => {
                write!(
                    f,
                    "a {:?} error occurred on sequence number {}",
                    x11.error_kind, x11.sequence,
                )
            }
            Inner::MissingExtension { name } => {
                write!(f, "missing extension: {}", name)
            }
            Inner::Message(ref msg) => f.write_str(msg),
            Inner::RequestTooLarge { x_len, max_len } => {
                write!(
                    f,
                    "Request of size {} bytes exceeds maximum length of {} bytes",
                    x_len * 4,
                    max_len * 4
                )
            }
            Inner::Disconnected => write!(f, "connection to the X server was closed"),
            #[cfg(feature = "async")]
            Inner::AsyncSendInProgress { seq } => {
                write!(f, "async send in progress for sequence {}", seq)
            }
            #[cfg(feature = "std")]
            Inner::Io(ref e) => write!(f, "{}", e),
        }
    }
}

impl fmt::Display for Unsupported {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Unsupported::Fds => f.write_str("file descriptors"),
            Unsupported::Socket => f.write_str("unix sockets"),
        }
    }
}

impl fmt::Display for InvalidState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            InvalidState::UnexpectedFds => f.write_str("unexpected file descriptors"),
            InvalidState::NotEnoughSetup => f.write_str("not enough data for setup"),
            InvalidState::ScreenOutOfRange => f.write_str("screen out of range"),
            InvalidState::ZeroIdMask => f.write_str("zero id mask"),
            InvalidState::BadError => f.write_str("misformatted error"),
            InvalidState::XidsExhausted => f.write_str("server ran out of xids"),
        }
    }
}

// public api
impl From<X11Error> for Error {
    fn from(x11: X11Error) -> Self {
        Self::from_inner(Inner::X11Error(x11))
    }
}

cfg_std! {
    impl From<IoError> for Error {
        fn from(e: IoError) -> Self {
            Self::from_inner(Inner::Io(e))
        }
    }

    impl StdError for Error {
        fn source(&self) -> Option<&(dyn StdError + 'static)> {
            match self.inner {
                Inner::ParseError(ref pe) => Some(pe),
                #[cfg(feature = "std")]
                Inner::Io(ref e) => Some(e),
                _ => None,
            }
        }
    }
}

/// Indicates that any errors that occur during execution of this function
/// are initialization errors.
pub(crate) fn initialization<R>(f: impl FnOnce() -> Result<R>) -> Result<R> {
    match f() {
        Ok(r) => Ok(r),
        Err(mut e) => {
            e.initialization = true;
            Err(e)
        }
    }
}

/// A convenience type that is equivalent to
/// `Result<T, [Error]>`.
pub type Result<T = ()> = core::result::Result<T, Error>;
