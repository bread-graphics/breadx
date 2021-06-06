// MIT/Apache2 License

use super::{WaitLoopFuture, WaitLoopHandler};
use crate::{
    display::{AsyncDisplay, DisplayBase},
    event::Event,
    XID,
};

/// The future returned by `AsyncDisplayExt::WaitForSpecialEvent`.
pub type WaitForSpecialEventFuture<'a, D: ?Sized> =
    WaitLoopFuture<'a, D, WaitForSpecialEventHandler>;

impl<'a, D: ?Sized> WaitForSpecialEventFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &mut D, xid: XID) -> Self {
        Self::construct(display, WaitForSpecialEventHandler { xid })
    }
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub struct WaitForSpecialEventHandler {
    xid: XID,
}

impl WaitLoopHandler for WaitForSpecialEventHandler {
    type Output = Event;

    #[inline]
    fn handle<D: AsyncDisplay + ?Sized>(&self, display: &mut &mut D) -> Option<Event> {
        display.pop_special_event(self.xid)
    }
}
