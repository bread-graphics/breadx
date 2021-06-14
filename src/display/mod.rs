// MIT/Apache2 License

//! This module provides the bulk of the utilities used to communicate with the X11 server.
//!
//! Although one can just connect to the X server using your average `TcpStream` or `UnixStream` and communicate
//! that way, it tends to be very inconvenient to deal in terms of bytes and not the actual types. This module
//! aims to provide types to simplify dealings with the X server.
//!
//! Note that this module is unpacked into the crate root, so that any item in this module can also be used from
//! the crate directly.
//!
//! # `DisplayBase`, `Display` and `AsyncDisplay`
//!
//! * The [`DisplayBase`] trait is implemented for objects that communicate with the X server. It provides non-IO
//!   functionality like keeping track of pending items and setup information.
//! * The [`Display`] trait is implemented for objects that communicate with the X server in a blocking manner.
//!   It provides functions like `wait_for_event` and `send_request`, although you're more likely to use
//!   functions on its extension traits.
//! * The [`AsyncDisplay`] trait is implemented for objects that communication with the X server in a
//!   non-blocking manner. It is similar to the `Display` trait and tries to mirror its functionality.
//!
//! # `Display` Implementors
//!
//! By default, this module provides the [`BasicDisplay`] and [`CellDisplay`] types, which allow for mutable
//! and immutable access, respectively. Both implement `DisplayBase`, `Display`, and `AsyncDisplay`. If you
//! enable the `sync-display` feature, you will also gain access to the [`SyncDisplay`] type, which allows for
//! immutable and thread-safe usage.
//!
//! # `Connection`, `AsyncConnection`, and `NameConnection`
//!
//! The [`Connection`] and [`AsyncConnection`] traits are implemented for the raw connection types that the
//! `Display` implementors wrap themselves around, like `TcpStream` and `UnixStream`. However, you are most
//! likely to use the [`NameConnection`](name::NameConnection), since it represents a connection to the X server
//! resolved from the server's name.

use crate::{
    auto::{
        xproto::{Colormap, GetInputFocusRequest, Screen, Setup, Visualid, Visualtype, Window},
        AsByteSequence,
    },
    error::BreadError,
    event::Event,
    util::expand_or_truncate_to_length,
    Fd, Request, XID,
};
use alloc::{boxed::Box, vec::Vec};
use core::{fmt, iter, marker::PhantomData, mem, num::NonZeroU32};
use tinyvec::TinyVec;

#[cfg(feature = "async")]
use crate::xid::XidType;
#[cfg(feature = "async")]
use core::task::{Context, Poll};

mod basic;
pub(crate) mod bigreq;
mod cell;
mod connection;

pub mod traits;
// "traits" contains some important types.
pub use traits::{rgb, GcParameters, KeyboardMapping, WindowParameters};

pub use basic::*;
pub use cell::*;
pub use connection::*;

#[cfg(feature = "async")]
pub(crate) mod futures;
#[cfg(feature = "async")]
pub use futures::*;

#[cfg(feature = "sync-display")]
mod sync;
#[cfg(feature = "sync-display")]
pub use sync::*;

pub(crate) mod input;
pub(crate) mod output;

#[cfg(feature = "async")]
pub(crate) mod common;

#[cfg(feature = "std")]
pub mod name;

/// A set of traits to import to enable functionality.
pub mod prelude {
    pub use super::traits::*;
    #[cfg(feature = "async")]
    pub use super::{AsyncDisplay, AsyncDisplayExt};
    pub use super::{Display, DisplayBase, DisplayExt};
}

/// A `Setup` where all of its lists are guaranteed to live forever (i.e. it owns all of its lists).
pub type StaticSetup = Setup<'static, 'static, 'static, 'static, 'static>;
/// A `Screen` where all of its lists are guaranteed to live forever (i.e. it owns all of its lists).
pub type StaticScreen = Screen<'static, 'static>;

pub(crate) const EXT_KEY_SIZE: usize = 24;

/// This trait represents a connection to the X11 server. Most operations in `breadx` revolve around an object
/// implementing this trait in some way, shape, or form.
///
/// Internally, this should act as a layer of abstraction over the inner `Connection` object that keeps track
/// of the setup, outgoing and pending requests and replies, the event queue, et cetera. Orthodoxically,
/// X11 usually takes place over a TCP stream or a Unix socket connection.
///
/// Upon its instantiation, the `DisplayBase` should send bytes to the server requesting the setup information,
/// and then stores it for later use. This can be done via the `establish` convenience method provided by the
/// `Connection` and `AsyncConnection` traits. Afterwards, it awaits commands from the programmer to send
/// requests, receive replies or process events.
///
/// Objects implementing this trait should further implement `Display` and/or `AsyncDisplay`.
///
/// # Usage
///
/// When writing a function that takes an object implementing a `DisplayBase` as a parameter, consider rewriting
/// it to the form:
///
/// ```rust,ignore
/// fn my_function<D: DisplayBase>(display: &mut D) { /* ... */ }
/// ```
///
/// This allows you to change to another type of display easily without needing to go back and change the
/// signature of all of your functions.
pub trait DisplayBase {
    /// Get the `Setup` object that defines the X11 connection.
    ///
    /// The `Setup` object provides a litany of information regarding the X11 server, such as the server's
    /// vendor, the visual formats, and the roots of the `Screen` windows. The implementor should store a
    /// `Setup` somewhere in the object, and return a reference to it for this function.
    fn setup(&self) -> &StaticSetup;

    /// Get the default screen index.
    ///
    /// The `Setup` contains a list of "roots", and this object serves as an index into this array to determine
    /// which one to use as a default screen. This is usually provided by either the user or the name string.
    fn default_screen_index(&self) -> usize;

    /// Generate the next request number to be used to define a request.
    ///
    /// The X server provides each request with a 64-bit number in order to identify its replies. This number
    /// starts at 1 and counts up for each request, eventually wrapping back to 0 at the end. The implementor
    /// should keep track of this running count.
    fn next_request_number(&mut self) -> u64;

    /// Generate an XID within appropriate bounds. Returns `None` if our XIDs are exhausted.
    ///
    /// The `Setup` contains information useful for building an XID generator. The implementation should build an
    /// appropriate XID generator and then use it in order to
    fn generate_xid(&mut self) -> Option<XID>;

    /// Add a `PendingItem` to this display's map.
    ///
    /// The implementation is expected to keep a map matching `PendingItem`s to their request sequences. This
    /// method inserts into that map.
    fn add_pending_item(&mut self, req_id: u16, item: PendingItem);

    /// Clone a `PendingItem` from this display's map and return it. See `add_pending_item` for more
    /// information.
    fn get_pending_item(&mut self, req_id: u16) -> Option<PendingItem>;

    /// Remove a `PendingItem` from this display's map. See `add_pending_item` for more information.
    fn take_pending_item(&mut self, req_id: u16) -> Option<PendingItem>;

    /// Push an event into this display's event queue.
    ///
    /// The display is expected to keep a queue of events that it receives whenever it runs `wait` or
    /// `poll_wait`. This is provided so the implementations for `wait` and `poll_wait` can add events to this
    /// internal queue.
    fn push_event(&mut self, event: Event);

    /// Pop an event from this display's event queue. See `push_event` for more information.
    fn pop_event(&mut self) -> Option<Event>;

    /// Create a new special event queue.
    ///
    /// Some extensions, like `present`, may want to keep a different queue of events than the general event
    /// queue. To do this, it tells the server to tag the events with some XID, and then uses that XID to create
    /// the special event queue. Then, events tagged with this specific XID are sorted into the special event
    /// queue instead of the general one.
    fn create_special_event_queue(&mut self, xid: XID);

    /// Push an event into the special event queue. See `create_special_event_queue` for more information.
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event>;

    /// Pop an event from the special event queue. See `create_special_event_queue` for more information.
    fn pop_special_event(&mut self, xid: XID) -> Option<Event>;

    /// Delete a special event queue. See `create_special_event_queue` for more information.
    fn delete_special_event_queue(&mut self, xid: XID);

    /// Whether or not zero-length replies are checked.
    ///
    /// When handling requests that do not have a reply (i.e. zero-length replies), there are two approaches. You
    /// can either simply ignore the request and return a defaulted reply (unchecked mode), or send a small
    /// request and poll the server until the request is replied to, in order to ensure that the server has
    /// already processed the original request (checked mode). Although unchecked mode is significantly faster,
    /// checked mode allows for more accurate error diagnostics.
    fn checked(&self) -> bool;

    /// Set whether or not zero-length replies are checked. See `checked` for more information.
    fn set_checked(&mut self, checked: bool);

    /// Whether or not this display uses the `bigreq` extension, whereas requests consisting of over
    /// 262140 bytes are allowed to be sent over the connection.
    fn bigreq_enabled(&self) -> bool;

    /// The current maximum request length. This is the maximum number of bytes the server can handle at a time.
    fn max_request_len(&self) -> usize;

    /// Get the opcode for an extension.
    ///
    /// The implementation is expected to keep a map of extension names to the opcodes of said extensions, as
    /// revealed by the server.
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8>;

    /// Set the opcode for an extension. See `get_extension_opcode` for more information.
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8);

    /// Get the `WM_PROTOCOLS` atom, which we cache in the display.
    ///
    /// `WM_PROTOCOLS` is used often by the display, so we cache it in order to ensure we don't have to request
    /// it more than once.
    fn wm_protocols_atom(&self) -> Option<NonZeroU32>;

    /// Set the `WM_PROTOCOLS` atom. See `wm_protocols_atom` for more information.
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32);

    // -- Item-based functions.

    /// Insert a pending request into this display. This simply wraps the `PendingRequest` into a `PendingItem`
    /// and then calls `add_pending_item`.
    #[inline]
    fn add_pending_request(&mut self, req_id: u16, pereq: PendingRequest) {
        self.add_pending_item(req_id, PendingItem::Request(pereq));
    }

    /// Get a pending request from this display. This calls `get_pending_item` and returns `None` if the
    /// object is not a pending request.
    #[inline]
    fn get_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        self.get_pending_item(req_id).and_then(PendingItem::request)
    }

    /// Take a pending request from this display. This calls `take_pending_item`, inserting the object back into
    /// the map if it is not a request.
    #[inline]
    fn take_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        match self.take_pending_item(req_id) {
            Some(PendingItem::Request(req)) => Some(req),
            Some(other) => {
                self.add_pending_item(req_id, other);
                None
            }
            None => None,
        }
    }

    /// Insert a pending reply into this display. This simply wraps the `PendingRequest` into a `PendingItem`
    /// and then calls `add_pending_item`.
    #[inline]
    fn add_pending_reply(&mut self, req_id: u16, perep: PendingReply) {
        self.add_pending_item(req_id, PendingItem::Reply(perep));
    }

    /// Take a pending reply from this display. This calls `take_pending_item`, inserting the object back into
    /// the map if it is not a reply.
    #[inline]
    fn take_pending_reply(&mut self, req_id: u16) -> Option<PendingReply> {
        match self.take_pending_item(req_id) {
            Some(PendingItem::Reply(repl)) => Some(repl),
            Some(other) => {
                self.add_pending_item(req_id, other);
                None
            }
            None => None,
        }
    }

    /// Insert a pending error into this display. This simply wraps the `BreadError` into a `PendingItem`
    /// and then calls `add_pending_item`.
    #[inline]
    fn add_pending_error(&mut self, req_id: u16, err: BreadError) {
        self.add_pending_item(req_id, PendingItem::Error(err));
    }

    /// Check this display for the pending error. This calls `take_pending_item` and returns `Ok` if the object
    /// if not an error.
    #[inline]
    fn check_for_pending_error(&mut self, req_id: u16) -> crate::Result {
        match self.take_pending_item(req_id) {
            Some(PendingItem::Error(err)) => Err(err),
            Some(other) => {
                self.add_pending_item(req_id, other);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    // -- Setup-based functions.

    /// Get the list of screens in this display.
    #[inline]
    fn screens(&self) -> &[StaticScreen] {
        &self.setup().roots
    }

    /// Get the default screen in this display.
    #[inline]
    fn default_screen(&self) -> &StaticScreen {
        &self.setup().roots[self.default_screen_index()]
    }

    /// Get the default root for this display.
    #[inline]
    fn default_root(&self) -> Window {
        self.default_screen().root
    }

    /// Get the default pixel used for the white color.
    #[inline]
    fn default_white_pixel(&self) -> u32 {
        self.default_screen().white_pixel
    }

    /// Get the default pixel used for the black color.
    #[inline]
    fn default_black_pixel(&self) -> u32 {
        self.default_screen().black_pixel
    }

    /// Get the default visual ID for the screen.
    #[inline]
    fn default_visual_id(&self) -> Visualid {
        self.default_screen().root_visual
    }

    /// Get the default visual for the default screen.
    #[inline]
    fn default_visual(&self) -> &Visualtype {
        self.visual_id_to_visual(self.default_visual_id()).unwrap()
    }

    /// Get the colormap for the default screen.
    #[inline]
    fn default_colormap(&self) -> Colormap {
        self.default_screen().default_colormap
    }

    /// Get a visual type from a visual ID.
    #[inline]
    fn visual_id_to_visual(&self, id: Visualid) -> Option<&Visualtype> {
        self.setup()
            .roots
            .iter()
            .flat_map(|s| s.allowed_depths.iter())
            .flat_map(|d| d.visuals.iter())
            .find(|v| v.visual_id == id)
    }

    /// Get the depth of the specified visual ID.
    #[inline]
    fn depth_of_visual(&self, id: Visualid) -> Option<u8> {
        self.setup()
            .roots
            .iter()
            .flat_map(|s| s.allowed_depths.iter())
            .find_map(|d| {
                if d.visuals.iter().any(|v| v.visual_id == id) {
                    Some(d.depth)
                } else {
                    None
                }
            })
    }
}

// So references to a Display can be used as a Display
impl<D: DisplayBase + ?Sized> DisplayBase for &mut D {
    #[inline]
    fn setup(&self) -> &StaticSetup {
        (**self).setup()
    }

    #[inline]
    fn default_screen_index(&self) -> usize {
        (**self).default_screen_index()
    }

    #[inline]
    fn next_request_number(&mut self) -> u64 {
        (**self).next_request_number()
    }

    #[inline]
    fn push_event(&mut self, event: Event) {
        (**self).push_event(event)
    }

    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        (**self).pop_event()
    }

    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        (**self).generate_xid()
    }

    #[inline]
    fn add_pending_item(&mut self, req_id: u16, item: PendingItem) {
        (**self).add_pending_item(req_id, item)
    }

    #[inline]
    fn get_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        (**self).get_pending_item(req_id)
    }

    #[inline]
    fn take_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        (**self).take_pending_item(req_id)
    }

    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        (**self).create_special_event_queue(xid)
    }

    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        (**self).push_special_event(xid, event)
    }

    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        (**self).pop_special_event(xid)
    }

    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        (**self).delete_special_event_queue(xid)
    }

    #[inline]
    fn checked(&self) -> bool {
        (**self).checked()
    }

    #[inline]
    fn set_checked(&mut self, checked: bool) {
        (**self).set_checked(checked)
    }

    #[inline]
    fn bigreq_enabled(&self) -> bool {
        (**self).bigreq_enabled()
    }

    #[inline]
    fn max_request_len(&self) -> usize {
        (**self).max_request_len()
    }

    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        (**self).get_extension_opcode(key)
    }

    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        (**self).set_extension_opcode(key, opcode)
    }

    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        (**self).wm_protocols_atom()
    }

    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        (**self).set_wm_protocols_atom(a)
    }
}

/// A wrapper around a synchronous connection to the X11 server.
///
/// There are two ways to send and receive data along a connection to the server: synchronously or
/// asynchronously. This type of display communicates synchronously with the server. Its function are assumed
/// to block the flow of execution while it is waiting for data to be sent to and received from the server.
///
/// See [`DisplayBase`] for more information on the function of a display object.
pub trait Display: DisplayBase {
    /// Wait for something to happen on the connection.
    ///
    /// This function waits for 32 or more bytes to be sent by the server, processes those bytes, and then
    /// appends the result of that processing to either the pending items map or one of the event queues. It can
    /// be assumed that, if `wait` returns without an error, either an event, an error, or a reply has been
    /// sent from the server and added to this display.
    ///
    /// # Errors
    ///
    /// If the server sends all zero bytes, the implementation should return `BreadError::ClosedConnection`,
    /// since this usually indicates that the connection has closed unless the client is out of sync with the
    /// server. If the server returns a reply that the client is not expecting because the corresponding request
    /// was never sent, the implementation should return `BreadError::NoMatchingRequest`. If an error is sent
    /// by the server and there is no corresponding pending request in the item map, that error should be
    /// converted into a `BreadError` and returned.
    ///
    /// In addition, system IO errors should be wrapped into a `BreadError` and returned.
    fn wait(&mut self) -> crate::Result;

    /// Send a request across the connection, given the monomorphized request info.
    ///
    /// This function sends the bytes and occasionally file descriptors contained in the given [`RequestInfo`]
    /// across the wrapped connection. The number of bytes should be a multiple of 4, and it should consist of a
    /// valid request that the X11 server can read.
    ///
    /// If the request belongs to an extension whose opcode has yet to be registered in the display's
    /// extension map, the display should first send and resolve a `QueryExtensionRequest` in order to determine
    /// which opcode the request needs.
    ///
    /// # Errors
    ///
    /// Since this function may potentially have to resolve a `QueryExtensionRequest`, it can return any error
    /// that `wait` is capable of returning. In addition, if the extension if not present, it will return
    /// `BreadError::ExtensionNotPresent`. Aside from querying for the extension, it may bubble up IO errors
    /// emitted by the internal system.
    fn send_request_raw(&mut self, request_info: RequestInfo) -> crate::Result<u16>;

    /// Synchronize this display, ensuring that all data sent across it has been replied to.
    ///
    /// It is occasionally useful to make sure the display has processed all of the information we have sent it.
    /// This is mostly done when running requests with zero-sized replies. This can be easily done by sending
    /// a small request (in this case, a `GetInputFocusRequest`) and waiting for it to be resolved. By resolving
    /// the request, the server acknowledges that it has also received and processed every request that came
    /// before it.
    ///
    /// # Errors
    ///
    /// This function calls `send_request_raw` and then `wait` in a loop. Therefore, it can return any error that
    /// `wait` and `send_request_raw` can.
    #[inline]
    fn synchronize(&mut self) -> crate::Result {
        log::debug!("Synchronizing display");
        let mut gifr = RequestInfo::from_request(
            GetInputFocusRequest::default(),
            self.bigreq_enabled(),
            self.max_request_len(),
        );
        gifr.discard_reply = true;
        let sequence = self.send_request_raw(gifr)?;
        // essentially a do/while loop
        while {
            // run wait() until the simple request we sent shows up in the replies
            self.wait()?;
            self.get_pending_request(sequence).is_some()
        } {}

        Ok(())
    }

    /// Resolve for a request, returning only the raw data of the reply. The default implementation assumes that
    /// the reply is not zero-sized.
    ///
    /// This function attempts to take the pending reply for the given request ID and, if the reply is not found,
    /// starts waiting in a loop. If the reply is zero-sized, this may be an infinite loop.
    ///
    /// # Errors
    ///
    /// This function can return any error that `wait` can.
    #[inline]
    fn resolve_request_raw(&mut self, req_id: u16) -> crate::Result<PendingReply> {
        loop {
            match self.take_pending_reply(req_id) {
                Some(p) => return Ok(p),
                None => self.wait()?,
            }
        }
    }

    /// Wait for an event to be sent from the X server.
    ///
    /// Similarly to `resolve_request_raw`, this calls `wait` in a loop until an event is sent by the server.
    ///
    /// # Errors
    ///
    /// This function can return any error that `wait` can.
    #[inline]
    fn wait_for_event(&mut self) -> crate::Result<Event> {
        loop {
            match self.pop_event() {
                Some(e) => return Ok(e),
                None => self.wait()?,
            }
        }
    }

    /// Wait for a special event to be sent from the X server. See `wait_for_event` for more information on how
    /// it functions.
    #[inline]
    fn wait_for_special_event(&mut self, xid: XID) -> crate::Result<Event> {
        loop {
            match self.pop_special_event(xid) {
                Some(e) => break Ok(e),
                None => self.wait()?,
            }
        }
    }
}

impl<D: Display + ?Sized> Display for &mut D {
    #[inline]
    fn wait(&mut self) -> crate::Result {
        (**self).wait()
    }

    #[inline]
    fn send_request_raw(&mut self, request_info: RequestInfo) -> crate::Result<u16> {
        (**self).send_request_raw(request_info)
    }
}

/// A wrapper around an asynchronous connection to the X server.
///
/// These kinds of `Display`s should be used in order to communicate with the X server asynchronously. The
/// functions in this trait and its extension traits closely mirror those of the [`Display`] trait.
///
/// See the [`DisplayBase`] trait for more information on what the display is expected to do.
#[cfg(feature = "async")]
pub trait AsyncDisplay: DisplayBase {
    /// Poll the current status of waiting for more input. There is no default implementation; the buffering
    /// strategy depends on the implementor. If this is called after a `Poll::Ready` is returned, it is assumed
    /// that the user wants to wait again.
    ///
    /// This function should rarely be called by the user directly; instead, use [`AsyncDisplayExt::wait_async`],
    /// or one of the functions that use it.
    fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result>;

    /// Begin sending a raw request to the server. In order to poll the status of this operation, use the
    /// `poll_send_request_raw` function.
    ///
    /// This function should rarely be called by the user directly; instead, use
    /// [`AsyncDisplayExt::send_request_raw_async`], or one of the functions that use it.
    fn begin_send_request_raw(
        &mut self,
        req: RequestInfo,
        cx: &mut Context<'_>,
    ) -> PollOr<(), RequestInfo>;

    /// Poll an ongoing raw request operation. It should return `Poll::Ready` once the bytes passed in from
    /// `begin_send_request_raw` have been sent.
    ///
    /// This function should rarely be called by the user directly; instead, use
    /// [`AsyncDisplayExt::send_request_raw_async`], or one of the functions that use it.
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>>;
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplay for &mut D {
    #[inline]
    fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result> {
        (**self).poll_wait(cx)
    }

    #[inline]
    fn begin_send_request_raw(
        &mut self,
        req: RequestInfo,
        cx: &mut Context<'_>,
    ) -> PollOr<(), RequestInfo> {
        (**self).begin_send_request_raw(req, cx)
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        (**self).poll_send_request_raw(cx)
    }
}

/// Monomorphized methods we can't put into the `Display` trait proper.
pub trait DisplayExt {
    /// Send a request to the server.
    ///
    /// This functions turns the given object into a `RequestInfo` and then calls `send_request_raw` with that
    /// `RequestInfo`. See [`Display::send_request_raw`] for more information. It returns a `RequestCookie`,
    /// which is a sequence number with type information attached. It can be "redeemed" in the
    /// `resolve_request` function to resolve for the reply.
    ///
    /// # Errors
    ///
    /// This function returns every error that `send_request_raw` can return.
    fn send_request<R: Request>(&mut self, request: R) -> crate::Result<RequestCookie<R>>;

    /// Resolve a request that we sent to the server.
    ///
    /// Given a request cookie returned by either `send_request` or `RequestCookie::from_sequence`, it will do
    /// one of three things:
    ///
    /// * If the reply is zero sized and the display is in unchecked mode, it discards the request cookie and
    ///   returns the default version of the reply.
    /// * If the reply is zero sized and the display is in checked mode, it synchronizes the display and checks
    ///   for errors before returning the default version of the reply.
    /// * If the reply is not zero sized, it calls `resolve_request_raw` with the sequence number, gets the
    ///   bytes for the reply, and converts it into the reply type.
    ///
    /// # Errors
    ///
    /// In addition to the errors `resolve_request_raw` can return, this function may also return
    /// `BreadError::BadObjectRead` if the object the server replies with does not match `R::Reply`'s layout.
    fn resolve_request<R: Request>(&mut self, token: RequestCookie<R>) -> crate::Result<R::Reply>
    where
        R::Reply: Default;

    /// Send a request to the server and immediately resolve for its reply. This is equivalent to calling
    /// `send_request` followd by `resolve_request`.
    #[inline]
    fn exchange_request<R: Request>(&mut self, request: R) -> crate::Result<R::Reply>
    where
        R::Reply: Default,
    {
        log::info!("Sending {} to server", core::any::type_name::<R>());
        let tok = self.send_request(request)?;
        log::info!("Resolving request...");
        self.resolve_request(tok)
    }
}

impl<D: Display + ?Sized> DisplayExt for D {
    #[inline]
    fn send_request<R: Request>(&mut self, request: R) -> crate::Result<RequestCookie<R>> {
        let r = RequestInfo::from_request(request, self.bigreq_enabled(), self.max_request_len());
        let req_id = self.send_request_raw(r)?;
        Ok(RequestCookie::from_sequence(req_id))
    }

    #[inline]
    fn resolve_request<R: Request>(&mut self, token: RequestCookie<R>) -> crate::Result<R::Reply>
    where
        R::Reply: Default,
    {
        if mem::size_of::<R::Reply>() == 0 {
            if self.checked() {
                self.synchronize()?;
                let seq = token.sequence();
                self.take_pending_request(seq);
                self.check_for_pending_error(seq)?;
            }

            return Ok(Default::default());
        }

        let PendingReply { data, fds } = self.resolve_request_raw(token.sequence())?;
        decode_reply::<R>(&data, fds)
    }
}

/// Monomorphized methods we can't put into the `AsyncDisplay` trait proper.
#[cfg(feature = "async")]
pub trait AsyncDisplayExt: AsyncDisplay {
    /// Wait until we recieve data.
    ///
    /// This future is essentially a wrapper around the `poll_wait` function, and is the asynchronous
    /// equivalent to the [`Display::wait`] function. See that function for more information on what this future
    /// is expected to return.
    fn wait_async(&mut self) -> WaitFuture<'_, Self>;

    /// Send a raw request to the server.
    ///
    /// This future is essentially a wrapper around the `begin_send_request_raw` and `poll_send_request_raw`
    /// functions, and is the asynchronous equivalent to the [`Display::send_request_raw`] function. See that
    /// function for more information on what it is expected to do and return.
    fn send_request_raw_async(&mut self, req: RequestInfo) -> SendRequestRawFuture<'_, Self>;

    /// Send a request to the server. This is the async equivalent of the [`DisplayExt::send_request`] function.
    /// See that function for more information on what this is expected to do.
    fn send_request_async<R: Request>(&mut self, request: R) -> SendRequestFuture<'_, Self, R>;

    /// Resolve a request that we sent to the server. This is the async equivalent of the
    /// [`DisplayExt::resolve_request`] function. See that function for more information on what this is expected
    /// to do.
    fn resolve_request_async<R: Request>(
        &mut self,
        token: RequestCookie<R>,
    ) -> ResolveRequestFuture<'_, Self, R>
    where
        R::Reply: Default;

    /// Synchronize this display so that every request that has been sent is resolved. This is the async
    /// equivalent of the [`Display::synchronize`] function. See that function for more information on what this
    /// is expected to do.
    fn synchronize_async(&mut self) -> SynchronizeFuture<'_, Self>;

    /// Resolve for a raw request. This is the async equivalent of the [`Display::resolve_request_raw`]
    /// function. See that function for more information on what this is expected to do.
    fn resolve_request_raw_async(&mut self, req_id: u16) -> ResolveRequestRawFuture<'_, Self>;

    /// Wait for an event to be sent from the X server. This is the async equivalent of the
    /// [`Display::wait_for_event`] function. See that function for more information on what this is expected
    /// to do.
    fn wait_for_event_async(&mut self) -> WaitForEventFuture<'_, Self>;

    /// Wait for a special event to be sent from the X server. This is the async equivalent of the
    /// [`Display::wait_for_special_event`] function. See that function for more information on what this is
    /// expected to do.
    fn wait_for_special_event_async(&mut self, xid: XID) -> WaitForSpecialEventFuture<'_, Self>;

    /// Send a request and wait for a reply back. This is the async equivalent of the
    /// [`DisplayExt::exchange_request`] function. See that function for more information on what this is
    /// expected to do.
    fn exchange_request_async<R: Request>(
        &mut self,
        request: R,
    ) -> ExchangeRequestFuture<'_, Self, R>;

    /// Send a no-op request to the server, but resolve for an XID. This is similar to `exchange_request_async`,
    /// but generates an XID before beginning the request send. This is largely a convenience wrapper.
    fn exchange_xid_async<R: Request, U: XidType + Unpin, F: FnOnce(U) -> R>(
        &mut self,
        to_request: F,
    ) -> ExchangeXidFuture<'_, Self, R, U, F>;
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplayExt for D {
    #[inline]
    fn wait_async(&mut self) -> WaitFuture<Self> {
        WaitFuture::run(self)
    }

    #[inline]
    fn send_request_raw_async(&mut self, req: RequestInfo) -> SendRequestRawFuture<'_, Self> {
        SendRequestRawFuture::run(self, req)
    }

    #[inline]
    fn send_request_async<R: Request>(&mut self, request: R) -> SendRequestFuture<'_, Self, R> {
        SendRequestFuture::run(self, request)
    }

    #[inline]
    fn resolve_request_async<R: Request>(
        &mut self,
        token: RequestCookie<R>,
    ) -> ResolveRequestFuture<'_, Self, R>
    where
        R::Reply: Default,
    {
        ResolveRequestFuture::run(self, token)
    }

    #[inline]
    fn synchronize_async(&mut self) -> SynchronizeFuture<'_, Self> {
        SynchronizeFuture::run(self)
    }

    #[inline]
    fn resolve_request_raw_async(&mut self, req_id: u16) -> ResolveRequestRawFuture<'_, Self> {
        ResolveRequestRawFuture::run(self, req_id)
    }

    #[inline]
    fn wait_for_event_async(&mut self) -> WaitForEventFuture<'_, Self> {
        WaitForEventFuture::run(self)
    }

    #[inline]
    fn wait_for_special_event_async(&mut self, xid: XID) -> WaitForSpecialEventFuture<'_, Self> {
        WaitForSpecialEventFuture::run(self, xid)
    }

    #[inline]
    fn exchange_request_async<R: Request>(
        &mut self,
        request: R,
    ) -> ExchangeRequestFuture<'_, Self, R> {
        ExchangeRequestFuture::run(self, request)
    }

    #[inline]
    fn exchange_xid_async<R: Request, U: XidType + Unpin, F: FnOnce(U) -> R>(
        &mut self,
        to_request: F,
    ) -> ExchangeXidFuture<'_, Self, R, U, F> {
        ExchangeXidFuture::run(self, to_request)
    }
}

/// Request information, monomorphized from the Request trait.
///
/// This object contains every bit of information needed to send a request to the server. Normally, this info is
/// contained in the `Request` trait. However, in order to pass that information into trait objects, it needs
/// to be "monomorphized" into this struct. In addition, this struct processes the request into bytes and
/// file descriptors.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequestInfo {
    pub(crate) data: TinyVec<[u8; 32]>,
    pub(crate) fds: Vec<Fd>,
    pub(crate) zero_sized_reply: bool,
    pub(crate) opcode: u8,
    pub(crate) extension: Option<&'static str>,
    pub(crate) expects_fds: bool,
    pub(crate) discard_reply: bool,
    pub(crate) sequence: Option<u16>,
}

impl RequestInfo {
    /// Generate a `RequestInfo` given a specific `Request` to generate from, as well as specs on how to convert
    /// the `RequestInfo` into bytes.
    ///
    /// # Panics
    ///
    /// This function panics if the request generates more bytes than `max_request_len`.
    #[inline]
    pub fn from_request<R: Request>(mut req: R, use_bigreq: bool, max_request_len: usize) -> Self {
        const SHORT_REQUEST_LIMIT: usize = (u16::MAX as usize) * 4;
        debug_assert!(use_bigreq || max_request_len <= SHORT_REQUEST_LIMIT);

        // TODO: somehow write using uninitialzied data
        let mut data = iter::repeat(0)
            .take(req.size())
            .collect::<TinyVec<[u8; 32]>>();
        let mut len = req.as_bytes(&mut data);

        // make sure it's aligned to a multiple of 4
        len = (len + 3) & (!0x03);
        expand_or_truncate_to_length(&mut data, len);

        // note: we assume max_request_len is already normalized
        assert!(
            max_request_len >= len,
            "Request's size was larger than the maximum request length"
        );

        // If we fit in the short request limit, third and fourth bytes need to be length
        let x_len = len / 4;
        log::trace!("xlen is {}", x_len);
        if use_bigreq {
            let length_bytes = ((x_len + 1) as u32).to_ne_bytes();
            // apply the length bytes to the end, and then rotate so it's at position 4
            data[2] = 0;
            data[3] = 0;
            data.extend_from_slice(&length_bytes);
            data[4..].rotate_right(length_bytes.len());
        } else {
            let len_bytes = (x_len as u16).to_ne_bytes();
            data[2] = len_bytes[0];
            data[3] = len_bytes[1];
        }

        RequestInfo {
            data,
            fds: match req.file_descriptors() {
                Some(fd) => mem::take(fd),
                None => Vec::new(),
            },
            zero_sized_reply: mem::size_of::<R::Reply>() == 0,
            opcode: R::OPCODE,
            extension: R::EXTENSION,
            expects_fds: R::REPLY_EXPECTS_FDS,
            discard_reply: false,
            sequence: None,
        }
    }

    /// Set the sequence number for this `RequestInfo`.
    #[inline]
    pub(crate) fn set_sequence(&mut self, seq: u16) {
        self.sequence = Some(seq);
    }
}

/// A reply, pending returning from the display. This contains the data of the reply, as well as any file
/// descriptors.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingReply {
    /// The raw data returned by the server.
    pub data: TinyVec<[u8; 32]>,
    /// The file descriptors returned by the server.
    pub fds: Box<[Fd]>,
}

/// A cookie for a request.
///
/// Requests usually take time to resolve into replies. Therefore, the `Display::send_request` method returns
/// the `RequestCookie`, which is later used to block (or await) for the request's eventual result.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Default, Eq, Hash)]
#[repr(transparent)]
pub struct RequestCookie<R: Request> {
    sequence: u16,
    _phantom: PhantomData<Option<R::Reply>>,
}

impl<R: Request> fmt::Debug for RequestCookie<R> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestCookie")
            .field("sequence", &self.sequence)
            .finish()
    }
}

impl<R: Request> RequestCookie<R> {
    /// Construct a `RequestCookie` from a sequence number. If the sequence number given if not associated with
    /// a reply of type `R::Reply`, it may cause errors down the line.
    #[must_use]
    #[inline]
    pub fn from_sequence(sequence: u16) -> Self {
        Self {
            sequence,
            _phantom: PhantomData,
        }
    }

    /// Get the sequence number associated with this cookie.
    #[inline]
    #[must_use]
    pub fn sequence(self) -> u16 {
        self.sequence
    }
}

/// A request, pending reply from the server. It contains the request number of the request as well as extra
/// flags determining how it should be handled by the display.
#[derive(Debug, Default, Clone, Copy)]
pub struct PendingRequest {
    /// The sequence number associated with this request.
    pub request: u16,
    /// The flags indicating how this request should be handled.
    pub flags: PendingRequestFlags,
}

/// Flags indicating how the request/reply should be handled by the display.
#[derive(Default, Debug, Copy, Clone)]
pub struct PendingRequestFlags {
    /// Discard the reply. This is usually only used when synchronizing.
    pub discard_reply: bool,
    /// Whether or not the display was checked when the request was sent.
    pub checked: bool,
    /// We are expecting file descriptors along with this request.
    pub expects_fds: bool,
    /// The reply requires a workaround to be applied.
    pub workaround: RequestWorkaround,
}

/// Which workaround we have to use when we receive the reply bytes from the server.
#[derive(Debug, Copy, Clone)]
pub enum RequestWorkaround {
    /// We don't need to apply a workaround.
    NoWorkaround,
    /// We need to apply a workaround to fix the GLX fbconfig bug.
    GlxFbconfigBug,
}

impl Default for RequestWorkaround {
    #[inline]
    fn default() -> Self {
        Self::NoWorkaround
    }
}

/// Combines `PendingRequest`, `PendingReply`, and `BreadError` into one type, to simplify some of the APIs.
#[derive(Debug, Clone)]
pub enum PendingItem {
    /// A pending request.
    Request(PendingRequest),
    /// A pending reply.
    Reply(PendingReply),
    /// A pending error.
    Error(BreadError),
}

impl PendingItem {
    /// Convert this into either a `PendingRequest` or `None`.
    #[must_use]
    #[inline]
    pub fn request(self) -> Option<PendingRequest> {
        match self {
            PendingItem::Request(pr) => Some(pr),
            _ => None,
        }
    }

    /// Convert this into either a `PendingReply` or `None`.
    #[must_use]
    #[inline]
    pub fn reply(self) -> Option<PendingReply> {
        match self {
            PendingItem::Reply(pr) => Some(pr),
            _ => None,
        }
    }

    /// Convert this into either a `BreadError` or `None`.
    #[must_use]
    #[inline]
    pub fn error(self) -> Option<BreadError> {
        match self {
            PendingItem::Error(err) => Some(err),
            _ => None,
        }
    }
}

/// Utility type to represent a polling result that returns another object if it fails.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PollOr<T, D> {
    Ready(T),
    Pending(D),
}

/// Decode the reply bytes and fds into a reply, returning a `BadObjectRead` if it is invalid.
#[inline]
pub(crate) fn decode_reply<R: Request>(reply: &[u8], fds: Box<[Fd]>) -> crate::Result<R::Reply> {
    let mut r = R::Reply::from_bytes(reply)
        .ok_or(BreadError::BadObjectRead(None))?
        .0;

    if let Some(fdslot) = r.file_descriptors() {
        *fdslot = fds.into_vec();
    }

    Ok(r)
}

/// Convenience function that wraps `generate_xid` and returns an error instead of `None` if it is out of XIDs.
#[inline]
pub(crate) fn generate_xid<D: DisplayBase + ?Sized>(display: &mut D) -> crate::Result<XID> {
    display
        .generate_xid()
        .ok_or(crate::BreadError::StaticMsg("Ran out of XIDs"))
}
