// MIT/Apache2 License

use core::convert::TryInto;

use crate::{Error, Fd, HashMap, InvalidState, Result};
use alloc::{sync::Arc, vec::Vec};
use x11rb_protocol::{
    connection::{Connection as ProtoConnection, ReplyFdKind},
    id_allocator::IdAllocator,
    packet_reader::PacketReader,
    protocol::{xproto::Setup, Event},
    x11_utils::{ExtInfoProvider, ExtensionInformation, X11Error},
    RawFdContainer,
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
}

struct ExtMap(HashMap<&'static str, ExtensionInformation>);

impl From<HashMap<&'static str, ExtensionInformation>> for ExtMap {
    fn from(map: HashMap<&'static str, ExtensionInformation>) -> Self {
        Self(map)
    }
}

impl ExtMap {
    /// Find an extension info entry by a closure.
    fn find(
        &self,
        mut closure: impl FnMut(&ExtensionInformation) -> bool,
    ) -> Option<(&str, ExtensionInformation)> {
        self.0.iter().find_map(|(name, ext_info)| {
            if closure(ext_info) {
                Some((*name, *ext_info))
            } else {
                None
            }
        })
    }
}
