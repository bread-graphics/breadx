// MIT/Apache2 License

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

impl<'this, Dpy: AsyncDisplay + ?Sized> WaitForReplyRaw<'this, Dpy> {
    pub(crate) fn polling(display: &'this mut Dpy, seq: u64) -> Self {
        // setup the function
        let mut flushed = false;
        let func: Box<
            dyn FnMut(&mut Dpy, &mut Context<'_>) -> Result<AsyncStatus<RawReply>> + Send + 'static,
        > = Box::new(move |display, ctx| {
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
