// MIT/Apache2 License

use super::{
    input, output, BasicDisplay, Connection, Display, DisplayBase, PendingReply, PendingRequest,
    RequestInfo, EXT_KEY_SIZE,
};
use crate::{auto::xproto::Setup, BreadError, CellXidGenerator, Event, XID};
use alloc::collections::VecDeque;
use core::{
    cell::{Cell, RefCell},
    num::NonZeroU32,
};
use hashbrown::HashMap;

#[cfg(feature = "async")]
use super::{
    common::{SendBuffer, WaitBuffer, WaitBufferReturn},
    AsyncConnection, AsyncDisplay, RequestWorkaround,
};
#[cfg(feature = "async")]
use alloc::{vec, vec::Vec};
#[cfg(feature = "async")]
use core::{
    mem,
    task::{Context, Poll},
};

/// An implementor of [`Display`] and [`AsyncDisplay`] that uses [`Cell`] and [`RefCell`] in order to allow
/// for immutable use of the `Display`. The primary downside is that it is not [`Sync`].
///
/// This is useful in cases where the [`Display`] needs to be kept in an [`Rc`], thread-local [`OnceCell`],
/// reentrant mutex or any other place where one would only have an immutable reference to a `Display`. However,
/// async users should be aware that the connection is protected by a re-entrancy lock that will panic if two
/// I/O operations (e.g. sending two requests at once, or sending a request and then waiting) are attempted
/// at once.
///
/// ## Construction
///
/// `CellDisplay` is constructed using the `Into` implementation for [`BasicDisplay`], e.g:
///
/// ```rust,no_run
/// use breadx::display::{CellDisplay, DisplayConnection};
///
/// let mut display = DisplayConnection::create(None, None).unwrap();
/// let display: CellDisplay<_> = display.into();
/// ```
///
/// ## Alternatives
///
/// If you *can* restructure your program so that interior mutability is not required, it is considered better
/// form to use `BasicDisplay`. However, if interior mutability is necessary, `CellDisplay` is preferred over
/// `RefCell<BasicDisplay>`.
#[derive(Debug)]
pub struct CellDisplay<Conn> {
    // the connection to the server
    connection: Option<Conn>,
    // the connection is in use if the io lock is true
    io_lock: Cell<bool>,

    // the setup received from the server
    setup: Setup,

    // xid generator
    xid: CellXidGenerator,

    // the screen to be used by default
    default_screen: usize,

    // data that needs to be guarded behind a RefCell
    inner: RefCell<Data>,

    request_number: Cell<u64>,

    // store the interned atoms
    wm_protocols_atom: Cell<Option<NonZeroU32>>,

    // tell whether or not we care about the output of zero-sized replies
    checked: Cell<bool>,

    // used for polling
    #[cfg(feature = "async")]
    wait_buffer: RefCell<Option<WaitBuffer>>,
    #[cfg(feature = "async")]
    send_buffer: RefCell<SendBuffer>,
    #[cfg(feature = "async")]
    discard_reply: Cell<Option<bool>>,
}

/// Collection types for `CellDisplay` that need to be put behind an interior mutability lock.
#[derive(Debug)]
struct Data {
    event_queue: VecDeque<Event>,
    pending_requests: HashMap<u16, PendingRequest>,
    pending_errors: HashMap<u16, BreadError>,
    pending_replies: HashMap<u16, PendingReply>,
    special_event_queues: HashMap<XID, VecDeque<Event>>,
    extensions: HashMap<[u8; EXT_KEY_SIZE], u8>,
    #[cfg(feature = "async")]
    workarounders: Vec<u16>,
}

impl<Conn> From<BasicDisplay<Conn>> for CellDisplay<Conn> {
    /// Convert a `BasicDisplay` into a `CellDisplay`.
    #[inline]
    fn from(display: BasicDisplay<Conn>) -> Self {
        let BasicDisplay {
            connection,
            setup,
            xid,
            default_screen,
            event_queue,
            pending_requests,
            pending_errors,
            pending_replies,
            special_event_queues,
            request_number,
            wm_protocols_atom,
            checked,
            extensions,
            ..
        } = display;

        Self {
            connection,
            io_lock: Cell::new(false),
            setup,
            xid: xid.into(),
            default_screen,
            inner: RefCell::new(Data {
                event_queue,
                pending_requests,
                pending_errors,
                pending_replies,
                special_event_queues,
                extensions,
                #[cfg(feature = "async")]
                workarounders: vec![],
            }),
            request_number: Cell::new(request_number),
            wm_protocols_atom: Cell::new(wm_protocols_atom),
            checked: Cell::new(checked),
            #[cfg(feature = "async")]
            wait_buffer: RefCell::new(None),
            #[cfg(feature = "async")]
            send_buffer: Default::default(),
            #[cfg(feature = "async")]
            discard_reply: Cell::new(None),
        }
    }
}

impl<Conn> CellDisplay<Conn> {
    #[inline]
    fn lock_internal(&mut self) {
        let io_lock = self.io_lock.get_mut();
        if *io_lock {
            panic!("Attempted to re-entrantly use connection")
        } else {
            *io_lock = true;
        }
    }

    #[inline]
    fn lock_internal_immutable(&self) {
        let io_lock = self.io_lock.get();
        if io_lock {
            panic!("Attempted to re-entrantly use connection")
        } else {
            self.io_lock.set(true);
        }
    }
}

impl<Conn> DisplayBase for CellDisplay<Conn> {
    #[inline]
    fn setup(&self) -> &Setup {
        &self.setup
    }
    #[inline]
    fn default_screen_index(&self) -> usize {
        self.default_screen
    }
    #[inline]
    fn next_request_number(&mut self) -> u64 {
        self.request_number
            .replace(self.request_number.get().wrapping_add(1))
    }
    #[inline]
    fn push_event(&mut self, event: Event) {
        self.inner.get_mut().event_queue.push_back(event);
    }
    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        self.inner.get_mut().event_queue.pop_front()
    }
    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        self.xid.next_xid()
    }
    #[inline]
    fn add_pending_request(&mut self, req_id: u16, pereq: PendingRequest) {
        #[cfg(feature = "async")]
        if matches!(pereq.flags.workaround, RequestWorkaround::GlxFbconfigBug) {
            self.inner.get_mut().workarounders.push(req_id);
        }
        self.inner.get_mut().pending_requests.insert(req_id, pereq);
    }
    #[inline]
    fn get_pending_request(&self, req_id: u16) -> Option<PendingRequest> {
        self.inner.borrow().pending_requests.get(&req_id).copied()
    }
    #[inline]
    fn take_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        #[cfg(feature = "async")]
        self.inner.get_mut().workarounders.retain(|&r| r != req_id);
        self.inner.get_mut().pending_requests.remove(&req_id)
    }
    #[inline]
    fn add_pending_error(&mut self, req_id: u16, error: BreadError) {
        self.inner.get_mut().pending_errors.insert(req_id, error);
    }
    #[inline]
    fn check_for_pending_error(&mut self, req_id: u16) -> crate::Result<()> {
        match self.inner.get_mut().pending_errors.remove(&req_id) {
            Some(pe) => Err(pe),
            None => Ok(()),
        }
    }
    #[inline]
    fn add_pending_reply(&mut self, req_id: u16, reply: PendingReply) {
        self.inner.get_mut().pending_replies.insert(req_id, reply);
    }
    #[inline]
    fn take_pending_reply(&mut self, req_id: u16) -> Option<PendingReply> {
        self.inner.get_mut().pending_replies.remove(&req_id)
    }
    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        self.inner
            .get_mut()
            .special_event_queues
            .insert(xid, VecDeque::new());
    }
    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        match self.inner.get_mut().special_event_queues.get_mut(&xid) {
            Some(queue) => {
                queue.push_back(event);
                Ok(())
            }
            None => Err(event),
        }
    }
    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        self.inner
            .get_mut()
            .special_event_queues
            .get_mut(&xid)
            .and_then(VecDeque::pop_front)
    }
    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.inner.get_mut().special_event_queues.remove(&xid);
    }
    #[inline]
    fn checked(&self) -> bool {
        self.checked.get()
    }
    #[inline]
    fn set_checked(&mut self, checked: bool) {
        *self.checked.get_mut() = checked;
    }
    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        self.inner.get_mut().extensions.get(key).copied()
    }
    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        self.inner.get_mut().extensions.insert(key, opcode);
    }
    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        self.wm_protocols_atom.get()
    }
    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        *self.wm_protocols_atom.get_mut() = Some(a);
    }
}

impl<Connect: Connection> Display for CellDisplay<Connect> {
    #[inline]
    fn wait(&mut self) -> crate::Result {
        self.lock_internal();
        let mut connection = self.connection.take().expect("Poisoned!");

        let res = input::wait(self, &mut connection);

        self.connection = Some(connection);
        *self.io_lock.get_mut() = false;
        res
    }

    #[inline]
    fn send_request_raw(&mut self, req: RequestInfo) -> crate::Result<u16> {
        self.lock_internal();
        let mut connection = self.connection.take().expect("Poisoned!");

        let result = output::send_request(self, &mut connection, req);

        self.connection = Some(connection);
        *self.io_lock.get_mut() = false;
        result
    }
}

#[cfg(feature = "async")]
impl<Connect: AsyncConnection + Unpin> AsyncDisplay for CellDisplay<Connect> {
    #[inline]
    fn poll_wait(&mut self, ctx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut conn = self.connection.take().expect("Poisoned!");
        let wait_buffer = match self.wait_buffer.get_mut() {
            Some(wait_buffer) => wait_buffer,
            None => {
                self.lock_internal();
                let wait_buffer = Default::default();
                *self.wait_buffer.get_mut() = Some(wait_buffer);
                self.wait_buffer.get_mut().as_mut().unwrap()
            }
        };
        let data = self.inner.get_mut();
        let res = wait_buffer.poll_wait(&mut conn, &data.workarounders, ctx);

        self.connection = Some(conn);

        let (bytes, fds) = match res {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => {
                *self.io_lock.get_mut() = false;
                self.wait_buffer.get_mut().take();
                match res {
                    Ok(WaitBufferReturn { data, fds }) => (data, fds),
                    Err(e) => return Poll::Ready(Err(e)),
                }
            }
        };

        Poll::Ready(input::process_bytes(self, bytes, fds))
    }

    #[inline]
    fn begin_send_request_raw(&mut self, req: RequestInfo) {
        self.lock_internal();
        self.send_buffer.get_mut().fill_hole(req);
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        let mut send_buffer = mem::replace(self.send_buffer.get_mut(), SendBuffer::OccupiedHole);
        let mut conn = self.connection.take().expect("Poisoned!");
        let res = send_buffer.poll_send_request(self, &mut conn, cx);
        *self.send_buffer.get_mut() = send_buffer;
        self.connection = Some(conn);

        if res.is_ready() {
            self.send_buffer.get_mut().dig_hole();
            *self.io_lock.get_mut() = false;
        }
        match res {
            Poll::Ready(Ok(pr)) => Poll::Ready(Ok(output::finish_request(self, pr))),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a, Conn> DisplayBase for &'a CellDisplay<Conn> {
    #[inline]
    fn setup(&self) -> &Setup {
        &self.setup
    }
    #[inline]
    fn default_screen_index(&self) -> usize {
        self.default_screen
    }
    #[inline]
    fn next_request_number(&mut self) -> u64 {
        self.request_number
            .replace(self.request_number.get().wrapping_add(1))
    }
    #[inline]
    fn push_event(&mut self, event: Event) {
        self.inner.borrow_mut().event_queue.push_back(event);
    }
    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        self.inner.borrow_mut().event_queue.pop_front()
    }
    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        self.xid.next_xid()
    }
    #[inline]
    fn add_pending_request(&mut self, req_id: u16, pereq: PendingRequest) {
        let mut inner = self.inner.borrow_mut();
        #[cfg(feature = "async")]
        if matches!(pereq.flags.workaround, RequestWorkaround::GlxFbconfigBug) {
            inner.workarounders.push(req_id);
        }
        inner.pending_requests.insert(req_id, pereq);
    }
    #[inline]
    fn get_pending_request(&self, req_id: u16) -> Option<PendingRequest> {
        self.inner.borrow().pending_requests.get(&req_id).copied()
    }
    #[inline]
    fn take_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        let mut inner = self.inner.borrow_mut();
        #[cfg(feature = "async")]
        inner.workarounders.retain(|&r| r != req_id);
        inner.pending_requests.remove(&req_id)
    }
    #[inline]
    fn add_pending_error(&mut self, req_id: u16, error: BreadError) {
        self.inner.borrow_mut().pending_errors.insert(req_id, error);
    }
    #[inline]
    fn check_for_pending_error(&mut self, req_id: u16) -> crate::Result<()> {
        match self.inner.borrow_mut().pending_errors.remove(&req_id) {
            Some(pe) => Err(pe),
            None => Ok(()),
        }
    }
    #[inline]
    fn add_pending_reply(&mut self, req_id: u16, reply: PendingReply) {
        self.inner
            .borrow_mut()
            .pending_replies
            .insert(req_id, reply);
    }
    #[inline]
    fn take_pending_reply(&mut self, req_id: u16) -> Option<PendingReply> {
        self.inner.borrow_mut().pending_replies.remove(&req_id)
    }
    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        self.inner
            .borrow_mut()
            .special_event_queues
            .insert(xid, VecDeque::new());
    }
    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        match self.inner.borrow_mut().special_event_queues.get_mut(&xid) {
            Some(queue) => {
                queue.push_back(event);
                Ok(())
            }
            None => Err(event),
        }
    }
    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        self.inner
            .borrow_mut()
            .special_event_queues
            .get_mut(&xid)
            .and_then(VecDeque::pop_front)
    }
    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.inner.borrow_mut().special_event_queues.remove(&xid);
    }
    #[inline]
    fn checked(&self) -> bool {
        self.checked.get()
    }
    #[inline]
    fn set_checked(&mut self, checked: bool) {
        self.checked.set(checked);
    }
    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        self.inner.borrow_mut().extensions.get(key).copied()
    }
    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        self.inner.borrow_mut().extensions.insert(key, opcode);
    }
    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        self.wm_protocols_atom.get()
    }
    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        self.wm_protocols_atom.set(Some(a));
    }
}

impl<'a, Connect> Display for &'a CellDisplay<Connect>
where
    &'a Connect: Connection,
{
    #[inline]
    fn wait(&mut self) -> crate::Result {
        self.lock_internal_immutable();

        let res = input::wait(self, &mut self.connection.as_ref().expect("Poisoned!"));

        self.io_lock.set(false);
        res
    }

    #[inline]
    fn send_request_raw(&mut self, req: RequestInfo) -> crate::Result<u16> {
        self.lock_internal_immutable();

        let result =
            output::send_request(self, &mut self.connection.as_ref().expect("Poisoned!"), req);

        self.io_lock.set(false);
        result
    }
}

#[cfg(feature = "async")]
impl<'a, Connect> AsyncDisplay for &'a CellDisplay<Connect>
where
    &'a Connect: AsyncConnection + Unpin,
{
    #[inline]
    fn poll_wait(&mut self, ctx: &mut Context<'_>) -> Poll<crate::Result> {
        let data = self.inner.borrow_mut();
        let mut wait_buffer = self.wait_buffer.borrow_mut();
        let (bytes, fds) = match wait_buffer
            .get_or_insert_with(|| {
                // try to lock
                self.lock_internal_immutable();
                WaitBuffer::default()
            })
            .poll_wait(
                &mut self.connection.as_ref().unwrap(),
                &data.workarounders,
                ctx,
            ) {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => {
                drop(wait_buffer);
                self.wait_buffer.borrow_mut().take();
                self.io_lock.set(false);
                match res {
                    Ok(WaitBufferReturn { data, fds }) => (data, fds),
                    Err(e) => return Poll::Ready(Err(e)),
                }
            }
        };

        Poll::Ready(input::process_bytes(self, bytes, fds))
    }

    #[inline]
    fn begin_send_request_raw(&mut self, req: RequestInfo) {
        self.lock_internal_immutable();
        self.send_buffer.borrow_mut().fill_hole(req);
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        let mut sbslot = self.send_buffer.borrow_mut();
        let mut send_buffer = mem::replace(&mut *sbslot, SendBuffer::OccupiedHole);
        let res = send_buffer.poll_send_request(
            self,
            &mut self.connection.as_ref().expect("Poisoned!"),
            cx,
        );
        *sbslot = send_buffer;
        if res.is_ready() {
            sbslot.dig_hole();
            self.io_lock.set(false);
        }
        match res {
            Poll::Ready(Ok(pr)) => Poll::Ready(Ok(output::finish_request(self, pr))),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}
