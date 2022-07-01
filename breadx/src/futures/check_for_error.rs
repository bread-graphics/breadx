//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

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

/// The future returned by the `check_for_error` function.
pub struct CheckForError<'this, Dpy: ?Sized> {
    innards: TryWithDyn<'this, (), Dpy>,
}

type FnTy = Box<
    dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<()>> + Send + 'static,
>;

impl<'this, Dpy: AsyncDisplay + ?Sized> CheckForError<'this, Dpy> {
    pub(crate) fn polling(display: &'this mut Dpy, seq: u64) -> Self {
        // setup the function
        let func: FnTy = Box::new(move |display, ctx| display.try_check_for_error(seq, ctx));

        let try_with = display.try_with(func);

        Self { innards: try_with }
    }

    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        self.innards.cannibalize()
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Future for CheckForError<'this, Dpy> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();

        this.innards.poll_unpin(ctx)
    }
}
