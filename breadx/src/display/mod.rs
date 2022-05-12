// MIT/Apache2 License

//! Displays, used to connect to the X11 server.

mod basic;
use alloc::{boxed::Box, vec::Vec};
pub use basic::BasicDisplay;

mod cell;
pub use cell::CellDisplay;

mod cookie;
pub use cookie::Cookie;

mod ext;
pub use ext::*;

mod extension_map;
pub(crate) use extension_map::ExtensionMap;

mod sans_io;
pub(crate) use sans_io::X11Core;

mod poison;
pub(crate) use poison::Poisonable;

mod prefetch;
pub(crate) use prefetch::Prefetch;

mod raw_request;
pub use raw_request::{RawReply, RawRequest};

cfg_sync! {
    mod sync;
    pub use sync::SyncDisplay;
}

use crate::{Error, Fd, Result};
use x11rb_protocol::{
    connection::ReplyFdKind,
    protocol::{xproto::Setup, Event},
    x11_utils::{ReplyFDsRequest, ReplyRequest, Request, TryParseFd, VoidRequest},
};

cfg_async! {
    use core::task::{Context, Poll};
}

/// An interface to the X11 server.
pub trait DisplayBase {
    /// Get the `Setup` associated with this display.
    fn setup(&self) -> &Setup;

    /// Get the screen associated with this display.
    fn default_screen_index(&self) -> usize;

    /// Poll to see if a reply matching the sequence number has been received.
    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>>;

    /// Poll to see if we have received an event.
    fn poll_for_event(&mut self) -> Result<Option<Event>>;
}

/// A blocking interface to the X11 server.
pub trait Display: DisplayBase {
    /// Send a raw request to the X11 server.
    fn send_request_raw(&mut self, req: RawRequest<'_>) -> Result<u64>;

    /// Wait for a reply from the X11 server.
    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply>;

    /// Wait for an event.
    fn wait_for_event(&mut self) -> Result<Event>;

    /// Get the maximum request length that can be sent.
    ///
    /// Returned in units of 4 bytes.
    fn maximum_request_length(&mut self) -> Result<usize>;

    /// Synchronize this display with the server.
    fn synchronize(&mut self) -> Result<()>;

    /// Flush all pending requests to the server.
    fn flush(&mut self) -> Result<()>;
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(not(feature = "async"), doc(hidden))]
pub enum AsyncStatus<T> {
    /// The function's result is ready.
    Ready(T),
    /// We are waiting for a read to be available.
    Read,
    /// We are waiting for a write to be available.
    Write,
    /// We are waiting for something else.
    ///
    /// The implementation is expected to wake the provided waker
    /// once the function is ready to be called again.
    UserControlled,
}

impl<T> AsyncStatus<T> {
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }

    pub fn map<R>(self, f: impl FnOnce(T) -> R) -> AsyncStatus<R> {
        match self {
            Self::Ready(t) => AsyncStatus::Ready(f(t)),
            Self::Read => AsyncStatus::Read,
            Self::Write => AsyncStatus::Write,
            Self::UserControlled => AsyncStatus::UserControlled,
        }
    }

    #[cfg(feature = "async")]
    pub fn interest(&self) -> Option<Interest> {
        match self {
            Self::Read => Some(Interest::Readable),
            Self::Write => Some(Interest::Writable),
            _ => None,
        }
    }
}

cfg_async! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Interest {
        Readable,
        Writable,
    }

    pub trait CanBeAsyncDisplay: DisplayBase {
        fn format_request(
            &mut self,
            req: &mut RawRequest<'_>,
            ctx: &mut Context<'_>
        ) -> Result<AsyncStatus<()>>;

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest<'_>,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>>;

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>>;

        fn try_wait_for_event_raw(
            &mut self,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<Event>>;
    }

    /// A non-blocking interface to the X11 server.
    pub trait AsyncDisplay: DisplayBase {
        /// Poll to see if this display is able to participate in the given interest.
        fn poll_interest(&mut self, interest: Interest, cx: &mut Context<'_>) -> Poll<Result<()>>;
    }
}
