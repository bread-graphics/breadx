//               Copyright John Nunley, 2022.
// Distributed under the Boost Software License, Version 1.0.
//       (See accompanying file LICENSE or copy at
//         https://www.boost.org/LICENSE_1_0.txt)

use crate::Result;

use super::{Display, DisplayBase, RawReply, RawRequest};
use alloc::sync::Arc;
use core::cell::RefCell;
use x11rb_protocol::protocol::{xproto::Setup, Event};

cfg_async! {
    use super::{Interest, AsyncDisplay, CanBeAsyncDisplay};
    use core::task::{Context, Poll};
}

/// An implementation of `Display` that can be used immutably through
/// thread-unsafe cells.
pub struct CellDisplay<Dpy: ?Sized> {
    setup: Arc<Setup>,
    default_screen_index: usize,
    inner: RefCell<Dpy>,
}

impl<Dpy: DisplayBase> From<Dpy> for CellDisplay<Dpy> {
    fn from(inner: Dpy) -> Self {
        let setup = inner.setup().clone();
        Self {
            default_screen_index: inner.default_screen_index(),
            inner: RefCell::new(inner),
            setup,
        }
    }
}

impl<Dpy: DisplayBase + ?Sized> DisplayBase for CellDisplay<Dpy> {
    fn default_screen_index(&self) -> usize {
        self.default_screen_index
    }

    fn setup(&self) -> &Arc<Setup> {
        &self.setup
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.inner.get_mut().poll_for_reply_raw(seq)
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.inner.get_mut().poll_for_event()
    }
}

impl<Dpy: Display + ?Sized> Display for CellDisplay<Dpy> {
    fn send_request_raw(&mut self, req: RawRequest<'_, '_>) -> Result<u64> {
        self.inner.get_mut().send_request_raw(req)
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        self.inner.get_mut().wait_for_event()
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        self.inner.get_mut().wait_for_reply_raw(seq)
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        self.inner.get_mut().maximum_request_length()
    }

    fn generate_xid(&mut self) -> Result<u32> {
        self.inner.get_mut().generate_xid()
    }

    fn synchronize(&mut self) -> Result<()> {
        self.inner.get_mut().synchronize()
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.get_mut().flush()
    }

    fn check_for_error(&mut self, seq: u64) -> Result<()> {
        self.inner.get_mut().check_for_error(seq)
    }
}

impl<Dpy: DisplayBase + ?Sized> DisplayBase for &CellDisplay<Dpy> {
    fn default_screen_index(&self) -> usize {
        self.default_screen_index
    }

    fn setup(&self) -> &Arc<Setup> {
        &self.setup
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.inner.borrow_mut().poll_for_event()
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.inner.borrow_mut().poll_for_reply_raw(seq)
    }
}

impl<Dpy: Display + ?Sized> Display for &CellDisplay<Dpy> {
    fn send_request_raw(&mut self, req: RawRequest<'_, '_>) -> Result<u64> {
        self.inner.borrow_mut().send_request_raw(req)
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        self.inner.borrow_mut().wait_for_event()
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        self.inner.borrow_mut().wait_for_reply_raw(seq)
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        self.inner.borrow_mut().maximum_request_length()
    }

    fn generate_xid(&mut self) -> Result<u32> {
        self.inner.borrow_mut().generate_xid()
    }

    fn synchronize(&mut self) -> Result<()> {
        self.inner.borrow_mut().synchronize()
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.borrow_mut().flush()
    }

    fn check_for_error(&mut self, seq: u64) -> Result<()> {
        self.inner.borrow_mut().check_for_error(seq)
    }
}

cfg_async! {
    impl<Dpy: CanBeAsyncDisplay + ?Sized> CanBeAsyncDisplay for CellDisplay<Dpy> {
        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<()>> {
            self.inner.get_mut().try_send_request_raw(req, ctx)
        }

        fn format_request(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<u64>> {
            self.inner.get_mut().format_request(req, ctx)
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<super::AsyncStatus<Event>> {
            self.inner.get_mut().try_wait_for_event(ctx)
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<RawReply>> {
            self.inner.get_mut().try_wait_for_reply_raw(seq, ctx)
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<super::AsyncStatus<()>> {
            self.inner.get_mut().try_flush(ctx)
        }

        fn try_maximum_request_length(
            &mut self,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<usize>> {
            self.inner.get_mut().try_maximum_request_length(ctx)
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<super::AsyncStatus<u32>> {
            self.inner.get_mut().try_generate_xid(ctx)
        }

        fn try_check_for_error(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<()>> {
            self.inner.get_mut().try_check_for_error(seq, ctx)
        }
    }

    impl<Dpy: CanBeAsyncDisplay + ?Sized> CanBeAsyncDisplay for &CellDisplay<Dpy> {
        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<()>> {
            self.inner.borrow_mut().try_send_request_raw(req, ctx)
        }

        fn format_request(
            &mut self,
            req: &mut RawRequest<'_, '_>,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<u64>> {
            self.inner.borrow_mut().format_request(req, ctx)
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<super::AsyncStatus<Event>> {
            self.inner.borrow_mut().try_wait_for_event(ctx)
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<RawReply>> {
            self.inner.borrow_mut().try_wait_for_reply_raw(seq, ctx)
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<super::AsyncStatus<()>> {
            self.inner.borrow_mut().try_flush(ctx)
        }

        fn try_maximum_request_length(
            &mut self,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<usize>> {
            self.inner.borrow_mut().try_maximum_request_length(ctx)
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<super::AsyncStatus<u32>> {
            self.inner.borrow_mut().try_generate_xid(ctx)
        }

        fn try_check_for_error(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<super::AsyncStatus<()>> {
            self.inner.borrow_mut().try_check_for_error(seq, ctx)
        }
    }

    impl<Dpy: AsyncDisplay + ?Sized> AsyncDisplay for CellDisplay<Dpy> {
        fn poll_for_interest(
            &mut self,
            interest: Interest,
            callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context< '_>) -> Result<()>,
            ctx: &mut Context< '_>
        ) -> Poll<Result<()>> {
            self.inner.get_mut().poll_for_interest(interest, callback, ctx)
        }
    }

    impl<Dpy: AsyncDisplay + ?Sized> AsyncDisplay for &CellDisplay<Dpy> {
        fn poll_for_interest(
            &mut self,
            interest: Interest,
            callback: &mut dyn FnMut(&mut dyn AsyncDisplay, &mut Context< '_>) -> Result<()>,
            ctx: &mut Context< '_>
        ) -> Poll<Result<()>> {
            self.inner.borrow_mut().poll_for_interest(interest, callback, ctx)
        }
    }
}
