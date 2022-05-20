// MIT/Apache2 License

use super::TryWith;
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, AsyncStatus, RawRequest},
    Result,
};
use alloc::boxed::Box;
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::FutureExt;

/// The future returned by the `send_request_raw` function.
pub struct SendRequestRaw<'this, Dpy: ?Sized> {
    innards: TryWith<'this, u64, FnTy<Dpy>, Dpy>,
}

type FnTy<Dpy> =
    Box<dyn FnMut(&mut Dpy, &mut Context<'_>) -> Result<AsyncStatus<u64>> + Send + 'static>;

impl<'this, 'req, Dpy: AsyncDisplay + ?Sized> SendRequestRaw<'this, Dpy> {
    pub(crate) fn polling(display: &'this mut Dpy, mut request: RawRequest) -> Self {
        // set up the function
        let mut sequence = None;
        let func: FnTy<Dpy> = Box::new(move |display, ctx| {
            loop {
                match sequence {
                    None => {
                        // calculate the sequence number
                        match display.format_request(&mut request, ctx) {
                            Ok(AsyncStatus::Ready(seq)) => sequence = Some(seq),
                            Ok(status) => return Ok(status.map(|_| unreachable!())),
                            Err(e) => return Err(e),
                        }
                    }
                    Some(seq) => {
                        return display
                            .try_send_request_raw(&mut request, ctx)
                            .map(move |res| res.map(move |()| seq));
                    }
                }
            }
        });

        let try_with = display.try_with(func);

        Self { innards: try_with }
    }

    /// Destroys this future and returns the inner display reference.
    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        self.innards.cannibalize()
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Future for SendRequestRaw<'this, Dpy> {
    type Output = Result<u64>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<u64>> {
        let this = self.get_mut();

        this.innards.poll_unpin(ctx)
    }
}
