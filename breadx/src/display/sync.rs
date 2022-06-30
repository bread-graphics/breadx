// MIT/Apache2 License

#![cfg(feature = "sync_display")]

use super::{Display, DisplayBase, RawReply, RawRequest};
use crate::{mutex::Mutex, Result};
use alloc::sync::Arc;
use x11rb_protocol::protocol::{xproto::Setup, Event};

cfg_async! {
    use super::{AsyncStatus, CanBeAsyncDisplay, AsyncDisplay, Interest};
    use concurrent_queue::ConcurrentQueue;
    use core::task::{Context, Waker, Poll};
}

/// A `Display` that uses a mutex to coordinate access.
pub struct SyncDisplay<Dpy: ?Sized> {
    setup: Arc<Setup>,
    default_screen_index: usize,
    waiters: Waiters,
    inner: Mutex<Dpy>,
}

/// A helper type for `SyncDisplay` that allows us to use `Waker`s to wake up
/// the `SyncDisplay` when a lock is released.
///
/// Basically uses a "thunder lock". Performant for fewer concurrent accesses.
///
/// Is a no-op for non-async targets.
struct Waiters {
    #[cfg(feature = "async")]
    wakers: ConcurrentQueue<Waker>,
}

impl<Dpy: DisplayBase> From<Dpy> for SyncDisplay<Dpy> {
    fn from(bd: Dpy) -> Self {
        let setup = bd.setup().clone();
        Self {
            setup,
            default_screen_index: bd.default_screen_index(),
            inner: Mutex::new(bd),
            waiters: Waiters {
                #[cfg(feature = "async")]
                wakers: make_waker_queue(),
            },
        }
    }
}

#[cfg(all(feature = "async", feature = "std"))]
fn make_waker_queue() -> ConcurrentQueue<Waker> {
    // if the user wants an unbounded queue, oblige them
    if std::env::var_os("BREADX_UNBOUNDED_WAKER_QUEUE")
        .as_ref()
        .and_then(|t| t.to_str())
        == Some("1")
    {
        ConcurrentQueue::unbounded()
    } else {
        ConcurrentQueue::bounded(1024)
    }
}

#[cfg(all(feature = "async", not(feature = "std")))]
fn make_waker_queue() -> ConcurrentQueue<Waker> {
    ConcurrentQueue::bounded(1024)
}

impl<Dpy: ?Sized> SyncDisplay<Dpy> {
    fn with_inner<R>(&self, f: impl FnOnce(&mut Dpy) -> R) -> R {
        let res = f(&mut *self.inner.lock());

        // signal that we have released the mutex
        self.waiters.release();
        res
    }

    fn with_inner_mut<R>(&mut self, f: impl FnOnce(&mut Dpy) -> R) -> R {
        let res = f(self.inner.get_mut());
        // don't need to signal, &mut indicates that no one has an
        // & reference == no wakers
        res
    }

    fn try_with_inner<R>(
        &self,
        f: impl FnOnce(&mut Dpy) -> Result<Option<R>>,
    ) -> Result<Option<R>> {
        match self.inner.try_lock() {
            Some(mut lock) => {
                let res = f(&mut *lock);
                self.waiters.release();
                res
            }
            None => Ok(None),
        }
    }

    #[cfg(feature = "async")]
    fn try_status_inner<R>(
        &self,
        ctx: &mut Context<'_>,
        f: impl FnOnce(&mut Dpy, &mut Context<'_>) -> Result<AsyncStatus<R>>,
    ) -> Result<AsyncStatus<R>> {
        match self.inner.try_lock() {
            Some(mut lock) => {
                let res = f(&mut *lock, ctx);
                self.waiters.release();
                res
            }
            None => {
                self.waiters.push(ctx.waker());
                Ok(AsyncStatus::UserControlled)
            }
        }
    }

    #[cfg(feature = "async")]
    fn poll_inner<R>(
        &self,
        ctx: &mut Context<'_>,
        f: impl FnOnce(&mut Dpy, &mut Context<'_>) -> Poll<Result<R>>,
    ) -> Poll<Result<R>> {
        match self.inner.try_lock() {
            Some(mut lock) => {
                let res = f(&mut *lock, ctx);
                self.waiters.release();
                res
            }
            None => {
                self.waiters.push(ctx.waker());
                Poll::Pending
            }
        }
    }
}

impl Waiters {
    #[cfg(feature = "async")]
    fn push(&self, waker: &Waker) {
        self.wakers.push(waker.clone()).ok();
    }

    fn release(&self) {
        #[cfg(feature = "async")]
        while let Ok(waker) = self.wakers.pop() {
            waker.wake();
        }
    }
}

impl<Dpy: DisplayBase + ?Sized> DisplayBase for SyncDisplay<Dpy> {
    fn setup(&self) -> &Arc<Setup> {
        &self.setup
    }

    fn default_screen_index(&self) -> usize {
        self.default_screen_index
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.with_inner_mut(DisplayBase::poll_for_event)
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.with_inner_mut(move |inner| inner.poll_for_reply_raw(seq))
    }
}

impl<Dpy: DisplayBase + ?Sized> DisplayBase for &SyncDisplay<Dpy> {
    fn setup(&self) -> &Arc<Setup> {
        &self.setup
    }

    fn default_screen_index(&self) -> usize {
        self.default_screen_index
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.try_with_inner(DisplayBase::poll_for_event)
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.try_with_inner(move |inner| inner.poll_for_reply_raw(seq))
    }
}

impl<Dpy: Display + ?Sized> Display for SyncDisplay<Dpy> {
    fn send_request_raw(&mut self, req: RawRequest<'_, '_>) -> Result<u64> {
        self.with_inner_mut(move |inner| inner.send_request_raw(req))
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        self.with_inner_mut(Display::wait_for_event)
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        self.with_inner_mut(move |inner| inner.wait_for_reply_raw(seq))
    }

    fn flush(&mut self) -> Result<()> {
        self.with_inner_mut(Display::flush)
    }

    fn generate_xid(&mut self) -> Result<u32> {
        self.with_inner_mut(Display::generate_xid)
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        self.with_inner_mut(Display::maximum_request_length)
    }

    fn synchronize(&mut self) -> Result<()> {
        self.with_inner_mut(Display::synchronize)
    }

    fn check_for_error(&mut self, seq: u64) -> Result<()> {
        self.with_inner_mut(move |inner| inner.check_for_error(seq))
    }
}

impl<Dpy: Display + ?Sized> Display for &SyncDisplay<Dpy> {
    fn send_request_raw(&mut self, req: RawRequest<'_, '_>) -> Result<u64> {
        self.with_inner(move |inner| inner.send_request_raw(req))
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        self.with_inner(Display::wait_for_event)
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        self.with_inner(move |inner| inner.wait_for_reply_raw(seq))
    }

    fn flush(&mut self) -> Result<()> {
        self.with_inner(Display::flush)
    }

    fn generate_xid(&mut self) -> Result<u32> {
        self.with_inner(Display::generate_xid)
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        self.with_inner(Display::maximum_request_length)
    }

    fn synchronize(&mut self) -> Result<()> {
        self.with_inner(Display::synchronize)
    }

    fn check_for_error(&mut self, seq: u64) -> Result<()> {
        self.with_inner(move |inner| inner.check_for_error(seq))
    }
}

cfg_async! {
    impl<Dpy: CanBeAsyncDisplay + ?Sized> CanBeAsyncDisplay for SyncDisplay<Dpy> {
        fn format_request(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>> {
            self.with_inner_mut(move |inner| inner.format_request(req, ctx))
        }

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            self.with_inner_mut(move |inner| inner.try_send_request_raw(req, ctx))
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
            self.with_inner_mut(move |inner| inner.try_wait_for_event(ctx))
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>> {
            self.with_inner_mut(move |inner| inner.try_wait_for_reply_raw(seq, ctx))
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
            self.with_inner_mut(move |inner| inner.try_flush(ctx))
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
            self.with_inner_mut(move |inner| inner.try_generate_xid(ctx))
        }

        fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
            self.with_inner_mut(move |inner| inner.try_maximum_request_length(ctx))
        }

        fn try_check_for_error(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            self.with_inner_mut(move |inner| inner.try_check_for_error(seq, ctx))
        }
    }

    impl<Dpy: CanBeAsyncDisplay + ?Sized> CanBeAsyncDisplay for &SyncDisplay<Dpy> {
        fn format_request(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>> {
            self.try_status_inner(ctx, move |inner, ctx| inner.format_request(req, ctx))
        }

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            self.try_status_inner(ctx, move |inner, ctx| inner.try_send_request_raw(req, ctx))
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>> {
            self.try_status_inner(ctx, move |inner, ctx| {
                inner.try_wait_for_reply_raw(seq, ctx)
            })
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
            self.try_status_inner(ctx, CanBeAsyncDisplay::try_wait_for_event)
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
            self.try_status_inner(ctx, CanBeAsyncDisplay::try_flush)
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
            self.try_status_inner(ctx, CanBeAsyncDisplay::try_generate_xid)
        }

        fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
            self.try_status_inner(ctx, CanBeAsyncDisplay::try_maximum_request_length)
        }

        fn try_check_for_error(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            self.try_status_inner(ctx, move |inner, ctx| inner.try_check_for_error(seq, ctx))
        }
    }

    impl<Dpy: AsyncDisplay + ?Sized> AsyncDisplay for SyncDisplay<Dpy> {
        fn poll_for_interest(
            &mut self,
            interest: Interest,
            callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context< '_>) -> Result<()>,
            ctx: &mut Context< '_>,
        ) -> Poll<Result<()>> {
            self.poll_inner(ctx, |dpy, ctx| dpy.poll_for_interest(interest, callback, ctx))
        }
    }

    impl<Dpy: AsyncDisplay + ?Sized> AsyncDisplay for &SyncDisplay<Dpy> {
        fn poll_for_interest(
            &mut self,
            interest: Interest,
            callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context< '_>) -> Result<()>,
            ctx: &mut Context< '_>,
        ) -> Poll<Result<()>> {
            self.poll_inner(ctx, |dpy, ctx| dpy.poll_for_interest(interest, callback, ctx))
        }
    }
}
