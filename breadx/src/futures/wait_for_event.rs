//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use super::TryWith;
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, AsyncStatus},
    Result,
};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::FutureExt;
use x11rb_protocol::protocol::Event;

/// The future returned by the `wait_for_event` function.
pub struct WaitForEvent<'this, Dpy: ?Sized> {
    innards: TryWith<'this, Event, FnTy, Dpy>,
}

type FnTy = fn(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<Event>>;

impl<'this, Dpy: AsyncDisplay + ?Sized> WaitForEvent<'this, Dpy> {
    #[allow(clippy::redundant_closure_for_method_calls)]
    pub(crate) fn polling(display: &'this mut Dpy) -> Self {
        let func: FnTy = |display, ctx| display.try_wait_for_event(ctx);
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
