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

use super::{
    raw_request::{from_reply_request, BufferedRequest},
    Display, RawReply,
};
use crate::Result;

cfg_async! {
    use super::{AsyncStatus, CanBeAsyncDisplay};
    use core::task::Context;
}

/// Some internal data that needs to be fetched through
/// another request.
pub struct Prefetch<T: PrefetchTarget> {
    state: PrefetchState<T>,
}

enum PrefetchState<T: PrefetchTarget> {
    /// In the process of formatting.
    Formatting(Option<BufferedRequest>),
    /// We're in the process of sending this data.
    #[allow(dead_code)]
    Sending(Option<BufferedRequest>, u64),
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
        #[allow(clippy::redundant_closure_for_method_calls)]
        let req = from_reply_request(req, |req| req.into());
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

    /// Format the request.
    ///
    /// The user may use this in order to format the request.
    pub(crate) fn request_to_format(&mut self) -> &mut BufferedRequest {
        match self.state {
            PrefetchState::Formatting(Some(ref mut req)) => req,
            _ => panic!("Prefetch is not formatting"),
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

    /// Error out.
    pub(crate) fn on_x11_error(&mut self) {
        *self = PrefetchState::Complete(T::on_x11_error()).into();
    }

    /// Evaluate the prefetch while blocking.
    pub fn evaluate(&mut self, display: &mut impl Display) -> Result<&T::Target> {
        // call all functions in order
        // TODO: handle on_x11_error
        let request = self.request_to_format();
        let seq = request.take(|request| display.send_request_raw(request))?;
        self.sent_override(seq);
        match display.wait_for_reply_raw(self.sequence()) {
            Ok(reply) => {
                self.read_reply(reply)?;
            }
            Err(e) if e.is_protocol_error() => {
                self.on_x11_error();
            }
            Err(e) => return Err(e),
        };
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

        /// Indicate that the request has been sent.
        pub(crate) fn sent(&mut self) {
            match self.state {
                PrefetchState::Sending(_, seq) => *self = PrefetchState::Waiting(seq).into(),
                _ => panic!("Prefetch is not sending"),
            }
        }

        /// Indicate that we've formatted the request.
        pub(crate) fn formatted(&mut self, seq: u64) {
            match self.state {
                PrefetchState::Formatting(ref mut request) => {
                    *self = PrefetchState::Sending(request.take(), seq).into();
                }
                _ => panic!("Prefetch is not formatting"),
            }
        }

        /// Get the request to send.
        pub(crate) fn request_to_send(&mut self) -> &mut BufferedRequest {
            match self.state {
                PrefetchState::Sending(Some(ref mut req), ..)
                | PrefetchState::Formatting(Some(ref mut req)) => req,
                _ => panic!("Prefetch is not sending"),
            }
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
                let request = self.request_to_format();
                let seq = mtry! {
                    request.borrow(|request| {
                        display.format_request(request, ctx)
                    })
                };
                self.formatted(seq);
            }

            if self.not_yet_sent() {
                tracing::trace!("sending request");
                let request = self.request_to_send();
                mtry! {
                    request.borrow(|request| {
                        display.try_send_request_raw(request, ctx)
                    })
                };
                self.sent();
            }

            if self.not_yet_read() {
                tracing::trace!("receiving reply");
                match display.try_wait_for_reply_raw(self.sequence(), ctx) {
                    Ok(AsyncStatus::Ready(t)) => self.read_reply(t)?,
                    Ok(status) => return Ok(status.map(|_| unreachable!())),
                    Err(e) if e.is_protocol_error() => {
                        self.on_x11_error();
                    }
                    Err(e) => return Err(e),
                }
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
        tracing::error!("synchronization should never error out");
        GetInputFocusReply::default()
    }
}

impl PrefetchTarget for GetXIDRangeRequest {
    type Target = GetXIDRangeReply;

    fn map_reply(reply: Self::Reply) -> Self::Target {
        reply
    }

    fn on_x11_error() -> Self::Target {
        tracing::error!("XID refresh should never error out");
        GetXIDRangeReply::default()
    }
}
