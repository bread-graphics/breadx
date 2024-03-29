//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use crate::{
    display::{AsyncDisplay, AsyncStatus, Interest},
    Result,
};
use alloc::boxed::Box;
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};

/// The future returned by `try_with()`.
pub struct TryWith<'this, R, F, Dpy: ?Sized> {
    dpy: &'this mut Dpy,
    callback: F,
    straight_call: bool,
    interest: Option<Interest>,

    _marker: PhantomData<&'this mut R>,
}

pub(crate) type TryWithDyn<'this, R, Dpy> = TryWith<
    'this,
    R,
    Box<
        dyn FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<R>>
            + Send
            + 'static,
    >,
    Dpy,
>;

impl<'this, R, F, Dpy: ?Sized> TryWith<'this, R, F, Dpy> {
    /// Creates a new `TryWith` future.
    pub(crate) fn new(dpy: &'this mut Dpy, callback: F) -> Self {
        Self {
            dpy,
            callback,
            straight_call: true,
            interest: None,
            _marker: PhantomData,
        }
    }

    /// Destroys this object and returns the inner `Dpy` reference.
    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        self.dpy
    }
}

impl<
        'this,
        R,
        F: FnMut(&mut dyn AsyncDisplay, &mut Context<'_>) -> Result<AsyncStatus<R>> + Unpin,
        Dpy: AsyncDisplay + ?Sized,
    > Future for TryWith<'this, R, F, Dpy>
{
    type Output = Result<R>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<R>> {
        let mut this = self.get_mut();

        debug_assert!(this.straight_call || this.interest.is_some());

        // if we haven't tried to straight-call the callback, try it now
        loop {
            if this.straight_call {
                this.straight_call = false;

                tracing::trace!("trying straight call");

                match (this.callback)(&mut this.dpy, ctx) {
                    Ok(AsyncStatus::Ready(r)) => return Poll::Ready(Ok(r)),
                    Ok(AsyncStatus::Read) => this.interest = Some(Interest::Readable),
                    Ok(AsyncStatus::Write) => this.interest = Some(Interest::Writable),
                    Ok(AsyncStatus::UserControlled) => {
                        this.straight_call = true;
                    }
                    Err(e) => return Poll::Ready(Err(e)),
                }
            }

            // if we have an interest to poll, poll it
            if let Some(interest) = this.interest {
                tracing::trace!("re-polling for interest {:?}", interest);

                let mut res: Option<AsyncStatus<R>> = None;

                // try the polling process
                let callback = &mut this.callback;

                match this.dpy.poll_for_interest(
                    interest,
                    &mut |dpy, ctx| {
                        (callback)(dpy, ctx).and_then(|status| {
                            // create a would_block error if the status is not ready
                            let is_ready = status.is_ready();
                            res = Some(status);

                            if is_ready {
                                Ok(())
                            } else {
                                Err(crate::Error::io(std::io::ErrorKind::WouldBlock.into()))
                            }
                        })
                    },
                    ctx,
                ) {
                    Poll::Pending => return Poll::Pending,
                    Poll::Ready(Err(e)) => return Poll::Ready(Err(e)),
                    Poll::Ready(Ok(())) => {
                        // match on the status
                        match res.take().expect("malicious poll_for_interest impl") {
                            AsyncStatus::Ready(r) => return Poll::Ready(Ok(r)),
                            AsyncStatus::Read => this.interest = Some(Interest::Readable),
                            AsyncStatus::Write => this.interest = Some(Interest::Writable),
                            AsyncStatus::UserControlled => {
                                this.straight_call = true;
                            }
                        }
                    }
                }
            }
        }
    }
}
