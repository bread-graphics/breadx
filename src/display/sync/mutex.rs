// MIT/Apache2 License

use concurrent_queue::ConcurrentQueue;
use core::sync::atomic::{AtomicBool, Ordering};
use std::thread;

#[cfg(feature = "async")]
use core::task::{Context, Poll, Waker};

/// A simple implementation of a mutex.
#[derive(Debug)]
pub struct Mutex {
    locked: AtomicBool,
    wakers: ConcurrentQueue<ThreadOrWaker>,
}

impl Mutex {
    /// Create a new mutex.
    #[inline]
    pub fn new() -> Self { Self { locked: AtomicBool::new(false), wakers: ConcurrentQueue::unbounded() } }

    /// Try to lock the mutex. If it is locked, lock it a thread.
    #[inline]
    pub fn lock(&self) {
        while self
            .locked
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::AcqRel)
            .is_err()
        {
            let t = thread::current();
            self.wakers
                .push(ThreadOrWaker::Thread(t))
                .expect("Concurrent queue could not be pushed onto");
            thread::park();
        }
    }

    /// Try to lock the mutex. If it is locked, register the waker.
    #[cfg(feature = "async")]
    #[inline]
    pub fn poll_lock(&self, cx: &mut Context<'_>) -> Poll<()> {
        match self
            .locked
            .compare_exchange(false, true, Ordering::AcqRel, Ordering::AcqRel)
        {
            Ok(_) => Poll::Ready(()),
            Err(_) => {
                self.wakers.push(ThreadOrWaker::Waker(cx.waker().clone()));
                Poll::Pending
            }
        }
    }

    /// Unlock the mutex.
    #[inline]
    pub fn unlock(&self) {
        if self
            .locked
            .compare_exchange(true, false, Ordering::AcqRel, Ordering::AcqRel)
            .is_ok()
        {
            while self.locked.load(Ordering::Acquire) {
                match self.wakers.pop() {
                    Ok(waker) => waker.wake(),
                    Err(_) => break,
                }
            }
        }
    }
}

enum ThreadOrWaker {
    Thread(Thread),
    #[cfg(feature = "async")]
    Waker(Waker),
}

impl ThreadOrWaker {
    #[inline]
    pub(crate) fn wake(self) {
        match self {
            ThreadOrWaker::Thread(t) => t.wake(),
            #[cfg(feature = "async")]
            ThreadOrWaker::Waker(w) => w.wake(),
        }
    }
}
