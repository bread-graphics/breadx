// MIT/Apache2 License

mod mutex;
use mutex::Mutex;

use super::{
    input, output, BasicDisplay, Connection, Display, DisplayBase, PendingItem, RequestInfo,
    StaticSetup, EXT_KEY_SIZE,
};
use crate::{
    event::Event,
    xid::{AtomicXidGenerator, XID},
};
use alloc::{collections::VecDeque, sync::Arc};
use concurrent_queue::ConcurrentQueue;
use core::{
    num::NonZeroU32,
    sync::atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
};
use dashmap::DashMap;

#[cfg(feature = "async")]
use super::{
    common::{SendBuffer, WaitBuffer, WaitBufferReturn},
    AsyncConnection, AsyncDisplay, PollOr, RequestWorkaround,
};
#[cfg(feature = "async")]
use core::{
    mem,
    task::{Context, Poll},
};
#[cfg(feature = "async")]
use futures_lite::ready;
#[cfg(feature = "async")]
use spinning_top::Spinlock;

/// A display that uses concurrent primitives in order to allow for thread-safe immutable access to the X
/// connection.
///
/// Using a combination of atomic primitives, concurrent queues, concurrent maps, immutable-access connections
/// and a mutex to serve as an IO lock, `SyncDisplay` allows for immutable usage combined with thread safety.
/// It is intended to be put into an `Arc`, synchronous `OnceCell`, or other multithread-accessible location
/// and used in that way.
///
/// Note that `SyncDisplay` is generally designed with the idea in mind that one thread will use the connection
/// at a time. Although non-IO usage (e.g. `DisplayBase` functions) is entirely concurrent, the display will lock
/// up while one thread uses it and unlock when it is done. If you expect this object to be under high levels of
/// contention (e.g. multiple threads are attempting to send requests or listen to it at once), it is preferred
/// to use a `Mutex<BasicDisplay>` or `RwLock<BasicDisplay>` instead. If thread safety is not required, using
/// `CellDisplay` is much faster than using atomics and locks.
///
/// ## Construction
///
/// `SyncDisplay` implements `From<BasicDisplay>`, and this is how it is intended to be constructed.
///
/// ```rust,no_run
/// use breadx::display::{DisplayConnection, SyncDisplay};
///
/// let conn = DisplayConnection::create(None).unwrap();
/// let conn: SyncDisplay<_> = conn.into();
/// ```
///
/// ## Preferred Usage
///
/// Usage of `SyncDisplay` from a mutable reference is not recommended as, due to the synchronous primitives
/// involved, mutable usage provides only marginal speedup as compared to immutable usage. If mutable usage is
/// necessary, consider using the `BasicDisplay` structure instead, as it is thread-safe and doesn't require
/// any locking.
#[derive(Debug)]
pub struct SyncDisplay<Conn> {
    // the connection to the server
    connection: Option<Conn>,
    // connection lock
    io_lock: Mutex,

    // setup from the server
    setup: StaticSetup,

    // xid generator
    xid: AtomicXidGenerator,

    // whether or not bigreq is enabled
    bigreq_enabled: bool,

    // maximum request length
    max_request_len: usize,

    // default screen index
    default_screen: usize,

    // main event queue
    event_queue: ConcurrentQueue<Event>,

    // map of pending requests, pending replies, and pending errors, combined into one map
    // in an `Arc` so we can clone it and pass it into the GLX workaround closure
    pending_items: Arc<DashMap<u16, PendingItem>>,

    // map of special event queues
    special_event_queues: DashMap<XID, VecDeque<Event>>,

    // map of extensions to extension opcodes
    // TODO: this is insert only, there's probably a more optimized version out there
    extensions: DashMap<[u8; EXT_KEY_SIZE], u8>,

    // request number
    request_number: AtomicU64,

    // interned atoms
    wm_protocols_atom: AtomicU32,

    // do we care about zero sized replies?
    checked: AtomicBool,

    // we don't actually spin on these spinlocks, they're just used for mutable access that we can panic if
    // we get mutual access to it
    #[cfg(feature = "async")]
    wait_buffer: Spinlock<Option<WaitBuffer>>,
    #[cfg(feature = "async")]
    send_buffer: Spinlock<SendBuffer>,
}

impl<Conn> From<BasicDisplay<Conn>> for SyncDisplay<Conn> {
    #[inline]
    fn from(bd: BasicDisplay<Conn>) -> Self {
        let BasicDisplay {
            connection,
            setup,
            bigreq_enabled,
            max_request_len,
            xid,
            default_screen,
            event_queue,
            pending_items,
            special_event_queues,
            request_number,
            wm_protocols_atom,
            checked,
            extensions,
            ..
        } = bd;

        SyncDisplay {
            connection,
            io_lock: Mutex::new(),
            setup,
            bigreq_enabled,
            max_request_len,
            xid: xid.into(),
            default_screen,
            event_queue: into_concurrent_queue(event_queue),
            pending_items: Arc::new(pending_items.into_iter().collect()),
            special_event_queues: special_event_queues.into_iter().collect(),
            extensions: extensions.into_iter().collect(),
            request_number: AtomicU64::new(request_number),
            wm_protocols_atom: AtomicU32::new(match wm_protocols_atom {
                None => 0,
                Some(wpa) => wpa.get(),
            }),
            checked: AtomicBool::new(checked),
            #[cfg(feature = "async")]
            wait_buffer: Spinlock::new(None),
            #[cfg(feature = "async")]
            send_buffer: Spinlock::new(Default::default()),
        }
    }
}

impl<Conn> DisplayBase for SyncDisplay<Conn> {
    #[inline]
    fn setup(&self) -> &StaticSetup {
        &self.setup
    }

    #[inline]
    fn default_screen_index(&self) -> usize {
        self.default_screen
    }

    #[inline]
    fn next_request_number(&mut self) -> u64 {
        let request_number = self.request_number.get_mut();
        let old = *request_number;
        *request_number += 1;
        old
    }

    #[inline]
    fn has_pending_event(&self) -> bool {
        !self.event_queue.is_empty()
    }

    #[inline]
    fn push_event(&mut self, event: Event) {
        self.event_queue.push(event).ok();
    }

    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        self.event_queue.pop().ok()
    }

    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        self.xid.next_xid()
    }

    #[inline]
    fn add_pending_item(&mut self, req_id: u16, item: PendingItem) {
        self.pending_items.insert(req_id, item);
    }

    #[inline]
    fn get_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.pending_items.get(&req_id).as_deref().cloned()
    }

    #[inline]
    fn take_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.pending_items.remove(&req_id).map(|(_, v)| v)
    }

    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        self.special_event_queues.insert(xid, VecDeque::new());
    }

    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        match self.special_event_queues.get_mut(&xid) {
            Some(mut queue) => {
                queue.push_back(event);
                Ok(())
            }
            None => Err(event),
        }
    }

    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        self.special_event_queues
            .get_mut(&xid)
            .and_then(|mut v| v.pop_front())
    }

    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.special_event_queues.remove(&xid);
    }

    #[inline]
    fn checked(&self) -> bool {
        self.checked.load(Ordering::Relaxed)
    }

    #[inline]
    fn set_checked(&mut self, checked: bool) {
        *self.checked.get_mut() = checked;
    }

    #[inline]
    fn bigreq_enabled(&self) -> bool {
        self.bigreq_enabled
    }

    #[inline]
    fn max_request_len(&self) -> usize {
        self.max_request_len
    }

    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        self.extensions.get(key).as_deref().copied()
    }

    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        self.extensions.insert(key, opcode);
    }

    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        NonZeroU32::new(self.wm_protocols_atom.load(Ordering::Relaxed))
    }

    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        *self.wm_protocols_atom.get_mut() = a.get();
    }
}

impl<Conn: Connection> Display for SyncDisplay<Conn> {
    #[inline]
    fn wait(&mut self) -> crate::Result {
        self.io_lock.lock();
        let mut connection = self.connection.take().expect("Poisoned!");

        let result = input::wait(self, &mut connection);

        self.connection = Some(connection);
        self.io_lock.unlock();
        result
    }

    #[inline]
    fn send_request_raw(&mut self, req: RequestInfo) -> crate::Result<u16> {
        self.io_lock.lock();
        let mut connection = self.connection.take().expect("Poisoned!");

        let result = output::send_request(self, &mut connection, req);

        self.connection = Some(connection);
        self.io_lock.unlock();
        result
    }
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection + Unpin> AsyncDisplay for SyncDisplay<Conn> {
    #[inline]
    fn poll_wait(&mut self, ctx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut conn = self.connection.take().expect("Poisoned!");
        let wait_buffer = match self.wait_buffer.get_mut() {
            Some(wait_buffer) => wait_buffer,
            None => {
                if let Poll::Pending = self.io_lock.poll_lock(ctx) {
                    self.connection = Some(conn);
                    return Poll::Pending;
                }

                let wait_buffer = Default::default();
                *self.wait_buffer.get_mut() = Some(wait_buffer);
                self.wait_buffer.get_mut().as_mut().unwrap()
            }
        };
        // low-cost, it's wrapped in an Arc
        let pending_items = self.pending_items.clone();
        let res = wait_buffer.poll_wait(
            &mut conn,
            move |seq| match pending_items.get(&seq).as_deref() {
                Some(PendingItem::Request(pereq)) => {
                    matches!(pereq.flags.workaround, RequestWorkaround::GlxFbconfigBug)
                }
                _ => false,
            },
            ctx,
        );

        self.connection = Some(conn);

        let (bytes, fds) = match res {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => {
                self.io_lock.unlock();
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
    fn begin_send_request_raw(
        &mut self,
        req: RequestInfo,
        cx: &mut Context<'_>,
    ) -> PollOr<(), RequestInfo> {
        match self.io_lock.poll_lock(cx) {
            Poll::Ready(()) => {
                self.send_buffer.get_mut().fill_hole(req);
                PollOr::Ready(())
            }
            Poll::Pending => PollOr::Pending(req),
        }
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
            self.io_lock.unlock();
        }

        match res {
            Poll::Ready(Ok(pr)) => Poll::Ready(Ok(output::finish_request(self, pr))),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

impl<'a, Conn> DisplayBase for &'a SyncDisplay<Conn> {
    #[inline]
    fn setup(&self) -> &StaticSetup {
        &self.setup
    }

    #[inline]
    fn default_screen_index(&self) -> usize {
        self.default_screen
    }

    #[inline]
    fn next_request_number(&mut self) -> u64 {
        self.request_number.fetch_add(1, Ordering::SeqCst)
    }

    #[inline]
    fn has_pending_event(&self) -> bool {
        !self.event_queue.is_empty()
    }

    #[inline]
    fn push_event(&mut self, event: Event) {
        self.event_queue.push(event).ok();
    }

    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        self.event_queue.pop().ok()
    }

    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        self.xid.next_xid()
    }

    #[inline]
    fn add_pending_item(&mut self, req_id: u16, item: PendingItem) {
        self.pending_items.insert(req_id, item);
    }

    #[inline]
    fn get_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.pending_items.get(&req_id).as_deref().cloned()
    }

    #[inline]
    fn take_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.pending_items.remove(&req_id).map(|(_, v)| v)
    }

    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        self.special_event_queues.insert(xid, VecDeque::new());
    }

    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        match self.special_event_queues.get_mut(&xid) {
            Some(mut queue) => {
                queue.push_back(event);
                Ok(())
            }
            None => Err(event),
        }
    }

    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        self.special_event_queues
            .get_mut(&xid)
            .and_then(|mut v| v.pop_front())
    }

    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.special_event_queues.remove(&xid);
    }

    #[inline]
    fn checked(&self) -> bool {
        self.checked.load(Ordering::SeqCst)
    }

    #[inline]
    fn set_checked(&mut self, checked: bool) {
        self.checked.store(checked, Ordering::SeqCst);
    }

    #[inline]
    fn bigreq_enabled(&self) -> bool {
        self.bigreq_enabled
    }

    #[inline]
    fn max_request_len(&self) -> usize {
        self.max_request_len
    }

    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        self.extensions.get(key).as_deref().copied()
    }

    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        self.extensions.insert(key, opcode);
    }

    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        NonZeroU32::new(self.wm_protocols_atom.load(Ordering::SeqCst))
    }

    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        self.wm_protocols_atom.store(a.get(), Ordering::SeqCst);
    }
}

impl<'a, Conn> Display for &'a SyncDisplay<Conn>
where
    &'a Conn: Connection,
{
    #[inline]
    fn wait(&mut self) -> crate::Result {
        self.io_lock.lock();
        let mut conn = self.connection.as_ref().expect("Poisoned");
        let result = input::wait(self, &mut conn);
        self.io_lock.unlock();
        result
    }

    #[inline]
    fn send_request_raw(&mut self, req: RequestInfo) -> crate::Result<u16> {
        self.io_lock.lock();
        let mut conn = self.connection.as_ref().expect("Poisoned");
        let result = output::send_request(self, &mut conn, req);
        self.io_lock.unlock();
        result
    }
}

#[cfg(feature = "async")]
impl<'a, Conn> AsyncDisplay for &'a SyncDisplay<Conn>
where
    &'a Conn: AsyncConnection,
{
    #[inline]
    fn poll_wait(&mut self, ctx: &mut Context<'_>) -> Poll<crate::Result> {
        let mut conn = self.connection.as_ref().expect("Poisoned");
        let mut wbslot = self
            .wait_buffer
            .try_lock()
            .expect("Locking mechanism failed; tried to get locked wait slot");

        let wait_buffer = match &mut *wbslot {
            Some(wait_buffer) => wait_buffer,
            None => {
                ready!(self.io_lock.poll_lock(ctx));
                let wait_buffer = Default::default();
                *wbslot = Some(wait_buffer);
                wbslot.as_mut().unwrap()
            }
        };

        // low-cost, it's wrapped in an Arc
        let pending_items = self.pending_items.clone();
        let res = wait_buffer.poll_wait(
            &mut conn,
            move |seq| match pending_items.get(&seq).as_deref() {
                Some(PendingItem::Request(pereq)) => {
                    matches!(pereq.flags.workaround, RequestWorkaround::GlxFbconfigBug)
                }
                _ => false,
            },
            ctx,
        );

        let (bytes, fds) = match res {
            Poll::Pending => return Poll::Pending,
            Poll::Ready(res) => {
                wbslot.take();
                self.io_lock.unlock();
                match res {
                    Ok(WaitBufferReturn { data, fds }) => (data, fds),
                    Err(e) => return Poll::Ready(Err(e)),
                }
            }
        };

        Poll::Ready(input::process_bytes(self, bytes, fds))
    }

    #[inline]
    fn begin_send_request_raw(
        &mut self,
        req: RequestInfo,
        cx: &mut Context<'_>,
    ) -> PollOr<(), RequestInfo> {
        match self.io_lock.poll_lock(cx) {
            Poll::Ready(()) => {
                self.send_buffer
                    .try_lock()
                    .expect("Failed locking mechanism")
                    .fill_hole(req);
                PollOr::Ready(())
            }
            Poll::Pending => PollOr::Pending(req),
        }
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        let mut sbslot = self
            .send_buffer
            .try_lock()
            .expect("Locking mechanism failed: send buffer is currently locked");
        let mut send_buffer = mem::replace(&mut *sbslot, SendBuffer::OccupiedHole);
        let mut conn = self.connection.as_ref().expect("Poisoned!");
        let res = send_buffer.poll_send_request(self, &mut conn, cx);
        *sbslot = send_buffer;

        if res.is_ready() {
            sbslot.dig_hole();
            self.io_lock.unlock();
        }

        match res {
            Poll::Ready(Ok(pr)) => Poll::Ready(Ok(output::finish_request(self, pr))),
            Poll::Ready(Err(e)) => Poll::Ready(Err(e)),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Convenience function to turn an iteratable struct (most often a `VecDeque`) into an unbounded
/// `ConcurrentQueue`.
#[inline]
fn into_concurrent_queue<I: IntoIterator>(i: I) -> ConcurrentQueue<I::Item> {
    let c = ConcurrentQueue::unbounded();
    for item in i {
        c.push(item).ok();
    }
    c
}
