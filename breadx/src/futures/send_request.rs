//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use super::SendRequestRaw;
use crate::{
    display::{
        from_reply_fds_request, from_reply_request, from_void_request, AsyncDisplay,
        AsyncDisplayExt, Cookie,
    },
    Result,
};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::future::FutureExt;
use tracing::Span;
use x11rb_protocol::x11_utils::{ReplyFDsRequest, ReplyRequest, VoidRequest};

/// The future returned by the `send_request` method.
pub struct SendRequest<'this, Dpy: ?Sized, Reply> {
    innards: SendRequestRaw<'this, Dpy>,
    span: Option<Span>,
    _marker: PhantomData<Reply>,
}

impl<'this, Dpy: AsyncDisplay + ?Sized> SendRequest<'this, Dpy, ()> {
    pub(crate) fn for_void<Req: VoidRequest>(
        display: &'this mut Dpy,
        discard_reply: bool,
        req: Req,
    ) -> Self {
        from_void_request(req, discard_reply, move |req| Self {
            innards: display.send_request_raw(req),
            span: None,
            _marker: PhantomData,
        })
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized, Reply> SendRequest<'this, Dpy, Reply> {
    pub(crate) fn for_reply<Req: ReplyRequest<Reply = Reply>>(
        display: &'this mut Dpy,
        req: Req,
    ) -> Self {
        from_reply_request(req, move |req| Self {
            innards: display.send_request_raw(req),
            span: None,
            _marker: PhantomData,
        })
    }

    pub(crate) fn for_reply_fds<Req: ReplyFDsRequest<Reply = Reply>>(
        display: &'this mut Dpy,
        req: Req,
    ) -> Self {
        from_reply_fds_request(req, move |req| Self {
            innards: display.send_request_raw(req),
            span: None,
            _marker: PhantomData,
        })
    }

    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        self.innards.cannibalize()
    }
}

impl<'this, Dpy: ?Sized, Reply> SendRequest<'this, Dpy, Reply> {
    pub(crate) fn with_span(mut self, span: Span) -> Self {
        self.span = Some(span);
        self
    }

    pub(crate) fn take_span(&mut self) -> Option<Span> {
        self.span.take()
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized, Reply: Unpin> Future for SendRequest<'this, Dpy, Reply> {
    type Output = Result<Cookie<Reply>>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<Cookie<Reply>>> {
        let this = self.get_mut();

        // enter the span
        let _enter = this.span.as_ref().map(Span::enter);

        // call the innards
        this.innards
            .poll_unpin(ctx)
            .map_ok(|seq| Cookie::from_sequence(seq))
    }
}
