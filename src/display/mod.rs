// MIT/Apache2 License

//! This module defines the `Display` object, which acts as a connection to the X11 server, and the
//! `Connection` trait, which the `Display` object abstracts itself over. See the documentation for
//! these objects for more information.

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
use core::{fmt, iter, iter::FusedIterator, marker::PhantomData, mem, num::NonZeroU32};
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

pub(crate) mod input;
pub(crate) mod output;

#[cfg(feature = "async")]
pub(crate) mod common;

#[cfg(feature = "std")]
pub mod name;

pub mod prelude {
    pub use super::traits::*;
    #[cfg(feature = "async")]
    pub use super::{AsyncDisplay, AsyncDisplayExt};
    pub use super::{Display, DisplayBase, DisplayExt};
}

pub(crate) const EXT_KEY_SIZE: usize = 24;

/// This trait represents a connection to the X11 server. Most operations in `breadx` revolve around an object
/// implementing this trait in some way, shape, or form.
///
/// Internally, this should act as a layer of abstraction over the inner `Connection` object that keeps track
/// of the setup, outgoing and pending requests and replies, the event queue, et cetera. Orthodoxically,
/// X11 usually takes place over a TCP stream or a Unix socket connection; however, `Display` is able
/// to use any object implementing the `Connection` trait as a vehicle for the X11 protocol.
///
/// Upon its instantiation, the `DisplayBase` should send bytes to the server requesting the setup information,
/// and then stores it for later use. This can be done via the `establish` convenience method provided by the
/// `Connection` and `AsyncConnection` traits. Afterwards, it awaits commands from the programmer to send
/// requests, receive replies or process events.
///
/// Objects implementing this trait should further implement `Display` or `AsyncDisplay`,
pub trait DisplayBase {
    /// Get the `Setup` object that defines the X11 connection.
    fn setup(&self) -> &Setup;

    /// Get the default screen index.
    fn default_screen_index(&self) -> usize;

    /// Generate the next request number to be used to define a request.
    fn next_request_number(&mut self) -> u64;

    /// Push an event into this display's event queue.
    fn push_event(&mut self, event: Event);

    /// Pop an event from this display's event queue.
    fn pop_event(&mut self) -> Option<Event>;

    /// Generate an XID within appropriate bounds. Returns `None` if our XIDs are exhausted.
    fn generate_xid(&mut self) -> Option<XID>;

    /// Add a pending request to this display.
    fn add_pending_request(&mut self, req_id: u16, pereq: PendingRequest);

    /// Get a pending request from this display.
    fn get_pending_request(&self, req_id: u16) -> Option<PendingRequest>;

    /// Remove a pending request from this display.
    fn take_pending_request(&mut self, req_id: u16) -> Option<PendingRequest>;

    /// Add a pending error to this display.
    fn add_pending_error(&mut self, req_id: u16, error: BreadError);

    /// Remove a pending error, if it exists.
    fn check_for_pending_error(&mut self, req_id: u16) -> crate::Result<()>;

    /// Add a pending reply.
    fn add_pending_reply(&mut self, req_id: u16, reply: PendingReply);

    /// Remove the pending reply.
    fn take_pending_reply(&mut self, req_id: u16) -> Option<PendingReply>;

    /// Create a new special event queue.
    fn create_special_event_queue(&mut self, xid: XID);

    /// Push an event into the special event queue.
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event>;

    /// Pop an event from the special event queue.
    fn pop_special_event(&mut self, xid: XID) -> Option<Event>;

    /// Delete a special event queue.
    fn delete_special_event_queue(&mut self, xid: XID);

    /// Whether or not zero-length replies are checked.
    fn checked(&self) -> bool;

    /// Set whether or not zero-length replies are checked.
    fn set_checked(&mut self, checked: bool);

    /// Whether or not this display uses the `bigreq` extension, whereas requests consisting of over
    /// 262140 bytes are allowed to be sent over the connection.
    fn bigreq_enabled(&self) -> bool;

    /// The current maximum request length.
    fn max_request_len(&self) -> usize;

    /// Get the opcode for an extension.
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8>;

    /// Set the opcode for an extension.
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8);

    /// Get the `WM_PROTOCOLS` atom, which we cache in the display.
    fn wm_protocols_atom(&self) -> Option<NonZeroU32>;

    /// Set the `WM_PROTOCOLS` atom.
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32);

    // -- Setup-based functions.

    /// Get the list of screens in this display.
    #[inline]
    fn screens(&self) -> &[Screen] {
        &self.setup().roots
    }

    /// Get the default screen in this display.
    #[inline]
    fn default_screen(&self) -> &Screen {
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

    #[inline]
    fn default_visual(&self) -> &Visualtype {
        self.visual_id_to_visual(self.default_visual_id()).unwrap()
    }

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
    fn setup(&self) -> &Setup {
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
    fn add_pending_request(&mut self, req_id: u16, pereq: PendingRequest) {
        (**self).add_pending_request(req_id, pereq)
    }

    #[inline]
    fn get_pending_request(&self, req_id: u16) -> Option<PendingRequest> {
        (**self).get_pending_request(req_id)
    }

    #[inline]
    fn take_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        (**self).take_pending_request(req_id)
    }

    #[inline]
    fn add_pending_error(&mut self, req_id: u16, error: BreadError) {
        (**self).add_pending_error(req_id, error)
    }

    #[inline]
    fn check_for_pending_error(&mut self, req_id: u16) -> crate::Result<()> {
        (**self).check_for_pending_error(req_id)
    }

    #[inline]
    fn add_pending_reply(&mut self, req_id: u16, reply: PendingReply) {
        (**self).add_pending_reply(req_id, reply)
    }

    #[inline]
    fn take_pending_reply(&mut self, req_id: u16) -> Option<PendingReply> {
        (**self).take_pending_reply(req_id)
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
pub trait Display: DisplayBase {
    /// Wait for something to happen on the connection.
    fn wait(&mut self) -> crate::Result;

    /// Send a request across the connection, given the monomorphized request info.
    fn send_request_raw(&mut self, request_info: RequestInfo) -> crate::Result<u16>;

    /// Synchronize this display, ensuring that all data sent across it has been replied to.
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
    #[inline]
    fn wait_for_event(&mut self) -> crate::Result<Event> {
        loop {
            match self.pop_event() {
                Some(e) => break Ok(e),
                None => self.wait()?,
            }
        }
    }

    /// Wait for a special event to be sent from the X server.
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
#[cfg(feature = "async")]
pub trait AsyncDisplay: DisplayBase {
    /// Poll the current status of waiting for more input. There is no default implementation; the buffering
    /// strategy depends on the implementor. If this is called after a Poll::Ready is returned, it is assumed
    /// that the user wants to wait again.
    fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result>;

    /// Begin sending a raw request to the server. In order to poll the status of this operation, use the
    /// `poll_send_request_raw` function, or just use the `send_request_raw`/`send_request` function.
    fn begin_send_request_raw(&mut self, req: RequestInfo);

    /// Poll an ongoing raw request operation.
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>>;
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplay for &mut D {
    #[inline]
    fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result> {
        (**self).poll_wait(cx)
    }

    #[inline]
    fn begin_send_request_raw(&mut self, req: RequestInfo) {
        (**self).begin_send_request_raw(req)
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        (**self).poll_send_request_raw(cx)
    }
}

/// Monomorphized methods we can't put into the `Display` trait proper.
pub trait DisplayExt {
    /// Send a request to the server.
    fn send_request<R: Request>(&mut self, request: R) -> crate::Result<RequestCookie<R>>;

    /// Resolve a request that we sent to the server.
    fn resolve_request<R: Request>(&mut self, token: RequestCookie<R>) -> crate::Result<R::Reply>
    where
        R::Reply: Default;

    /// Send a request to the server and immediately resolve for its reply.
    #[inline]
    fn exchange_request<R: Request + 'static>(&mut self, request: R) -> crate::Result<R::Reply>
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
    fn wait_async(&mut self) -> WaitFuture<'_, Self>;

    /// Send a raw request to the server.
    fn send_request_raw_async(&mut self, req: RequestInfo) -> SendRequestRawFuture<'_, Self>;

    /// Send a request to the server.
    fn send_request_async<R: Request>(&mut self, request: R) -> SendRequestFuture<'_, Self, R>;

    /// Resolve a request that we sent to the server.
    fn resolve_request_async<R: Request>(
        &mut self,
        token: RequestCookie<R>,
    ) -> ResolveRequestFuture<'_, Self, R>
    where
        R::Reply: Default;

    /// Synchronize this display so that every request that has been sent is resolved.
    fn synchronize_async(&mut self) -> SynchronizeFuture<'_, Self>;

    /// Resolve for a raw request.
    fn resolve_request_raw_async(&mut self, req_id: u16) -> ResolveRequestRawFuture<'_, Self>;

    /// Wait for an event to be sent from the X server.
    fn wait_for_event_async(&mut self) -> WaitForEventFuture<'_, Self>;

    /// Wait for a special event to be sent from the X server.
    fn wait_for_special_event_async(&mut self, xid: XID) -> WaitForSpecialEventFuture<'_, Self>;

    /// Send a request and wait for a reply back
    fn exchange_request_async<R: Request>(
        &mut self,
        request: R,
    ) -> ExchangeRequestFuture<'_, Self, R>;

    /// Send a no-op request to the server, but resolve for an XID.
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
    /// Generate a `RequestInfo` given a specific `Request` to generate from.
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
            data = match data {
                TinyVec::Inline(data) => BigreqIterator {
                    inner: data.into_iter(),
                    length_bytes,
                    cursor: 0,
                }
                .collect(),
                TinyVec::Heap(data) => BigreqIterator {
                    inner: data.into_iter(),
                    length_bytes,
                    cursor: 0,
                }
                .collect(),
            };
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

struct BigreqIterator<I> {
    inner: I,
    cursor: usize,
    length_bytes: [u8; 4],
}

impl<I: Iterator<Item = u8>> Iterator for BigreqIterator<I> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        let res = match self.cursor {
            // 2..4
            2..=3 => {
                self.inner.next();
                Some(0)
            }
            // 4..8
            4..=7 => Some(self.length_bytes[self.cursor - 4]),
            _ => self.inner.next(),
        };

        self.cursor += 1;
        res
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lo, hi) = self.inner.size_hint();
        (lo + 4, hi.map(|hi| hi + 4))
    }
}

impl<I: Iterator<Item = u8> + FusedIterator> FusedIterator for BigreqIterator<I> {}

impl<I: Iterator<Item = u8> + ExactSizeIterator> ExactSizeIterator for BigreqIterator<I> {}

/// A reply, pending returning from the display.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingReply {
    pub data: TinyVec<[u8; 32]>,
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
    #[inline]
    pub(crate) fn from_sequence(sequence: u16) -> Self {
        Self {
            sequence,
            _phantom: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn sequence(self) -> u16 {
        self.sequence
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PendingRequest {
    pub request: u16,
    pub flags: PendingRequestFlags,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PendingRequestFlags {
    pub discard_reply: bool,
    pub checked: bool,
    pub expects_fds: bool,
    pub workaround: RequestWorkaround,
}

#[derive(Debug, Copy, Clone)]
pub enum RequestWorkaround {
    NoWorkaround,
    GlxFbconfigBug,
}

impl Default for RequestWorkaround {
    #[inline]
    fn default() -> Self {
        Self::NoWorkaround
    }
}

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

#[inline]
pub(crate) fn generate_xid<D: DisplayBase + ?Sized>(display: &mut D) -> crate::Result<XID> {
    display
        .generate_xid()
        .ok_or(crate::BreadError::StaticMsg("Ran out of XIDs"))
}
