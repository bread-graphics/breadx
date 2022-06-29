// MIT/Apache2 License

use crate::{Fd, connection::with_test_connection, display::{RawRequest, from_void_request, from_reply_request, from_reply_fds_request}};
use super::test_setup;
use alloc::vec::Vec;
use x11rb_protocol::{x11_utils::{VoidRequest, ReplyRequest, ReplyFDsRequest, TryParseFd, Serialize}, protocol::xproto::SetupRequest};

/// An expected action from either the client or the server.
#[derive(Debug)]
pub(crate) struct Action {
    sent_from_client: Vec<u8>,
    fds_from_client: usize,
    sent_from_server: Vec<u8>,
    fds_from_server: usize,
    dummy_max_len: usize,
    sequence: u64,
}

impl Action {
    /// Creates a new, empty `Action`.
    pub(crate) fn new() -> Self {
        Self {
            sent_from_client: Vec::new(),
            sent_from_server: Vec::new(),
            fds_from_client: 0,
            fds_from_server: 0,
            dummy_max_len: core::u16::MAX as usize * 4,
            sequence: 1,
        }
    }

    /// Receive bytes from the client.
    pub(crate) fn sent_from_client(&mut self, bytes: impl AsRef<[u8]>) -> &mut Self {
        self.sent_from_client.extend_from_slice(bytes.as_ref());
        self
    }

    /// Receive bytes from the server.
    pub(crate) fn sent_from_server(&mut self, bytes: impl AsRef<[u8]>) -> &mut Self {
        self.sent_from_server.extend_from_slice(bytes.as_ref());
        self
    }

    /// Update the maximum length.
    pub(crate) fn update_max_len(&mut self, max_len: usize) -> &mut Self {
        self.dummy_max_len = max_len;
        self
    }

    /// Indicate that the client should send a raw request.
    pub(crate) fn raw_request(&mut self, mut raw_req: RawRequest<'_, '_>) -> &mut Self {
        let (slices, fds) = raw_req.mut_parts();

        for (i, slice) in slices.iter().enumerate() {
            self.sent_from_client.extend_from_slice(slice);
        }

        self.fds_from_client += fds.len();

        self
    }

    /// Indicate that the client should send a request with a void
    /// reply.
    pub(crate) fn void_request(
        &mut self, 
        void: impl VoidRequest,
        discard_reply: bool,
        ext_opcode: Option<u8>
    ) -> &mut Self {
        from_void_request(void, discard_reply, move |mut req| {
            req.format(ext_opcode, self.dummy_max_len).expect("Failed to format request");
            self.raw_request(req)
        })
    }

    /// Indicate that the client should send a request with a reply.
    pub(crate) fn reply_request(
        &mut self, 
        reply: impl ReplyRequest,
        ext_opcode: Option<u8>
    ) -> &mut Self {
        from_reply_request(reply, move |mut req| {
            req.format(ext_opcode, self.dummy_max_len).expect("Failed to format request");
            self.raw_request(req)
        })
    }

    /// Indicate that the client should send a request with a reply
    /// and FDs.
    pub(crate) fn reply_fds_request(
        &mut self, 
        reply: impl ReplyFDsRequest,
        ext_opcode: Option<u8>
    ) -> &mut Self {
        from_reply_fds_request(reply, move |mut req| {
            req.format(ext_opcode, self.dummy_max_len).expect("Failed to format request");
            self.raw_request(req)
        })
    }

    /// Write a setup request from the client.
    pub(crate) fn setup_request(
        &mut self,
        sr: &SetupRequest,
    ) -> &mut Self {
        let bytes = sr.serialize();
        self.sent_from_client.extend_from_slice(&bytes);
        self
    }

    /// Write the testing setup from the server.
    pub(crate) fn respond_with_setup(
        &mut self,
    ) -> &mut Self {
        let bytes = test_setup().serialize();
        self.sent_from_server.extend_from_slice(&bytes);
        self
    }
}