// MIT/Apache2 License

use crate::{
    auth_info::AuthInfo,
    auto::{
        xc_misc::GetXIDRangeRequest,
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
use core::{
    fmt, iter,
    marker::PhantomData,
    mem::{self, MaybeUninit},
    task::Waker,
};
use hashbrown::HashMap;
use tinyvec::{tiny_vec, ArrayVec, TinyVec};

#[cfg(feature = "async")]
use async_std::prelude::*;

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
    _phantom: PhantomData<MaybeUninit<R::Reply>>,
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
fn endian_byte() -> u8 {
    const TEST: u32 = 0xABCD;
    const NATURAL_ENDIAN: [u8; 4] = TEST.to_ne_bytes();
    const BE: [u8; 4] = TEST.to_be_bytes();
    const LE: [u8; 4] = TEST.to_le_bytes();
    // Excerpt from the X Window System Protocol
    //
    // The client must send an initial byte of data to identify the byte order to be employed.
    // The value of the byte must be octal 102 or 154. The value 102 (ASCII uppercase B) means
    // values are transmitted most significant byte first, and value 154 (ASCII lowercase l)
    // means values are transmitted least significant byte first.
    const BE_SIGNIFIER: u8 = b'B';
    const LE_SIGNIFIER: u8 = b'l';

    if NATURAL_ENDIAN == BE {
        BE_SIGNIFIER
    } else {
        LE_SIGNIFIER
    }
}

impl<Conn: Connection> Display<Conn> {
    #[inline]
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
                Some(reply) => {
                    break Ok(R::Reply::from_bytes(&reply)
                        .ok_or_else(|| crate::Error::BadObjectRead)?
                        .0)
                }
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
            match self.pending_replies.remove(token.sequence) {
                Some(reply) => break Ok(reply),
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
            request_number: 1,
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
    pub async fn from_connection_async(connection: Conn) -> crate::Result<Self> {
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
        self.connection.send_packet(&bytes[0..len])?;
        let mut bytes: TinyVec<[u8; 8]> = cycled_zeroes(8);
        self.connection.read_packet(&mut bytes)?;

        match bytes[0] {
            0 | 2 => return Err(crate::Error::FailedToConnect),
            _ => (),
        }

        // read in the rest of the bytes
        let mut length_bytes: [u8; 2] = [bytes[6], bytes[7]];
        let length = (u16::from_ne_bytes(length_bytes) as usize) * 4;
        bytes.extend(iter::once(0).cycle().take(length));
        self.connection.read_packet(&mut bytes[8..])?;

        self.setup = Setup::from_bytes(&bytes)
            .ok_or_else(|| crate::Error::BadObjectRead)?
            .0;
        self.xid = XidGenerator::new(self.setup.resource_id_base, self.setup.resource_id_mask);

        Ok(())
    }

    /// Initialize the setup.
    #[cfg(feature = "async")]
    #[inline]
    async fn init_async(&mut self, auth: AuthInfo) -> crate::Result {
        let setup = Self::setup(auth);
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(32);
        let len = setup.as_bytes(&mut bytes);
        self.connection.send_packet_async(&bytes[0..len]).await?;
        let mut bytes: TinyVec<[u8; 8]> = cycled_zeroes(8);
        self.connection.read_packet_async(&mut bytes).await?;

        match bytes[0] {
            0 | 2 => return Err(crate::Error::FailedToConnect),
            _ => (),
        }

        // read in the rest of the bytes
        let mut length_bytes: [u8; 2] = [bytes[6], bytes[7]];
        let length = (u16::from_ne_bytes(length_bytes) as usize) * 4;
        bytes.extend(iter::once(0).cycle().take(length));
        self.connection.read_packet_async(&mut bytes[8..]).await?;

        self.setup = Setup::from_bytes(&bytes)
            .ok_or_else(|| crate::Error::BadObjectRead)?
            .0;
        self.xid = XidGenerator::new(self.setup.resource_id_base, self.setup.resource_id_mask);

        Ok(())
    }

    #[inline]
    pub(crate) fn default_root(&self) -> Window {
        self.setup.roots[self.default_screen].root
    }

    #[inline]
    fn eval_xid(&mut self) -> XID {
        self.xid.last | self.xid.base
    }

    /// Create another XID.
    #[inline]
    pub fn generate_xid(&mut self) -> crate::Result<XID> {
        // Algorithm based on: https://gitlab.freedesktop.org/xorg/lib/libxcb/-/blob/master/src/xcb_xid.c
        if self.xid.last >= self.xid.max.wrapping_sub(self.xid.inc).wrapping_add(1) {
            // we need to set up the initial range
            assert_eq!(self.xid.last, self.xid.max);
            if self.xid.last == 0 {
                self.xid.max = self.setup.resource_id_mask;
            } else {
                // query for the XC_MISC extension
                let query_for_xc_misc = QueryExtensionRequest {
                    name: "XC-MISC".to_owned(),
                };
                let tok = self.send_request(query_for_xc_misc)?;
                let query_for_xc_misc = self.resolve_request(tok)?;

                if (query_for_xc_misc.present == 0) {
                    return Err(crate::Error::ExtensionNotPresent("XcMisc"));
                }

                // now that we know XC_MISC exists, get the range reply
                let tok = self.send_request(GetXIDRangeRequest)?;
                let range = self.resolve_request(tok)?;

                if range.start_id == 0 && range.count == 1 {
                    return Err(crate::Error::StaticMsg("Unable to get XID range"));
                }

                self.xid.last = range.start_id;
                self.xid.max = range.start_id + (range.count - 1) * self.xid.inc;
            }
        } else {
            self.xid.last = self.xid.last.wrapping_add(self.xid.inc);
        }

        Ok(self.eval_xid())
    }

    /// Create an XID, async redox
    #[cfg(feature = "async")]
    #[inline]
    pub async fn generate_xid_async(&mut self) -> crate::Result<XID> {
        // Algorithm based on: https://gitlab.freedesktop.org/xorg/lib/libxcb/-/blob/master/src/xcb_xid.c
        if self.xid.last >= self.xid.max.wrapping_sub(self.xid.inc).wrapping_add(1) {
            // we need to set up the initial range
            assert_eq!(self.xid.last, self.xid.max);
            if self.xid.last == 0 {
                self.xid.max = self.setup.resource_id_mask;
            } else {
                // query for the XC_MISC extension
                let query_for_xc_misc = QueryExtensionRequest {
                    name: "XC-MISC".to_owned(),
                };

                log::debug!("Sending QueryExtensionRequest to server");
                let tok = self.send_request_async(query_for_xc_misc).await?;
                let query_for_xc_misc = self.resolve_request_async(tok).await?;
                log::debug!("Sent QueryExtensionRequest to server");

                if (query_for_xc_misc.present == 0) {
                    return Err(crate::Error::ExtensionNotPresent("XcMisc"));
                }

                // now that we know XC_MISC exists, get the range reply
                log::debug!("Sending GetXIDRangeRequest to server");
                let tok = self.send_request_async(GetXIDRangeRequest).await?;
                let range = self.resolve_request_async(tok).await?;
                log::debug!("Sent GetXIDRangeRequest to server");

                if range.start_id == 0 && range.count == 1 {
                    return Err(crate::Error::StaticMsg("Unable to get XID range"));
                }

                self.xid.last = range.start_id;
                self.xid.max = range.start_id + (range.count - 1) * self.xid.inc;
            }
        } else {
            self.xid.last += self.xid.inc;
        }

        Ok(self.eval_xid())
    }

    /// Wait for an event.
    #[inline]
    pub fn wait_for_event(&mut self) -> crate::Result<Event> {
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

// old stuff:
/*
    #[inline]
    fn write_object_blocking<O: AsByteSequence>(&mut self, o: &O) -> crate::Result {
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(O::size());
        o.as_bytes(&mut bytes);
        self.connection.send_packet(&bytes)?;
        Ok(())
    }

    #[inline]
    fn read_object_blocking<O: AsByteSequence>(&mut self) -> crate::Result<O> {
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(O::size());
        self.connection.read_packet(&mut bytes)?;
        let (object, _) = O::from_bytes(&bytes).ok_or_else(|| crate::Error::BadObjectRead)?;
        Ok(object)
    }

    #[cfg(feature = "async")]
    #[inline]
    async fn write_object_async<O: AsByteSequence>(&mut self, o: &O) -> crate::Result {
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(O::size());
        o.as_bytes(&mut bytes);
        self.connection.send_packet_async(&bytes).await?;
        Ok(())
    }

    #[cfg(feature = "async")]
    #[inline]
    async fn read_object_async<O: AsByteSequence>(&mut self) -> crate::Result<O> {
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(O::size());
        self.connection.read_packet_async(&mut bytes).await?;
        let (object, _) = O::from_bytes(&bytes).ok_or_else(|| crate::Error::BadObjectRead)?;
        Ok(())
    }

    /// Create a new display from a connection object.
    #[inline]
    pub fn from_connection(conn: Conn, auth_info: Option<AuthInfo>) -> crate::Result<Self> {
        let mut s = Self {
            connection: conn,
            setup: None,
            xid: None,
            default_screen: 0,
            event_queue: VecDeque::new(),
        };
        s.init(match auth_info {
            Some(ai) => ai,
            None => Default::default(),
        })?;
        Ok(s)
    }

    /// Create a new display from a connection object, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn from_connection_async(
        conn: Conn,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let mut s = Self {
            connection: conn,
            setup: None,
            xid: None,
            default_screen: 0,
            event_deque: VecDeque::new(),
        };
        s.init_async(match auth_info {
            Some(ai) => ai,
            None => Default::default(),
        })
        .await?;
        Ok(s)
    }

    /// Encode a request into bytes format.
    #[inline]
    fn encode_request<R: Request>(request: &R) -> TinyVec<[u8; 64]> {
        // Excerpt from the X Protocol Specification:
        //
        // Every request contains an 8-bit major opcode and a 16-bit length field expressed
        // in units of four bytes. Every request consists of four bytes of a header (containing
        // the major opcode, the length field, and a data byte) followed by zero or more additional
        // bytes of data. The length field defines the total length of the request, including
        // the header. The length field in a request must equal the minimum length required
        // to contain the request. If the specified length is smaller or larger than the
        // required length, an error is generated. Unused bytes in a request are not required
        // to be zero. Major opcodes 128 through 255 are reserved for extensions. Extensions
        // are intended to contain multiple requests, so extension requests typically have an
        // additional minor opcode encoded in the second data byte in the request header
        let mut bytes: TinyVec<[u8; 64]> = cycled_zeroes(R::size() + 4);
        bytes[0] = request.opcode();
        let len = request.as_bytes(&mut bytes[4..]) as u16;
        let len_bytes = len.to_ne_bytes();
        bytes[1] = len_bytes[0];
        bytes[2] = len_bytes[1];
        // TODO: implement minor opcode
        bytes
    }

    /// Decode bytes into a reply.
    #[inline]
    fn decode_reply_header(bytes: &[u8]) -> (u8, u16, u32) {
        // Excerpt from the X Protocol Specification:
        //
        // Every reply contains a 32-bit length field expressed in units of four bytes.
        // Every reply consists of 32 bytes followed by zero or more additional bytes of data,
        // as specified in the length field. Unused bytes within a reply are not guaranteed to be
        // zero. Every reply also contains the least significant 16 bits of the sequence number
        // of the corresponding request.

        // first byte is the opcode, second is padding
        let opcode = bytes[0];
        // next two bytes are the sequence
        let mut sequence_bytes: [u8; 2] = [0, 0];
        sequence_bytes.copy_from_slice(&bytes[2..3]);
        let sequence = u16::from_ne_bytes(sequence_bytes);

        // next four bytes are the length
        let mut length_bytes: [u8; 4] = [0; 4];
        length_bytes.copy_from_slice(&bytes[4..8]);
        let length = u32::from_ne_bytes(length_bytes);

        (opcode, sequence, length)
    }

    /// Decode bytes into a reply.
    #[inline]
    fn decode_reply<O: AsByteSequence>(bytes: &[u8]) -> crate::Result<O> {
        match O::from_bytes(bytes) {
            Some((bytes, _)) => Ok(bytes),
            None => Err(crate::Error::BadObjectRead),
        }
    }

    /// Send a request.
    #[inline]
    pub fn send_request<R: Request>(&mut self, request: &R) -> crate::Result<R::Reply>
    where
        R::Reply: Default,
    {
        let mut bytes = Self::encode_request(request);

        // send the bytes than block for receive
        self.connection.send_packet(&bytes)?;

        // a zero-sized reply indicates we're done here
        if core::mem::size_of::<R::Reply>() == 0 {
            return Ok(Default::default());
        }

        // read the bytes
        bytes.clear();
        bytes.extend(iter::once(0).cycle().take(32));

        self.connection.read_packet(&mut bytes)?;

        // decode the sequence
        let (opcode, sequence, length) = Self::decode_reply_header(&bytes);

        // read the remaining bytes
        if length > 0 {
            bytes.extend(iter::once(0).cycle().take(length as _));
            self.connection.read_packet(&mut bytes[32..])?;
        }

        Self::decode_reply(&bytes[2..])
    }

    /// Send a request.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn send_request_async<R: Request>(&mut self, request: &R) -> crate::Result<R::Reply> {
        let mut bytes = Self::encode_request(request);

        // send the bytes than block for receive
        self.connection.send_packet_async(&bytes).await?;

        // a zero-sized reply indicates we're done here
        if core::mem::size_of::<R::Reply>() == 0 {
            return Ok(Default::default());
        }

        // read the bytes
        bytes.clear();
        bytes.extend(iter::once(0).cycle().take(32));

        self.connection.read_packet_async(&mut bytes).await?;

        // decode the sequence
        let (opcode, sequence, length) = Self::decode_reply_header(&bytes);

        // read the remaining bytes
        if length > 0 {
            bytes.extend(iter::once(0).cycle().take(length as _));
            self.connection.read_packet_async(&mut bytes[32..]).await?;
        }

        Self::decode_reply(&bytes[2..])
    }


    /// Create a new window.
    #[inline]
    pub fn create_window(
        &mut self,
        parent: Option<Window>,
        depth: Option<u8>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        class: WindowClass,
        visual: Visualid,
    ) -> crate::Result<Window> {
        let wid = self.next_xid()?;
        let create_window = CreateWindowRequest {
            depth: depth.unwrap(),
            x,
            y,
            wid: Window::const_from_xid(wid),
            parent: parent.unwrap_or(Window::const_from_xid(0)),
            width,
            height,
            border_width,
            class: class as _,
            visual,
        };
        self.send_request(&create_window)?;
        Ok(Window::const_from_xid(wid))
    }

    /// Wait for the next event.
    #[inline]
    pub fn wait_for_event(&mut self) -> crate::Result<Event> {
        // Events are 32 bytes long.
        let mut evbytes: ArrayVec<[u8; 32]> = [0; 32].into();
        self.connection.read_packet(&mut evbytes)?;
        Event::read_from_bytes(&evbytes)
    }

    /// Wait for the next event, async redox.
    #[inline]

*/
