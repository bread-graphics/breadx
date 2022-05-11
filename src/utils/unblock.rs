// MIT/Apache2 License

//! A structure for generating an iterator and then offloading it
//! onto a thread pool.
//!
//! This is similar to the `Unblock` structure from the `blocking` crate,
//! but it only works for iterators. I'm doing this for two reasons:
//!
//! 1). Doing DNS resolution is a cold path on an already cold path, so
//!     specializing and using another runtime's thread pool really
//!     isn't worth it.
//!     
//! In 90 percent of cases, users connect over a Unix socket. In the
//! remaining cases, 99 percent of those are over loopback or with a known
//! IP address. It is very rare for DNS resolution to come into play.
//!
//! 2). Setting up infrastructure for the above is a lot of work.
//! 3). It takes a generation function that returns a Result<T>, specifically
//!     for ToSocketAddrs.

#![cfg(feature = "async")]

use core::{
    mem,
    task::{Context, Poll, Waker},
};
use futures_util::Stream;
use std::{
    sync::mpsc::{self, TryRecvError},
    thread,
};

/// The entire point of this module.
///
/// Takes a function that spawns an iterator, and runs it on another
/// thread.
pub(crate) enum Unblock<Func, Item, Err> {
    /// The iterator has not been spawned yet.
    Unspawned {
        /// The function to spawn the iterator with.
        spawn: Func,
    },
    /// The iterator is spawned and running on another thread.
    Spawned {
        /// Channel to receive iterator items over.
        items: mpsc::Receiver<Result<Item, Err>>,
        /// Channel to send wakers over.
        wakers: mpsc::Sender<Waker>,
    },
    /// Temporary "hole" state for when we're in the process of spawning.
    Hole,
}

impl<
        Item: Send + 'static,
        Iter: Iterator<Item = Item>,
        Err: Send + 'static,
        Func: FnOnce() -> Result<Iter, Err> + Send + 'static,
    > Unblock<Func, Item, Err>
{
    /// Creates a new `Unblock`.
    pub(crate) fn new(spawn: Func) -> Self {
        Self::Unspawned { spawn }
    }

    /// Spawns the function on the thread pool, if we haven't already.
    fn spawn(&mut self) {
        // get the inner function
        let spawn = match mem::replace(self, Self::Hole) {
            Self::Unspawned { spawn } => spawn,
            _ => unreachable!("Unblock::spawn called twice"),
        };

        // create the channels
        let (items_tx, items_rx) = mpsc::channel();
        let (wakers_tx, wakers_rx) = mpsc::channel::<Waker>();

        // spawn the thread
        thread::Builder::new()
            .name("breadx-unblock".into())
            .spawn(move || {
                // sub-function for waking all of our wakers
                let wake_all = move |wait_for_drop: bool| {
                    if wait_for_drop {
                        loop {
                            match wakers_rx.recv() {
                                Ok(waker) => waker.wake(),
                                Err(_) => break,
                            }
                        }
                    } else {
                        loop {
                            match wakers_rx.try_recv() {
                                Ok(waker) => waker.wake(),
                                Err(_) => break,
                            }
                        }
                    }
                };

                // run the function
                let iter = match spawn() {
                    Ok(iter) => iter,
                    Err(err) => {
                        // send the error to the items channel
                        items_tx.send(Err(err)).expect(CHANNEL_SEND_PANIC);
                        wake_all(true);
                        return;
                    }
                };

                // iterate over the items
                for item in iter {
                    // send it over the channel
                    items_tx.send(Ok(item)).expect(CHANNEL_SEND_PANIC);

                    // wake any waiting wakers
                    wake_all(false);
                }

                // we've sent all of our items, so drop the channel
                mem::drop(items_tx);

                // wake all of the wakers
                wake_all(true);
            })
            .expect("failed to spawn unblock thread");

        // set new state
        *self = Self::Spawned {
            items: items_rx,
            wakers: wakers_tx,
        };
    }

    /// Poll for an item from this stream.
    fn poll_for_item(&mut self, ctx: &mut Context<'_>) -> Poll<Result<Option<Item>, Err>> {
        loop {
            // match on the current state
            match mem::replace(self, Self::Hole) {
                Self::Hole => {
                    panic!("cannot poll an empty hole")
                }
                mut this @ Self::Unspawned { .. } => {
                    // spawn the iterator
                    this.spawn();
                    *self = this;
                }
                Self::Spawned { items, wakers } => {
                    // try to receive an item
                    match items.try_recv() {
                        Ok(item) => {
                            // if we got an item, return it
                            *self = Self::Spawned { items, wakers };
                            return Poll::Ready(item.map(Some));
                        }
                        Err(TryRecvError::Disconnected) => {
                            // we're out of items, return None
                            return Poll::Ready(Ok(None));
                        }
                        Err(TryRecvError::Empty) => {
                            // no items yet, so we need to wait
                            // for one to come in

                            // pusk waker to channel
                            wakers.send(ctx.waker().clone()).ok();
                            *self = Self::Spawned { items, wakers };
                            return Poll::Pending;
                        }
                    }
                }
            }
        }
    }
}

impl<
        Item: Send + 'static,
        Iter: Iterator<Item = Item> + Unpin,
        Err: Send + 'static,
        Func: FnOnce() -> Result<Iter, Err> + Send + Unpin + 'static,
    > Stream for Unblock<Func, Item, Err>
{
    type Item = Result<Item, Err>;

    fn poll_next(
        self: core::pin::Pin<&mut Self>,
        ctx: &mut Context<'_>,
    ) -> Poll<Option<Self::Item>> {
        match self.get_mut().poll_for_item(ctx) {
            Poll::Ready(item) => Poll::Ready(item.transpose()),
            Poll::Pending => Poll::Pending,
        }
    }
}

const CHANNEL_SEND_PANIC: &str = "failed to send channel item";

#[cfg(test)]
mod test {
    use super::*;
    use core::convert::Infallible;
    use futures_util::{stream::iter, StreamExt};
    use std::{thread::sleep, time::Duration};

    #[test]
    fn unblock_works() {
        spin_on::spin_on(async {
            let unblock = Unblock::new(|| {
                let iter = (0..10).map(|i| {
                    sleep(Duration::from_millis(1));
                    i
                });
                Result::<_, Infallible>::Ok(iter)
            });

            unblock
                .zip(iter(0..10))
                .for_each(|(i, j)| async move {
                    assert_eq!(i.unwrap(), j);
                })
                .await;
        });
    }
}
