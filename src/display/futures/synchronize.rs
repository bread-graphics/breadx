// MIT/Apache2 License

use super::{SendRequestRawFuture, WaitFuture};
use crate::{
    auto::xproto::GetInputFocusRequest,
    display::{AsyncDisplay, RequestInfo},
    log_debug, log_trace,
    util::take_mut,
};
use core::{
    future::Future,
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
    Complete { display: &'a mut D },
    /// An empty hole.
    #[doc(hidden)]
    Hole,
}

impl<'a, D: ?Sized> Default for SynchronizeFuture<'a, D> {
    #[inline]
    fn default() -> Self {
        Self::Hole
    }
}
impl<'a, D: ?Sized> Unpin for SynchronizeFuture<'a, D> {}

impl<'a, D: AsyncDisplay + ?Sized> SynchronizeFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D) -> Self {
        log::debug!("Beginning synchronization process");

        let mut gifr = RequestInfo::from_request(
            GetInputFocusRequest::default(),
            display.bigreq_enabled(),
            display.max_request_len(),
        );
        gifr.discard_reply = true;

        SynchronizeFuture::Sending {
            srrf: SendRequestRawFuture::run(display, gifr),
        }
    }

    /// Consumes this future and returns the display we are currently synchronizing.
    #[inline]
    pub(crate) fn cannibalize(self) -> &'a mut D {
        match self {
            SynchronizeFuture::Sending { srrf } => srrf.cannibalize(),
            SynchronizeFuture::Waiting { wf, .. } => wf.cannibalize(),
            SynchronizeFuture::Complete { display } => display,
            SynchronizeFuture::Hole => panic!("Cannot cannibalize an empty hole"),
        }
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for SynchronizeFuture<'a, D> {
    type Output = crate::Result;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut result = None;
        loop {
            take_mut(&mut *self, |this| match this {
                SynchronizeFuture::Sending { mut srrf } => match srrf.poll(cx) {
                    Poll::Pending => {
                        result = Some(Poll::Pending);
                        SynchronizeFuture::Sending { srrf }
                    }
                    Poll::Ready(Err(e)) => {
                        result = Some(Poll::Ready(Err(e)));
                        SynchronizeFuture::Complete {
                            display: srrf.cannibalize(),
                        }
                    }
                    Poll::Ready(Ok(seq)) => {
                        // it's time to begin the wait cycle
                        let display = srrf.cannibalize();
                        log_trace!("We are looking for a request of seq {}", seq);
                        SynchronizeFuture::Waiting {
                            wf: WaitFuture::run(display),
                            seq,
                        }
                    }
                },
                SynchronizeFuture::Waiting { mut wf, seq } => match wf.poll(cx) {
                    Poll::Pending => {
                        result = Some(Poll::Pending);
                        SynchronizeFuture::Waiting { wf, seq }
                    }
                    Poll::Ready(Err(e)) => {
                        result = Some(Poll::Ready(Err(e)));
                        SynchronizeFuture::Complete {
                            display: wf.cannibalize(),
                        }
                    }
                    Poll::Ready(Ok(())) => {
                        log_debug!("Finished synchronization wait...");

                        // check if we contain any pending requests yet
                        let display = wf.cannibalize();
                        if display.get_pending_request(seq).is_none() {
                            result = Some(Poll::Ready(Ok(())));
                            return SynchronizeFuture::Complete { display };
                        }

                        // reset and start again
                        SynchronizeFuture::Waiting {
                            wf: WaitFuture::run(display),
                            seq,
                        }
                    }
                },
                SynchronizeFuture::Complete { .. } => {
                    panic!("Attempted to poll future past completion")
                }
                SynchronizeFuture::Hole => panic!("Cannot pole an empty hole"),
            });
            if let Some(result) = result.take() {
                return result;
            }
        }
    }
}
