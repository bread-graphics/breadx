// MIT/Apache2 License

use super::TryWithDyn;
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, AsyncStatus},
    Result,
};
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::FutureExt;
use x11rb_protocol::protocol::Event;

/// The future returned by the `wait_for_event` function.
pub struct WaitForEvent<'this, Dpy: ?Sized> {
    innards: TryWithDyn<'this, Event, Dpy>,
}

type FnTy = Box<
    dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<Event>>
        + Send
        + 'static,
>;

impl<'this, Dpy: AsyncDisplay + ?Sized> WaitForEvent<'this, Dpy> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub(crate) fn polling(display: &'this mut Dpy) -> Self {
        let func: FnTy = Box::new(|display, ctx| display.try_wait_for_event(ctx));
        let try_with = display.try_with(func);

        Self { innards: try_with }
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Future for WaitForEvent<'this, Dpy> {
    type Output = Result<Event>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<Event>> {
        let this = self.get_mut();
        this.innards.poll_unpin(ctx)
    }
}
