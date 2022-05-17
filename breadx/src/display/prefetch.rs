// MIT/Apache2 License

use x11rb_protocol::{
    protocol::{
        bigreq::EnableRequest,
        xc_misc::{GetXIDRangeReply, GetXIDRangeRequest},
        xproto::{GetInputFocusReply, GetInputFocusRequest, QueryExtensionRequest},
    },
    x11_utils::{ExtensionInformation, ReplyRequest},
    SequenceNumber,
};

use super::{Display, RawReply, RawRequest};
use crate::Result;
use core::{mem, task::Context};

cfg_async! {
    use super::{AsyncStatus, CanBeAsyncDisplay};
}

/// Some internal data that needs to be fetched through
/// another request.
pub struct Prefetch<T: PrefetchTarget> {
    state: PrefetchState<T>,
}

enum PrefetchState<T: PrefetchTarget> {
    /// In the process of formatting.
    Formatting(Option<RawRequest>),
    /// We're in the process of sending this data.
    Sending(Option<RawRequest>, u64),
    /// We've sent this data and are waiting for a reply.
    Waiting(SequenceNumber),
    /// The data is available for us; the request is complete.
    Complete(T::Target),
}

/// A request that needs to be prefetched.
pub trait PrefetchTarget: ReplyRequest {
    /// The resulting data from the prefetch.
    type Target;

    /// Map the reply to the prefetch target.
    fn map_reply(reply: Self::Reply) -> Self::Target;

    /// The target when an X11 error occurs.
    fn on_x11_error() -> Self::Target;
}

impl<T: PrefetchTarget + Default> Default for Prefetch<T> {
    fn default() -> Self {
        Self::new(Default::default())
    }
}

impl<T: PrefetchTarget> From<PrefetchState<T>> for Prefetch<T> {
    fn from(state: PrefetchState<T>) -> Self {
        Self { state }
    }
}

impl<T: PrefetchTarget> Prefetch<T> {
    pub fn new(req: T) -> Self {
        let req = RawRequest::from_request_reply(req);
        PrefetchState::Formatting(Some(req)).into()
    }

    /// Get the target if and only if this prefetch has
    /// already resolved.
    pub(crate) fn get_if_resolved(&self) -> Option<&T::Target> {
        match self.state {
            PrefetchState::Complete(ref c) => Some(c),
            _ => None,
        }
    }

    pub(crate) fn get_assert(&self) -> &T::Target {
        match self.state {
            PrefetchState::Complete(ref c) => c,
            _ => panic!("Prefetch not yet resolved"),
        }
    }

    /// Format the request.
    ///
    /// The user may use this in order to format the request.
    pub(crate) fn request_to_format(&mut self) -> &mut RawRequest {
        match self.state {
            PrefetchState::Formatting(Some(ref mut req)) => req,
            _ => panic!("Prefetch is not formatting"),
        }
    }

    /// Indicate that we've formatted the request.
    pub(crate) fn formatted(&mut self, seq: u64) {
        match self.state {
            PrefetchState::Formatting(ref mut req) => {
                *self = PrefetchState::Sending(req.take(), seq).into()
            }
            _ => panic!("Prefetch is not formatting"),
        }
    }

    /// Get the request to send.
    pub(crate) fn request_to_send(&mut self) -> &mut RawRequest {
        match self.state {
            PrefetchState::Sending(Some(ref mut req), ..) => req,
            PrefetchState::Formatting(Some(ref mut req)) => req,
            _ => panic!("Prefetch is not sending"),
        }
    }

    /// Send with an overridden sequence number.
    pub(crate) fn sent_override(&mut self, seq: u64) {
        match self.state {
            PrefetchState::Sending(..) | PrefetchState::Formatting(..) => {
                *self = PrefetchState::Waiting(seq).into();
            }
            _ => panic!("Prefetch is not sending or formatting"),
        }
    }

    /// Indicate that the request has been sent.
    pub(crate) fn sent(&mut self) {
        match self.state {
            PrefetchState::Sending(_, seq) => *self = PrefetchState::Waiting(seq).into(),
            _ => panic!("Prefetch is not sending"),
        }
    }

    /// The sequence number of the reply we're waiting for.
    pub(crate) fn sequence(&self) -> u64 {
        match self.state {
            PrefetchState::Waiting(ref seq) => *seq,
            _ => panic!("Prefetch is not waiting"),
        }
    }

    /// Read in the reply.
    pub(crate) fn read_reply(&mut self, reply: RawReply) -> Result<()> {
        let reply: T::Reply = reply.into_reply()?;
        let mapped = T::map_reply(reply);
        *self = PrefetchState::Complete(mapped).into();
        Ok(())
    }

    /// Evaluate the prefetch while blocking.
    pub fn evaluate(&mut self, display: &mut impl Display) -> Result<&T::Target> {
        // call all functions in order
        let req = self.request_to_format();
        let req = mem::take(req);
        let seq = display.send_request_raw(req)?;
        self.sent_override(seq);
        let reply = display.wait_for_reply_raw(self.sequence())?;
        self.read_reply(reply)?;
        Ok(self.get_if_resolved().unwrap())
    }
}

cfg_async! {
    impl<T: PrefetchTarget> Prefetch<T> {
        fn not_yet_formatted(&self) -> bool {
            matches!(self.state, PrefetchState::Formatting(_))
        }

        fn not_yet_sent(&self) -> bool {
            matches!(self.state, PrefetchState::Formatting(_) | PrefetchState::Sending(..))
        }

        fn not_yet_read(&self) -> bool {
            !matches!(self.state, PrefetchState::Complete(_))
        }

        /// Evaluate the prefetch, but avoid blocking.
        pub fn try_evaluate(
            &mut self,
            display: &mut impl CanBeAsyncDisplay,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<&T::Target>> {
            let span = tracing::trace_span!(
                "try_evaluate",
                formatted = !self.not_yet_formatted(),
                sent = !self.not_yet_sent(),
                read = !self.not_yet_read()
            );
            let _enter = span.enter();

            // call all functions in order
            if self.not_yet_formatted() {
                tracing::trace!("formatting request");
                let req = self.request_to_format();
                let seq = mtry!(display.format_request(req, ctx));
                self.formatted(seq);
            }

            if self.not_yet_sent() {
                tracing::trace!("sending request");
                let req = self.request_to_send();
                mtry!(display.try_send_request_raw(req, ctx));
                self.sent();
            }

            if self.not_yet_read() {
                tracing::trace!("receiving reply");
                let reply = mtry!(display.try_wait_for_reply_raw(self.sequence(), ctx));
                self.read_reply(reply)?;
            }

            Ok(AsyncStatus::Ready(self.get_if_resolved().unwrap()))
        }
    }
}

// prefetch targets: query for bigreq and query for extension

impl PrefetchTarget for EnableRequest {
    type Target = Option<usize>;

    fn map_reply(reply: Self::Reply) -> Self::Target {
        let maxlen = reply.maximum_request_length as usize;
        Some(maxlen)
    }

    fn on_x11_error() -> Self::Target {
        None
    }
}

impl<'a> PrefetchTarget for QueryExtensionRequest<'a> {
    type Target = Option<ExtensionInformation>;

    fn map_reply(reply: Self::Reply) -> Self::Target {
        if !reply.present {
            return None;
        }

        let info = ExtensionInformation {
            major_opcode: reply.major_opcode,
            first_error: reply.first_error,
            first_event: reply.first_event,
        };

        Some(info)
    }

    fn on_x11_error() -> Self::Target {
        None
    }
}

impl PrefetchTarget for GetInputFocusRequest {
    type Target = GetInputFocusReply;

    fn map_reply(reply: Self::Reply) -> Self::Target {
        reply
    }

    fn on_x11_error() -> Self::Target {
        todo!()
    }
}

impl PrefetchTarget for GetXIDRangeRequest {
    type Target = GetXIDRangeReply;

    fn map_reply(reply: Self::Reply) -> Self::Target {
        reply
    }

    fn on_x11_error() -> Self::Target {
        todo!()
    }
}
