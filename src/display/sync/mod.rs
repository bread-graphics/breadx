// MIT/Apache2 License

mod mutex;
use mutex::Mutex;

/// A display that uses concurrent primitives in order to allow for thread-safe immutable access to the X
/// connection.
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
   
    // request number
    request_number: AtomicU64,

    // interned atoms
    wm_protocols_atom: AtomicU32,

    // do we care about zero sized replies?
    checked: AtomicBool,

    // we don't actually spin on these spinlocks, they're just used for mutual access that we can panic if
    // we get mutual access to it
    #[cfg(feature = "async")]
    wait_buffer: Spinlock<Option<WaitBuffer>>,
    #[cfg(feature = "async")]
    send_buffer: Spinlock<SendBuffer>, 
}
