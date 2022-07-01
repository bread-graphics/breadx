//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use super::TryWithDyn;
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, AsyncStatus, RawReply},
    Result,
};
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::FutureExt;

/// The future returned by the `wait_for_reply_raw` function.
pub struct WaitForReplyRaw<'this, Dpy: ?Sized> {
    innards: TryWithDyn<'this, RawReply, Dpy>,
}

type FnTy = Box<
    dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<RawReply>>
        + Send
        + 'static,
>;

impl<'this, Dpy: AsyncDisplay + ?Sized> WaitForReplyRaw<'this, Dpy> {
    pub(crate) fn polling(display: &'this mut Dpy, seq: u64) -> Self {
        // setup the function
        let mut flushed = false;
        let func: FnTy = Box::new(move |display, ctx| {
            if !flushed {
                match display.try_flush(ctx) {
                    Ok(AsyncStatus::Ready(())) => {
                        flushed = true;
                    }
                    Ok(status) => {
                        return Ok(status.map(|()| unreachable!()));
                    }
                    Err(e) => {
                        return Err(e);
                    }
                }
            }

            display.try_wait_for_reply_raw(seq, ctx)
        });

        let try_with = display.try_with(func);

        Self { innards: try_with }
    }

    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        self.innards.cannibalize()
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Future for WaitForReplyRaw<'this, Dpy> {
    type Output = Result<RawReply>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<RawReply>> {
        let this = self.get_mut();

        this.innards.poll_unpin(ctx)
    }
}
