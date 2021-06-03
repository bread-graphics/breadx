// MIT/Apache2 License

//! Common async implementation functionality between our connection types.

use super::{input, output, RequestInfo, RequestWorkaround};
use crate::{util::difference, Fd, auto::xproto::{QueryExtensionRequest, QueryExtensionReply}};
use core::{iter, mem};
use tinyvec::TinyVec;

/// A buffer used to hold variables related to the `poll_wait` function.
#[derive(Debug)]
pub(crate) struct WaitBuffer {
    /// The buffer used to hold info received from the server.
    buffer: TinyVec<[u8; 32]>,
    /// The buffer used to hold file descriptors received from the server.
    fds: Vec<Fd>,
    /// Whether or not we are on the initial 32 bytes per object sent from the server, or if we are
    /// looking for additional bytes.
    first_read: bool,
    /// Defines the portion of the buffer we need to pass to the connection.
    cursor: usize,
    /// Since this is essentially a pseudo-future, this allows us to panic if we're polled past
    /// completion.
    complete: bool,
}

impl Default for WaitBuffer {
    #[inline]
    fn default() -> Self {
        Self {
            buffer: iter::repeat(0).take(32).collect(),
            fds: vec![],
            first_read: true,
            cursor: 0,
            complete: false,
        }
    }
}

/// To avoid type complexity, this is the return type of `poll_wait`.
pub(crate) struct WaitBufferReturn {
    /// The data received from the wait.
    pub(crate) data: TinyVec<[u8; 32]>,
    /// The file descriptors received from the wait.
    pub(crate) fds: Vec<Fd>,
}

impl WaitBuffer {
    /// Mark this wait buffer as completed, preventing it from being polled after it is completed.
    #[inline]
    fn complete(&mut self) {
        self.complete = true;
    }

    /// Poll a connection with this `WaitBuffer`, possibly returning a result.
    #[inline]
    pub(crate) fn poll_wait<C: AsyncConnection + Unpin + ?Sized>(
        &mut self,
        conn: &mut C,
        workarounders: &[u16],
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result<WaitBufferReturn>> {
        // if this is already complete, panic
        if self.complete {
            panic!("Attempted to poll wait buffer past completion");
        }

        loop {
            // the portion of the buffer we're reading into
            let mut readme = &mut *self.buffer[self.cursor..];
            // read into the buffer as much as we can
            let res = conn.poll_read_packet(&mut readme, &mut self.fds, cx);
            // whether or not the end result is pending, "readme" will be adjusted so that it
            // represents the portion of the buffer that is not currently filled, so we adjust
            // "cursor" to accomodate
            let readme = &*readme;
            let diff = difference(readme, &*self.buffer);
            self.cursor = diff;

            match res {
                // if the result is pending, return that
                Poll::Pending => return Poll::Pending,
                // errors should also be propogated
                Poll::Ready(Err(e)) => {
                    self.dig_hole();
                    return Poll::Ready(Err(e));
                }
                Poll::Ready(Ok(())) => {}
            }

            // if the polling has completed, do one of two things:
            //  * if this is our first read, check if we need to grab additional bytes
            //    if we do, set "first_read" to false, expand the buffer, and refill it
            //  * otherwise, process and bytes and return them
            let buf = if self.first_read {
                self.first_read = false;

                // fix the GLX bug
                let mut buf = mem::take(&mut self.buffer);
                input::fix_glx_workaround(|seq| workarounders.contains(&seq), &mut buf);

                // check if we need additional bytes
                if let Some(ab) = input::additional_bytes(&buf[..8]) {
                    buf.extend(iter::repeat(0).take(ab));
                    self.buffer = buf;
                    continue; // redo the loop
                }
            } else {
                mem::take(&mut self.buffer)
            };

            // process the bytes/fds and return
            let fds = mem::take(&mut self.fds);
            self.dig_hole();
            return Poll::Ready(Ok(WaitBufferReturn { data: buf, fds }));
        }
    }
}

/// Either a `SendBuffer` or a `SendBuffer` in the process of creation.
#[derive(Debug)]
pub(crate) enum SendBuffer {
    Hole,
    OccupiedHole,
    Uninit(RequestInfo),
    Init(InnerSendBuffer),
    PollingForExt(RequestInfo, InnerSendBuffer),
    WaitingForExt(RequestInfo, u16, Option<WaitBuffer>),
}

impl Default for SendBuffer {
    #[inline]
    fn default() -> Self {
        Self::Hole
    }
}

impl SendBuffer {
    /// Populate this `SendBuffer` with a request.
    #[inline]
    pub(crate) fn fill_hole(&mut self, request_info: RequestInfo) {
        match self {
            SendBuffer::Init(..)
            | SendBuffer::Uninit(..)
            | SendBuffer::PollingForExt(..)
            | SendBuffer::WaitingForExt(..)
            | SendBuffer::OccupiedHole => {
                panic!("Attempted to call begin_send_request_raw before the other request is finished sending")
            }
            this => {
                *this = SendBuffer::Uninit(request_info);
            }
        }
    }

    /// Empty this `SendBuffer` and replace it with a hole.
    #[inline]
    pub(crate) fn dig_hole(&mut self) {
        *self = SendBuffer::Hole;
    }

    /// Poll for the creation of a new `SendBuffer`, given the `Display` one wants to create
    /// it with.
    #[inline]
    fn poll_init<D: AsyncDisplay + ?Sized>(
        &mut self,
        display: &mut D,
        cx: &mut Context<'_>,
    ) -> Poll<crate::Result> {
        let (req, opcode) = loop {
            match self {
                // cannot pole an empty hole
                SendBuffer::Hole => panic!(
                    "Attempted to call poll_send_request_raw before calling begin_send_request_raw"
                ),
                SendBuffer::OccupiedHole => {
                    panic!("Locking mechanism failed; attempted to poll an occupied hole")
                }
                // we are already initialized
                SendBuffer::Init(sb) => return Poll::Ready(Ok(())),
                // we are currently polling for requesting the extension opcode
                SendBuffer::PollingForExt(req, sb) => {
                    match sb.poll_send_request(display.connection(), cx) {
                        Poll::Ready(Ok(req_id)) => {
                            *self = SendBuffer::WaitingForExt(mem::take(req), req_id, None);
                        }
                        Poll::Ready(Err(e)) => {
                            self.dig_hole();
                            return Poll::Ready(Err(e));
                        }
                        Poll::Pending => return Poll::Pending,
                    }
                }
                // we are currently polling for receiving the extension opcode from the server
                SendBuffer::WaitingForExt(req, req_id, wait_buffer) => {
                    break loop {
                        if let Some(PendingReply { data, fds }) = display.take_pending_reply(req_id)
                        {
                            // decode the reply, which should be a QueryExtensionReply
                            let qer = match decode_reply::<QueryExtensionRequest>(&data, fds) {
                                Ok(qer) => qer,
                                Err(e) => {
                                    self.dig_hole();
                                    return Poll::Ready(Err(e));
                                }
                            };
                            // check to ensure our opcode is actually present
                            if !qer.present {
                                self.dig_hole();
                                return Poll::Ready(Err(crate::BreadError::ExtensionNotPresent(
                                    req.extension.unwrap().into(),
                                )));
                            }
                            // insert the opcode into the display
                            display.set_extension_opcode(
                                str_to_key(req.extension.unwrap()),
                                qer.major_opcode,
                            );
                            // TODO: first_event and first_error are probably important too
                            break (mem::take(req), Some(qer.opcode));
                        }

                        // run a wait cycle before checking again
                        let res = wait_buffer.get_or_insert_with(Default::default).poll_wait(
                            display.connection(),
                            &[], // we don't have any GLX workarounds here we need to check
                            cx,
                        );

                        match res {
                            Poll::Pending => return Poll::Pending,
                            Poll::Ready(Err(e)) => {
                                self.dig_hole();
                                return Poll::Ready(Err(e));
                            }
                            Poll::Ready(Ok(())) => {
                                *wait_buffer = None;
                            }
                        }
                    };
                }
                // we are not initialized at all
                SendBuffer::Uninit(req) => {
                    let req = mem::take(req);
                    match req.extension {
                        None => break (req, None),
                        Some(extension) => {
                            // see if we have it cached
                            let key = str_to_key(extension);
                            match display.get_extension_opcode(&key) {
                                Some(opcode) => break (req, Some(opcode)),
                                None => {
                                    // looks like we have to poll for it
                                    *self = SendBuffer::PollingForExt(
                                        req,
                                        InnerSendBuffer::new_internal(
                                            RequestInfo::from_request(QueryExtensionRequest {
                                                name: extension.to_string(),
                                                ..Default::default()
                                            }),
                                            None,
                                        ),
                                    );
                                }
                            }
                        }
                    }
                }
            }
        };

        *self = SendBuffer::Init(InnerSendBuffer::new_internal(req, opcode));

        Poll::Ready(Ok(()))
    }

    /// Poll for the sending of a raw request. This calls `poll_init` until the SendRequest buffer is initialized
    /// (read: the extension opcode is recognized) and then calls `poll_send_request` on the inner SendRequest.
    #[inline]
    pub(crate) fn poll_send_request<D: AsyncDisplay + ?Sized>(
        &mut self,
        display: &mut D,
        context: &mut Context<'_>,
    ) -> Poll<crate::Result<u16>> {
        loop {
            // if we're already initialized, start polling to actually send the request
            match self {
                SendBuffer::Init(isb) => {
                    return isb.poll_send_request(display.connection(), context);
                }
                this => {
                    // poll to initialize this send buffer
                    match self.poll_init(display, &mut lock_fn, context) {
                        Poll::Pending => return Poll::Pending,
                        Poll::Ready(Err(e)) => {
                            self.dig_hole();
                            return Poll::Ready(Err(e));
                        }
                        Poll::Ready(Ok(())) => continue,
                    }
                }
            }
        }
    }
}

/// A buffer for holding values necessary for `poll_send_request_raw`.
#[derive(Debug)]
struct InnerSendBuffer {
    /// The request we are trying to send.
    request: RequestInfo,
    /// Whether or not we've completed our task.
    complete: bool,
    /// Whether or not the data is modified to contain the opcode.
    impl_opcode: Opcode,
}

/// The status of our opcode.
#[derive(Debug, Copy, Clone)]
enum Opcode {
    /// The opcode has been implemented.
    Implemented,
    /// We currently possess the opcode and are waiting to substitute it into the request data.
    NotImplemented(Option<u8>),
}

impl InnerSendBuffer {
    /// Create a new `InnerSendBuffer` using the request info we want to send and its extension opcode,
    /// if applicable.
    #[inline]
    fn new_internal(request: RequestInfo, opcode: Option<u8>) -> Self {
        Self {
            request,
            complete: false,
            impl_opcode: Opcode::NotImplemented(opcode),
        }
    }

    /// Complete this buffer, allowing us to panic if it's polled after completion. See above for rationale.
    #[inline]
    fn complete(&mut self) {
        self.complete = true;
    }

    /// Poll to see if we can complete the raw request.
    #[inline]
    fn poll_send_request<C: AsyncConnection + Unpin + ?Sized>(
        &mut self,
        conn: &mut C,
        ctx: &mut Context<'_>,
    ) -> Poll<crate::Result<PendingRequest>> {
        // if we are already completed, panick
        if self.complete {
            panic!("Attempted to poll buffer past completion");
        }

        // if the opcode is not yet implemented, implement it
        if let Opcode::NotImplemented(opcode) = self.impl_opcode {
            let request_opcode = self.request.opcode;
            input::modify_for_opcode(&mut self.request.data, request_opcode, opcode);
            self.impl_opcode = Opcode::Implemented;
        }

        // begin polling for sending
        let mut sendme = &mut *sb.req.data;
        let res = conn.poll_send_packet(&mut sendme, &mut self.request.fds);

        // see above for explanation of what's happening here
        let sendme = &*sendme;
        let diff = difference(sendme, &*self.request.data);
        self.request.data = self.request.data.split_off(diff);

        // next action depends on the poll result
        match res {
            // bubble up pending and error, making sure to complete on error
            Poll::Pending => return Poll::Pending,
            Poll::Ready(Err(e)) => {
                self.dig_hole();
                return Poll::Ready(Err(e));
            }
            Poll::Ready(Ok(())) => {}
        }

        // take the request and return it, the display knows what to do with it
        self.dig_hole();
        Poll::Ready(Ok(mem::take(&mut self.request)))
    }
}

#[inline]
fn str_to_key(s: &str) -> [u8; EXT_KEY_SIZE] {
    let mut key = [0u8; EXT_KEY_SIZE];
    let b = s.as_bytes();
    key.copy_from_slice(&b[..EXT_KEY_SIZE]);
    key
}
