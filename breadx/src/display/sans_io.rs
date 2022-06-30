// MIT/Apache2 License

use crate::{Error, Fd, InvalidState, Result};
use alloc::vec::Vec;
use x11rb_protocol::{
    connection::{Connection as ProtoConnection, PollReply, ReplyFdKind},
    id_allocator::{IdAllocator, IdsExhausted},
    protocol::{xc_misc::GetXIDRangeReply, xproto::Setup, Event},
    x11_utils::{ExtInfoProvider, X11Error},
    DiscardMode,
};

use super::RawReply;

/// The core of all of the `breadx` displays.
///
/// This structure is configured such that all other `Display` implementors
/// build around it.
pub(crate) struct X11Core {
    /// The inner sans-I/O implementation of X11.
    proto: ProtoConnection,
    /// Keeps track of the XIDs we have allocated.
    id_allocator: IdAllocator,
}

impl X11Core {
    /// Create a new `X11Core` object wrapping around a `Setup`.
    pub(crate) fn from_setup(setup: &Setup) -> Result<Self> {
        Ok(Self {
            proto: ProtoConnection::new(),
            id_allocator: IdAllocator::new(setup.resource_id_base, setup.resource_id_mask)
                .map_err(Error::make_connect_error)?,
        })
    }

    /// Enqueue file descriptors into this connection.
    pub(crate) fn enqueue_fds(&mut self, fds: Vec<Fd>) {
        self.proto.enqueue_fds(fds);
    }

    /// Enqueue a packet into this connection.
    pub(crate) fn enqueue_packet(&mut self, packet: Vec<u8>) {
        self.proto.enqueue_packet(packet);
    }

    #[allow(clippy::unused_self)]
    fn try_parse_error(&self, err: Vec<u8>, ext_info: &dyn ExtInfoProvider) -> Result<Vec<u8>> {
        // first number has to be zero
        if err[0] != 0 {
            return Ok(err);
        }

        // parse it
        let err = X11Error::try_parse(&err, ext_info)
            .map_err(|_| Error::make_invalid_state(InvalidState::BadError))?;

        Err(err.into())
    }

    /// Fetch the reply matching the sequence number, if we have it.
    pub(crate) fn fetch_reply(
        &mut self,
        reply: u64,
        ext_info: &dyn ExtInfoProvider,
    ) -> Result<Option<RawReply>> {
        let (buf, fds) = match self.proto.poll_for_reply_or_error(reply) {
            Some(a) => a,
            None => return Ok(None),
        };

        // try to parse this into an error
        let buf = self.try_parse_error(buf, ext_info)?;

        Ok(Some(RawReply::new(buf.into_boxed_slice(), fds)))
    }

    /// Can we check to see if this sequence number has errored out?
    pub(crate) fn ready_for_error_check(&mut self, reply: u64) -> bool {
        self.proto.prepare_check_for_reply_or_error(reply)
    }

    /// Check to see if this sequence number has errored out.
    ///
    /// Returns `true` if the sequence number is ready. Returns an
    /// error if it's errored out.
    pub(crate) fn check_for_error(
        &mut self,
        reply: u64,
        ext_info: &dyn ExtInfoProvider,
    ) -> Result<bool> {
        match self.proto.poll_check_for_reply_or_error(reply) {
            PollReply::NoReply => Ok(true),
            PollReply::Reply(buf) => self.try_parse_error(buf, ext_info).map(|_| true),
            PollReply::TryAgain => Ok(false),
        }
    }

    /// Fetch the next event, if we have it.
    pub(crate) fn fetch_event(&mut self, ext_info: &dyn ExtInfoProvider) -> Result<Option<Event>> {
        let (buf, _) = match self.proto.poll_for_event_with_sequence() {
            Some(a) => a,
            None => return Ok(None),
        };

        let buf = self.try_parse_error(buf, ext_info)?;

        let event = Event::parse(&buf, ext_info).map_err(Error::make_parse_error)?;

        Ok(Some(event))
    }

    pub(crate) fn send_request(&mut self, variant: ReplyFdKind) -> Option<u64> {
        self.proto.send_request(variant)
    }

    pub(crate) fn discard_reply(&mut self, seq: u64, mode: DiscardMode) {
        self.proto.discard_reply(seq, mode);
    }

    pub(crate) fn generate_xid(&mut self) -> Option<u32> {
        self.id_allocator.generate_id()
    }

    pub(crate) fn update_xid_range(
        &mut self,
        range: GetXIDRangeReply,
    ) -> core::result::Result<(), IdsExhausted> {
        self.id_allocator.update_xid_range(&range)
    }
}
