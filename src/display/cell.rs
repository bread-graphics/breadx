// MIT/Apache2 License

use super::{Display, DisplayBase, PendingReply, PendingRequest};
use core::cell::{Cell, RefCell};

/// An implementor of `Display` and `AsyncDisplay` that uses `Cell` and `RefCell` in order to allow multiple
/// accesses. The only downside is that it is not `Sync`.
pub struct CellDisplay<Conn> {
    // the connection to the server
    connection: Option<Conn>,
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
}

#[derive(Debug)]
struct Data {
    event_queue: VecDeque<Event>,
    pending_requests: HashMap<u16, PendingRequest>,
    pending_errors: HashMap<u16, BreadError>,
    pending_replies: HashMap<u16, PendingReply>,
    special_event_queues: HashMap<XID, VecDeque<Event>>,
    extensions: HashMap<[u8; EXT_KEY_SIZE], u8>,
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
        self.inner.get_mut().pending_requests.insert(req_id, pereq);
    }
    #[inline]
    fn get_pending_request(&self, req_id: u16) -> Option<PendingRequest> {
        self.inner.borrow().pending_requests.get(&req_id).cloned()
    }
    #[inline]
    fn remove_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
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
        let io_lock = self.io_lock.get_mut();
        match *io_lock {
            true => panic!("Attempted to re-entrantly use connection"),
            false => {
                *io_lock = true;
            }
        }
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
    fn poll_lock(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        let io_lock = self.io_lock.get_mut();
        match *io_lock {
            true => panic!("Attempted to re-entrantly use connection"),
            false => {
                *io_lock = true;
            }
        }

        Poll::Ready(())
    }

    #[inline]
    fn unlock(&mut self) {
        *self.io_lock.get_mut() = false;
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
        self.inner
            .borrow_mut()
            .pending_requests
            .insert(req_id, pereq);
    }
    #[inline]
    fn get_pending_request(&self, req_id: u16) -> Option<PendingRequest> {
        self.inner.borrow().pending_requests.get(&req_id).cloned()
    }
    #[inline]
    fn remove_pending_request(&mut self, req_id: u16) -> Option<PendingRequest> {
        self.inner.borrow_mut().pending_requests.remove(&req_id)
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
        let io_lock = self.io_lock.get();
        match io_lock {
            true => panic!("Attempted to re-entrantly use connection"),
            false => {
                self.io_lock.set(true);
            }
        }
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
    fn poll_lock(&mut self, _ctx: &mut Context<'_>) -> Poll<()> {
        let io_lock = self.io_lock.get();
        match io_lock {
            true => panic!("Attempted to re-entrantly use connection"),
            false => {
                self.io_lock.set(true);
            }
        }

        Poll::Ready(Ok(()))
    }

    #[inline]
    fn unlock(&mut self) {
        self.io_lock.set(false);
    }
}
