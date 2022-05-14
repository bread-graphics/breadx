// MIT/Apache2 License

use crate::{connection::Connection, Result};

use super::{BasicDisplay, Display, DisplayBase, RawReply, RawRequest};
use alloc::sync::Arc;
use core::cell::RefCell;
use x11rb_protocol::protocol::{xproto::Setup, Event};

/// An implementation of `Display` that can be used immutably through
/// thread-unsafe cells.
pub struct CellDisplay<Conn> {
    inner: RefCell<BasicDisplay<Conn>>,
    setup: Arc<Setup>,
    default_screen_index: usize,
}

impl<Conn: Connection> From<BasicDisplay<Conn>> for CellDisplay<Conn> {
    fn from(inner: BasicDisplay<Conn>) -> Self {
        let setup = inner.setup.clone();
        Self {
            default_screen_index: inner.default_screen_index,
            inner: RefCell::new(inner),
            setup,
        }
    }
}

impl<Conn: Connection> DisplayBase for CellDisplay<Conn> {
    fn default_screen_index(&self) -> usize {
        self.default_screen_index
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.inner.get_mut().poll_for_reply_raw(seq)
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.inner.get_mut().poll_for_event()
    }
}

impl<Conn: Connection> Display for CellDisplay<Conn> {
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64> {
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
}

impl<Conn: Connection> DisplayBase for &CellDisplay<Conn> {
    fn default_screen_index(&self) -> usize {
        self.default_screen_index
    }

    fn setup(&self) -> &Setup {
        &self.setup
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.inner.borrow_mut().poll_for_event()
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        self.inner.borrow_mut().poll_for_reply_raw(seq)
    }
}

impl<Conn: Connection> Display for &CellDisplay<Conn> {
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64> {
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
}
