// MIT/Apache2 License

use core::{
    mem,
    task::{Context, Waker},
};

use super::{
    AsyncStatus, Display, DisplayBase, DisplayFunctionsExt, ExtensionMap, Poisonable, Prefetch,
    RawReply, RawRequest, X11Core,
};
use crate::{
    connection::{new_io_slice, new_io_slice_mut, BufConnection, Connection},
    Error, InvalidState, NameConnection, Result, ResultExt,
};

use alloc::{sync::Arc, vec, vec::Vec};
use x11rb_protocol::{
    connect::Connect,
    id_allocator::IdsExhausted,
    packet_reader::PacketReader,
    parse_display,
    protocol::{
        bigreq::EnableRequest,
        xc_misc::GetXIDRangeRequest,
        xproto::{GetInputFocusRequest, QueryExtensionRequest, Setup},
        Event,
    },
    x11_utils::ExtensionInformation,
    xauth,
};

use impl_details::{BlockingStrategy, Strategy, PollingStrategy};

cfg_async! {
    use super::CanBeAsyncDisplay;
    use impl_details::NonBlockingStrategy;
}

/// An implementation of `Display` that requires a mutable reference
/// to use.
pub struct BasicDisplay<Conn> {
    /// The protocol implementation core.
    core: X11Core,
    /// The setup returned from the server.
    pub(crate) setup: Arc<Setup>,
    /// The packet reader buffer.
    packet_reader: PacketReader,
    /// The connection to the server. This may be able to be poisoned.
    conn: Poisonable<Conn>,
    /// The maximum number of bytes we can send at once.
    max_request_size: Option<Prefetch<EnableRequest>>,
    /// The default screen index.
    pub(crate) default_screen_index: usize,
    /// Map between extension names and the extension information.
    extension_map: ExtensionMap,
    /// Tracking async state for in-flight requests/synchronization/etc.
    async_state: AsyncState,
}

#[derive(Default)]
struct AsyncState {
    /// The last sequence number request we are in the middle of sending.
    current_sending: Option<u64>,
    /// The in-flight XID regeneration request.
    xid_regeneration: Option<Prefetch<GetXIDRangeRequest>>,
    /// Current synchronization status.
    synchronization: Option<Prefetch<GetInputFocusRequest>>,
}

cfg_std! {
    pub type DisplayConnection = BasicDisplay<BufConnection<NameConnection>>;

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

                Self::connect_with_auth(conn.into(), screen.into(), name, data)
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
                conn: Poisonable::from(conn),
                max_request_size: Some(Default::default()),
                default_screen_index,
                extension_map: Default::default(),
                async_state: Default::default(),
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

            conn.flush()?;

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

    /// Wait for a packet from the server, using the given strategy.
    fn wait(&mut self, strategy: &mut impl Strategy<Conn>) -> Result<AsyncStatus<()>> {
        let span = tracing::debug_span!("wait", strategy = strategy.description());
        let _enter = span.enter();

        // try to read from the stream until we encouter a would-block error
        let mut fds = vec![];
        let packet_reader = &mut self.packet_reader;
        let amt = match self
            .conn
            .with(|conn| strategy.read_slices(conn, packet_reader.buffer(), &mut fds))
        {
            Ok(amt) => amt,
            Err(e) if e.would_block() => return Ok(AsyncStatus::Read),
            Err(e) => return Err(e),
        };

        tracing::debug!(amt = amt, num_fds = fds.len(), "read data from server");

        // enqueue the data we received into the core for processing
        self.core.enqueue_fds(fds);
        if let Some(packet) = self.packet_reader.advance(amt) {
            self.core.enqueue_packet(packet);
        }

        // we are now ready
        Ok(AsyncStatus::Ready(()))
    }

    /// Get a reply for the given sequence number from the server, using
    /// the given strategy.
    fn fetch_reply(
        &mut self,
        seq: u64,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<RawReply>> {
        let span = tracing::trace_span!("fetch_reply", seq=seq);
        let _enter = span.enter();

        // ensure that we have been flushed
        mtry!(self.partial_flush());

        loop {
            // see if the core has the reply available
            if let Some(reply) = self.core.fetch_reply(seq, &self.extension_map)? {
                return Ok(AsyncStatus::Ready(reply));
            }

            // otherwise, try to wait
            mtry!(self.wait(strategy));
        }
    }

    /// Get the latest event from the server, using the given strategy.
    fn fetch_event(&mut self, strategy: &mut impl Strategy<Conn>) -> Result<AsyncStatus<Event>> {
        mtry!(self.partial_flush());

        loop {
            // see if the core has the event available
            if let Some(event) = self.core.fetch_event(&self.extension_map)? {
                return Ok(AsyncStatus::Ready(event));
            }

            // otherwise, try to wait
            mtry!(self.wait(strategy));
        }
    }

    fn prefetch_maximum_length(
        &mut self,
        ctx: Option<&Waker>,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<(bool, usize)>> {
        tracing::info!("prefetching maximum length from server");

        // take it so lifetimes aren't an issue
        let mut prefetch = self.max_request_size.take().unwrap();

        // we're now evaluating bigreq
        let sz = strategy.prefetch(self, &mut prefetch, ctx).acopied();
        self.max_request_size = Some(prefetch);

        let sz = mtry!(sz);
        Ok(AsyncStatus::Ready((
            sz.is_some(),
            sz.unwrap_or(self.setup.maximum_request_length as usize),
        )))
    }

    fn bigreq(
        &mut self,
        ctx: Option<&Waker>,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<(bool, usize)>> {
        let span = tracing::debug_span!("bigreq");
        let _enter = span.enter();

        loop {
            match self
                .max_request_size
                .as_ref()
                .map(|mrs| mrs.get_if_resolved().copied())
            {
                None => {
                    // bigreq is being processed, return the setup size
                    return Ok(AsyncStatus::Ready((
                        false,
                        self.setup.maximum_request_length as usize,
                    )));
                }
                Some(None) => {
                    // prefetch
                    mtry!(self.prefetch_maximum_length(ctx, strategy));
                }
                Some(Some(sz)) => {
                    // we have a size
                    return Ok(AsyncStatus::Ready((
                        sz.is_some(),
                        sz.unwrap_or(self.setup.maximum_request_length as usize),
                    )));
                }
            }
        }
    }

    fn prefetch_extension(
        &mut self,
        name: &'static str,
        ctx: Option<&Waker>,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<Option<ExtensionInformation>>> {
        tracing::info!("prefetching extension {} from server", name);

        let mut pf = match self.extension_map.take_pf(name) {
            Some(pf) => pf,
            None => Prefetch::new(QueryExtensionRequest {
                name: name.as_bytes().into(),
            }),
        };

        // evaluate it
        let res = strategy.prefetch(self, &mut pf, ctx).acopied();

        // put prefetch back into map
        self.extension_map.insert(name, pf);

        res
    }

    fn extension_info(
        &mut self,
        name: &'static str,
        ctx: Option<&Waker>,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<ExtensionInformation>> {
        let span = tracing::info_span!("extension_info");
        let _enter = span.enter();

        loop {
            match self.extension_map.get(name) {
                Some(Some(info)) => return Ok(AsyncStatus::Ready(info)),
                Some(None) => return Err(Error::make_missing_extension(name)),
                None => {
                    // try to prefetch it
                    mtry!(self.prefetch_extension(name, ctx, strategy));
                }
            }
        }
    }

    fn partial_flush(&mut self) -> Result<AsyncStatus<()>> {
        tracing::trace!("flushing connection");

        match self.conn.with(|conn| conn.flush()) {
            Ok(()) => Ok(AsyncStatus::Ready(())),
            Err(e) if e.would_block() => Ok(AsyncStatus::Write),
            Err(e) => Err(e),
        }
    }

    /// Try to synchronize this client with the X11 server.
    fn partial_synchronize(
        &mut self,
        ctx: Option<&Waker>,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<()>> {
        let mut pf = self.async_state.synchronization.take().unwrap_or_default();
        let res = strategy.prefetch(self, &mut pf, ctx).acopied();

        if !matches!(res.as_ref().map(|a| a.is_ready()), Ok(true) | Err(_)) {
            self.async_state.synchronization = Some(pf);
        }

        res.map(|a| a.map(|_| ()))
    }

    /// Format the request to be compatible with our send mechanism.
    fn try_format_request(
        &mut self,
        req: &mut RawRequest,
        ctx: Option<&Waker>,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<u64>> {
        let span = tracing::info_span!("format_request", strategy = strategy.description());
        let _enter = span.enter();

        // get the formatting bits
        let (is_bigreq, _) = mtry!(self.bigreq(ctx, strategy));
        let extension = req.extension();

        let extension_opcode = match extension {
            Some(ext) => Some(mtry!(self.extension_info(ext, ctx, strategy)).major_opcode),
            None => None,
        };

        // get the sequence number
        let seq = loop {
            match self.core.send_request(req.variant()) {
                Some(seq) => break seq,
                None => {
                    // synchronize to ensure sequences are up to date
                    mtry!(self.partial_synchronize(ctx, strategy));
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

        Ok(AsyncStatus::Ready(seq))
    }

    /// Try to send the given request to the server.
    fn try_send_raw_request(
        &mut self,
        req: &mut RawRequest,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<()>> {
        loop {
            let (buf, fds) = req.mut_parts();

            if buf.is_empty() {
                break;
            }

            match self
                .conn
                .with(|conn| conn.send_slices_and_fds(&[new_io_slice(buf)], fds))
            {
                Ok(nwritten) => {
                    //tracing::trace!(nwritten = nwritten);
                    req.advance(nwritten)
                }
                Err(e) if e.would_block() => {
                    return Ok(AsyncStatus::Write);
                }
                Err(e) => return Err(e),
            }
        }

        Ok(AsyncStatus::Ready(()))
    }

    /// Try to generate an XID.
    fn partial_generate_xid(
        &mut self,
        ctx: Option<&Waker>,
        strategy: &mut impl Strategy<Conn>,
    ) -> Result<AsyncStatus<u32>> {
        loop {
            if let Some(id) = self.core.generate_xid() {
                // we just generated a new XID
                return Ok(AsyncStatus::Ready(id));
            }

            // we need to regenerate the XID range
            let mut pf = self.async_state.xid_regeneration.take().unwrap_or_default();
            let res = strategy.prefetch(self, &mut pf, ctx).acopied();

            if !matches!(res.as_ref().map(|a| a.is_ready()), Ok(true) | Err(_)) {
                self.async_state.xid_regeneration = Some(pf);
            }

            // we have the range to update
            let range = mtry!(res);
            self.core.update_xid_range(range).map_err(|IdsExhausted| {
                Error::make_invalid_state(InvalidState::XidsExhausted)
            })?;
        }
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
        self.fetch_reply(seq, &mut PollingStrategy)
            .map(|a| a.ready())
    }

    fn poll_for_event(&mut self) -> Result<Option<Event>> {
        self.fetch_event(&mut PollingStrategy).map(|a| a.ready())
    }
}

impl<Conn: Connection> Display for BasicDisplay<Conn> {
    fn send_request_raw(&mut self, mut req: RawRequest) -> Result<u64> {
        let span = tracing::info_span!("send_request_raw");
        let _enter = span.enter();

        cfg_if::cfg_if! {
            if #[cfg(feature = "async")] {
                // ensure that we're not stepping on anyone else's toes
                if let Some(seq) = self.async_state.current_sending {
                    return Err(Error::async_send_in_progress(seq))
                }
            }
        }

        let sequence = self
            .try_format_request(&mut req, None, &mut BlockingStrategy)?
            .unwrap();
        self.try_send_raw_request(&mut req, &mut BlockingStrategy)?
            .unwrap();
        Ok(sequence)
    }

    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply> {
        self.fetch_reply(seq, &mut BlockingStrategy)
            .map(|a| a.unwrap())
    }

    fn wait_for_event(&mut self) -> Result<Event> {
        self.fetch_event(&mut BlockingStrategy).map(|a| a.unwrap())
    }

    fn generate_xid(&mut self) -> Result<u32> {
        self.partial_generate_xid(None, &mut BlockingStrategy)
            .map(|a| a.unwrap())
    }

    fn maximum_request_length(&mut self) -> Result<usize> {
        let span = tracing::info_span!("maximum_request_length");
        let _enter = span.enter();

        let (_, max_len) = self.bigreq(None, &mut BlockingStrategy)?.unwrap();
        Ok(max_len)
    }

    fn flush(&mut self) -> Result<()> {
        // flush connection buffer
        self.conn.with(|conn| conn.flush())
    }
}

cfg_async! {
    impl<Conn: Connection> CanBeAsyncDisplay for BasicDisplay<Conn> {
        fn format_request(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<u64>> {
            self.try_format_request(req, Some(ctx.waker()), &mut NonBlockingStrategy)
        }

        fn try_send_request_raw(
            &mut self,
            req: &mut RawRequest,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<()>> {
            self.try_send_raw_request(req, &mut NonBlockingStrategy)
        }

        fn try_flush(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<()>> {
            self.partial_flush()
        }

        fn try_generate_xid(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<u32>> {
            self.partial_generate_xid(Some(ctx.waker()), &mut NonBlockingStrategy)
        }

        fn try_maximum_request_length(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<usize>> {
            let (_, max) = mtry!(self.bigreq(Some(ctx.waker()), &mut NonBlockingStrategy));
            Ok(AsyncStatus::Ready(max))
        }

        fn try_wait_for_event(&mut self, ctx: &mut Context<'_>) -> Result<AsyncStatus<Event>> {
            self.fetch_event(&mut NonBlockingStrategy)
        }

        fn try_wait_for_reply_raw(
            &mut self,
            seq: u64,
            ctx: &mut Context<'_>,
        ) -> Result<AsyncStatus<RawReply>> {
            self.fetch_reply(seq, &mut NonBlockingStrategy)
        }
    }
}

mod impl_details {
    use core::task::{Context, Waker};

    use alloc::vec::Vec;

    use crate::{
        connection::Connection,
        display::{prefetch::PrefetchTarget, AsyncStatus, Prefetch},
        Fd, Result,
    };

    use super::BasicDisplay;

    /// Whether we should use a non-blocking strategy or a blocking
    /// strategy for waiting for events.
    pub(crate) trait Strategy<Conn> {
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
            display: &mut BasicDisplay<Conn>,
            prefetch: &'p mut Prefetch<P>,
            ctx: Option<&Waker>,
        ) -> Result<AsyncStatus<&'p P::Target>>;

        /// Strategy description for tracing output.
        fn description(&self) -> &'static str;
    }

    /// Always assume that we are blocking.
    pub(crate) struct BlockingStrategy;

    impl<Conn: Connection> Strategy<Conn> for BlockingStrategy {
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
            display: &mut BasicDisplay<Conn>,
            prefetch: &'p mut Prefetch<P>,
            _ctx: Option<&Waker>,
        ) -> Result<AsyncStatus<&'p P::Target>> {
            prefetch.evaluate(display).map(|t| AsyncStatus::Ready(t))
        }

        fn description(&self) -> &'static str {
            "blocking"
        }
    }

    /// Assume that we are just polling for a reply or event.
    pub(crate) struct PollingStrategy;

    impl<Conn: Connection> Strategy<Conn> for PollingStrategy {
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
            display: &mut BasicDisplay<Conn>,
            prefetch: &'p mut Prefetch<P>,
            ctx: Option<&Waker>,
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

        impl<Conn: Connection> Strategy<Conn> for NonBlockingStrategy {
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
                display: &mut BasicDisplay<Conn>,
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
}
