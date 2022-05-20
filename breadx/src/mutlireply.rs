// MIT/Apache2 License

use crate::{
    display::{Cookie, Display, DisplayExt},
    Result,
};
use x11rb_protocol::{protocol::xproto::ListFontsWithInfoReply, x11_utils::TryParseFd};

#[cfg(feature = "record")]
use x11rb_protocol::protocol::record::EnableContextReply;

cfg_async! {
    use crate::{
        display::{AsyncDisplay, AsyncDisplayExt},
        futures::WaitForReply,
    };
    use core::{mem, task::Poll};
    use futures_util::{future::FutureExt, stream::Stream};
}

/// A reply that can have more than one reply.
pub trait Multireply {
    /// Tell if this reply will be the last reply in the sequence.
    fn is_last(&self) -> bool;
}

impl Multireply for ListFontsWithInfoReply {
    fn is_last(&self) -> bool {
        self.name.is_empty()
    }
}

#[cfg(feature = "record")]
impl Multireply for EnableContextReply {
    fn is_last(&self) -> bool {
        self.category == 5
    }
}

impl<R: Multireply + TryParseFd> Cookie<R> {
    /// Get an iterator over the replies of this request.
    pub fn into_iter<'dpy, Dpy: Display + ?Sized>(
        self,
        display: &'dpy mut Dpy,
    ) -> impl Iterator<Item = Result<R>> + 'dpy
    where
        R: 'dpy,
    {
        MultireplyIter {
            cookie: self,
            display,
        }
    }
}

cfg_async! {
    impl<R: Multireply + TryParseFd + Unpin> Cookie<R> {
        /// Get a stream over the replies of this request.
        pub fn into_stream<'dpy, Dpy: AsyncDisplay + ?Sized>(
            self,
            display: &'dpy mut Dpy,
        ) -> impl Stream<Item = Result<R>> + 'dpy
        where
            R: 'dpy,
        {
            MultireplyStream {
                cookie: self,
                state: State::NotWaiting(display),
            }
        }
    }
}

struct MultireplyIter<'dpy, R, Dpy: ?Sized> {
    cookie: Cookie<R>,
    display: &'dpy mut Dpy,
}

impl<'dpy, R: Multireply + TryParseFd, Dpy: Display + ?Sized> Iterator
    for MultireplyIter<'dpy, R, Dpy>
{
    type Item = Result<R>;

    fn next(&mut self) -> Option<Result<R>> {
        // wait for the given reply
        let reply = match self.display.wait_for_reply(self.cookie) {
            Ok(reply) => reply,
            Err(e) => return Some(Err(e)),
        };

        // see if it's the end
        if reply.is_last() {
            Some(Ok(reply))
        } else {
            None
        }
    }
}

cfg_async! {
    struct MultireplyStream<'dpy, R, Dpy: ?Sized> {
        cookie: Cookie<R>,
        state: State<'dpy, R, Dpy>,
    }

    enum State<'dpy, R, Dpy: ?Sized> {
        NotWaiting(&'dpy mut Dpy),
        Waiting(WaitForReply<'dpy, Dpy, R>),
        Hole,
    }

    impl<'dpy, R: Multireply + TryParseFd + Unpin, Dpy: AsyncDisplay + ?Sized> Stream for MultireplyStream<'dpy, R, Dpy> {
        type Item = Result<R>;

        fn poll_next(self: core::pin::Pin<&mut Self>, cx: &mut core::task::Context<'_>) -> Poll<Option<Self::Item>> {
            let this = self.get_mut();

            // get the underlying future to poll
            let underlying_fut = match this.state {
                State::NotWaiting(_) => {
                    let display_ref = match mem::replace(&mut this.state, State::Hole) {
                        State::NotWaiting(dpy) => dpy,
                        _ => unreachable!(),
                    };

                    this.state = State::Waiting(display_ref.wait_for_reply(this.cookie));

                    // get the ref out
                    match this.state {
                        State::Waiting(ref mut fut) => fut,
                        _ => unreachable!(),
                    }
                }
                State::Waiting(ref mut fut) => fut,
                State::Hole => unreachable!("Cannot poll an empty hole"),
            };

            // poll it
            let reply = match underlying_fut.poll_unpin(cx) {
                Poll::Pending => return Poll::Pending,
                Poll::Ready(Err(e)) => return Poll::Ready(Some(Err(e))),
                Poll::Ready(Ok(r)) => r,
            };

            // cannibalize the future
            let display_ref = match mem::replace(&mut this.state, State::Hole) {
                State::Waiting(fut) => fut.cannibalize(),
                _ => unreachable!(),
            };
            this.state = State::NotWaiting(display_ref);

            // see if this is a whole
            if reply.is_last() {
                Poll::Ready(None)
            } else {
                Poll::Ready(Some(Ok(reply)))
            }
        }
    }
}
