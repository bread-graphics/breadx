// MIT/Apache2 License

use core::mem;

use super::{
    AsyncStatus, Display, DisplayBase, DisplayFunctionsExt, ExtensionMap, Poisonable, Prefetch,
    RawReply, RawRequest, X11Core,
};
use crate::{
    connection::{new_io_slice, BufConnection, Connection},
    Error, HashMap, InvalidState, NameConnection, Result,
};

use alloc::{sync::Arc, vec, vec::Vec};
use x11rb_protocol::{
    connect::Connect,
    id_allocator::IdsExhausted,
    packet_reader::PacketReader,
    parse_display,
    protocol::{
        bigreq::EnableRequest,
        xproto::{GetInputFocusRequest, QueryExtensionRequest, Setup},
        Event,
    },
    x11_utils::ExtensionInformation,
    xauth,
};

cfg_async! {
    use super::CanBeAsyncDisplay;
}

/// An implementation of `Display` that requires a mutable reference
/// to use.
pub struct BasicDisplay<Conn> {
    // all of the fields here are pub(crate) so that the From
    // impl for SyncDisplay can access them.
    /// The protocol implementation core.
    pub(crate) core: X11Core,
    /// The setup returned from the server.
    pub(crate) setup: Arc<Setup>,
    /// The packet reader buffer.
    pub(crate) packet_reader: PacketReader,
    /// The connection to the server. This may be able to be poisoned.
    pub(crate) conn: Conn,
    /// The maximum number of bytes we can send at once.
    pub(crate) max_request_size: Option<Prefetch<EnableRequest>>,
    /// The default screen index.
    pub(crate) default_screen_index: usize,
    /// Map between extension names and the extension information.
    pub(crate) extension_map: ExtensionMap,
}

cfg_std! {
    pub type DisplayConnection = BasicDisplay<NameConnection>;

    impl DisplayConnection {
        /// Connect to the server using the given display name.
        pub fn connect(display: Option<&str>) -> Result<Self> {
            let span = tracing::info_span!("connect");
            let _enter = span.enter();

            crate::initialization(|| {
                // try to create a NameConnection
                let dpy = parse_display::parse_display(display)
                              .ok_or_else(|| Error::couldnt_parse_display(display.is_none()))?;

                tracing::trace!(display = ?dpy);

                let screen = dpy.screen;
                let display_num = dpy.display;
                let conn = NameConnection::from_parsed_display(dpy, display.is_none())?;

                // find an xauth entry for it
                let (family, address) = conn.get_address()?;

                let (name, data) = match xauth::get_auth(family, &address, display_num).map_err(Error::io)? {
                    Some(tuple) => tuple,
                    None => {
                        tracing::warn!("no Xauth entry found for display {}", display_num);

                        (vec![], vec![])
                    },
                };

                mem::drop(_enter);

                Self::connect_with_auth(conn, screen.into(), name, data)
            })
        }
    }
}

impl<Conn: Connection> BasicDisplay<Conn> {
    /// Create a new `BasicDisplay` from an existing connection and a
    /// `Setup`.
    pub fn with_connection(conn: Conn, setup: Setup, default_screen_index: usize) -> Result<Self> {
        crate::initialization(move || {
            let core = X11Core::from_setup(&setup)?;
            let default_screen_index = default_screen_index;

            Ok(Self {
                core,
                setup: Arc::new(setup),
                packet_reader: PacketReader::new(),
                conn,
                max_request_size: Some(Default::default()),
                default_screen_index,
                extension_map: Default::default(),
            })
        })
    }

    /// Connect to the X11 server, using the given connection and auth
    /// information.
    ///
    /// # Blocking
    ///
    /// This function is expected to block, and is not built around not
    /// blocking. Using a non-blocking connection with this object will
    /// result in a fatal error.
    pub fn connect_with_auth(
        mut conn: Conn,
        default_screen_index: usize,
        auth_name: Vec<u8>,
        auth_info: Vec<u8>,
    ) -> Result<Self> {
        let span = tracing::info_span!("connect_with_auth");
        let _enter = span.enter();

        crate::initialization(move || {
            // initialize our connection
            let (mut connect, setup_request) = Connect::with_authorization(auth_name, auth_info);

            // write the setup request
            tracing::info!("writing the setup request to server");
            let mut nwritten = 0;
            while nwritten < setup_request.len() {
                let n = conn.send_slice(&setup_request[nwritten..])?;
                nwritten += n;

                tracing::debug!(written = n, total = nwritten, "wrote bytes for setup",);
            }

            // read the setup reply
            tracing::info!("reading the setup from server");
            loop {
                let adv = conn.recv_slice(connect.buffer())?;
                if adv == 0 {
                    return Err(Error::make_invalid_state(InvalidState::NotEnoughSetup));
                }

                tracing::debug!(read = adv, "read bytes for setup");

                if connect.advance(adv) {
                    // we've finished
                    break;
                }
            }

            // get the inner setup
            let setup = connect.into_setup().map_err(Error::make_connect_error)?;

            // create the display
            let display = Self::with_connection(conn, setup, default_screen_index)?;
            Ok(display)
        })
    }

    fn non_blocking_wait(&mut self) -> Result<AsyncStatus<()>> {
        // otherwise, try to read from the stream
        // read until we encounter a would-block error
        let mut fds = vec![];
        let amt = match self
            .conn
            .non_blocking_recv_slice_and_fds(self.packet_reader.buffer(), &mut fds)
        {
            Ok(amt) => amt,
            Err(e) if e.would_block() => return Ok(AsyncStatus::Read),
            Err(e) => return Err(e),
        };

        self.core.enqueue_fds(fds);
        if let Some(packet) = self.packet_reader.advance(amt) {
            self.core.enqueue_packet(packet);
        }

        Ok(AsyncStatus::Ready(()))
    }

    /// Block until a new packet is available.
    fn wait(&mut self) -> Result<()> {
        let span = tracing::info_span!("wait");
        let _enter = span.enter();

        let mut fds = vec![];
        let amt = self
            .conn
            .recv_slice_and_fds(self.packet_reader.buffer(), &mut fds)?;

        tracing::debug!(read = amt, num_fds = fds.len(), "read data from server");

        self.core.enqueue_fds(fds);
        if let Some(packet) = self.packet_reader.advance(amt) {
            tracing::debug!("completed packet");
            self.core.enqueue_packet(packet);
        }

        Ok(())
    }

    fn prefetch_maximum_length(&mut self) -> Result<usize> {
        tracing::info!("prefetching maximum length from server");

        // take it so lifetimes aren't an issue
        let mut prefetch = self.max_request_size.take().unwrap();

        // we're now evaluating bigreq
        let sz = prefetch.evaluate(self).copied();
        self.max_request_size = Some(prefetch);

        Ok(sz?.unwrap_or(self.setup.maximum_request_length as usize))
    }

    fn bigreq(&mut self) -> Result<(bool, usize)> {
        let span = tracing::info_span!("bigreq");
        let _enter = span.enter();

        loop {
            match self
                .max_request_size
                .as_ref()
                .map(|mrs| mrs.get_if_resolved().copied())
            {
                None => {
                    // bigreq is being processed, return the setup size
                    return Ok((false, self.setup.maximum_request_length as usize));
                }
                Some(None) => {
                    // prefetch
                    self.prefetch_maximum_length()?;
                }
                Some(Some(sz)) => {
                    // we have a size
                    return Ok((
                        sz.is_some(),
                        sz.unwrap_or(self.setup.maximum_request_length as usize),
                    ));
                }
            }
        }
    }

    fn prefetch_extension(
        &mut self,
        name: &'static str,
    ) -> Result<Prefetch<QueryExtensionRequest<'static>>> {
        tracing::info!("prefetching extension {} from server", name);

        let mut pf = Prefetch::new(QueryExtensionRequest {
            name: name.as_bytes().into(),
        });

        // evaluate it
        pf.evaluate(self)?;
        Ok(pf)
    }

    fn extension_info(&mut self, name: &'static str) -> Result<ExtensionInformation> {
        let span = tracing::info_span!("extension_info");
        let _enter = span.enter();

        match self.extension_map.get(name) {
            Some(info) => Ok(info),
            None => {
                // try to prefetch it
                let pf = self.prefetch_extension(name)?;
                let info = pf.get_assert().as_ref().cloned().unwrap();
                self.extension_map.insert(name, pf);
                Ok(info)
            }
        }
    }

    /// Format the request to be compatible with our send mechanism.
    ///
    /// # Blocking
    ///
    /// This will block.
    fn format_request(&mut self, req: &mut RawRequest<'_>) -> Result<u64> {
        let span = tracing::info_span!("format_request");
        let _enter = span.enter();

        // get the formatting bits
        let (is_bigreq, _) = self.bigreq()?;
        let extension_opcode = req
            .extension()
            .map(|ext| self.extension_info(ext))
            .transpose()?
            .map(|ext| ext.major_opcode);

        // get the sequence number
        let seq = loop {
            match self.core.send_request(req.variant()) {
                Some(seq) => break seq,
                None => {
                    // synchronize to ensure sequences are up to date
                    self.synchronize()?;
                }
            }
        };

        tracing::debug!(
            seq = seq,
            is_bigreq = is_bigreq,
            extension_opcode = extension_opcode,
            "formatting request for sending",
        );

        // format the request
        req.compute_length(is_bigreq);
        if let Some(opcode) = extension_opcode {
            req.set_extension_opcode(opcode);
        }

        Ok(seq)
    }
}

impl<Conn: Connection> DisplayBase for BasicDisplay<Conn> {
    fn setup(&self) -> &Setup {
        &self.setup
    }

    fn default_screen_index(&self) -> usize {
        self.default_screen_index
    }

    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>> {
        let span = tracing::info_span!("poll_for_reply_raw", seq = seq);
        let _enter = span.enter();

        loop {
            // check if we have a reply already
            if let Some(reply) = self.core.fetch_reply(seq, &self.extension_map)? {
                return Ok(Some(reply));
            }

            // otherwise, try to wait
            if !self.non_blocking_wait()?.is_ready() {
                return Ok(None);
            }
        }
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        let span = tracing::info_span!("poll_for_event");
        let _enter = span.enter();

        loop {
            // check if we have an event pending
            if let Some(event) = self.core.fetch_event(&self.extension_map)? {
                return Ok(Some(event));
            }

            // otherwise, try to read from the stream
            if !self.non_blocking_wait()?.is_ready() {
                return Ok(None);
            }
        }
    }
}

impl<Conn: Connection> Display for BasicDisplay<Conn> {
    fn send_request_raw(&mut self, mut req: RawRequest<'_>) -> Result<u64> {
        let span = tracing::info_span!("send_request_raw");
        let _enter = span.enter();

        let sequence = self.format_request(&mut req)?;

        // send the request
        let (buf, mut fds) = req.into_raw_parts();
        let mut nwritten = 0;

        tracing::info!(sequence = sequence, "sending request to server");

        while nwritten < buf.len() {
            let iov = [new_io_slice(&buf[nwritten..])];
            let amt = self.conn.send_slices_and_fds(&iov, &mut fds)?;
            nwritten += amt;

            tracing::debug!(written = amt, total = nwritten, "sending bytes to server",);
        }

        Ok(sequence)
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        let span = tracing::info_span!("wait_for_reply_raw", seq = seq);
        let _enter = span.enter();

        loop {
            if let Some(reply) = self.core.fetch_reply(seq, &self.extension_map)? {
                return Ok(reply);
            }

            // wait until the reply appears
            self.wait()?;
        }
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        let span = tracing::info_span!("wait_for_event");
        let _enter = span.enter();

        loop {
            if let Some(event) = self.core.fetch_event(&self.extension_map)? {
                return Ok(event);
            }

            // wait until the event appears
            self.wait()?;
        }
    }

    fn synchronize(&mut self) -> Result<()> {
        let span = tracing::info_span!("synchronize");
        let _enter = span.enter();

        // send the sync request
        let get_input_focus = GetInputFocusRequest {};
        let req = RawRequest::from_request_reply(get_input_focus);
        let seq = self.send_request_raw(req)?;

        // flush the stream
        self.flush()?;

        // wait for the reply
        self.wait_for_reply_raw(seq).map(|_| ())
    }

    fn generate_xid(&mut self) -> Result<u32> {
        // try to advance the XID ticket
        loop {
            match self.core.generate_xid() {
                Some(id) => return Ok(id),
                None => {
                    // we need to update the xid range
                    let reply = self.xc_misc_get_xid_range_immediate()?;
                    self.core.update_xid_range(reply).map_err(|IdsExhausted| {
                        Error::make_invalid_state(InvalidState::XidsExhausted)
                    })?;
                }
            }
        }
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        let span = tracing::info_span!("maximum_request_length");
        let _enter = span.enter();

        let (_, max_len) = self.bigreq()?;
        Ok(max_len)
    }

    fn flush(&mut self) -> Result<()> {
        let span = tracing::info_span!("flush");
        let _enter = span.enter();

        // flush connection buffer
        self.conn.flush()
    }
}
