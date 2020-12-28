// MIT/Apache2 License

//! [Last time](../basics/index.html), we implemented a basic window. Today, we'll add event handling to that
//! window, and use that to add some functionality.
//!
//! If requests and replies are the blood of X, events are the nerves. It's how you react to the outside world.
//! So far, our window doesn't really do much outside of exist. Let's change that.
//!
//! Note that we already handle an event: the `ClientMessageEvent` required to close the window.
//!
//! ```no_run
//! match ev {
//!     Event::ClientMessage(cme) => {
//!         if cme.data.longs()[0] == wm_delete_window.xid {
//!             break;
//!         }
//!     }
//!     _ => (),
//! }
//! ```
//!
//! It's slightly more complicated to handle other events.
//!
//! # Event Masks
//!
//! Before we dive into events, we need to talk about event masks. In order to avoid sending events that the
//! client is not interesting in, X uses a "mask" value to determine which events to send to the client. This
//! type is represented by the [`EventMask`](../../auto/xproto/struct.EventMask.html) structure.
//!
//! **TODO: rework this section once I finalize how bitflags are going to be represented**
//!
//! After constructing the event mask, the [`Window::set_event_mask`](../../auto/xproto/struct.Window.html#method.set_event_mask) function is used to set the event mask for a particular window. If you have more than one
//! window, each needs its own event mask.
//!
//! ```no_run
//! use breadx::{BreadError, Event, EventMask, DisplayConnection};
//!
//! fn main() -> Result<(), BreadError> {
//!     // simplified form of the code from last tutorial
//!     let mut conn = DisplayConnection::create(None, None)?;
//!     let window = conn.create_simple_window(
//!         conn.default_root(),
//!         0,
//!         0,
//!         640,
//!         400,
//!         0,
//!         conn.default_black_pixel(),
//!         conn.default_white_pixel(),
//!     )?;
//!     window.set_title(&mut conn, "Event Mask")?;
//!     window.map(&mut conn)?;
//!     let wm_delete_window = conn
//!         .intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false)?;
//!     window.set_wm_protocols(&mut conn, &[wm_delete_window])?;
//!
//!     // NEW: create an event mask that listens for ButtonPress events
//!     //      the default event mask is empty
//!     let mut event_mask: EventMask = EventMask::BUTTON_PRESS; 
//!
//!     // NEW: call set_event_mask on the window
//!     window.set_event_mask(&mut conn, event_mask)?;
//!
//!     loop {
//!         let ev = conn.wait_for_event()?;
//!
//!         match ev {
//!             Event::ClientMessage(cme) => {
//!                 if cme.data.longs()[0] == wm_delete_window.xid {
//!                     break;
//!                 }
//!             }
//!             // NEW: listen for Event::ButtonPress, then print the click location to the console
//!             Event::ButtonPress(bpe) => {
//!                 println!("Clicked at {}, {}", bpe.event_x, bpe.event_y);
//!             }
//!             _ => (),
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! If you run this program and click on the resulting window, it will display the following output,
//! depending on where you clicked:
//!
//! ```plaintext
//! Clicked at 381, 232
//! Clicked at 474, 132
//! Clicked at 281, 121
//! Clicked at 224, 285
//! Clicked at 366, 324
//! Clicked at 7, 7
//! ```
//!
//! Now, try removing the `set_button_press` call, and note that you no longer receive button press events.
//!
//! <div class="another-perspective">
//! <h2>Another Perspective on `EventMask`</h2>
//!
//! **TODO: implementation of bitflags explanation**
//! </div>
//!
//! # Events
//!
//! The [`Event`](../../event/enum.Event.html) enum can contain every event defined by the X Core Protocol, as
//! well as a `NoneOfTheAbove` bit collection that is reserved to events either sent by the user or from
//! unrecognized extensions.
//!
//! This section will break down the core events used by `breadx`, and how they ought to be handled.
//!
//! ## `ClientMessageEvent`
//!
//! [`ClientMessageEvent`](../../auto/xproto/struct.ClientMessageEvent.html), at first, meant "reserved for the
//! client to use as they see fit". It can also be used to convey window manager messages. We've already used
//! this one as a way of responding to window close events.
//!
//! The key point is the `data` field, which is of type [`ClientMessageData`](../../client_message_data/struct.ClientMessageData.html). It can represent 5 32-bit numbers, 10 16-bit numbers, or 20 8-bit numbers.
//!
//! Usually, you shouldn't listen for this one unless you explicitly know you need to listen for it.
//!
//! ## `ButtonPressEvent` and `ButtonReleaseEvent`
//!
//! As one would expect, [`ButtonPressEvent`](../../auto/xproto/struct.ButtonPressEvent.html) represents one of
//! the mouse buttons being pressed. The `detail` field tells you which button is being pressed, from 1 to 5.
//! `event_x` and `event_y` tell you the click's location relative to the origin of the window (top-left
//! corner), while `root_x` and `root_y` tell you the click's location relative to the origin of the screen.
//!
//! The inverse of this event, [`ButtonReleaseEvent`](../../auto/xproto/struct.ButtonReleaseEvent.html), gives
//! you these details when the mouse button is released.
//!
//! ## `ExposeEvent`
//!
//! [`ExposeEvent`](../../auto/xproto/struct.ExposeEvent.html) occurs when the window is being repainted. We'll
//! discuss this in more detail when we discuss drawing, but for now keep in mind that you should draw on the
//! window whenever you see this event.
//!
//! **TODO: populate with other events**
//!
//! [Next time, we'll talk about color management and the two primary approaches to creating color.](../color/index.html)
