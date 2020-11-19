// MIT/Apache2 License

use crate::{
    auth_info::AuthInfo,
    auto::{
        xc_misc::GetXidRangeRequest,
        xproto::{
            CreateWindowRequest, QueryExtensionRequest, Setup, SetupRequest, Visualid, Window,
            WindowClass,
        },
        AsByteSequence,
    },
    event::Event,
    util::cycled_zeroes,
    xid::XidGenerator,
    Request, XID,
};
use alloc::{
    borrow::{Cow, ToOwned},
    boxed::Box,
    collections::VecDeque,
    vec::Vec,
};
use core::{fmt, iter, marker::PhantomData, mem, task::Waker};
use hashbrown::HashMap;
use tinyvec::{tiny_vec, ArrayVec, TinyVec};

#[cfg(feature = "async")]
use futures_io::{AsyncRead, AsyncWrite};

mod connection;
pub use connection::*;
#[cfg(feature = "std")]
pub mod name;

mod functions;
mod input;
mod output;

pub use functions::*;

const UNINIT_ERR: &str = "Display was not yet initialized. This is most likely an internal error.";

/// Connection to the X11 server.
pub struct Display<Conn> {
    // the connection to the server
    pub(crate) connection: Conn,

    // the setup received from the server
    pub(crate) setup: Setup,

    // xid generator
    xid: XidGenerator,

    // the screen to be used by default
    default_screen: usize,

    // input variables
    pub(crate) event_queue: VecDeque<Event>,
    pub(crate) pending_requests: VecDeque<input::PendingRequest>,
    pub(crate) pending_replies: HashMap<u64, Box<[u8]>>,

    // output variables
    request_number: u64,
}

/// Request cookie.
#[derive(Copy, Clone, Debug, PartialOrd, Ord, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct RequestCookie<R: Request> {
    sequence: u64,
    _phantom: PhantomData<Option<R::Reply>>,
}

impl<R: Request> RequestCookie<R> {
    #[inline]
    pub(crate) fn from_sequence(sequence: u64) -> Self {
        Self {
            sequence,
            _phantom: PhantomData,
        }
    }
}

#[derive(Default, Debug)]
pub(crate) struct PendingRequestFlags {
    pub discard_reply: bool,
    pub checked: bool,
}

impl<Conn> fmt::Debug for Display<Conn> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        struct ConnDummy;

        impl fmt::Debug for ConnDummy {
            #[inline]
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                f.write_str("X11 Server Connection")
            }
        }

        f.debug_struct("Display")
            .field("connection", &ConnDummy)
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

impl<Conn: Connection> Display<Conn> {
    #[inline]
    fn decode_reply<R: Request>(reply: Box<[u8]>) -> crate::Result<R::Reply> {
        Ok(R::Reply::from_bytes(&reply)
            .ok_or_else(|| crate::BreadError::BadObjectRead(None))?
            .0)
    }

    #[inline(always)]
    pub fn send_request<R: Request>(&mut self, req: R) -> crate::Result<RequestCookie<R>> {
        self.send_request_internal(req)
    }

    #[inline]
    pub fn resolve_request<R: Request>(
        &mut self,
        token: RequestCookie<R>,
    ) -> crate::Result<R::Reply>
    where
        R::Reply: Default,
    {
        if mem::size_of::<R::Reply>() == 0 {
            log::debug!("Immediately resolving for reply of size 0");
            return Ok(Default::default());
        }

        loop {
            match self.pending_replies.remove(&token.sequence) {
                Some(reply) => break Self::decode_reply::<R>(reply),
                None => self.wait()?,
            }
        }
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn send_request_async<R: Request>(
        &mut self,
        req: R,
    ) -> crate::Result<RequestCookie<R>> {
        self.send_request_internal_async(req).await
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn resolve_request_async<R: Request>(
        &mut self,
        token: RequestCookie<R>,
    ) -> crate::Result<R::Reply>
    where
        R::Reply: Default,
    {
        if mem::size_of::<R::Reply>() == 0 {
            return Ok(Default::default());
        }

        loop {
            match self.pending_replies.remove(&token.sequence) {
                Some(reply) => {
                    break Self::decode_reply::<R>(reply);
                }
                None => self.wait_async().await?,
            }
        }
    }

    #[inline]
    fn from_connection_internal(connection: Conn) -> Self {
        Self {
            connection,
            setup: Default::default(),
            xid: Default::default(),
            default_screen: 0,
            event_queue: VecDeque::new(),
            pending_requests: VecDeque::new(),
            pending_replies: HashMap::new(),
            request_number: 0,
        }
    }

    #[inline]
    pub fn from_connection(connection: Conn, auth: Option<AuthInfo>) -> crate::Result<Self> {
        let mut d = Self::from_connection_internal(connection);
        d.init(auth.unwrap_or(Default::default()))?;
        Ok(d)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn from_connection_async(
        connection: Conn,
        auth: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let mut d = Self::from_connection_internal(connection);
        d.init_async(auth.unwrap_or(Default::default())).await?;
        Ok(d)
    }

    /// Generate the setup from the authentication info.
    #[inline]
    fn setup(auth: AuthInfo) -> SetupRequest {
        let AuthInfo { name, data } = auth;
        SetupRequest {
            byte_order: endian_byte(),
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: name,
            authorization_protocol_data: data,
        }
    }

    /// Initialize the setup.
    #[inline]
    fn init(&mut self, auth: AuthInfo) -> crate::Result {
        let setup = Self::setup(auth);
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(32);
        let len = setup.as_bytes(&mut bytes);
        bytes.truncate(len);
        self.connection.send_packet(&bytes[0..len])?;
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(8);
        self.connection.read_packet(&mut bytes)?;

        match bytes[0] {
            0 | 2 => return Err(crate::BreadError::FailedToConnect),
            _ => (),
        }

        // read in the rest of the bytes
        let mut length_bytes: [u8; 2] = [bytes[6], bytes[7]];
        let length = (u16::from_ne_bytes(length_bytes) as usize) * 4;
        bytes.extend(iter::once(0).cycle().take(length));
        self.connection.read_packet(&mut bytes[8..])?;

        let (setup, slen) = Setup::from_bytes(&bytes)
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

    /// Initialize the setup.
    #[cfg(feature = "async")]
    #[inline]
    async fn init_async(&mut self, auth: AuthInfo) -> crate::Result {
        let setup = Self::setup(auth);
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(32);
        let len = setup.as_bytes(&mut bytes);
        bytes.truncate(len);
        self.connection.send_packet_async(&bytes[0..len]).await?;
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(8);
        self.connection.read_packet_async(&mut bytes).await?;

        match bytes[0] {
            0 | 2 => return Err(crate::BreadError::FailedToConnect),
            _ => (),
        }

        // read in the rest of the bytes
        let mut length_bytes: [u8; 2] = [bytes[6], bytes[7]];
        let length = (u16::from_ne_bytes(length_bytes) as usize) * 4;
        bytes.extend(iter::once(0).cycle().take(length));
        self.connection.read_packet_async(&mut bytes[8..]).await?;

        let (setup, slen) = Setup::from_bytes(&bytes)
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

    #[inline]
    pub(crate) fn default_root(&self) -> Window {
        self.setup.roots[self.default_screen].root
    }

    /// Create another XID.
    #[inline]
    pub fn generate_xid(&mut self) -> crate::Result<XID> {
        Ok(self.xid.next().unwrap())
    }

    /// Create an XID, async redox
    #[cfg(feature = "async")]
    #[inline]
    pub async fn generate_xid_async(&mut self) -> crate::Result<XID> {
        Ok(self.xid.next().unwrap())
    }

    /// Wait for an event.
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

    /// Wait for an event, async redox.
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
}

#[cfg(feature = "std")]
pub type DisplayConnection = Display<name::NameConnection>;

#[cfg(feature = "std")]
impl DisplayConnection {
    /// Create a new connection from a name.
    #[inline]
    pub fn create(name: Option<Cow<'_, str>>, auth_info: Option<AuthInfo>) -> crate::Result<Self> {
        let connection = name::NameConnection::connect_internal(name)?;
        Self::from_connection(connection, auth_info)
    }

    /// Create a new connection from a name.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_async(
        name: Option<Cow<'_, str>>,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let connection = name::NameConnection::connect_internal_async(name).await?;
        Self::from_connection_async(connection, auth_info).await
    }
}
