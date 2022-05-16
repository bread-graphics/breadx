// MIT/Apache2 License

//! Extension traits.

use x11rb_protocol::x11_utils::{ReplyFDsRequest, ReplyRequest, TryParseFd, VoidRequest};

use super::{Cookie, Display, DisplayBase, RawRequest};
use crate::Result;
use alloc::vec::Vec;
use core::mem;

cfg_async! {
    use super::{AsyncStatus, AsyncDisplay};
    use crate::futures;
    use core::task::Context;
}

pub trait DisplayBaseExt: DisplayBase {
    /// Poll for a reply matching the given sequence number.
    fn poll_for_reply<R: TryParseFd>(&mut self, seq: u64) -> Result<Option<R>> {
        match self.poll_for_reply_raw(seq)? {
            Some(reply) => reply.into_reply().map(Some),
            None => Ok(None),
        }
    }
}

impl<D: DisplayBase + ?Sized> DisplayBaseExt for D {}

/// Extension traits to allow for sending generic requests and replies.
pub trait DisplayExt: Display {
    /// Send a request with no reply.
    fn send_void_request(&mut self, request: impl VoidRequest) -> Result<Cookie<()>> {
        let seq = self.send_request_raw(RawRequest::from_request_void(request))?;

        Ok(Cookie::from(seq))
    }

    /// Send a request with a reply.
    fn send_reply_request<R: ReplyRequest>(&mut self, request: R) -> Result<Cookie<R::Reply>> {
        let seq = self.send_request_raw(RawRequest::from_request_reply(request))?;

        Ok(Cookie::from(seq))
    }

    /// Send a request with a reply containing file descriptors.
    fn send_reply_fd_request<R: ReplyFDsRequest>(
        &mut self,
        request: R,
    ) -> Result<Cookie<R::Reply>> {
        let seq = self.send_request_raw(RawRequest::from_request_reply_fds(request))?;

        Ok(Cookie::from(seq))
    }

    /// Receive a reply from the server.
    fn wait_for_reply<R: TryParseFd>(&mut self, cookie: Cookie<R>) -> Result<R> {
        let span = tracing::debug_span!(
            "wait_for_reply",
            cookie = %cookie.sequence(),
        );
        let _enter = span.enter();

        if mem::size_of::<R>() == 0 {
            // zero sized reply indicates that this is a void request,
            // check if we need to
            tracing::info!("void request, beginning synchronize");
            self.synchronize()?;
            return Ok(R::try_parse_fd(&[], &mut Vec::new())
                .unwrap_or_else(|_| unreachable!())
                .0);
        }

        let reply = self.wait_for_reply_raw(cookie.into())?;
        reply.into_reply()
    }
}

impl<D: Display + ?Sized> DisplayExt for D {}

cfg_async! {
    /// Extension trait to allow for sending generic requests and replies.
    pub trait AsyncDisplayExt : AsyncDisplay {
        /// Wrap a function that returns an `AsyncStatus` in the runtime
        /// that this is connected to.
        fn try_with<'this, R, F: FnMut(&mut Self, &mut Context<'_>) -> Result<AsyncStatus<R>>>(
            &'this mut self,
            f: F
        ) -> futures::TryWith<'this, R, F, Self>  {
            futures::TryWith::new(self, f)
        }

        /// Wait for the X11 server to return the given sequence number.
        fn wait_for_reply_raw(
            &mut self,
            seq: u64,
        ) -> futures::WaitForReplyRaw<'_, Self> {
            futures::WaitForReplyRaw::polling(self, seq)
        }

        /// Wait for the X11 server to send a reply.
        fn wait_for_event(
            &mut self,
        ) -> futures::WaitForEvent<'_, Self> {
            futures::WaitForEvent::polling(self)
        }

        /// Synchronize with the X11 server.
        fn synchronize(&mut self) -> futures::Synchronize<'_, Self> {
            futures::Synchronize::new(self)
        }

        /// Flush the display.
        fn flush(&mut self) -> futures::Flush<'_, Self> {
            futures::Flush::polling(self)
        }

        /// Send a raw request to the X11 server.
        fn send_request_raw<'this, 'req>(
            &'this mut self,
            request: RawRequest,
        ) -> futures::SendRequestRaw<'this, Self>{
            futures::SendRequestRaw::polling(self, request)
        }

        /// Send a void request to the X11 server.
        fn send_void_request(
            &mut self,
            request: impl VoidRequest,
        ) -> futures::SendRequest<'_, Self, ()> {
            futures::SendRequest::for_void(self, request)
        }

        /// Send a request with a reply to the X11 server.
        fn send_reply_request<R: ReplyRequest>(
            &mut self,
            request: R
        ) -> futures::SendRequest<'_, Self, R::Reply> {
            futures::SendRequest::for_reply(self, request)
        }

        /// Send a request with a reply containing file descriptors to the X11 server.
        fn send_reply_fd_request<R: ReplyFDsRequest>(
            &mut self,
            request: R,
        ) -> futures::SendRequest<'_, Self, R::Reply> {
            futures::SendRequest::for_reply_fds(self, request)
        }

        /// Wait for a reply from the X11 server.
        fn wait_for_reply<R: TryParseFd>(
            &mut self,
            cookie: Cookie<R>,
        ) -> futures::WaitForReply<'_, Self, R> {
            futures::WaitForReply::new(self, cookie)
        }
    }

    impl<D: AsyncDisplay + ?Sized> AsyncDisplayExt for D {}
}
