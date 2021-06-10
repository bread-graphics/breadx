// MIT/Apache2 License

use crate::display::{AsyncDisplay, PendingReply, PendingRequest, PollOr, RequestInfo};
use core::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};

/// The future returned by `send_request_raw_async`. This polls the `AsyncDisplay` instance until it
/// returns properly.
#[derive(Debug)]
#[must_use = "futures do nothing unless you poll or .await them"]
pub struct SendRequestRawFuture<'a, D: ?Sized> {
    display: &'a mut D,
    request_info: Option<RequestInfo>,
    is_finished: bool,
}

impl<'a, D: ?Sized> Unpin for SendRequestRawFuture<'a, D> {}

impl<'a, D: AsyncDisplay + ?Sized> SendRequestRawFuture<'a, D> {
    #[inline]
    pub(crate) fn run(display: &'a mut D, request: RequestInfo) -> Self {
        // begin the send request process
        Self {
            display,
            request_info: Some(request),
            is_finished: false,
        }
    }

    /// Consumes this future and returns the display we are currently sending a request to.
    #[inline]
    pub(crate) fn cannibalize(self) -> &'a mut D {
        self.display
    }
}

impl<'a, D: AsyncDisplay + ?Sized> Future for SendRequestRawFuture<'a, D> {
    type Output = crate::Result<u16>;

    #[inline]
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        if self.is_finished {
            panic!("Attempted to poll future after completion");
        }

        // begin the send request process; once we've done that,
        let res = loop {
            match self.request_info.take() {
                Some(request_info) => {
                    match self.display.begin_send_request_raw(request_info, cx) {
                        PollOr::Pending(req) => {
                            self.request_info = Some(req);
                            break Poll::Pending;
                        }
                        PollOr::Ready(()) => { /* request_info is already set to None */ }
                    }
                }
                None => break self.display.poll_send_request_raw(cx),
            }
        };

        if res.is_ready() {
            self.is_finished = true;
        }
        res
    }
}
