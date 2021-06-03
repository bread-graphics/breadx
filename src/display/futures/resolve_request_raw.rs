// MIT/Apache2 License

use super::{WaitLoopFuture, WaitLoopHandler};
use crate::display::{AsyncDisplay, DisplayBase, PendingReply};

/// The future returned by `AsyncDisplayExt::ResolveRequestRaw`.
pub type ResolveRequestRawFuture<'a, D: ?Sized> = WaitLoopFuture<'a, D, ResolveRequestRawHandler>;

impl<'a, D: ?Sized> ResolveRequestRawFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, req_id: u16) -> Self {
        Self::construct(display, ResolveRequestRawHandler { req_id })
    }
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub struct ResolveRequestRawHandler {
    req_id: u16,
}

impl WaitLoopHandler for ResolveRequestRawHandler {
    type Output = PendingReply;

    #[inline]
    fn handle<D: AsyncDisplay + ?Sized>(display: &mut &mut D) -> Option<PendingReply> {
        (**display).take_pending_reply(self.req_id)
    }
}
