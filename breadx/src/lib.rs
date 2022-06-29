// MIT/Apache2 License

//! `breadx` is a comprehensive implementation of the [X11 client protocol]
//! with an aim to be featureful and powerful, but also easy to use.
//!
//! `breadx` aims to be a minimal implementation of the X11 protocol that
//! can be used in any case where a client is needed. `breadx` comes built
//! in with the following features:
//!
//! - **Comprehensive:** `breadx` has first class support for all X11 protocol
//!   extensions. These extensions can be enabled and disabled as features.
//! - **Lock-free**: The [default connection implementation] uses no locks and
//!   no waiting outside of standard I/O primitives. The goal is to ensure that
//!   there are as few layers as possible between the user's intended goal and
//!   actually sending data to the server.
//! - **Safe**: `breadx` has `#[forbid(unsafe_code)]`, which means that there never
//!   will be any unsafe code in `breadx`. This means that `breadx` will never be
//!   the cause of any undefined behavior.
//! - **Versatile:** For cases where sharing the connection is necessary, `breadx`
//!   provides [thread unsafe] and [thread safe] variants.
//! - **`no_std`:** By disabling the `std` feature, `breadx` can be used
//!   without depending on the standard library.
//! - **Asynchronous:** With the `async` feature enabled, `breadx`'s primitives
//!   can be used in asynchronous contexts. By default, `breadx` is runtime-agnostic,
//!   but support can be enabled for [`tokio`] and [`async-std`].
//! - **Simple:** With all of this, a client can be created and used in `breadx`
//!   in very few lines of code.
//!
//! Features that `breadx` does not provide:
//!
//! - **Data Manipulation** - APIs to make image manipulation/ICCCM/etc easier are
//!   located in other crates.
//! - **Interfacing with Xlib/XCB** - `breadx` does not provide a way to interact
//!   with Xlib/XCB directly.
//!
//! [X11 client protocol]: https://en.wikipedia.org/wiki/X_Window_System
//! [default connection implementation]: crate::display::BasicDisplay
//! [thread unsafe]: crate::display::CellDisplay
//! [thread safe]: crate::display::SyncDisplay
//! [`tokio`]: crate::rt_support::tokio_support
//! [`async-std`]: crate::rt_support::async_std_support
//!
//! # Usage
//!
//! All functions in `breadx` exist in the context of a [`Display`]. There are
//! many ways to create a [`Display`], but in most cases, [`DisplayConnection::connect()`]
//! will connect to the currently running X11 server without any fuss.
//!
//! From there, most functions that are actually used in `breadx` exist on the
//! [`DisplayFunctionsExt`] extension trait.
//!
//! [`Display`]: crate::display::Display
//! [`DisplayConnection::connect()`]: crate::display::DisplayConnection::connect
//! [`DisplayFunctionsExt`]: crate::display::DisplayFunctionsExt
//!
#![cfg_attr(feature = "std", doc = " ```rust,no_run")]
#![cfg_attr(feature = "std", doc = " use breadx::{prelude::*, display::DisplayConnection, protocol::xproto};")]
#![cfg_attr(feature = "std", doc = "")]
#![cfg_attr(feature = "std", doc = " # fn main() -> breadx::Result<()> {")]
#![cfg_attr(feature = "std", doc = " // establish a connection to the X11 server")]
#![cfg_attr(feature = "std", doc = " let mut connection = DisplayConnection::connect(None)?;")]
#![cfg_attr(feature = "std", doc = "")]
#![cfg_attr(feature = "std", doc = " // create a window")]
#![cfg_attr(feature = "std", doc = " // note the \"_checked\" suffix, this indicates that the result of the")]
#![cfg_attr(feature = "std", doc = " // function will be checked by the server after it is run")]
#![cfg_attr(feature = "std", doc = " // also note that we need to create an XID for the window ahead of time")]
#![cfg_attr(feature = "std", doc = " let wid = connection.generate_xid()?;")]
#![cfg_attr(feature = "std", doc = " connection.create_window_checked(")]
#![cfg_attr(feature = "std", doc = "     0, // depth")]
#![cfg_attr(feature = "std", doc = "     wid,")]
#![cfg_attr(feature = "std", doc = "     connection.default_screen().root, // parent")]
#![cfg_attr(feature = "std", doc = "     0, // x")]
#![cfg_attr(feature = "std", doc = "     0, // y")]
#![cfg_attr(feature = "std", doc = "     600, // width")]
#![cfg_attr(feature = "std", doc = "     400, // height")]
#![cfg_attr(feature = "std", doc = "     0, // border width")]
#![cfg_attr(feature = "std", doc = "     xproto::WindowClass::COPY_FROM_PARENT,")]
#![cfg_attr(feature = "std", doc = "     0, // visual")]
#![cfg_attr(feature = "std", doc = "     xproto::CreateWindowAux::new()")]
#![cfg_attr(feature = "std", doc = "         .background_pixel(connection.default_screen().white_pixel)")]
#![cfg_attr(feature = "std", doc = " )?;")]
#![cfg_attr(feature = "std", doc = "")]
#![cfg_attr(feature = "std", doc = " // map the window to the screen")]
#![cfg_attr(feature = "std", doc = " // note the lack of _checked here")]
#![cfg_attr(feature = "std", doc = " connection.map_window(wid)?;")]
#![cfg_attr(feature = "std", doc = "")]
#![cfg_attr(feature = "std", doc = " // primary event loop")]
#![cfg_attr(feature = "std", doc = " loop {")]
#![cfg_attr(feature = "std", doc = "     let event = connection.wait_for_event()?;")]
#![cfg_attr(feature = "std", doc = "")]
#![cfg_attr(feature = "std", doc = "     match event {")]
#![cfg_attr(feature = "std", doc = "         // match on the Event struct in here")]
#![cfg_attr(feature = "std", doc = "         # _ => {},")]
#![cfg_attr(feature = "std", doc = "     }")]
#![cfg_attr(feature = "std", doc = " }")]
#![cfg_attr(feature = "std", doc = " # Ok(()) }")]
#![cfg_attr(feature = "std", doc = " ```")]
//!
//! See the [tutorial](crate::tutorials::introduction) for more information
//! on the usage of `breadx`.

#![no_std]
#![forbid(future_incompatible, unsafe_code, rust_2018_idioms)]
#![warn(clippy::pedantic)]
#![allow(
    clippy::module_name_repetitions,
    clippy::missing_errors_doc,
    clippy::wildcard_imports
)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod gates;

mod error;
pub use error::*;

mod utils;
pub(crate) use utils::*;

cfg_async! {
    pub mod rt_support;
}

mod void;
pub use void::Void;

cfg_std! {
    mod name;
    pub use name::*;
}

cfg_sync! {
    mod mutex;
}

cfg_async! {
    pub mod futures;
}

// inline some of x11rb_protocol's docs into ours
#[doc(inline)]
pub use x11rb_protocol::{connect, x11_utils};

/// The protocol used during communication.
pub mod protocol {
    #[doc(inline)]
    pub use x11rb_protocol::{connection::ReplyFdKind, protocol::*, x11_utils::*};
}

cfg_std! {
    #[doc(inline)]
    pub use x11rb_protocol::resource_manager;
}

#[allow(
    clippy::cast_possible_truncation,
    clippy::let_and_return,
    clippy::needless_lifetimes,
    clippy::too_many_arguments
)]
pub(crate) mod automatically_generated;

#[doc(inline)]
pub use x11rb_protocol::RawFdContainer as Fd;

pub mod connection;
pub mod display;

mod mutlireply;
pub use mutlireply::*;

/// Contains a set of traits to be automatically imported for full
/// functionality.
pub mod prelude {
    pub use super::display::{
        Display, DisplayBase, DisplayBaseExt, DisplayExt, DisplayFunctionsExt,
    };

    cfg_async! {
        pub use super::display::{
            AsyncDisplay, AsyncDisplayExt, AsyncDisplayFunctionsExt
        };
    }
}

#[cfg(feature = "std")]
pub mod tutorials;
