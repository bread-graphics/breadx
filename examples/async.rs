// MIT/Apache2 License

// This example requires the "async" feature to be activated.

// Note: I use smol here, as it is a). my personal favorite async runtime in the Rust ecosystem at the
//       moment, and b). breadx is implemented in terms of its objects and executors. You should be able to
//       use tokio, async-std, or an executor-less system here as well without much headache.
use breadx::{
    auto::xproto::ExposeEvent, rgb, DisplayConnection, Event, EventMask, GcParameters, Rectangle,
};
use easy_parallel::Parallel;
use futures_lite::{future, FutureExt, StreamExt};
use smol::{
    channel::{unbounded, Receiver, Sender},
    Executor, Timer,
};
use std::{mem, time::Duration};

// coroutine base: wait 3 seconds, generate three random numbers, and send them down the channel
#[inline]
async fn rng(sender: Sender<[u8; 3]>) {
    let mut timer = Timer::interval(Duration::from_secs(3));

    loop {
        // send a set of random numbers into the channel
        let random: [u8; 3] = [fastrand::u8(..), fastrand::u8(..), fastrand::u8(..)];
        let res = sender.send(random).await;

        // if the result is an error, the other end has been closed, so return false to signify
        // that the routine should be ended
        if res.is_err() {
            break;
        }

        // wait for the timer to elapse
        // note: this may cause the program to stall for a few seconds after close. if we had some way to
        //       say "wait until the channel closes", we could do:
        // timer.next().or(sender.wait_for_close()).await
        //       alas, this is presently impossible, as far as I know
        timer.next().await.unwrap();
    }
}

// coroutine base: set up an X connection and, given the receiver channel, change color based on its result
#[inline]
async fn x_process(receiver: Receiver<[u8; 3]>) -> breadx::Result<()> {
    /// Are we processing events, or are we processing what we get from the channel?
    enum MainLoopDirective {
        ProcessingEvent(breadx::Result<Event>),
        ProcessingChannel([u8; 3]),
    }

    // you'll see why these are mutable later...
    let mut width: u16 = 600;
    let mut height: u16 = 480;

    // create the connection through an async process
    // note that most of this is basically the same as examples/basic.rs, but with the "async" suffix
    let mut conn = DisplayConnection::create_async(None, None).await?;
    let win = conn
        .create_simple_window_async(
            conn.default_screen().root,
            0,
            0,
            width,
            height,
            0,
            conn.default_black_pixel(),
            conn.default_white_pixel(),
        )
        .await?;
    win.set_event_mask_async(&mut conn, EventMask::EXPOSURE | EventMask::STRUCTURE_NOTIFY)
        .await?;
    win.map_async(&mut conn).await?;
    win.set_title_async(&mut conn, "Async Example").await?;

    let gc = conn
        .create_gc_async(
            win,
            GcParameters {
                foreground: Some(conn.default_black_pixel()),
                graphics_exposures: Some(0),
                ..Default::default()
            },
        )
        .await?;

    let wdw = conn
        .intern_atom_immediate_async("WM_DELETE_WINDOW".to_owned(), false)
        .await?;
    win.set_wm_protocols_async(&mut conn, &[wdw]).await?;

    // now that breadx is set up, start the main loop
    loop {
        // either process an event or a signal from the channel, with preference towards the channel
        let directive = async {
            MainLoopDirective::ProcessingChannel(
                receiver
                    .recv()
                    .await
                    .expect("Sender shouldn't be dropped before receiver"),
            )
        }
        .or(async { MainLoopDirective::ProcessingEvent(conn.wait_for_event_async().await) })
        .await;

        match directive {
            MainLoopDirective::ProcessingChannel(new_color) => {
                // set the color to our new color
                gc.change_async(
                    &mut conn,
                    GcParameters {
                        foreground: Some(rgb(new_color[0], new_color[1], new_color[2])),
                        ..Default::default()
                    },
                )
                .await?;

                // send an exposure event to force the window to redraw itself
                conn.send_event_async(
                    win,
                    EventMask::EXPOSURE,
                    Event::Expose(ExposeEvent {
                        window: win,
                        x: 0,
                        y: 0,
                        width,
                        height,
                        count: 0,
                        ..Default::default()
                    }),
                )
                .await?;
            }
            MainLoopDirective::ProcessingEvent(ev) => match ev? {
                Event::ClientMessage(cme) => {
                    // is the window closed now?
                    if cme.data.longs()[0] == wdw.xid {
                        break;
                    }
                }
                Event::Expose(_) => {
                    gc.fill_rectangle_async(
                        &mut conn,
                        win,
                        Rectangle {
                            x: 0,
                            y: 0,
                            width,
                            height,
                        },
                    )
                    .await?;
                }
                Event::ConfigureNotify(cne) => {
                    // update width and height
                    width = cne.width;
                    height = cne.height;
                }
                _ => (),
            },
        }
    }

    receiver.close();

    Ok(())
}

// async entry point
async fn entry(ex: &Executor<'_>) -> breadx::Result<()> {
    let (sender, receiver) = unbounded::<[u8; 3]>();

    // spawn two seperate coroutines on the executor
    let co_rng = ex.spawn(rng(sender));
    let co_x11 = ex.spawn(x_process(receiver));

    // run both futures simulaneously
    let ((), x11_res) = future::zip(co_rng, co_x11).await;
    x11_res
}

fn main() -> breadx::Result<()> {
    // spawn the executor
    let (signal, shutdown) = unbounded::<()>();
    let ex = Executor::new();

    Parallel::new()
        .each(0..2, |_| async_io::block_on(ex.run(shutdown.recv())))
        .finish(|| {
            let res = async_io::block_on(entry(&ex));
            mem::drop(signal);
            res
        })
        .1
}
