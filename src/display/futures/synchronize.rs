// MIT/Apache2 License

use super::{SendRequestRawFuture, WaitFuture};
use crate::{
    auto::xproto::GetInputFocusRequest,
    display::{output, AsyncDisplay, RequestInfo},
};
use core::{
    future::Future,
    mem,
    pin::Pin,
    task::{Context, Poll},
};
use futures_lite::prelude::*;

/// The future returned by `AsyncDisplayExt::synchronize_async`, which sends a simple request and then
/// waits for a reply to appear.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled or .awaited"]
pub enum SynchronizeFuture<'a, D: ?Sized> {
    /// We are currently sending the simple request to the server.
    #[doc(hidden)]
    Sending { srrf: SendRequestRawFuture<'a, D> },
    /// We are currently waiting.
    #[doc(hidden)]
    Waiting { wf: WaitFuture<'a, D>, seq: u16 },
    /// The future has completed.
    #[doc(hidden)]
    Complete { display: Option<&'a mut D> },
}

impl<'a, D: AsyncDisplay + ?Sized> SynchronizeFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D) -> Self {
        let mut gifr = RequestInfo::from_request(GetInputFocusRequest::default());
        gifr.discard_reply = true;

        SynchronizeFuture::Sending {
            srrf: SendRequestRawFuture::run(display, gifr),
        }
    }

    /// Returns the display we are currently synchronizing.
    #[inline]
    pub(crate) fn display(&mut self) -> &'a mut D {
        match self {
            SynchronizeFuture::Sending { srrf } => srrf.display(),
            SynchronizeFuture::Waiting { wf, .. } => wf.display(),
            SynchronizeFuture::Complete { display } => display.take().expect("Display was taken"),
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for SynchronizeFuture<'a, D> {
    type Output = crate::Result;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        loop {
            match mem::replace(&mut *self, SynchronizeFuture::Complete { display: None }) {
                SynchronizeFuture::Sending { mut srrf } => match srrf.poll(cx) {
                    Poll::Pending => break Poll::Pending,
                    Poll::Ready(Err(e)) => {
                        *self = SynchronizeFuture::Complete {
                            display: Some(srrf.display()),
                        };
                        break Poll::Ready(Err(e));
                    }
                    Poll::Ready(Ok(seq)) => {
                        // it's time to begin the wait cycle
                        let display = srrf.display();
                        *self = SynchronizeFuture::Waiting {
                            wf: WaitFuture::run(display),
                            seq,
                        };
                    }
                },

                SynchronizeFuture::Waiting { mut wf, seq } => match wf.poll(cx) {
                    Poll::Pending => break Poll::Pending,
                    Poll::Ready(Err(e)) => {
                        *self = SynchronizeFuture::Complete {
                            display: Some(wf.display()),
                        };
                        break Poll::Ready(Err(e));
                    }
                    Poll::Ready(Ok(())) => {
                        // check if we contain any pending requests yet
                        let display = wf.display();
                        if display.take_pending_request(seq).is_some() {
                            *self = SynchronizeFuture::Complete {
                                display: Some(display),
                            };
                            break Poll::Ready(Ok(()));
                        }

                        // reset and start again
                        *self = SynchronizeFuture::Waiting {
                            wf: WaitFuture::run(display),
                            seq,
                        };
                    }
                },
                SynchronizeFuture::Complete { .. } => {
                    panic!("Attempted to poll future after completion")
                }
            }
        }
    }
}
