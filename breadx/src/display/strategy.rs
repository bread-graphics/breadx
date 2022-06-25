// MIT/Apache2 License

use alloc::vec::Vec;
use core::task::Waker;

cfg_async! {
    use core::task::Context;
}

use crate::{
    connection::ReadHalf,
    display::{prefetch::PrefetchTarget, AsyncStatus, Prefetch},
    Fd, Result,
};

use super::{CanBeAsyncDisplay, Display};

/// Whether we should use a non-blocking strategy or a blocking
/// strategy for waiting for events.
pub(crate) trait Strategy<Conn: ?Sized, Dpy: ?Sized> {
    /// Read in slices from the connection.
    fn read_slices(
        &mut self,
        conn: &mut Conn,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize>;

    /// Process a prefetch item.
    fn prefetch<'p, P: PrefetchTarget>(
        &mut self,
        display: &mut Dpy,
        prefetch: &'p mut Prefetch<P>,
        ctx: Option<&Waker>,
    ) -> Result<AsyncStatus<&'p P::Target>>;

    /// Strategy description for tracing output.
    fn description(&self) -> &'static str;
}

/// Always assume that we are blocking.
pub(crate) struct BlockingStrategy;

impl<Conn: ReadHalf + ?Sized, Dpy: Display + ?Sized> Strategy<Conn, Dpy> for BlockingStrategy {
    fn read_slices(
        &mut self,
        conn: &mut Conn,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        conn.recv_slice_and_fds(slice, fds)
    }

    fn prefetch<'p, P: PrefetchTarget>(
        &mut self,
        display: &mut Dpy,
        prefetch: &'p mut Prefetch<P>,
        _ctx: Option<&Waker>,
    ) -> Result<AsyncStatus<&'p P::Target>> {
        prefetch.evaluate(display).map(AsyncStatus::Ready)
    }

    fn description(&self) -> &'static str {
        "blocking"
    }
}

/// Assume that we are just polling for a reply or event.
pub(crate) struct PollingStrategy;

impl<Conn: ReadHalf + ?Sized, Dpy: ?Sized> Strategy<Conn, Dpy> for PollingStrategy {
    fn read_slices(
        &mut self,
        conn: &mut Conn,
        slice: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize> {
        conn.non_blocking_recv_slice_and_fds(slice, fds)
    }

    fn prefetch<'p, P: PrefetchTarget>(
        &mut self,
        _display: &mut Dpy,
        _prefetch: &'p mut Prefetch<P>,
        _ctx: Option<&Waker>,
    ) -> Result<AsyncStatus<&'p P::Target>> {
        unreachable!()
    }

    fn description(&self) -> &'static str {
        "polling"
    }
}

cfg_async! {
    /// Always assume that we are not blocking.
    pub(crate) struct NonBlockingStrategy;

    impl<Conn: ReadHalf + ?Sized, Dpy: CanBeAsyncDisplay + ?Sized> Strategy<Conn, Dpy> for NonBlockingStrategy {
        fn read_slices(
            &mut self,
            conn: &mut Conn,
            slice: &mut [u8],
            fds: &mut Vec<Fd>,
        ) -> Result<usize> {
            conn.non_blocking_recv_slice_and_fds(slice, fds)
        }

        fn prefetch<'p, P: PrefetchTarget>(
            &mut self,
            display: &mut Dpy,
            prefetch: &'p mut Prefetch<P>,
            ctx: Option<&Waker>,
        ) -> Result<AsyncStatus<&'p P::Target>> {
            let mut ctx = Context::from_waker(ctx.unwrap());
            prefetch.try_evaluate(display, &mut ctx)
        }

        fn description(&self) -> &'static str {
            "non-blocking"
        }
    }
}
