// MIT/Apache2 License

//! Displays, used to connect to the X11 server.

/// An interface to the X11 server.
pub trait DisplayBase {
    /// Get the `Setup` associated with this display.
    fn setup(&self) -> &Setup;

    /// Get the screen associated with this display.
    fn default_screen_index(&self) -> usize;

    /// Get the max request size for this display.
    fn max_request_size(&self) -> usize;

    /// Poll to see if a reply matching the sequence number has been received.
    fn poll_for_reply_raw(&mut self, seq: u64) -> Result<Option<RawReply>>;

    /// Poll to see if we have received an event.
    fn poll_for_event(&mut self) -> Result<Option<Event>>;
}

/// A blocking interface to the X11 server.
pub trait Display: DisplayBase {
    /// Send a raw request to the X11 server.
    fn send_request_raw(&mut self, req: RawRequest) -> Result<u64>;

    /// Wait for a reply from the X11 server.
    fn wait_for_reply_raw(&mut self, seq: u64) -> Result<RawReply>;

    /// Wait for an event.
    fn wait_for_event(&mut self) -> Result<Event>;
}

cfg_async! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub enum Interest {
        Readable,
        Writable,
    }

    /// A non-blocking interface to the X11 server.
    pub trait AsyncDisplay: DisplayBase {
        /// Poll to see if this display is able to participate in the given interest.
        fn poll_interest(&mut self, interest: Interest, cx: &mut Context<'_>) -> Poll<Result<()>>;

        /// Wait until this display is able to participate in the given interest.
        fn wait_for_interest(&mut self, interest: Interest) -> WaitForInterest<'_, Self>;

        /// Send a raw request to the X11 server.
        fn send_request_raw(&mut self, req: RawRequest) -> SendRequestRaw<'_, Self>;
    }
}