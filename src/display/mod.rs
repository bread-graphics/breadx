// MIT/Apache2 License

use crate::{
    auth_info::AuthInfo,
    auto::{xproto::SetupRequest, AsByteSequence},
    Request,
};
use alloc::{borrow::Cow, boxed::Box};
use core::iter;
use tinyvec::{tiny_vec, TinyVec};

#[cfg(feature = "async")]
use async_std::prelude::*;

mod connection;
pub use connection::*;
#[cfg(feature = "std")]
pub mod name;

/// Connection to the X11 server.
pub struct Display<Conn> {
    connection: Conn,
}

// equality isn't supported in where clauses yet
trait IsAU8: Default + Clone {
    #[inline]
    fn zero() -> Self;
}

impl IsAU8 for u8 {
    #[inline]
    fn zero() -> Self {
        0
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

#[inline]
fn cycled_zeroes<U: IsAU8, A: tinyvec::Array<Item = U>>(len: usize) -> TinyVec<A> {
    iter::once(U::zero()).cycle().take(len).collect()
}

impl<Conn: Connection> Display<Conn> {
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
        let object =
            O::from_bytes(&bytes).ok_or_else(|| crate::Error::BadObjectRead(stringify!(O)))?;
        Ok(())
    }

    /// Create a new display from a connection object.
    #[inline]
    pub fn from_connection(conn: Conn, auth_info: Option<AuthInfo>) -> crate::Result<Self> {
        let mut s = Self { connection: conn };
        s.init(match auth_info {
            Some(ai) => ai,
            None => Default::default(),
        })?;
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
        let mut bytes: TinyVec<[u8; 33]> = cycled_zeroes(R::size() + 1);
        bytes[0] = R::opcode();
        request.as_bytes(&mut bytes[1..]);

        // send the bytes than block for receive
        self.connection.write_bytes(&bytes).await?;

        // read the bytes
        bytes.clear();
        bytes.extend(iter::once(0).cycle().take(32));
        self.connection.read_packet(&mut bytes).await?;
        R::Reply::from_bytes(&bytes[1..])
            .ok_or_else(|| crate::Error::BadObjectRead(stringify!(R::Reply)))
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
        self.connection.send_packet(&bytes[0..len]);
        let mut bytes: TinyVec<[u8; 8]> = cycled_zeroes(8);
        self.connection.read_packet(&mut bytes)?;
        println!("Result bytes: {}", &bytes);

        match bytes[0] {
            0 | 2 => Err(crate::Error::FailedToConnect),
            _ => Ok(()),
        }
    }

    /// Initialize the setup.
    #[cfg(feature = "async")]
    #[inline]
    async fn init_async(&mut self, auth: AuthInfo) -> crate::Result {
        let setup = Self::setup(auth);
        self.write_object_async(&setup).await?;
        let mut bytes: TinyVec<[u8; 8]> = TinyVec::with_capacity(8);
        self.connection.read_packet_async(&mut bytes).await?;

        match bytes[0] {
            0 | 2 => Err(crate::Error::FailedToConnect),
            _ => Ok(()),
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
    pub async fn create_async<'a, IC: Into<Cow<'a, str>>, IO: Into<Option<IC>>>(
        name: IO,
        auth_info: Option<AuthInfo>,
    ) -> crate::Result<Self> {
        let name: Option<Cow<'_, str>> = name.into().map(|c| c.into());
        let connection = name::NameConnection::connect_internal_async(name).await?;
        Self::from_connection_async(connection, auth_info).await
    }
}
