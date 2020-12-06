// MIT/Apache2 License

//! `breadx` is an implementation of the X Window System Protocol. In addition to providing the raw data
//! structures needed to create and parse requests, replies and events, it also provides convenient, functional
//! abstractions over the protocol.
//!
//! The design goals of `breadx` include speed, safety, and a reduction in complexity.
//!
//! * **Speed** - `breadx` takes advantage of modern languages features to reduce the time spent not sending
//!               protocol across the connection. First and foremost, `breadx` takes advantage of Rust's
//!               mutability system to ensure only one thread has access to the connection at once, negating
//!               the usual need for computationally expensive mutexes. In addition, `breadx` keeps as much
//!               data as possible on the stack, ensuring an increase in speed on modern computers.
//! * **Safety** - Rust's safety checking system is world-class in preventing bugs from propogating before
//!                they can even happen. `breadx` abides by these rules by including as little unsafe code as
//!                possible. The crate itself is `#![forbid(unsafe_code)]`, and its dependencies either use
//!                no unsafe code (`tinyvec`) or extensively check to ensure its unsafe code is correct
//!                (`bytemuck`).
//! * **Simple** - `breadx` tries to be as easy to understand as possible. Its API is well-documented, and
//!                it is accessible to both veteran X programmers and people who've never used it before.
//!
//! In addition, `breadx` implements Rust's new `async` system to allow it to be a part of a greater whole
//! when it comes to large, complex systems running on the async runtime. It also implements the standard
//! `log` logging facade for easy debugging.
//!
//! # Tutorials
//!
//! The official tutorial series begins [here](./tutorials/intro/index.html), and covers usage of `breadx`
//! and the X Window System Protocol.
//!
//! # Example
//!
//! This example opens a connection to the X server, then creates a basic window titled "Hello World!".
//!
//! ```rust,no_run
//! use breadx::{DisplayConnection, Event, WindowClass};
//! use std::{boxed::Box, error::Error};
//!
//! fn main() -> Result<(), Box<dyn Error>> {
//!     // open up the connection
//!     // note that the connection must be mutable
//!     let mut conn = DisplayConnection::create(None, None)?;
//!    
//!     // create a 640x400 window.
//!     let root = conn.default_screen().root;
//!     let window = conn
//!         .create_window(
//!             root, // parent
//!             WindowClass::CopyFromParent, // window class
//!             None, // depth (none means inherit from parent)
//!             None, // visual (none means "       "    "    )
//!             0, // x
//!             0, // y
//!             640, // width
//!             400, // height
//!             0, // border width
//!             Default::default() // additional properties
//!         )?;
//!
//!     // map the window and set its title
//!     window.map(&mut conn)?;
//!     window.set_title(&mut conn, "Hello World!")?;
//!
//!     // set up the exit protocol, this ensures the window exits when the "X"
//!     // button is clicked
//!     let wm_delete_window = conn
//!         .intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false)?;
//!     window.set_wm_protocols(&mut conn, &[wm_delete_window])?;
//!
//!     'evloop: loop {
//!         // grab an event from the connection
//!         let ev = conn.wait_for_event()?;
//!
//!         // if the event is telling us to close, then close
//!         if let Event::ClientMessage(cme) = ev {
//!             // check if the client message indicates the deletion protocol
//!             if cme.data.longs()[0] == wm_delete_window.xid {
//!                 break 'evloop;
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! See the `examples` directory in the repository root for more example `breadx` programs.
//!
//! # Features
//!
//! * `std` - Enabled by default. This enables the use of the standard library, and enables
//!           `DisplayConnection` and `DisplayConnection::create`. This library can be used without
//!           the standard library; however, it requires the programmer to provide a connection, rather
//!           than building a connection itself.
//! * `async` - Enables the `_async` suffix family of functions. These functions and methods are similar
//!             to their blocking variants, but they use non-blocking variants of network calls. This uses
//!             the [`async_net`](https://crates.io/crates/async-net) crate to provide non-blocking calls.
//!             However, it nearly triples the size of this package's dependency tree.
//! * `image-support` - Coming soon.
//! * `nightly-min-specialization` - Coming soon.
//! * `parallel` - Uses the [`rayon`](https://crates.io/crates/rayon) crate to parallelize computationally
//!                expensive operations.

#![deny(deprecated)]
#![forbid(unsafe_code)]
#![warn(missing_copy_implementations)]
#![warn(unused_qualifications)]
#![no_std]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation, // used on purpose quite abit
    clippy::cast_possible_wrap,
    clippy::default_trait_access, // more readable, IMO
    clippy::map_err_ignore, // sometimes we just need to drop the error
    clippy::missing_errors_doc, // lots of "async redox" functions
    clippy::module_name_repetitions, // doesn't matter IMO
    clippy::needless_pass_by_value,
    clippy::too_many_arguments, // we need this sometimes for compliance
    clippy::unknown_clippy_lints, // nightly has lints that stable doesn't
    clippy::used_underscore_binding
)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;
extern crate core;

mod auth_info;
pub mod auto;
pub mod client_message_data;
pub mod colormap;
mod cursor;
pub mod display;
pub mod drawable;
pub mod error;
pub mod event;
pub mod gcontext;
pub mod image;
pub(crate) mod paramatizer;
pub mod pixmap;
pub(crate) mod util;
pub mod window;
mod xid;

pub use auth_info::*;
pub use colormap::*;
pub use display::*;
pub use error::*;
pub use event::Event;
pub use gcontext::*;
pub use image::Image;
pub use pixmap::*;
pub use window::*;
pub use xid::*;

/// A request that can be sent as an instruction to the X server.
pub trait Request: AsByteSequence {
    type Reply: AsByteSequence;
    // Excerpt from the X Window System Protocol:
    //
    // Every request contains an 8-bit major opcode
    const OPCODE: u8;
}

//pub use display::*;
pub use xid::{XidType, XID};

pub use auto::xproto::{
    Arc, EventMask, ImageFormat, Rectangle, Segment, Visualid, Window, WindowClass,
};

#[path = "../tutorials/mod.rs"]
pub mod tutorials;
