// MIT/Apache2 License

use super::{WaitLoopFuture, WaitLoopHandler};
use crate::{
    display::{AsyncDisplay, DisplayBase},
    event::Event,
};

/// The future returned by `AsyncDisplayExt::wait_for_event_async`.
pub type WaitForEventFuture<'a, D: ?Sized> = WaitLoopFuture<'a, D, WaitForEventHandler>;

impl<'a, D: ?Sized> WaitForEventFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &mut D) -> Self {
        Self::construct(display, WaitForEventHandler)
    }
}

#[doc(hidden)]
#[derive(Debug, Copy, Clone)]
pub struct WaitForEventHandler;

impl WaitLoopHandler for WaitForEventHandler {
    type Output = Event;

    #[inline]
    fn handle<D: AsyncDisplay + ?Sized>(display: &mut &mut D) -> Option<Event> {
        display.pop_event()
    }
}
