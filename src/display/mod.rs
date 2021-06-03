// MIT/Apache2 License

//! This module defines the `Display` object, which acts as a connection to the X11 server, and the
//! `Connection` trait, which the `Display` object abstracts itself over. See the documentation for
//! these objects for more information.

use crate::{
    auth_info::AuthInfo,
    auto::{
        xproto::{
            Colormap, GetInputFocusRequest, Screen, Setup, SetupRequest, Visualid, Visualtype,
            Window,
        },
        AsByteSequence,
    },
    error::BreadError,
    event::Event,
    util::cycled_zeroes,
    xid::XidGenerator,
    Fd, Request, XID,
};
use alloc::{borrow::Cow, boxed::Box, collections::VecDeque, vec, vec::Vec};
use core::{fmt, iter, marker::PhantomData, mem, num::NonZeroU32};
use hashbrown::HashMap;
use tinyvec::TinyVec;

#[cfg(feature = "async")]
use core::{future::Future, pin::Pin};

mod basic;
mod cell;
mod connection;
mod functions;

pub use basic::*;
pub use cell::*;
pub use connection::*;
pub use functions::*;

#[cfg(feature = "async")]
mod futures;
#[cfg(feature = "async")]
pub use futures::*;

pub(crate) mod input;
pub(crate) mod output;

#[cfg(feature = "async")]
pub(crate) mod common;

#[cfg(feature = "std")]
pub mod name;

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
/// and then stores it for later use. This can be done via the "create_setup" convenience method provided by the
/// `Connection` and `AsyncConnection` traits. Afterwards, it awaits commands from the programmer to send
/// requests, receive replies or process events.
///
/// Objects implementing this trait should further implement `Display` or `AsyncDisplay`,
pub trait DisplayBase {
    /// Get the `Setup` object that defines the X11 connection.
    fn setup(&self) -> &Setup;

    /// Get the default screen index.
    fn default_screen(&self) -> usize;

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
    fn remove_pending_request(&mut self, req_id: u16) -> Option<PendingRequest>;

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

    /// Get the opcode for an extension.
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8>;

    /// Set the opcode for an extension.
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8);

    /// Get the WM_PROTOCOLS atom, which we cache in the display.
    fn wm_protocols_atom(&self) -> Option<NonZeroU32>;

    /// Set the WM_PROTOCOLS atom.
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
        &self.setup().roots[self.default_screen()]
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

impl<D: DisplayBase + ?Sized> DisplayBase for &mut D {
    #[inline]
    fn setup(&self) -> &Setup {
        (**self).setup()
    }

    #[inline]
    fn default_screen(&self) -> usize {
        (**self).default_screen()
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
    fn remove_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        (**self).remove_pending_request(req_id)
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
    /// The connection used in our display.    
    type Conn: Connection;

    /// Get the connection.
    fn connection(&mut self) -> &mut Self::Conn;

    /// Lock the connection. This exists as an I/O lock so that multithreaded users don't step on each
    /// other's toes.
    fn lock(&mut self);

    /// Unlock the connection.
    fn unlock(&mut self);

    /// Wait for something to happen on the connection.
    #[inline]
    fn wait(&mut self) -> crate::Result {
        input::wait(self)
    }

    /// Send a request across the connection, given the monomorphized request info.
    #[inline]
    fn send_request_raw(&mut self, request_info: RequestInfo) -> crate::Result<u16> {
        output::send_request(self, request_info)
    }

    /// Synchronize this display, ensuring that all data sent across it has been replied to.
    #[inline]
    fn synchronize(&mut self) -> crate::Result {
        log::debug!("Synchronizing display");
        let mut gifr = RequestInfo::from_request(GetInputFocusRequest::default());
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
                Some(p) => break Ok(p),
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
    type Conn = D::Conn;

    #[inline]
    fn connection(&mut self) -> &mut Self::Conn {
        (**self).connection()
    }

    #[inline]
    fn lock(&mut self) {
        (**self).lock()
    }

    #[inline]
    fn unlock(&mut self) {
        (**self).unlock()
    }
}

/// A wrapper around an asynchronous connection to the X server.
#[cfg(feature = "async")]
pub trait AsyncDisplay {
    /// The connection used in our display.
    type Conn: AsyncConnection;

    /// Get the inner connection.
    fn connection(&mut self) -> &mut Self::Conn;

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
    type Conn = D::Conn;

    #[inline]
    fn connection(&mut self) -> &mut Self::Conn {
        (**self).connection()
    }

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
}

impl<D: Display + ?Sized> DisplayExt for D {
    #[inline]
    fn send_request<R: Request>(&mut self, request: R) -> crate::Result<RequestCookie<R>> {
        let r = RequestInfo::from_request(request);
        let req_id = self.send_request_raw(r, false)?;
        Ok(RequestCookie::from_request(req_id))
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
                self.remove_pending_request(seq);
                self.check_for_pending_error()?;
            }

            return Ok(Default::default());
        }

        let PendingReply { data, fds } = self.resolve_request_raw(token.sequence())?;
        decode_reply(&data, fds)
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
    fn wait_for_special_event_async(&mut self) -> WaitForSpecialEventFuture<'_, Self>;
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplayExt for D {
    #[inline]
    fn wait_async(&mut self) -> WaitFuture<Self> {
        WaitFuture::run(self)
    }

    #[inline]
    fn send_request_raw_async(&mut self, req: RequestInfo) -> SendRequestRawFuture<'_, Self> {
        SendRequestRawFuture::run(self, req, false)
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
    fn wait_for_special_event_async(&mut self) -> WaitForSpecialEventFuture<'_, Self> {
        WaitForSpecialEventFuture::run(self)
    }
}

/// Request information, monomorphized from the Request trait.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequestInfo {
    pub(crate) data: TinyVec<[u8; 32]>,
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
    pub fn from_request<R: Request>(req: R) -> Self {
        // TODO: somehow write using uninitialzied data
        let mut data = iter::repeat(0)
            .take(r.size())
            .collect::<TinyVec<[u8; 32]>>();
        let mut len = req.as_bytes(&mut data);

        // pad to a multiple of 4 if we can
        len = (len + 3) & (!0x03);
        data.truncate(len);

        // Third and fourth bytes need to be length
        let x_len = len / 4;
        log::trace!("xlen is {}", x_len);
        let len_bytes = x_len.to_ne_bytes();
        data[2] = len_bytes[0];
        data[3] = len_bytes[1];

        RequestInfo {
            data,
            zero_sized_reply: mem::size_of::<R::Reply> == 0,
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
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Default, Eq, Hash)]
#[repr(transparent)]
pub struct RequestCookie<R: Request> {
    sequence: u16,
    _phantom: PhantomData<Option<R::Reply>>,
}

impl<R: Request> RequestCookie<R> {
    #[inline]
    pub(crate) fn from_sequence(sequence: u16) -> Self {
        Self {
            sequence: sequence,
            _phantom: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn sequence(self) -> u16 {
        self.sequence
    }
}

impl<Conn: fmt::Debug> fmt::Debug for Display<Conn> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Display")
            .field("connection", &self.connection)
            .field("setup", &self.setup)
            .field("xid", &self.xid)
            .field("default_screen", &self.default_screen)
            .field("event_queue", &self.event_queue)
            .field("pending_requests", &self.pending_requests)
            .field("pending_replies", &self.pending_replies)
            .field("request_number", &self.request_number)
            .finish()
    }
}

#[derive(Debug, Default, Clone)]
pub struct PendingRequest {
    pub request: u64,
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
