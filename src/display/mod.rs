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
use alloc::{boxed::Box, collections::VecDeque, vec, vec::Vec};
use core::{fmt, iter, marker::PhantomData, mem, num::NonZeroU32};
use cty::c_int;
use hashbrown::HashMap;
use tinyvec::TinyVec;

#[cfg(feature = "std")]
use std::borrow::Cow;

#[cfg(feature = "async")]
use core::{future::Future, pin::Pin};

mod connection;
pub use connection::*;
#[cfg(feature = "std")]
pub mod name;

mod like;
pub use like::*;

mod functions;
mod input;
mod output;

pub use functions::*;

pub(crate) const EXT_KEY_SIZE: usize = 24;

/// The connection to the X11 server. Most operations done in breadx revolve around this object
/// in some way, shape or form.
///
/// Internally, this acts as a layer of abstraction over the inner `Conn` object that keeps track
/// of the setup, outgoing and pending requests and replies, the event queue, et cetera. Orthodoxically,
/// X11 usually takes place over a TCP stream or a Unix socket connection; however, `Display` is able
/// to use any object implementing the `Connection` trait as a vehicle for the X11 protocol.
///
/// Upon its instantiation, the `Display` sends bytes to the server requesting the setup information, and
/// then stores it for later use. Afterwards, it awaits commands from the programmer to send requests,
/// receive replies or process events.
///
/// # Example
///
/// Open a connection to the X11 server and get the screen resolution.
///
/// ```rust,no_run
/// use breadx::DisplayConnection;
///
/// let mut conn = DisplayConnection::create(None, None).unwrap();
///
/// let default_screen = conn.default_screen();
/// println!("Default screen is {} x {}", default_screen.width_in_pixels, default_screen.height_in_pixels);
/// ```
pub struct Display<Conn> {
    // the connection to the server
    pub(crate) connection: Option<Conn>,

    // the setup received from the server
    pub(crate) setup: Setup,

    // xid generator
    xid: XidGenerator,

    // the screen to be used by default
    default_screen: usize,

    pub(crate) event_queue: VecDeque<Event>,
    pub(crate) pending_requests: HashMap<u16, PendingRequest>,
    pub(crate) pending_errors: HashMap<u16, BreadError>,
    #[allow(clippy::type_complexity)]
    pub(crate) pending_replies: HashMap<u16, (Box<[u8]>, Box<[Fd]>)>,

    // special events queue
    pub(crate) special_event_queues: HashMap<XID, VecDeque<Event>>,

    request_number: u64,

    // store the interned atoms
    pub(crate) wm_protocols_atom: Option<NonZeroU32>,

    // tell whether or not we care about the output of zero-sized replies
    pub(crate) checked: bool,

    // context db
    //    context: HashMap<(XID, ContextID), NonNull<c_void>>,

    // hashmap linking extension names to major opcodes
    // we use byte arrays instead of static string pointers
    // here because cache locality leads to an overall speedup (todo: verify)
    extensions: HashMap<[u8; EXT_KEY_SIZE], u8>,
}

/// Unique identifier for a context.
pub type ContextID = c_int;

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
    pub(crate) fn from_sequence(sequence: u64) -> Self {
        Self {
            sequence: sequence as u16, // truncate to lower bits
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
pub(crate) struct PendingRequest {
    pub request: u64,
    pub flags: PendingRequestFlags,
}

#[derive(Default, Debug, Copy, Clone)]
pub(crate) struct PendingRequestFlags {
    pub discard_reply: bool,
    pub checked: bool,
    pub expects_fds: bool,
    pub workaround: RequestWorkaround,
}

#[derive(Debug, Copy, Clone)]
pub(crate) enum RequestWorkaround {
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
const fn endian_byte() -> u8 {
    // Excerpt from the X Window System Protocol
    //
    // The client must send an initial byte of data to identify the byte order to be employed.
    // The value of the byte must be octal 102 or 154. The value 102 (ASCII uppercase B) means
    // values are transmitted most significant byte first, and value 154 (ASCII lowercase l)
    // means values are transmitted least significant byte first.
    #[cfg(not(target_endian = "little"))]
    {
        const BE_SIGNIFIER: u8 = b'B';
        BE_SIGNIFIER
    }
    #[cfg(target_endian = "little")]
    {
        const LE_SIGNIFIER: u8 = b'l';
        LE_SIGNIFIER
    }
}

impl<Conn> Display<Conn> {
    /// Gets the connection associated with this display, and producing an error if the connection
    /// is tainted.
    pub(crate) fn connection(&mut self) -> crate::Result<&mut Conn> {
        self.connection.as_mut().ok_or(crate::BreadError::Tainted)
    }

    #[inline]
    fn decode_reply<R: Request>(reply: Box<[u8]>, fds: Box<[Fd]>) -> crate::Result<R::Reply> {
        let mut r = R::Reply::from_bytes(&reply)
            .ok_or(crate::BreadError::BadObjectRead(None))?
            .0;
        if let Some(fdslot) = r.file_descriptors() {
            *fdslot = fds.into_vec();
        }

        Ok(r)
    }

    /// Register a queue for special events in the display.
    #[inline]
    pub fn register_special_event(&mut self, eid: XID) {
        self.special_event_queues
            .insert(eid, VecDeque::with_capacity(8));
    }

    /// Unregister for a special event.
    #[inline]
    pub fn unregister_special_event(&mut self, eid: XID) {
        self.special_event_queues.remove(&eid);
    }

    /// Try to get a special event without waiting for it.
    #[inline]
    pub fn get_special_event(&mut self, eid: XID) -> Option<Event> {
        self.special_event_queues
            .get_mut(&eid)
            .and_then(VecDeque::pop_front)
    }

    /// Try to get special events without waiting for them.
    #[inline]
    pub fn get_special_events(&mut self, eid: XID) -> impl Iterator<Item = Event> + '_ {
        enum SpecEventIter<'a> {
            QueueDrain(alloc::collections::vec_deque::Drain<'a, Event>),
            Empty,
        }

        impl<'a> Iterator for SpecEventIter<'a> {
            type Item = Event;

            #[inline]
            fn next(&mut self) -> Option<Event> {
                match self {
                    Self::Empty => None,
                    Self::QueueDrain(ref mut qd) => qd.next(),
                }
            }

            #[inline]
            fn size_hint(&self) -> (usize, Option<usize>) {
                match self {
                    Self::QueueDrain(ref qd) => qd.size_hint(),
                    Self::Empty => (0, Some(0)),
                }
            }
        }

        impl<'a> ExactSizeIterator for SpecEventIter<'a> {}

        match self.special_event_queues.get_mut(&eid) {
            Some(queue) => SpecEventIter::QueueDrain(queue.drain(..)),
            None => SpecEventIter::Empty,
        }
    }

    #[inline]
    fn from_connection_internal(connection: Conn) -> Self {
        Self {
            connection: Some(connection),
            setup: Default::default(),
            xid: Default::default(),
            default_screen: 0,
            event_queue: VecDeque::with_capacity(8),
            // setting this to 1 because breadglx with DRI3 will always append one entry to this map,
            // and expanding this map is considered to be a cold operation
            special_event_queues: HashMap::with_capacity(1),
            pending_requests: HashMap::with_capacity(4),
            pending_replies: HashMap::with_capacity(4),
            pending_errors: HashMap::with_capacity(4),
            request_number: 1,
            wm_protocols_atom: None,
            checked: true,
            //            context: HashMap::new(),
            extensions: HashMap::with_capacity(8),
        }
    }

    /// Generate the setup from the authentication info.
    #[inline]
    fn create_setup(auth: AuthInfo) -> SetupRequest {
        let AuthInfo { name, data, .. } = auth;
        SetupRequest {
            byte_order: endian_byte(),
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: name,
            authorization_protocol_data: data,
        }
    }

    /// Generate a unique X ID for a window, colormap, or other object. Usually, `Display`'s helper functions
    /// will generate this for you. If you'd like to circumvent them, this will generate ID's for you.
    #[inline]
    pub fn generate_xid(&mut self) -> crate::Result<XID> {
        Ok(self.xid.next().unwrap())
    }

    /// Get the setup associates with this display.
    #[inline]
    pub fn setup(&self) -> &Setup {
        &self.setup
    }

    #[inline]
    pub fn default_root(&self) -> Window {
        self.default_screen().root
    }

    #[inline]
    pub fn screens(&self) -> &[Screen] {
        &self.setup.roots
    }

    #[inline]
    pub fn default_screen_index(&self) -> usize {
        self.default_screen
    }

    #[inline]
    pub fn default_screen(&self) -> &Screen {
        &self.setup.roots[self.default_screen]
    }

    #[inline]
    pub fn default_white_pixel(&self) -> u32 {
        self.default_screen().white_pixel
    }

    #[inline]
    pub fn default_black_pixel(&self) -> u32 {
        self.default_screen().black_pixel
    }

    #[inline]
    pub fn default_visual_id(&self) -> Visualid {
        self.default_screen().root_visual
    }

    #[inline]
    pub fn default_visual(&self) -> &Visualtype {
        self.visual_id_to_visual(self.default_visual_id()).unwrap()
    }

    #[inline]
    pub fn default_colormap(&self) -> Colormap {
        self.default_screen().default_colormap
    }

    /// Get a visual type from a visual ID.
    #[inline]
    pub fn visual_id_to_visual(&self, id: Visualid) -> Option<&Visualtype> {
        self.setup
            .roots
            .iter()
            .flat_map(|s| s.allowed_depths.iter())
            .flat_map(|d| d.visuals.iter())
            .find(|v| v.visual_id == id)
    }

    /// Get the depth of the specified visual ID.
    #[inline]
    pub fn depth_of_visual(&self, id: Visualid) -> Option<u8> {
        self.setup
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

    /// If there is an event currently in the queue that matches the predicate, returns true.
    #[inline]
    pub fn check_if_event<F: FnMut(&Event) -> bool>(&self, predicate: F) -> bool {
        self.event_queue.iter().any(predicate)
    }

    /// Set whether or not to synchronize the display after every void request to check for errors.
    /// Turning this off improves speed, but makes it difficult to match errors to certain calls.
    /// This is on by default.
    #[inline]
    pub fn set_checked(&mut self, checked: bool) {
        self.checked = checked;

        // if we're turning off checked mode, we no longer care about any of the "checked"
        // objects in pending_requests
        if !checked {
            self.pending_requests.retain(|_, val| !val.flags.checked);
        }
    }

    #[inline]
    pub fn checked(&self) -> bool {
        self.checked
    }
}

impl<Conn: Connection> Display<Conn> {
    /// Forces the server to synchronize itself and send all packets.
    /// This is done by sending a `GetInputFocusRequest` to the server and discarding the reply. Once
    /// we're sure we've received the discarded `GetInputFocusReply`, we know we've successfully
    /// emptied the `pending_requests` array.
    #[inline]
    pub fn synchronize(&mut self) -> crate::Result {
        log::debug!("Synchronizing display");
        let sequence = self
            .send_request_internal(GetInputFocusRequest::default(), true)?
            .sequence();
        // essentially a do/while loop
        while {
            self.wait()?;
            self.pending_requests.contains_key(&sequence)
        } {}

        Ok(())
    }

    /// Send a request object to the X11 server.
    ///
    /// Given a request object, this function sends it across the connection to the X11 server and returns
    /// a cookie used to determine when this request will resolve. Usually, the `Display` object has functions
    /// that act as a wrapper around this object; however, if you'd like to circumvent those, this is usually
    /// the best option.
    #[inline]
    pub fn send_request<R: Request>(&mut self, req: R) -> crate::Result<RequestCookie<R>> {
        self.send_request_internal(req, false)
    }

    /// Wait for a request from the X11 server.
    ///
    /// This function checks the `Display`'s queues to see if a reply matching the given `RequestCookie`
    /// has been processed by the X11 server. If not, it polls the server for new events until it has
    /// determined that the request has resolved.
    #[inline]
    pub fn resolve_request<R: Request>(
        &mut self,
        token: RequestCookie<R>,
    ) -> crate::Result<R::Reply>
    where
        R::Reply: Default,
    {
        if mem::size_of::<R::Reply>() == 0 {
            // check the request for errors by synchronizing the connection, assuming we care about
            // that
            if self.checked {
                self.synchronize()?;
                let seq = token.sequence();
                self.pending_requests.remove(&seq);
                if let Some(err) = self.pending_errors.remove(&seq) {
                    return Err(err);
                }
            }

            return Ok(Default::default());
        }

        loop {
            log::trace!("Current replies: {:?}", &self.pending_replies);

            match self.pending_replies.remove(&token.sequence) {
                Some((reply, fds)) => break Self::decode_reply::<R>(reply, fds),
                None => self.wait()?,
            }
        }
    }

    /// Wait for a special event.
    #[inline]
    pub fn wait_for_special_event(&mut self, eid: XID) -> crate::Result<Event> {
        loop {
            let queue = match self.special_event_queues.get_mut(&eid) {
                Some(queue) => queue,
                None => {
                    return Err(crate::BreadError::StaticMsg(
                        "Tried to poll a special event that didn't exist",
                    ))
                }
            };

            match queue.pop_front() {
                Some(event) => break Ok(event),
                None => self.wait()?,
            }
        }
    }

    /// Creates a new `Display` from a connection and authentication info.
    ///
    /// It is expected that the connection passed in has not had any information sent into it aside from
    /// what is necessary for the underlying protocol. After the object is created, the `Display` will poll
    /// the server for setup information.
    #[inline]
    pub fn from_connection(connection: Conn, auth: Option<AuthInfo>) -> crate::Result<Self> {
        let mut d = Self::from_connection_internal(connection);
        d.init(auth)?;
        Ok(d)
    }

    /// Initialize the setup.
    #[inline]
    fn init(&mut self, auth: Option<AuthInfo>) -> crate::Result {
        log::debug!("Establishing connection to server.");

        let setup = Self::create_setup(match auth {
            Some(auth) => auth,
            None => AuthInfo::get(),
        });
        let mut _fds: Vec<Fd> = vec![];
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(setup.size());
        let len = setup.as_bytes(&mut bytes);
        bytes.truncate(len);

        log::trace!("Sending setup request to server.");
        self.connection()?.send_packet(&bytes[0..len], &mut _fds)?;
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(8);

        log::trace!("Reading setup response from server.");
        self.connection()?.read_packet(&mut bytes, &mut _fds)?;

        match bytes[0] {
            0 => return Err(crate::BreadError::FailedToConnect),
            2 => return Err(crate::BreadError::FailedToAuthorize),
            _ => (),
        }

        // read in the rest of the bytes
        let length_bytes: [u8; 2] = [bytes[6], bytes[7]];
        let length = (u16::from_ne_bytes(length_bytes) as usize) * 4;
        bytes.extend(iter::once(0).cycle().take(length));

        log::trace!("Reading remainder of setup.");
        self.connection()?.read_packet(&mut bytes[8..], &mut _fds)?;

        let (setup, _) =
            Setup::from_bytes(&bytes).ok_or(crate::BreadError::BadObjectRead(Some("Setup")))?;
        self.setup = setup;
        self.xid = XidGenerator::new(self.setup.resource_id_base, self.setup.resource_id_mask);

        log::debug!("resource_id_base is {:#032b}", self.setup.resource_id_base);
        log::debug!("resource_id_mask is {:#032b}", self.setup.resource_id_mask);
        log::debug!(
            "resource_id inc. is {:#032b}",
            self.setup.resource_id_mask & self.setup.resource_id_mask.wrapping_neg()
        );

        Ok(())
    }

    /// Wait for an event to be generated by the X server.
    ///
    /// This checks the event queue for a new event. If the queue is empty, the `Display` will poll the
    /// server for new events.
    #[inline]
    pub fn wait_for_event(&mut self) -> crate::Result<Event> {
        log::debug!("Beginning event wait...");
        loop {
            match self.event_queue.pop_front() {
                Some(event) => break Ok(event),
                None => self.wait()?,
            }
        }
    }
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection + Send> Display<Conn> {
    /// Forces the server to synchronize itself, async redox.
    #[inline]
    pub fn synchronize_async<'future>(
        &'future mut self,
    ) -> Pin<Box<dyn Future<Output = crate::Result> + Send + 'future>> {
        Box::pin(async move {
            log::debug!("Synchronizing display");
            let sequence = self
                .send_request_internal_async(GetInputFocusRequest::default(), true)
                .await?
                .sequence();
            while self.pending_requests.contains_key(&sequence) {
                self.wait_async().await?;
            }

            Ok(())
        })
    }

    /// Send a request object to the X11 server, async redox. See the `send_request` function for more
    /// information.
    #[inline]
    pub fn send_request_async<'future, R: Request + Send + 'future>(
        &'future mut self,
        req: R,
    ) -> Pin<Box<dyn Future<Output = crate::Result<RequestCookie<R>>> + Send + 'future>> {
        Box::pin(self.send_request_internal_async(req, false))
    }

    /// Wait for a request from the X11 server, async redox. See the `resolve_request` function for more
    /// information.
    #[inline]
    pub async fn resolve_request_async<R: Request>(
        &mut self,
        token: RequestCookie<R>,
    ) -> crate::Result<R::Reply>
    where
        R::Reply: Default,
    {
        if mem::size_of::<R::Reply>() == 0 {
            // check the request for errors by synchronizing the connection
            if self.checked {
                self.synchronize_async().await?;
                let seq = token.sequence();
                self.pending_requests.remove(&seq);
                if let Some(err) = self.pending_errors.remove(&seq) {
                    return Err(err);
                };
            }
            return Ok(Default::default());
        }

        loop {
            match self.pending_replies.remove(&token.sequence) {
                Some((reply, fds)) => {
                    break Self::decode_reply::<R>(reply, fds);
                }
                None => self.wait_async().await?,
            }
        }
    }

    /// Wait for a special event, async redox.
    #[inline]
    pub async fn wait_for_special_event_async(&mut self, eid: XID) -> crate::Result<Event> {
        loop {
            let queue = match self.special_event_queues.get_mut(&eid) {
                Some(queue) => queue,
                None => {
                    return Err(crate::BreadError::StaticMsg(
                        "Tried to poll a special event that didn't exist",
                    ))
                }
            };

            match queue.pop_front() {
                Some(event) => break Ok(event),
                None => self.wait_async().await?,
            }
        }
    }

    /// Creates a new `Display` from a connection and authentication info, async redox. See the `from_connection`
    /// function for more information.
    #[inline]
    pub async fn from_connection_async(
        connection: Conn,
        auth: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let mut d = Self::from_connection_internal(connection);
        d.init_async(auth).await?;
        Ok(d)
    }

    /// Wait for an event to be generated by the X server, async redox. See the `wait_for_event` function for
    /// more information.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn wait_for_event_async(&mut self) -> crate::Result<Event> {
        loop {
            match self.event_queue.pop_front() {
                Some(event) => break Ok(event),
                None => self.wait_async().await?,
            }
        }
    }

    /// Initialize the setup, async redox.
    ///
    /// TODO; lots of copy-pasted code, redo this at some point
    #[cfg(feature = "async")]
    #[inline]
    async fn init_async(&mut self, auth: Option<AuthInfo>) -> crate::Result {
        let setup = Self::create_setup(match auth {
            Some(auth) => auth,
            None => AuthInfo::get_async().await,
        });
        let mut _fds: Vec<Fd> = vec![];
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(setup.size());
        let len = setup.as_bytes(&mut bytes);
        bytes.truncate(len);
        self.connection()?
            .send_packet(&bytes[0..len], &mut _fds)
            .await?;
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(8);
        self.connection()?
            .read_packet(&mut bytes, &mut _fds)
            .await?;

        match bytes[0] {
            0 => return Err(crate::BreadError::FailedToConnect),
            2 => return Err(crate::BreadError::FailedToAuthorize),
            _ => (),
        }

        // read in the rest of the bytes
        let length_bytes: [u8; 2] = [bytes[6], bytes[7]];
        let length = (u16::from_ne_bytes(length_bytes) as usize) * 4;
        bytes.extend(iter::once(0).cycle().take(length));
        self.connection()?
            .read_packet(&mut bytes[8..], &mut _fds)
            .await?;

        let (setup, _) = Setup::from_bytes(&bytes)
            .ok_or_else(|| crate::BreadError::BadObjectRead(Some("Setup")))?;
        self.setup = setup;
        self.xid = XidGenerator::new(self.setup.resource_id_base, self.setup.resource_id_mask);

        log::debug!("resource_id_base is {:#032b}", self.setup.resource_id_base);
        log::debug!("resource_id_mask is {:#032b}", self.setup.resource_id_mask);
        log::debug!(
            "resource_id inc. is {:#032b}",
            self.setup.resource_id_mask & self.setup.resource_id_mask.wrapping_neg()
        );

        Ok(())
    }
}

/// A variant of `Display` that uses X11's default connection mechanisms to connect to the server. In
/// most cases, you should be using this over any variant of `Display`.
#[cfg(feature = "std")]
pub type DisplayConnection = Display<name::NameConnection>;

#[cfg(all(feature = "std", feature = "async"))]
pub type AsyncDisplayConnection = Display<name::AsyncNameConnection>;

#[cfg(feature = "std")]
impl DisplayConnection {
    /// Create a new connection to the X server, given an optional name and authorization information.
    #[inline]
    pub fn create(name: Option<Cow<'_, str>>, auth_info: Option<AuthInfo>) -> crate::Result<Self> {
        let connection = name::NameConnection::connect_internal(name)?;
        Self::from_connection(connection, auth_info)
    }
}

#[cfg(all(feature = "std", feature = "async"))]
impl AsyncDisplayConnection {
    /// Create a new connection to the X server, given an optional name and authorization information, async
    /// redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_async(
        name: Option<Cow<'_, str>>,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let connection = name::AsyncNameConnection::connect_internal_async(name).await?;
        Self::from_connection_async(connection, auth_info).await
    }
}
