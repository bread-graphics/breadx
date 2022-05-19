// MIT/Apache2 License

#![cfg(feature = "sync_display")]

use core::task::{Context, Waker};

use super::{AsyncStatus, BasicDisplay, Display, DisplayBase, RawReply, RawRequest};
use crate::{connection::Connection, mutex::Mutex, Result};
use alloc::sync::Arc;
use concurrent_queue::ConcurrentQueue;
use x11rb_protocol::protocol::{xproto::Setup, Event};

cfg_async! {
    use super::{CanBeAsyncDisplay};
}

/// A `Display` that uses a mutex to coordinate access.
pub struct SyncDisplay<Conn> {
    inner: Mutex<BasicDisplay<Conn>>,
    setup: Arc<Setup>,
    default_screen_index: usize,
    waiters: Waiters,
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

impl<Conn: Connection> From<BasicDisplay<Conn>> for SyncDisplay<Conn> {
    fn from(bd: BasicDisplay<Conn>) -> Self {
        let setup = bd.setup.clone();
        Self {
            setup,
            default_screen_index: bd.default_screen_index,
            inner: Mutex::new(bd),
            waiters: Waiters {
                #[cfg(feature = "async")]
                wakers: ConcurrentQueue::unbounded(),
            },
        }
    }
}

impl<Conn: Connection> SyncDisplay<Conn> {
    fn with_inner<R>(&self, f: impl FnOnce(&mut BasicDisplay<Conn>) -> R) -> R {
        let res = f(&mut *self.inner.lock());

        // signal that we have released the mutex
        self.waiters.release();
        res
    }

    fn with_inner_mut<R>(&mut self, f: impl FnOnce(&mut BasicDisplay<Conn>) -> R) -> R {
        let res = f(self.inner.get_mut());
        self.waiters.release();
        res
    }

    fn try_with_inner<R>(
        &self,
        f: impl FnOnce(&mut BasicDisplay<Conn>) -> Result<Option<R>>,
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
    fn poll_inner<R>(
        &self,
        ctx: &mut Context<'_>,
        f: impl FnOnce(&mut BasicDisplay<Conn>, &mut Context<'_>) -> Result<AsyncStatus<R>>,
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
}

impl Waiters {
    fn push(&self, waker: &Waker) {
        #[cfg(feature = "async")]
        self.wakers.push(waker.clone()).ok();
    }

    fn release(&self) {
        #[cfg(feature = "async")]
        while let Ok(waker) = self.wakers.pop() {
            waker.wake();
        }
    }
}

impl<Conn: Connection> DisplayBase for SyncDisplay<Conn> {
    fn setup(&self) -> &Setup {
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

impl<Conn: Connection> DisplayBase for &SyncDisplay<Conn> {
    fn setup(&self) -> &Setup {
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

impl<Conn: Connection> Display for SyncDisplay<Conn> {
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64> {
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
}

impl<Conn: Connection> Display for &SyncDisplay<Conn> {
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64> {
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
}

cfg_async! {
    impl<Conn: Connection> CanBeAsyncDisplay for SyncDisplay<Conn> {
        fn format_request(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>> {
            self.with_inner_mut(move |inner| inner.format_request(req, ctx))
        }

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest,
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
    }

    impl<Conn: Connection> CanBeAsyncDisplay for &SyncDisplay<Conn> {
        fn format_request(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>> {
            self.poll_inner(ctx, move |inner, ctx| inner.format_request(req, ctx))
        }

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            self.poll_inner(ctx, move |inner, ctx| inner.try_send_request_raw(req, ctx))
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>> {
            self.poll_inner(ctx, move |inner, ctx| {
                inner.try_wait_for_reply_raw(seq, ctx)
            })
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
            self.poll_inner(ctx, CanBeAsyncDisplay::try_wait_for_event)
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
            self.poll_inner(ctx, CanBeAsyncDisplay::try_flush)
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
            self.poll_inner(ctx, CanBeAsyncDisplay::try_generate_xid)
        }

        fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
            self.poll_inner(ctx, CanBeAsyncDisplay::try_maximum_request_length)
        }
    }
}
