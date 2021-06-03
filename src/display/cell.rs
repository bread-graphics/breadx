// MIT/Apache2 License

use super::{Display, DisplayBase, PendingReply, PendingRequest};
use core::{
    cell::{Cell, RefCell},
    num::{NonZeroU32, NonZeroUsize},
};

#[cfg(feature = "async")]
use super::{
    common::{SendBuffer, WaitBuffer, WaitBufferReturn},
    input, output,
};

/// An implementor of `Display` and `AsyncDisplay` that uses `Cell` and `RefCell` in order to allow multiple
/// accesses. The only downside is that it is not `Sync`.
#[derive(Debug)]
pub struct CellDisplay<Conn> {
    // the connection to the server
    connection: Option<Conn>,
    // the connection is in use if the io lock is true
    io_lock: Cell<bool>,

    // the setup received from the server
    setup: Setup,

    // xid generator
    xid: XidGenerator,

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
    wait_buffer: RefCell<Option<WaitBuffer>>,
    send_buffer: RefCell<SendBuffer>,
}

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

impl<Conn> CellDisplay<Conn> {
    #[inline]
    fn lock_internal(&mut self) {
        let io_lock = self.io_lock.get_mut();
        match *io_lock {
            true => panic!("Attempted to re-entrantly use connection"),
            false => {
                *io_lock = true;
            }
        }
    }

    #[inline]
    fn lock_internal_immutable(&self) {
        let io_lock = self.io_lock.get();
        match io_lock {
            true => panic!("Attempted to re-entrantly use connection"),
            false => {
                self.io_lock.set(true);
            }
        }
    }
}

impl<Conn> DisplayBase for CellDisplay<Conn> {
    #[inline]
    fn setup(&self) -> &Setup {
        &self.setup
    }
    #[inline]
    fn default_screen(&self) -> usize {
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
        self.xid.next()
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
        self.inner.borrow().pending_requests.get(&req_id).cloned()
    }
    #[inline]
    fn remove_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        #[cfg(feature = "async")]
        self.inner.get_mut().workarounders.retain(|&r| r != req_id);
        self.inner.get_mut().pending_requests.remove(&req_id)
    }
    #[inline]
    fn add_pending_error(&mut self, req_id: u16, error: BreadError) {
        self.inner.get_mut().pending_errors.insert(req_id, error)
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
            .insert(xid, VecDeque::new())
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
            .and_then(move |queue| queue.pop_front())
    }
    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.inner.get_mut().special_event_queues.remove(&xid)
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
        self.inner.get_mut().extensions.insert(key, opcode)
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
    type Conn = Connect;

    #[inline]
    fn connection(&mut self) -> &mut Self::Conn {
        self.connection.as_mut().expect("Display has been poisoned")
    }

    // we do have to worry about locking, even with &mut, since someone can later pull up an & access
    #[inline]
    fn lock(&mut self) {
        self.lock_internal()
    }

    #[inline]
    fn unlock(&mut self) {
        *self.io_lock.get_mut() = false;
    }
}

#[cfg(feature = "async")]
impl<Connect: AsyncConnection + Unpin> AsyncDisplay for CellDisplay<Connect> {
    type Conn = Connect;

    #[inline]
    fn connection(&mut self) -> &mut Self::Conn {
        self.connection.as_mut().expect("Display has been poisoned")
    }

    #[inline]
    fn poll_wait(&mut self, ctx: &mut Context<'_>) -> Poll<crate::Result> {
        let data = self.inner.get_mut();
        let (bytes, fds) = match self
            .wait_buffer
            .get_mut()
            .get_or_insert_with(|| {
                // try to lock
                self.lock_internal();
                WaitBuffer::default()
            })
            .poll_wait(self.connection.as_mut().unwrap(), &data.workarounders, ctx) {
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
        let res = send_buffer.poll_send_request(self, cx);
        *self.send_buffer.get_mut() = send_buffer;
        if res.is_ready() {
            self.send_buffer.get_mut().dig_hole();
            *self.io_lock.get_mut() = false;
        }
        match res {
            Poll::Ready(Ok(pr)) => Poll::Ready(output::finish_request(self, pr)),
            res => res,
        }
    }
}

impl<'a, Conn> DisplayBase for &'a CellDisplay<Conn> {
    #[inline]
    fn setup(&self) -> &Setup {
        &self.setup
    }
    #[inline]
    fn default_screen(&self) -> usize {
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
        self.xid.next()
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
        self.inner.borrow().pending_requests.get(&req_id).cloned()
    }
    #[inline]
    fn remove_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        let mut inner = self.inner.borrow_mut();
        #[cfg(feature = "async")]
        inner.workarounders.retain(|&r| r != req_id);
        inner.pending_requests.remove(&req_id)
    }
    #[inline]
    fn add_pending_error(&mut self, req_id: u16, error: BreadError) {
        self.inner.borrow_mut().pending_errors.insert(req_id, error)
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
            .insert(xid, VecDeque::new())
    }
    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        match self
            .inner
            .borrow_mut()
            .special_event_queues
            .borrow_mut(&xid)
        {
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
            .borrow_mut(&xid)
            .and_then(move |queue| queue.pop_front())
    }
    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.inner.borrow_mut().special_event_queues.remove(&xid)
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
        self.inner.borrow_mut().extensions.insert(key, opcode)
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

impl<'a, Connect: Connection> Display for &'a CellDisplay<Connect> {
    type Conn = &'a Connect;

    #[inline]
    fn connection(&mut self) -> &mut Self::Conn {
        match &self.connection {
            Some(ref mut c) => c,
            None => panic!("Display has been poisoned"),
        }
    }

    #[inline]
    fn lock(&mut self) {
        self.lock_internal_immutable();
    }

    #[inline]
    fn unlock(&mut self) {
        self.io_lock.set(false);
    }
}

#[cfg(feature = "async")]
impl<'a, Connect: AsyncConnection + Unpin> AsyncDisplay for &'a CellDisplay<Connect> {
    type Conn = &'a Connect;

    #[inline]
    fn connection(&mut self) -> &mut Self::Conn {
        match &self.connection {
            Some(ref mut c) => c,
            None => panic!("Display has been poisoned"),
        }
    }

    #[inline]
    fn poll_wait(&mut self, ctx: &mut Context<'_>) -> Poll<crate::Result> {
        let data = self.inner.borrow_mut();
        let wait_buffer = self.wait_buffer.borrow_mut();
        let (bytes, fds) = match wait_buffer
            .get_or_insert_with(|| {
                // try to lock
                self.lock_internal_immutable();
                WaitBuffer::default()
            })
            .poll_wait(self.connection.as_mut().unwrap(), &data.workarounders, ctx)
        {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => {
                mem::drop(wait_buffer);
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
        let res = send_buffer.poll_send_request(self, cx);
        *sblslot = send_buffer;
        if res.is_ready() {
            sbslot.dig_hole();
            self.io_lock.set(false);
        }
        match res {
            Poll::Ready(Ok(pr)) => Poll::Ready(output::finish_request(self, pr)),
            res => res,
        }
    }
}
