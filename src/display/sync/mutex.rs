// MIT/Apache2 License

use concurrent_queue::ConcurrentQueue;
use core::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[cfg(feature = "async")]
use core::task::{Context, Poll, Waker};

/// A simple implementation of non-RAII mutex. It supports both blocking and non-blocking usage, in order to
/// allow its use by both blocking and non-blocking clients, as well as to facilitate using it without mutex
/// guards.
#[derive(Debug)]
pub struct Mutex {
    /// Whether or not the mutex is currently locked.
    locked: AtomicBool,
    /// The list of wakers. Whenever the mutex is unlocked, this queue is drained and all of the wakers are
    /// woken until the mutex is locked again. It is less efficient than most contemporary mutex implementations,
    /// but it has the advantage of not needing an RAII guard. In addition, unparkers on most platforms are
    /// backed by either `Mutex`es or other operating-specific ways of parking the thread (IIRC futexes on Linux
    /// and events on Windows), so even in the worst case it should just act as a `Mutex` with a slight amount of
    /// overhead.
    wakers: ConcurrentQueue<ThreadOrWaker>,
}

impl Mutex {
    /// Create a new mutex.
    #[inline]
    pub fn new() -> Self {
        Self {
            locked: AtomicBool::new(false),
            wakers: ConcurrentQueue::unbounded(),
        }
    }

    /// Try to lock the mutex. If it is locked, lock it a thread.
    #[inline]
    pub fn lock(&self) {
        // until we've successfully locked the mutex...
        while self
            .locked
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::AcqRel)
            .is_err()
        {
            // park the current thread
            let t = thread::current();
            // push an unparker into the wakers queue
            self.wakers
                .push(ThreadOrWaker::Thread(t))
                .expect("Concurrent queue could not be pushed onto");
            // note: this loop does defeat spurious wakeups
            thread::park();
        }
    }

    /// Try to lock the mutex. If it is locked, register the waker.
    #[cfg(feature = "async")]
    #[inline]
    pub fn poll_lock(&self, cx: &mut Context<'_>) -> Poll<()> {
        // try to lock the mutex
        match self
            .locked
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::AcqRel)
        {
            // if we do, great!
            Ok(_) => Poll::Ready(()),
            Err(_) => {
                // if we don't, push the context's waker into the mutex list and wake it when the mutex is
                // open
                self.wakers.push(ThreadOrWaker::Waker(cx.waker().clone()));
                Poll::Pending
            }
        }
    }

    /// Unlock the mutex.
    #[inline]
    pub fn unlock(&self) {
        // unlock the mutex
        if self
            .locked
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::AcqRel)
            .is_ok()
        {
            // in the course of waking all of the wakers, they will probably try to lock the mutex. if the mutex
            // is locked after a wake operation, there's no real need to keep waking more wakers, as they'll
            // just be wastefully added back onto the queue
            while self.locked.load(Ordering::Acquire) {
                // if we can, wake a waker
                match self.wakers.pop() {
                    Ok(waker) => waker.wake(),
                    Err(_) => break,
                }
            }
        }
    }
}

/// Wakes a task. Either an unparker for a thread, or a waker for an async task. For non-async users, this is
/// a (hopefully) no-op wrapper around the unparker.
enum ThreadOrWaker {
    /// A thread handle, used to unpark a given thread.
    Thread(Thread),
    /// A waker, used to wake a given async task.
    #[cfg(feature = "async")]
    Waker(Waker),
}

impl ThreadOrWaker {
    /// Wake either the thread or the async context that this waker is designated to wake.
    #[inline]
    pub(crate) fn wake(self) {
        match self {
            ThreadOrWaker::Thread(t) => t.unpark(),
            #[cfg(feature = "async")]
            ThreadOrWaker::Waker(w) => w.wake(),
        }
    }
}
