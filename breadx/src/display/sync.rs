// MIT/Apache2 License

#![cfg(feature = "sync_display")]

use alloc::vec;
use alloc::vec::Vec;
use x11rb_protocol::{
    packet_reader::PacketReader,
    protocol::{
        bigreq::EnableRequest,
        xproto::{GetInputFocusRequest, QueryExtensionRequest, Setup},
        Event,
    },
    x11_utils::ExtensionInformation,
};

use crate::{
    connection::{new_io_slice, Connection},
    mutex::{Mutex, RwLock},
    Error, Fd, Result,
};

use super::{
    AsyncStatus, BasicDisplay, Display, DisplayBase, ExtensionMap, Prefetch, RawReply, RawRequest,
    X11Core,
};
use core::mem;
use std::sync::Arc;

cfg_async! {
    use core::task::Waker;
    use concurrent_queue::ConcurrentQueue;
}

/// An implementation of `Display` that can be used immutably through
/// a thread-safe mutex.
pub struct SyncDisplay<Conn> {
    /// The actual state of the X11 connection.
    ///
    /// When this is locked, it should only ever be locked temporarily.
    /// Waiting should not occur while this lock is held.
    core: Mutex<State>,
    /// I/O state information.
    ///
    /// This lock can he held during periods of waiting.
    io: Mutex<Io<Conn>>,
    /// Waiter list.
    ///
    /// For async, this contains a list of wakers waiting for
    /// the `io` mutex to be unlocked. Sync types will just wait on
    /// the `io` mutex.
    waiters: Waiters,
    /// Extension map.
    extension_map: RwLock<ExtensionMap>,

    // fields already in BasicDisplay
    setup: Arc<Setup>,
    default_screen_index: usize,
}

/// Fields necessary for I/O.
struct Io<Conn> {
    conn: Conn,
    packet_reader: PacketReader,
}

/// Fields necessary for keeping state.
struct State {
    core: X11Core,
    max_request_size: Option<Prefetch<EnableRequest>>,
}

struct Waiters {
    #[cfg(feature = "async")]
    wakers: ConcurrentQueue<Waker>,
}

impl Default for Waiters {
    fn default() -> Self {
        Self {
            #[cfg(feature = "async")]
            wakers: ConcurrentQueue::unbounded(),
        }
    }
}

impl<Conn: Connection> From<BasicDisplay<Conn>> for SyncDisplay<Conn> {
    fn from(display: BasicDisplay<Conn>) -> Self {
        // dissassemble the BasicDisplay
        let BasicDisplay {
            core,
            setup,
            packet_reader,
            conn,
            max_request_size,
            default_screen_index,
            extension_map,
        } = display;

        // assemble the SyncDisplay
        let io = Io {
            conn,
            packet_reader,
        };
        let state = State {
            core,
            max_request_size,
        };
        let waiters = Waiters::default();
        let extension_map = RwLock::new(extension_map);

        Self {
            core: Mutex::new(state),
            io: Mutex::new(io),
            waiters,
            setup,
            default_screen_index,
            extension_map,
        }
    }
}

impl<Conn: Connection> SyncDisplay<Conn> {
    fn wait(&self) -> Result<()> {
        let span = tracing::trace_span!("SyncDisplay::wait");
        let _enter = span.enter();

        // acquire an I/O guard and run it
        let mut fds = vec![];
        let packet = {
            let mut io = self.io.lock();
            let amt = io.read_to_buffer(&mut fds)?;

            tracing::debug!(read = amt, num_fds = fds.len(), "read data from server",);

            // see if we have a packet ready
            io.packet_reader.advance(amt)
        };

        // acquire a lock on the core
        let mut core = self.core.lock();

        // push iems onto it
        core.core.enqueue_fds(fds);
        if let Some(packet) = packet {
            tracing::debug!("completed packet");
            core.core.enqueue_packet(packet);
        }

        Ok(())
    }

    fn non_blocking_wait(&self, waker: Option<&Waker>) -> Result<AsyncStatus<()>> {
        // try to read from the stream unless the mutex is locked
        // or we encounter a would-block error
        let mut fds = vec![];
        let packet = {
            let mut io = match self.io.try_lock() {
                Some(io) => io,
                None => {
                    // unable to lock, register the waiter for when
                    // we release the lock later
                    // TODO: register waker
                    return Ok(AsyncStatus::UserControlled);
                }
            };

            let amt = match io.read_to_buffer_non_blocking(&mut fds) {
                Ok(amt) => amt,
                Err(e) if e.would_block() => return Ok(AsyncStatus::Read),
                Err(e) => return Err(e),
            };

            io.packet_reader.advance(amt)
        };

        // lock the core
        let mut core = self.core.lock();
        core.core.enqueue_fds(fds);
        if let Some(packet) = packet {
            core.core.enqueue_packet(packet);
        }

        Ok(AsyncStatus::Ready(()))
    }

    fn prefetch_maximum_length(&self) -> Result<usize> {
        // take it so lifetimes aren't an issue
        let mut prefetch = self.core.lock().max_request_size.take().unwrap();

        // we're now evaluating bigreq
        let mut this = self;
        let sz = prefetch.evaluate(&mut this).copied();
        self.core.lock().max_request_size = Some(prefetch);

        Ok(sz?.unwrap_or(self.setup.maximum_request_length as usize))
    }

    fn prefetch_extension(
        &self,
        name: &'static str,
    ) -> Result<Prefetch<QueryExtensionRequest<'static>>> {
        let mut pf = Prefetch::new(QueryExtensionRequest {
            name: name.as_bytes().into(),
        });

        let mut this = self;
        pf.evaluate(&mut this)?;
        Ok(pf)
    }

    fn extension_info(&self, name: &'static str) -> Result<ExtensionInformation> {
        match self.extension_map.read().get(name) {
            Some(info) => Ok(info),
            None => {
                // try to prefetch it
                let pf = self.prefetch_extension(name)?;
                let info = pf.get_assert().as_ref().cloned().unwrap();
                self.extension_map.write().insert(name, pf);
                Ok(info)
            }
        }
    }

    fn bigreq(&self) -> Result<(bool, usize)> {
        loop {
            // temporarily lock the state
            let state = self.core.lock();
            match state
                .max_request_size
                .as_ref()
                .map(|mrs| mrs.get_if_resolved().copied())
            {
                Some(Some(sz)) => {
                    return Ok((
                        sz.is_some(),
                        sz.unwrap_or(self.setup.maximum_request_length as usize),
                    ))
                }
                Some(None) => {
                    // prefetch
                    mem::drop(state);
                    self.prefetch_maximum_length()?;
                }
                None => {
                    // bigreq is currently being processed, return the
                    // setup size
                    return Ok((false, self.setup.maximum_request_length as usize));
                }
            }
        }
    }

    fn format_request(&self, req: &mut RawRequest<'_>) -> Result<u64> {
        let (is_bigreq, _) = self.bigreq()?;
        let extension_opcode = req
            .extension()
            .map(|ext| self.extension_info(ext))
            .transpose()?
            .map(|ext| ext.major_opcode);

        let seq = loop {
            match self.core.lock().core.send_request(req.variant()) {
                Some(seq) => break seq,
                None => self.synchronize_inner()?,
            }
        };

        // format the request
        req.compute_length(is_bigreq);
        if let Some(extension_opcode) = extension_opcode {
            req.set_extension_opcode(extension_opcode);
        }

        Ok(seq)
    }

    fn send_request_raw_inner(&self, mut req: RawRequest<'_>) -> Result<u64> {
        // format the request
        let sequence = self.format_request(&mut req)?;

        // send the request
        let (buf, mut fds) = req.into_raw_parts();
        let mut nwritten = 0;
        let mut io = self.io.lock();

        while nwritten < buf.len() {
            let iov = [new_io_slice(&buf[nwritten..])];
            nwritten += io.conn.send_slices_and_fds(&iov, &mut fds)?;
        }

        Ok(sequence)
    }

    fn synchronize_inner(&self) -> Result<()> {
        let get_input_focus = GetInputFocusRequest {};
        let req = RawRequest::from_request_reply(get_input_focus);
        let seq = (&*self).send_request_raw(req)?;

        (&*self).wait_for_reply_raw(seq).map(|_| ())
    }
}

impl<Conn: Connection> Io<Conn> {
    fn read_to_buffer(&mut self, fds: &mut Vec<Fd>) -> Result<usize> {
        self.conn
            .recv_slice_and_fds(self.packet_reader.buffer(), fds)
    }

    fn read_to_buffer_non_blocking(&mut self, fds: &mut Vec<Fd>) -> Result<usize> {
        self.conn
            .non_blocking_recv_slice_and_fds(self.packet_reader.buffer(), fds)
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
        loop {
            if let Some(reply) = self.core.get_mut().core.fetch_event(&self.extension_map)? {
                return Ok(Some(reply));
            }

            // wait for the stream
            if !self.non_blocking_wait(None)?.is_ready() {
                return Ok(None);
            }
        }
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        loop {
            if let Some(reply) = self
                .core
                .get_mut()
                .core
                .fetch_reply(seq, &self.extension_map)?
            {
                return Ok(Some(reply));
            }

            // wait for the stream
            if !self.non_blocking_wait(None)?.is_ready() {
                return Ok(None);
            }
        }
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
        loop {
            if let Some(reply) = self.core.lock().core.fetch_event(&self.extension_map)? {
                return Ok(Some(reply));
            }

            // wait for the stream
            if !self.non_blocking_wait(None)?.is_ready() {
                return Ok(None);
            }
        }
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        loop {
            if let Some(reply) = self
                .core
                .lock()
                .core
                .fetch_reply(seq, &self.extension_map)?
            {
                return Ok(Some(reply));
            }

            // wait for the stream
            if !self.non_blocking_wait(None)?.is_ready() {
                return Ok(None);
            }
        }
    }
}

impl<Conn: Connection> Display for SyncDisplay<Conn> {
    fn send_request_raw(&mut self, req: RawRequest<'_>) -> Result<u64> {
        self.send_request_raw_inner(req)
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        loop {
            if let Some(reply) = self.core.get_mut().core.fetch_event(&self.extension_map)? {
                return Ok(reply);
            }

            // wait for an event
            self.wait()?;
        }
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        loop {
            if let Some(reply) = self
                .core
                .get_mut()
                .core
                .fetch_reply(seq, &self.extension_map)?
            {
                return Ok(reply);
            }

            self.wait()?;
        }
    }

    fn synchronize(&mut self) -> Result<()> {
        self.synchronize_inner()
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        let (_, len) = self.bigreq()?;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.get_mut().conn.flush()
    }
}

impl<Conn: Connection> Display for &SyncDisplay<Conn> {
    fn send_request_raw(&mut self, req: RawRequest<'_>) -> Result<u64> {
        self.send_request_raw_inner(req)
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        loop {
            if let Some(reply) = self.core.lock().core.fetch_event(&self.extension_map)? {
                return Ok(reply);
            }

            // wait for an event
            self.wait()?;
        }
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        loop {
            if let Some(reply) = self
                .core
                .lock()
                .core
                .fetch_reply(seq, &self.extension_map)?
            {
                return Ok(reply);
            }

            self.wait()?;
        }
    }

    fn synchronize(&mut self) -> Result<()> {
        self.synchronize_inner()
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        let (_, len) = self.bigreq()?;
        Ok(len)
    }

    fn flush(&mut self) -> Result<()> {
        self.io.lock().conn.flush()
    }
}
