// MIT/Apache2 License

mod mutex;
use mutex::Mutex;

use super::{EXT_KEY_SIZE, PendingRequest, input, output, PendingReply, DisplayBase, Display};
use crate::{auto::xproto::Setup, xid::{XID, AtomicXidGenerator}, event::Event};
use concurrent_queue::ConcurrentQueue;
use core::sync::atomic::{AtomicU64, AtomicU32, AtomicBool};
use dashmap::DashMap;
use spinning_top::Spinlock;

#[cfg(feature = "async")]
use super::{AsyncDisplay, common};

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
/// let conn = DisplayConnection::create(None, None).unwrap();
/// let conn: SyncDisplay<_> = conn.into();
/// ```
#[derive(Debug)]
pub struct SyncDisplay<Conn> {
    // the connection to the server
    connection: Option<Conn>,
    // connection lock
    io_lock: Mutex,

    // setup from the server
    setup: Setup,

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
    pending_items: DashMap<u16, PendingItem>,

    // map of special event queues
    special_event_queues: DashMap<XID, ConcurrentQueue<Event>>,

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

#[derive(Debug)]
enum PendingItem {
    Request(PendingRequest),
    Reply(PendingReply),
    Error(crate::BreadError),
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
            pending_requests,
            pending_errors,
            pending_repies,
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
            pending_items: pending_requests
                .into_iter()
                .map(|(k, v)| (k, PendingItem::Request(v)))
                .chain(
                    pending_replies
                        .into_iter()
                        .map(|(k, v)| (k, PendingItem::Reply(v))),
                )
                .chain(
                    pending_errors
                        .into_iter()
                        .map(|(k, v)| (k, PendingItem::Error(v))),
                )
                .collect(),
            special_event_queues: into_concurrent_queue(special_event_queues),
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

/// Convenience function to turn an iteratable struct (most often a `VecDeque`) into a `ConcurrentQueue`
#[inline]
fn into_concurrent_queue<I: IntoIterator>(i: I) -> ConcurrentQueue<I::Item> {
    let c = ConcurrentQueue::unbounded();
    for item in i {
        c.push(item);
    }
    c
}
