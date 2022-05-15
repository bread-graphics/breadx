// MIT/Apache2 License

//! Displays, used to connect to the X11 server.

/// `try!()`, but for `Result<AsyncStatus<_>>`.
#[doc(hidden)]
#[macro_export]
macro_rules! mtry {
    ($expr: expr) => {
        match ($expr)? {
            AsyncStatus::Ready(t) => t,
            status => return Ok(status.map(|_| unreachable!())),
        }
    };
}

mod basic;
pub use basic::{BasicDisplay, DisplayConnection};

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

mod prefetch;
pub(crate) use prefetch::Prefetch;

mod raw_request;
pub use raw_request::{RawReply, RawRequest};

cfg_sync! {
    mod sync;
    pub use sync::SyncDisplay;
}

pub use crate::automatically_generated::DisplayFunctionsExt;
cfg_async! {
    pub use crate::automatically_generated::AsyncDisplayFunctionsExt;
}

use crate::Result;
use x11rb_protocol::protocol::{
    xproto::{GetInputFocusRequest, Screen, Setup},
    Event,
};

cfg_async! {
    use crate::{Error, futures};
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

    /* Helper methods */

    /// Get the screens for this display.
    fn screens(&self) -> &[Screen] {
        &self.setup().roots
    }

    /// Get the default screen for this display.
    fn default_screen(&self) -> &Screen {
        self.screens()
            .get(self.default_screen_index())
            .unwrap_or_else(|| {
                panic!(
                    "Default screen index {} is not a valid screen",
                    self.default_screen_index()
                )
            })
    }
}

/// A blocking interface to the X11 server.
pub trait Display: DisplayBase {
    /// Send a raw request to the X11 server.
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64>;

    /// Wait for a reply from the X11 server.
    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply>;

    /// Wait for an event.
    fn wait_for_event(&mut self) -> Result<Event>;

    /// Get the maximum request length that can be sent.
    ///
    /// Returned in units of 4 bytes.
    fn maximum_request_length(&mut self) -> Result<usize>;

    /// Get a unique ID valid for use by the server.
    fn generate_xid(&mut self) -> Result<u32>;

    /// Synchronize this display with the server.
    fn synchronize(&mut self) -> Result<()> {
        let span = tracing::info_span!("synchronize");
        let _enter = span.enter();

        // send the sync request
        let get_input_focus = GetInputFocusRequest {};
        let req = RawRequest::from_request_reply(get_input_focus);
        let seq = self.send_request_raw(req)?;

        // flush the stream
        self.flush()?;

        // wait for the reply
        self.wait_for_reply_raw(seq).map(|_| ())
    }

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
}

impl<T> AsyncStatus<&T> {
    pub fn copied(self) -> AsyncStatus<T> {
        self.map(|&t| t)
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
            req: &mut RawRequest,
            ctx: &mut Context<'_>
        ) -> Result<AsyncStatus<u64>>;

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>>;

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>>;

        fn try_wait_for_event(
            &mut self,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<Event>>;

        fn try_flush(
            &mut self,
            ctx: &mut Context<'_>
        ) -> Result<AsyncStatus<()>>;

        fn try_generate_xid(
            &mut self,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u32>>;

        fn try_maximum_request_length(
            &mut self,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<usize>>;
    }

    /// A non-blocking interface to the X11 server.
    pub trait AsyncDisplay: CanBeAsyncDisplay {
        /// Poll for an interest. If the interest if ready, call
        /// the provided callback.
        fn poll_for_interest(
            &mut self,
            interest: Interest,
            callback: &mut dyn FnMut(&mut Self, &mut Context<'_>) -> Result<()>,
            ctx: &mut Context<'_>,
        ) -> Poll<Result<()>>;
    }
}
