// MIT/Apache2 License

//! Extension traits.

use x11rb_protocol::x11_utils::{ReplyFDsRequest, ReplyRequest, TryParseFd, VoidRequest};

use super::{Cookie, Display, DisplayBase, RawRequest};
use crate::Result;
use alloc::vec::Vec;
use core::mem;

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
            tracing::info!("void request, not waiting for reply");
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
