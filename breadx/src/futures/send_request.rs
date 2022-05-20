// MIT/Apache2 License

use super::SendRequestRaw;
use crate::{
    display::{AsyncDisplay, AsyncDisplayExt, Cookie, RawRequest},
    Result,
};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::future::FutureExt;
use x11rb_protocol::x11_utils::{ReplyFDsRequest, ReplyRequest, VoidRequest};

/// The future returned by the `send_request` method.
pub struct SendRequest<'this, Dpy: ?Sized, Reply> {
    innards: SendRequestRaw<'this, Dpy>,
    _marker: PhantomData<Reply>,
}

impl<'this, Dpy: AsyncDisplay + ?Sized> SendRequest<'this, Dpy, ()> {
    pub(crate) fn for_void<Req: VoidRequest>(display: &'this mut Dpy, req: Req) -> Self {
        Self {
            innards: display.send_request_raw(RawRequest::from_request_void(req)),
            _marker: PhantomData,
        }
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized, Reply> SendRequest<'this, Dpy, Reply> {
    pub(crate) fn for_reply<Req: ReplyRequest<Reply = Reply>>(
        display: &'this mut Dpy,
        req: Req,
    ) -> Self {
        Self {
            innards: display.send_request_raw(RawRequest::from_request_reply(req)),
            _marker: PhantomData,
        }
    }

    pub(crate) fn for_reply_fds<Req: ReplyFDsRequest<Reply = Reply>>(
        display: &'this mut Dpy,
        req: Req,
    ) -> Self {
        Self {
            innards: display.send_request_raw(RawRequest::from_request_reply_fds(req)),
            _marker: PhantomData,
        }
    }

    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        self.innards.cannibalize()
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized, Reply: Unpin> Future for SendRequest<'this, Dpy, Reply> {
    type Output = Result<Cookie<Reply>>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<Cookie<Reply>>> {
        self.get_mut()
            .innards
            .poll_unpin(ctx)
            .map_ok(|seq| Cookie::from_sequence(seq))
    }
}
