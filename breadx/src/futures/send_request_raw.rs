//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use super::TryWith;
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, AsyncStatus, BufferedRequest, RawRequest},
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
    innards: TryWith<'this, u64, FnTy, Dpy>,
}

type FnTy = Box<
    dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<u64>> + Send + 'static,
>;

impl<'this, Dpy: AsyncDisplay + ?Sized> SendRequestRaw<'this, Dpy> {
    pub(crate) fn polling(display: &'this mut Dpy, request: RawRequest<'_, '_>) -> Self {
        // buffer the request
        // TODO: work through the lifetimes here, this can't be the most efficient
        // way of doing it
        // but it's gotta somehow work with SendRequest<>, selfref or boxing async{}
        // may be the only option but each has caveats
        let mut request: BufferedRequest = request.into();

        // set up the function
        let mut sequence = None;
        let func: FnTy = Box::new(move |display, ctx| {
            loop {
                match sequence {
                    None => {
                        // calculate the sequence number
                        match request.borrow(|request| display.format_request(request, ctx)) {
                            Ok(AsyncStatus::Ready(seq)) => sequence = Some(seq),
                            Ok(status) => return Ok(status.map(|_| unreachable!())),
                            Err(e) => return Err(e),
                        }
                    }
                    Some(seq) => {
                        return request
                            .borrow(|request| display.try_send_request_raw(request, ctx))
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
