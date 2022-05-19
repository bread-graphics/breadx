// MIT/Apache2 License

//! Displays, used to connect to the X11 server.
//!
//! Communication with the X11 server takes place by sending or receiving
//! requests through the [`Display`] apparatus. These [`Display`]s, which
//! are usually some kind of state coupled with an I/O [`Connection`], are
//! responsible for sending and receiving the data, as well as serializing
//! and deserializing the requests and replies.
//!
//! ## Traits
//!
//! There are three primary traits in this module that define the behavior of
//! these apparati:
//!
//! - [`DisplayBase`] provides the basic functionality that either does not
//!   require I/O, or where blocking is not a consideration. This includes
//!   accessing the [`Setup`] and polling for replies or events.
//! - [`Display`] is a blocking interface that can be used to send requests,
//!   receive replies, and receive events.
//! - [`AsyncDisplay`] (requires the `async` feature) is a non-blocking
//!   version of [`Display`].
//!
//! Most user functionality is implemented in extension traits like
//! [`DisplayFunctionsExt`], and wraps around the methods defined in the
//! above traits.
//!
//! ## Implementations
//!
//! In addition, this module provides implementations of these traits that
//! wrap [`Connection`] implementations, written in pure Rust. All of these
//! types implement [`DisplayBase`] and [`Display`]. If wrapped in the
//! appropriate runtime primitives, they may also implement [`AsyncDisplay`].
//!
//! - [`BasicDisplay`] is the main implementation of [`Display`]. It is a
//!   reasonable default for most applications. The main downside is that it
//!   requires mutable (`&mut`) access.
//! - [`CellDisplay`] allows for concurrent access, but is not thread safe.
//! - [`SyncDisplay`] (requires the `sync_display` feature) allows for
//!   thread-safe concurrent access, but at a minor performance cost.
//!
//! [`Display`]: crate::display::Display
//! [`Connection`]: crate::connection::Connection
//! [`DisplayBase`]: crate::display::DisplayBase
//! [`Setup`]: crate::protocol::xproto::Setup
//! [`AsyncDisplay`]: crate::display::AsyncDisplay
//! [`DisplayFunctionsExt`]: crate::display::DisplayFunctionsExt
//! [`BasicDisplay`]: crate::display::BasicDisplay
//! [`CellDisplay`]: crate::display::CellDisplay
//! [`SyncDisplay`]: crate::display::SyncDisplay

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
use alloc::boxed::Box;
pub use basic::{BasicDisplay, DisplayConnection};
use core::task::{Context, Poll};

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

pub use crate::automatically_generated::DisplayFunctionsExt;
cfg_async! {
    pub use crate::automatically_generated::AsyncDisplayFunctionsExt;
}

use crate::Result;
use x11rb_protocol::protocol::{
    xproto::{GetInputFocusRequest, Screen, Setup},
    Event,
};

/// An interface to the X11 server, containing functionality that is neither
/// inherently synchronous nor asynchronous.
///
/// This trait is the supertrait for both [`Display`] and [`AsyncDisplay`].
/// It only contains functions for retrieving the setup information and
/// for polling the server in a non-blocking manner.
///
/// These functions should be used if, for instance, you want to fetch an
/// event, but if it is not available you need to do something else instead.
/// In addition, `Setup` information is useful for determining screen
/// parameters, among other things.
///
/// [`Display`]: crate::display::Display
/// [`AsyncDisplay`]: crate::display::AsyncDisplay
pub trait DisplayBase {
    /// Get the [`Setup`] associated with this display.
    ///
    /// The [`Setup`] contains a variety of information that is useful
    /// for determining aspects of the currently available environment.
    /// This includes screen size, the maximum length of a request,
    /// keycodes, and more.
    ///
    /// [`Setup`]: crate::protocol::xproto::Setup
    fn setup(&self) -> &Setup;

    /// Get the screen associated with this display.
    ///
    /// At startup time, the default screen to use in the process is
    /// determined by the user. This is the index of that screen.
    fn default_screen_index(&self) -> usize;

    /// Poll to see if a reply matching the sequence number has been received.
    ///
    /// This checks the reply map to see if a reply has been received for the
    /// given sequence number. If it has, it returns the reply. Otherwise, it
    /// tries to fetch it from the server.
    ///
    /// # Blocking
    ///
    /// This function should never block. Non-blocking I/O should be used to
    /// fetch the reply, and `None` should be returned if the call would
    /// block.
    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>>;

    /// Poll to see if we have received an event.
    ///
    /// This checks the event queue to see if an event has been received. If
    /// it has, it returns the event. Otherwise, it tries to fetch it from
    /// the server.
    ///
    /// # Blocking
    ///
    /// This function should never block. Non-blocking I/O should be used to
    /// fetch the event, and `None` should be returned if the call would
    /// block.
    fn poll_for_event(&mut self) -> Result<Option<Event>>;

    /* Helper methods */

    /// Get the screens for this display.
    ///
    /// This is equivalent to `self.setup().roots`, but is marginally
    /// more convenient.
    fn screens(&self) -> &[Screen] {
        &self.setup().roots
    }

    /// Get the default screen for this display.
    ///
    /// This is equivalent to `self.setup().roots[self.default_screen_index()]`.
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
///
/// This interface can be used to do everything that [`DisplayBase`]
/// can do, but it is also capable of blocking on I/O. This allows it
/// to send requests, receive replies/events, among other things.
///
/// Objects of this type should never be put into non-blocking mode.
///
/// [`DisplayBase`]: crate::display::DisplayBase
pub trait Display: DisplayBase {
    /// Send a raw request to the X11 server.
    ///
    /// This function formats `req`, sends it to the server, and returns
    /// the sequence number associated with that request.
    ///
    /// # Blocking
    ///
    /// This function should block until the request has been sent to the
    /// server.
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64>;

    /// Wait for a reply from the X11 server.
    ///
    /// This function waits for a reply to the given sequence number.
    ///
    /// Note that if the request has no reply, this function will likely
    /// hang forever.
    ///
    /// # Blocking
    ///
    /// This function should block until a reply is received from the
    /// server.
    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply>;

    /// Wait for an event.
    ///
    /// This function waits for an event to be received from the server.
    ///
    /// # Blocking
    ///
    /// This function should block until an event is received from the
    /// server.
    fn wait_for_event(&mut self) -> Result<Event>;

    /// Get the maximum request length that can be sent.
    ///
    /// Returned in units of 4 bytes.
    ///
    /// # Blocking
    ///
    /// The server may be using the Big Requests extension, which allows
    /// for requests to be larger than the default maximum request length.
    /// If so, this function will block until the server has updated its
    /// maximum request length.
    fn maximum_request_length(&mut self) -> Result<usize>;

    /// Get a unique ID valid for use by the server.
    ///
    /// This function returns a unique ID that can be used by the server
    /// to identify the client's resources.
    ///
    /// # Blocking
    ///
    /// If the client has exhausted its XID range, this function will
    /// block until the server has repopulated it.
    fn generate_xid(&mut self) -> Result<u32>;

    /// Synchronize this display with the server.
    ///
    /// By default, this function sends and receives a low-cost
    /// request. This is useful for ensuring that the server has
    /// "caught up" with the client.
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
    ///
    /// # Blocking
    ///
    /// This function should block until all pending requests have been
    /// sent to the server.
    fn flush(&mut self) -> Result<()>;
}

/// The status of an async function.
///
/// An async function can either be ready, or be unready for a number of
/// reasons. This enum is used to represent the status of an async
/// function.
///
/// It can be thought of as similar to [`Poll`], but instead of returning
/// `Pending`, it returns the exact reason why it is pending.
///
/// [`Poll`]: std::task::Poll
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
    /// Returns `true` if the `AsyncStatus` is `Ready`.
    pub fn is_ready(&self) -> bool {
        matches!(self, Self::Ready(_))
    }

    /// Maps the `AsyncStatus` from one type to another, if the
    /// `AsyncStatus` is `Ready`.
    pub fn map<R>(self, f: impl FnOnce(T) -> R) -> AsyncStatus<R> {
        match self {
            Self::Ready(t) => AsyncStatus::Ready(f(t)),
            Self::Read => AsyncStatus::Read,
            Self::Write => AsyncStatus::Write,
            Self::UserControlled => AsyncStatus::UserControlled,
        }
    }

    /// Unwrap the `AsyncStatus`, returning the inner value or
    /// panicking otherwise.
    /// 
    /// # Panics
    /// 
    /// Panics if the `AsyncStatus` is not `Ready`.
    pub fn unwrap(self) -> T {
        match self {
            Self::Ready(t) => t,
            _ => panic!("unwrap() called on non-ready AsyncStatus"),
        }
    }

    /// Convert the `AsyncStatus` into an `Option`.
    pub fn ready(self) -> Option<T> {
        match self {
            Self::Ready(t) => Some(t),
            _ => None,
        }
    }
}

impl<T: Copy> AsyncStatus<&T> {
    /// Copy the reference in an `AsyncStatus` to a value.
    #[must_use]
    pub fn copied(self) -> AsyncStatus<T> {
        self.map(|&t| t)
    }
}

cfg_async! {
    /// The interest to poll the runtime for.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Interest {
        /// We are waiting for the stream to be readable.
        Readable,
        /// We are waiting for the stream to be writable.
        Writable,
    }

    /// A display that *can* be async, if wrapped in an async runtime.
    ///
    /// This trait exposes functions similar to those in [`Display`], but
    /// instead of blocking and returning a value, they return an
    /// [`AsyncStatus`] if the value is available.
    ///
    /// Async runtime hooks should use this trait as the underlying object
    /// for [`AsyncDisplay`] implementations. Objects of this trait can't
    /// be used in the normal way, since an async runtime is needed to
    /// drive the I/O.
    ///
    /// [`Display`]: crate::display::Display
    /// [`AsyncDisplay`]: crate::display::AsyncDisplay
    /// [`AsyncStatus`]: crate::display::AsyncStatus
    pub trait CanBeAsyncDisplay: DisplayBase {
        /// Partially format the request.
        ///
        /// This method tries to format the request in such a way that
        /// it can be passed to [`try_send_request_raw`]. If additional
        /// information is needed to complete the request (e.g. extension
        /// info or bigreq), this method is allowed to return a non-ready
        /// status.
        ///
        /// This method returns the sequence number of the request, which
        /// can be used to wait for the reply.
        fn format_request(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>>;

        /// Partially send the request.
        ///
        /// This function actually sends the request to the server. Note
        /// that the request has to be formatted using [`format_request`]
        /// before this method is called.
        ///
        /// # Cancel Safety
        ///
        /// This function does not have to be cancel safe. In fact, it is
        /// likely impossible to make this cancel safe, since sending a
        /// fragment of a request will corrupt the X11 server.
        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>>;

        /// Wait for the reply.
        ///
        /// This function waits for the reply to the request with the
        /// given sequence number. If the reply is not yet available,
        /// this function returns a non-ready status.
        ///
        /// # Cancel Safety
        ///
        /// This function should be cancel safe. It should read into some
        /// kind of buffer, and then return a ready status if the buffer
        /// is completed.
        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>>;

        /// Wait for an event.
        ///
        /// This function waits for an event to be available. If an event
        /// is not available, this function returns a non-ready status.
        ///
        /// # Cancel Safety
        ///
        /// This function should be cancel safe. It should read into some
        /// kind of buffer, and then return a ready status if the buffer
        /// is completed.
        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>>;

        /// Flush the output buffer.
        ///
        /// This function flushes the output buffer. If the output buffer
        /// cannot be emptied, this function returns a non-ready status.
        ///
        /// # Cancel Safety
        ///
        /// This function doesn't have to be cancel safe. It is likely
        /// impossible to make this cancel safe, since flushing the
        /// output buffer only partially will corrupt the X11 server.
        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>>;

        /// Generate a unique XID.
        ///
        /// This function generates a unique XID. It may involve
        /// `try_send_request_raw`, so it has to be partial.
        ///
        /// # Cancel Safety
        ///
        /// This function doesn't have to be cancel safe.
        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>>;

        /// Get the maximum length of a request that can be sent.
        ///
        /// This function returns the maximum length of a request that
        /// can be sent.
        ///
        /// # Cancel Safety
        ///
        /// This function doesn't have to be cancel safe.
        fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>>;
    }

    /// A non-blocking interface to the X11 server.
    ///
    /// This is an asynchronous version of the [`Display`] trait. It can
    /// do everything that a [`Display`] can do, but it doesn't
    /// block and return values. Instead, it returns a [`Future`] that can
    /// be used to wait for the result.
    ///
    /// [`Display`]: crate::display::Display
    /// [`Future`]: std::future::Future
    pub trait AsyncDisplay: CanBeAsyncDisplay {
        /// Poll for an interest. If the interest if ready, call
        /// the provided callback.
        ///
        /// This function is used to drive I/O. It should be called
        /// whenever the runtime is ready to wait on I/O, and should be
        /// implemented as follows:
        ///
        /// - Poll for the specific interest.
        /// - If the interest is ready, call the given callback.
        /// - Otherwise, return `Pending`.
        fn poll_for_interest(
            &mut self,
            interest: Interest,
            callback: &mut dyn FnMut(&mut Self, &mut Context<'_>) -> Result<()>,
            ctx: &mut Context<'_>,
        ) -> Poll<Result<()>>;
    }
}

/* Mut impls */

impl<D: DisplayBase + ?Sized> DisplayBase for &mut D {
    fn setup(&self) -> &Setup {
        (**self).setup()
    }

    fn default_screen_index(&self) -> usize {
        (**self).default_screen_index()
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        (**self).poll_for_event()
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        (**self).poll_for_reply_raw(seq)
    }
}

impl<D: Display + ?Sized> Display for &mut D {
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64> {
        (**self).send_request_raw(req)
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        (**self).wait_for_event()
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        (**self).wait_for_reply_raw(seq)
    }

    fn flush(&mut self) -> Result<()> {
        (**self).flush()
    }

    fn synchronize(&mut self) -> Result<()> {
        (**self).synchronize()
    }

    fn generate_xid(&mut self) -> Result<u32> {
        (**self).generate_xid()
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        (**self).maximum_request_length()
    }
}

cfg_async! {
    impl<D: CanBeAsyncDisplay + ?Sized> CanBeAsyncDisplay for &mut D {
        fn format_request(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>> {
            (**self).format_request(req, ctx)
        }

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            (**self).try_send_request_raw(req, ctx)
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
            (**self).try_wait_for_event(ctx)
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>> {
            (**self).try_wait_for_reply_raw(seq, ctx)
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
            (**self).try_flush(ctx)
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
            (**self).try_generate_xid(ctx)
        }

        fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
            (**self).try_maximum_request_length(ctx)
        }
    }
}

/* Box impls */

impl<D: DisplayBase + ?Sized> DisplayBase for Box<D> {
    fn setup(&self) -> &Setup {
        (**self).setup()
    }

    fn default_screen_index(&self) -> usize {
        (**self).default_screen_index()
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        (**self).poll_for_event()
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        (**self).poll_for_reply_raw(seq)
    }
}

impl<D: Display + ?Sized> Display for Box<D> {
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64> {
        (**self).send_request_raw(req)
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        (**self).maximum_request_length()
    }

    fn flush(&mut self) -> Result<()> {
        (**self).flush()
    }

    fn generate_xid(&mut self) -> Result<u32> {
        (**self).generate_xid()
    }

    fn synchronize(&mut self) -> Result<()> {
        (**self).synchronize()
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        (**self).wait_for_event()
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        (**self).wait_for_reply_raw(seq)
    }
}

cfg_async! {
    impl<D: CanBeAsyncDisplay + ?Sized> CanBeAsyncDisplay for Box<D> {
        fn format_request(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>> {
            (**self).format_request(req, ctx)
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
            (**self).try_flush(ctx)
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
            (**self).try_generate_xid(ctx)
        }

        fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
            (**self).try_maximum_request_length(ctx)
        }

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            (**self).try_send_request_raw(req, ctx)
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
            (**self).try_wait_for_event(ctx)
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>> {
            (**self).try_wait_for_reply_raw(seq, ctx)
        }
    }
}
