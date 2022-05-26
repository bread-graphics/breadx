// MIT/Apache2 License

use core::{
    mem,
    pin::Pin,
    task::{Context, Poll},
};
use futures_util::{Future, FutureExt};
use x11rb_protocol::protocol::xproto::GetInputFocusRequest;

use crate::{
    display::{from_reply_request, AsyncDisplay, AsyncDisplayExt},
    Result,
};

use super::{SendRequestRaw, WaitForReplyRaw};

/// Future for the "synchronize" operation.
pub struct Synchronize<'this, Dpy: ?Sized> {
    innards: Innards<'this, Dpy>,
}

enum Innards<'this, Dpy: ?Sized> {
    Sending(SendRequestRaw<'this, Dpy>),
    Waiting(WaitForReplyRaw<'this, Dpy>),
    Hole,
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Synchronize<'this, Dpy> {
    pub(crate) fn new(dpy: &'this mut Dpy) -> Self {
        let req = GetInputFocusRequest {};

        from_reply_request(req, move |req| Self {
            innards: Innards::Sending(dpy.send_request_raw(req)),
        })
    }

    pub(crate) fn cannibalize(self) -> &'this mut Dpy {
        match self.innards {
            Innards::Sending(innards) => innards.cannibalize(),
            Innards::Waiting(innards) => innards.cannibalize(),
            Innards::Hole => panic!("cannibalized Synchronize"),
        }
    }
}

impl<'this, Dpy: AsyncDisplay + ?Sized> Future for Synchronize<'this, Dpy> {
    type Output = Result<()>;

    fn poll(self: Pin<&mut Self>, ctx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();

        loop {
            match this.innards {
                Innards::Sending(ref mut send_request_raw) => {
                    match send_request_raw.poll_unpin(ctx) {
                        Poll::Ready(Ok(seq)) => {
                            // plug the seq into wait_for_reply_raw
                            let display_ref = match mem::replace(&mut this.innards, Innards::Hole) {
                                Innards::Sending(send_request_raw) => {
                                    send_request_raw.cannibalize()
                                }
                                _ => unreachable!(),
                            };

                            this.innards = Innards::Waiting(display_ref.wait_for_reply_raw(seq));
                        }
                        res => return res.map_ok(|_| ()),
                    }
                }
                Innards::Waiting(ref mut wait_for_reply_raw) => {
                    return wait_for_reply_raw.poll_unpin(ctx).map_ok(|_| ());
                }
                Innards::Hole => unreachable!("cannot poll an empty hole"),
            }
        }
    }
}
