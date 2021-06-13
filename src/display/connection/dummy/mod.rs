// MIT/Apache2 License

#![cfg(test)]
//! Provides a dummy connection to be used for doctests. See the `DummyConnection` object for more information.

mod setup;

use super::Connection;
use crate::{display::{BasicDisplay, StaticSetup}, auto::{AsByteSequence, xproto::{SetupRequest, Setup}}};
use alloc::collections::VecDeque;
use core::iter;
use tinyvec::TinyVec;

/// An imitation connection to a fake X11 server.
/// 
/// In order to run doctests, we need to instantiate a connection to the X11 server. Unfortunately, this tends to
/// be problematic, especially on computers without X servers (e.g. Windows computers or C.I. services). This
/// connection is intended to ensure the utility methods on `Display` extension traits are actually doing what
/// they say they are.
/// 
/// This connection can `expect` to receive a certain request, or `reply` with a certain reply, event, or error.
/// When bytes are sent across the connection, it compares them to the expected bytes, and panics if they do not
/// match up. When bytes are requested from the connection, it `reply`s with the set bytes, or with zeroes if
/// it is out of `reply` bytes.
/// 
/// By using this, the only thing left to chance are the actual implementations of the connections themselves,
/// but those can't actually be tested without an X server.
#[derive(Debug)]
pub struct DummyConnection {
    /// Bytes that we expect to see from the server.
    expected: VecDeque<u8>,
    /// Bytes that we are sending from the server.
    reply: VecDeque<u8>,
}

impl DummyConnection {
    /// Create a new `DummyConnection`, with none of the defaults.
    #[inline]
    pub fn with_no_defaults() -> Self {
        Self { expected: VecDeque::new(), reply: VecDeque::new() }
    }

    /// Expect to see a certain sequence of bytes.
    #[inline]
    pub fn expects_bytes<I: IntoIterator<Item = u8>>(&mut self, bytes: I) {
        self.expected.extend(bytes);
    }

    /// Reply with a certain sequence of bytes.
    #[inline]
    pub fn reply_bytes<I: IntoIterator<Item = u8>>(&mut self, bytes: I) {
        self.reply.extend(bytes);
    }

    /// Expect to see a certain object.
    #[inline]
    pub fn expects<O: AsByteSequence>(&mut self, object: O) {
        let mut data: TinyVec<[u8; 32]> = iter::repeat(0).take(object.size()).collect();
        object.as_bytes(&mut data);
        self.expects_bytes(data);
    }

    /// Reply with a certain object.
    #[inline]
    pub fn reply<O: AsByteSequence>(&mut self, object: O) {
        let mut data: TinyVec<[u8; 32]> = iter::repeat(0).take(object.size()).collect();
        object.as_bytes(&mut data);
        self.reply_bytes(data)
    }

    /// Create a new `DummyConnection`. By default, it expects a `SetupRequest` and replies with a `Setup`, since
    /// that is what you need to setup an X11 connection.
    #[inline]
    pub fn new() -> Self {
        let mut this = Self::with_no_defaults();
        this.expect(SetupRequest {
            byte_order: if cfg!(target_endian = "little") { b"l" } else { b"B" },
            protocol_major_version: 11,
            protocol_minor_version: 0,
            authorization_protocol_name: "".into(),
            authorization_protocol_data: &[].into(),
        });
        this.reply(default_setup());
        this
    }
}

impl BasicDisplay<DummyConnection> {
    /// Create a new `BasicDisplay` based on a dummy connection.
    #[inline]
    pub fn dummy() -> Self {
        Self::from_connection(DummyConnection::new()).expect("Failed to create dummy display")
    }
}

#[inline]
fn default_setup() -> StaticSetup {
    // close to the default setup on my system
    setup::default_setup()
}
