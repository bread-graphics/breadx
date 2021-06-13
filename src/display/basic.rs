// MIT/Apache2 License

use super::{
    bigreq, input, output, Connection, Display, DisplayBase, PendingItem, RequestInfo, StaticSetup,
    EXT_KEY_SIZE,
};
use crate::{auth_info::AuthInfo, event::Event, log_trace, XidGenerator, XID};
use alloc::{borrow::Cow, collections::VecDeque};
use core::num::NonZeroU32;
use hashbrown::HashMap;

#[cfg(feature = "std")]
use super::name::NameConnection;

#[cfg(feature = "async")]
use super::{
    common::{SendBuffer, WaitBuffer, WaitBufferReturn},
    name::AsyncNameConnection,
    AsyncConnection, AsyncDisplay, PollOr, RequestWorkaround,
};
#[cfg(feature = "async")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "async")]
use core::{
    mem,
    task::{Context, Poll},
};

/// An implementor of `Display` and `AsyncDisplay` that requires &mut access in order to use.
///
/// This is the standard implementation of a display for the X11 protocol. In addition to storing the connection
/// to the server and the setup information received from the server, it contains the event queues, pending item
/// map, and provides XIDs and sequence numbers.
///
/// It is recommended to create an instance of this object via `DisplayConnection::create()`, since that
/// function uses the system's variables in order to connect to the X11 server. However, niche use cases who need
/// to communicate with the X server in other ways should use `[BasicDisplay::from_connection]`.
///
/// # Connection Poisoning
///
/// If a panic occurs while sending/receiving along the connection, or if a future involving sending/receiving
/// across the connection is dropped, the connection may be "poisoned". Attempting to use the connection after
/// it is poisoned will result in a panic, since the current state of the `BasicDisplay` is invalid and will
/// result in invalid communications with the X11 server. The exact semantics of what, exactly, will cause a
/// connection poisoning are not well defined at the moment. However, the connection should never be poisoned
/// during normal operation of the `BasicDisplay`.
///
/// # Mutability
///
/// `BasicDisplay` requires an `&mut` reference for most operations, including sending and receiving requests and
/// replies. Although this "inherited mutability", as we call it, may be convenient for architectures of programs
/// where multiple objects require access to the X server, it allows the connection to prevent undefined
/// behavior from occurring. To quote the rust std documentation for
/// [`std::cell`](https://doc.rust-lang.org/std/cell/index.html):
///
/// > The more common inherited mutability, where one must have unique access to mutate a value, is one of the
/// > key language elements that enables Rust to reason strongly about pointer aliasing, statically preventing
/// > crash bugs. Because of that, inherited mutability is preferred, and interior mutability is something of a
/// > last resort.
///
/// That being said, the [`CellDisplay`] object is preferred over using `RefCell<BasicDisplay>`, since it uses
/// several different cell types in order to reduce reentrant panics. However, using [`SyncDisplay`] instead of
/// `Mutex<BasicDisplay>` may not be preferred depending on your use case. See the documentation for
/// `SyncDisplay` for more information.
///
/// [`CellDisplay`]: struct.CellDisplay.html
/// [`SyncDisplay`]: struct.SyncDisplay.html
#[derive(Debug)]
pub struct BasicDisplay<Conn> {
    // NOTE: every field in this structure is pub(crate), because the implementations of From
    //       for CellDisplay and SyncDisplay need to deconstruct it
    /// The connection to the server. It is in an `Option`, so that way if it is `None` we know
    /// the connection has been poisoned.
    pub(crate) connection: Option<Conn>,

    /// The setup received from the server.
    pub(crate) setup: StaticSetup,

    /// Whether or not the "bigreq" system has been enabled, which expands the number of permitted bytes to send
    /// across the request. For more information, see the following webpage.
    ///
    /// <https://www.x.org/releases/X11R7.7/doc/bigreqsproto/bigreq.html>
    pub(crate) bigreq_enabled: bool,

    /// The current maximum number of bytes we can send across the connection.
    pub(crate) max_request_len: usize,

    /// XID generator.
    pub(crate) xid: XidGenerator,

    /// The index of the screen to be used by default.
    pub(crate) default_screen: usize,

    /// Queue for events; more recent events are at the front.
    pub(crate) event_queue: VecDeque<Event>,
    /// Map associating request numbers to pending requests, pending replies, and pending errors.
    pub(crate) pending_items: HashMap<u16, PendingItem>,
    /// Map associating XID's to special event queues. For some extensions, they produce events that need to be
    /// put into their own species queues.
    pub(crate) special_event_queues: HashMap<XID, VecDeque<Event>>,

    /// The running sequence number for the requests in the display.
    pub(crate) request_number: u64,

    /// Caches the "WM_PROTOCOLS" atom, which tends to be commonly used.
    pub(crate) wm_protocols_atom: Option<NonZeroU32>,

    /// If this is true, we store zero-sized replies as pending requests and check for their synchronization.
    /// If false, this discards their replies. It is much faster than checked mode.
    pub(crate) checked: bool,

    /// A hashmap linking the names of extensions to their opcodes.
    pub(crate) extensions: HashMap<[u8; EXT_KEY_SIZE], u8>,

    /// Internal buffer for polling for waiting
    #[cfg(feature = "async")]
    wait_buffer: Option<WaitBuffer>,

    /// Internal buffer for sending a request
    #[cfg(feature = "async")]
    send_buffer: SendBuffer,

    /// List of requests we need to consider the GLX workaround for. This simplifies
    /// async operations.
    #[cfg(feature = "async")]
    workarounders: Vec<u16>,
}

impl<Conn> BasicDisplay<Conn> {
    /// Initialize a `BasicDisplay` from a connection, without sending anything.
    #[inline]
    fn from_connection_internal(connection: Conn, default_screen: usize) -> Self {
        Self {
            connection: Some(connection),
            setup: Default::default(),
            xid: Default::default(),
            default_screen,
            event_queue: VecDeque::with_capacity(8),
            bigreq_enabled: false,
            max_request_len: 0,
            // setting this to 1 because breadglx with DRI3 will always append one entry to this map,
            // and expanding this map is considered to be a cold operation
            special_event_queues: HashMap::with_capacity(1),
            pending_items: HashMap::with_capacity(4),
            request_number: 1,
            wm_protocols_atom: None,
            checked: cfg!(debug_assertions),
            //            context: HashMap::new(),
            extensions: HashMap::with_capacity(8),
            #[cfg(feature = "async")]
            wait_buffer: None,
            #[cfg(feature = "async")]
            send_buffer: Default::default(),
            #[cfg(feature = "async")]
            workarounders: vec![],
        }
    }

    /// Get the connection that this display wraps around. Modifying the connection may have unexpected results.
    ///
    /// # Panics
    ///
    /// Panics if the connection has been poisoned.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use breadx::display::DisplayConnection;
    ///
    /// let conn = DisplayConnection::create(None, None).unwrap();
    /// conn.connection();
    /// ```
    #[inline]
    pub fn connection(&self) -> &Conn {
        self.connection.as_ref().expect("Poisoned!")
    }

    /// Get a mutable reference to the connection.
    ///
    /// # Panics
    ///
    /// Panics if the connection has been poisoned.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use breadx::display::DisplayConnection;
    ///
    /// let mut conn = DisplayConnection::create(None, None).unwrap();
    /// conn.connection_mut();
    /// ```
    #[inline]
    pub fn connection_mut(&mut self) -> &mut Conn {
        self.connection.as_mut().expect("Poisoned!")
    }
}

impl<Conn: Connection> BasicDisplay<Conn> {
    /// Establishes this display using an inner connection type. This receives setup information from the
    /// server on the other side of the connection in order to populate the display.
    ///
    /// In order to produce the `Setup`, `from_connection` sends a `SetupRequest` object across the provided
    /// connection. This `SetupRequest` is populated with the [`AuthInfo`] provided by either the user, or the
    /// file specified by the `XAUTHORITY` environment variable if the `auth_info` parameter is `None`. Once the
    /// `Setup` has been received, it is stored in the `BasicDisplay` and the `BasicDisplay` is considered valid.
    /// Finally, `default_screen` is used to dictate the default screen this display uses.
    ///
    /// In addition, this function also tries to establish `bigreq` support, and if it does it enables the
    /// corresponding variables in the `BasicDisplay.
    ///
    /// # Errors
    ///
    /// If the server returns error code 0, this function will return `BreadError::FailedToConnect`, indicating
    /// that the server refused our connection. If the server returns error code 2, this function will return
    /// `BreadError::FailedToAuthorize`, indicating that the `AuthInfo` provided either by the user or the
    /// "XAUTHORITY" is invalid. This function may also return an IO error if a system error occurs while
    /// communicating with the server.
    ///
    /// In addition, this function may also return `BreadError::BadObjectRead` if the `Setup` provided by the
    /// server is invalid, but this will likely never occur unless a malicious X11 server is used.
    #[cfg_attr(feature = "std", doc = "")]
    #[cfg_attr(feature = "std", doc = "# Example")]
    #[cfg_attr(feature = "std", doc = "")]
    #[cfg_attr(feature = "std", doc = "```rust,no_run")]
    #[cfg_attr(feature = "std", doc = "use breadx::display::BasicDisplay;")]
    #[cfg_attr(feature = "std", doc = "use std::net::TcpStream;")]
    #[cfg_attr(feature = "std", doc = "")]
    #[cfg_attr(feature = "std", doc = "# fn main() -> breadx::Result {")]
    #[cfg_attr(
        feature = "std",
        doc = "let server = TcpStream::connect(\"127.0.0.1:60000\")?;"
    )]
    #[cfg_attr(
        feature = "std",
        doc = "let conn = BasicDisplay::from_connection(server, 0, None)?;"
    )]
    #[cfg_attr(feature = "std", doc = "# Ok(())")]
    #[cfg_attr(feature = "std", doc = "# }")]
    #[cfg_attr(feature = "std", doc = "```")]
    #[inline]
    pub fn from_connection(
        connection: Conn,
        default_screen: usize,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let mut this = Self::from_connection_internal(connection, default_screen);
        let mut conn = this.connection.take().unwrap();
        let (setup, xid) = conn.establish(auth_info)?;
        this.connection = Some(conn);

        this.max_request_len = (setup.maximum_request_length as usize).saturating_mul(4);

        if let Some(max_request_len) = bigreq::try_bigreq(&mut this)? {
            this.bigreq_enabled = true;
            this.max_request_len = (max_request_len as usize).saturating_mul(4);
        }
        this.setup = setup;
        this.xid = xid;
        Ok(this)
    }
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection + Unpin> BasicDisplay<Conn> {
    /// Establishes this display using an inner connection type. This receives setup information from the server
    /// on the other side of the connection in order to populate the display. This method uses async types in
    /// order to ensure that the operation is non-blocking.
    ///
    /// See [`BasicDisplay::from_connection`] for more information.
    ///
    /// # Example
    ///
    /// ```rust,no_run
    /// use async_io::Async;
    /// use blocking::unblock;
    /// use breadx::display::BasicDisplay;
    /// use std::net::TcpStream;
    ///
    /// # fn main() -> breadx::Result {
    /// # futures_lite::future::block_on(async {
    /// let server = blocking::unblock(|| TcpStream::connect("127.0.0.1:60000")).await?;
    /// let server = Async::new(server)?;
    /// let conn = BasicDisplay::from_connection_async(server).await?;
    /// # Ok(())
    /// # })
    /// # }
    /// ```
    #[inline]
    pub async fn from_connection_async(
        connection: Conn,
        default_screen: usize,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let mut this = Self::from_connection_internal(connection, default_screen);
        let (setup, xid) = this
            .connection
            .as_mut()
            .unwrap()
            .establish_async(auth_info)
            .await?;
        this.max_request_len = (setup.maximum_request_length as usize).saturating_mul(4);

        if let Some(max_request_len) = bigreq::try_bigreq_async(&mut this).await? {
            this.bigreq_enabled = true;
            this.max_request_len = (max_request_len as usize).saturating_mul(4);
        }
        this.setup = setup;
        this.xid = xid;
        Ok(this)
    }
}

impl<Conn> DisplayBase for BasicDisplay<Conn> {
    #[inline]
    fn setup(&self) -> &StaticSetup {
        &self.setup
    }

    #[inline]
    fn default_screen_index(&self) -> usize {
        self.default_screen
    }

    #[inline]
    fn next_request_number(&mut self) -> u64 {
        let rn = self.request_number;
        self.request_number = self.request_number.wrapping_add(1);
        rn
    }

    #[inline]
    fn push_event(&mut self, event: Event) {
        self.event_queue.push_back(event)
    }

    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        self.event_queue.pop_front()
    }

    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        self.xid.next_xid()
    }

    #[inline]
    fn add_pending_item(&mut self, req_id: u16, item: PendingItem) {
        log_trace!("Adding pending item for {}: {:?}", req_id, &item);
        #[cfg(feature = "async")]
        {
            if let PendingItem::Request(ref pereq) = item {
                if matches!(pereq.flags.workaround, RequestWorkaround::GlxFbconfigBug) {
                    self.workarounders.push(req_id);
                }
            }
        }
        self.pending_items.insert(req_id, item);
    }

    #[inline]
    fn get_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.pending_items.get(&req_id).cloned()
    }

    #[inline]
    fn take_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        log_trace!("Removing pending item for {}", req_id);
        #[cfg(feature = "async")]
        self.workarounders.retain(|&r| r != req_id);
        self.pending_items.remove(&req_id)
    }

    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        self.special_event_queues.insert(xid, VecDeque::new());
    }

    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        match self.special_event_queues.get_mut(&xid) {
            Some(queue) => {
                queue.push_back(event);
                Ok(())
            }
            None => Err(event),
        }
    }

    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        self.special_event_queues
            .get_mut(&xid)
            .and_then(VecDeque::pop_front)
    }

    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.special_event_queues.remove(&xid);
    }

    #[inline]
    fn checked(&self) -> bool {
        self.checked
    }

    #[inline]
    fn set_checked(&mut self, checked: bool) {
        self.checked = checked;
    }

    #[inline]
    fn bigreq_enabled(&self) -> bool {
        self.bigreq_enabled
    }

    #[inline]
    fn max_request_len(&self) -> usize {
        self.max_request_len
    }

    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        self.extensions.get(key).copied()
    }

    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        self.extensions.insert(key, opcode);
    }

    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        self.wm_protocols_atom
    }

    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        self.wm_protocols_atom = Some(a);
    }
}

impl<Connect: Connection> Display for BasicDisplay<Connect> {
    #[inline]
    fn wait(&mut self) -> crate::Result {
        let mut conn = self.connection.take().expect("Poisoned!");
        let res = input::wait(self, &mut conn);
        self.connection = Some(conn);
        res
    }

    #[inline]
    fn send_request_raw(&mut self, request_info: RequestInfo) -> crate::Result<u16> {
        let mut conn = self.connection.take().expect("Poisoned!");
        let res = output::send_request(self, &mut conn, request_info);
        self.connection = Some(conn);
        res
    }
}

#[cfg(feature = "async")]
impl<Connect: AsyncConnection + Unpin> AsyncDisplay for BasicDisplay<Connect> {
    #[inline]
    fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut conn = self.connection.take().expect("Poisoned!");
        let workarounders = &self.workarounders;
        let res = self
            .wait_buffer
            .get_or_insert_with(WaitBuffer::default)
            .poll_wait(&mut conn, move |seq| workarounders.contains(&seq), cx);
        self.connection = Some(conn);
        let (bytes, fds) = match res {
            Poll::Ready(res) => {
                self.wait_buffer.take();
                match res {
                    Ok(WaitBufferReturn { data, fds }) => (data, fds),
                    Err(e) => return Poll::Ready(Err(e)),
                }
            }
            Poll::Pending => return Poll::Pending,
        };

        Poll::Ready(input::process_bytes(self, bytes, fds))
    }

    #[inline]
    fn begin_send_request_raw(
        &mut self,
        req: RequestInfo,
        _cx: &mut Context<'_>,
    ) -> PollOr<(), RequestInfo> {
        self.send_buffer.fill_hole(req);
        PollOr::Ready(())
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        let mut send_buffer = mem::replace(&mut self.send_buffer, SendBuffer::OccupiedHole);
        let mut conn = self.connection.take().expect("Poisoned!");
        let res = send_buffer.poll_send_request(self, &mut conn, cx);
        self.send_buffer = send_buffer;
        self.connection = Some(conn);

        if res.is_ready() {
            self.send_buffer.dig_hole();
        }
        match res {
            Poll::Ready(Ok(pr)) => Poll::Ready(Ok(output::finish_request(self, pr))),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// A variant of `BasicDisplay` that uses X11's default connection mechanisms to connect to the server. In
/// most cases, you should be using either this, or converting this type to a `CellDisplay` or `SyncDisplay`.
///
/// See `DisplayConnection::create` for how to instantiate this object.
#[cfg(feature = "std")]
pub type DisplayConnection = BasicDisplay<NameConnection>;

/// [`DisplayConnection`], but using an async connection type instead of the standard blocking one.
#[cfg(all(feature = "std", feature = "async"))]
pub type AsyncDisplayConnection = BasicDisplay<AsyncNameConnection>;

#[cfg(feature = "std")]
impl DisplayConnection {
    /// Create a new connection to the X server, given an optional name and authorization information.
    ///
    /// This function instantiates a new [`NameConnection`] using either the name specification string passed
    /// by the user, or the environment variable "DISPLAY" if the `name` parameter is `None`. The
    /// `NameConnection` is either a TCP stream connected to port `60000 + X` on the host, where `X` is the
    /// display number, or a Unix socket connection to one of the X11 system sockets.
    ///
    /// Once the `NameConnection` is created, it is passed into [`BasicDisplay::from_connection`] method to
    /// establish the `BasicDisplay`.
    ///
    /// This is the recommended way to create a `BasicDisplay` object, since most system running an X11 server
    /// expect the user to use the "DISPLAY" variable in order to connect to it. `from_connection` should only
    /// be used for niche use cases.
    ///
    /// # Errors
    ///
    /// In addition to the errors that `from_connection` can return, this method may return
    /// `BreadError::UnableToParseConnection` if the name string is improperly formatted, or an IO error if a
    /// system error occurs while trying to connect to the server.
    ///
    /// # Example
    ///
    /// Connect to the default X11 server, as specified by the environment variables.
    ///
    /// ```rust,no_run
    /// use breadx::DisplayConnection;
    ///
    /// # fn main() -> breadx::Result {
    /// let conn = DisplayConnection::create(None, None)?;
    /// # Ok(())
    /// # }
    /// ```
    ///
    /// Connect to the server on display 3, using the authorization info read from a file.
    ///
    /// ```rust,no_run
    /// use breadx::{AuthInfo, DisplayConnection};
    /// use std::fs::File;
    ///
    /// # fn main() -> breadx::Result {
    /// let mut file = File::open("my_auth_info.txt")?;
    /// let mut auth_info = AuthInfo::from_stream(&mut file).expect("AuthInfo not found");
    /// let conn = DisplayConnection::create(Some(":3".into()), Some(auth_info.remove(0)))?;
    /// # Ok(())
    /// # }
    /// ```
    #[inline]
    pub fn create(name: Option<Cow<'_, str>>, auth_info: Option<AuthInfo>) -> crate::Result<Self> {
        let (connection, screen) = NameConnection::connect_internal(name)?;
        Self::from_connection(connection, screen, auth_info)
    }
}

#[cfg(all(feature = "std", feature = "async"))]
impl AsyncDisplayConnection {
    /// Create a new connection to the X server, given an optional name and authorization information, async
    /// redox. See `DisplayConnection::create()` for more information regarding this function.
    #[inline]
    pub async fn create_async(
        name: Option<Cow<'_, str>>,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let (connection, screen) = AsyncNameConnection::connect_internal_async(name).await?;
        Self::from_connection_async(connection, screen, auth_info).await
    }
}
