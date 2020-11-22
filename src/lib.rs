// MIT/Apache2 License

#![forbid(unsafe_code)]
#![no_std]
#![warn(clippy::pedantic)]
#![allow(
    clippy::cast_possible_truncation,
    clippy::default_trait_access,
    clippy::map_err_ignore,
    clippy::missing_errors_doc,
    clippy::module_name_repetitions,
    clippy::needless_pass_by_value,
    clippy::too_many_arguments,
    clippy::used_underscore_binding
)]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;
extern crate core;

#[macro_use]
extern crate scopeguard;

mod auth_info;
pub mod auto;
pub mod client_message_data;
pub mod colormap;
pub mod display;
pub mod drawable;
pub mod error;
pub mod event;
pub mod gcontext;
pub mod image;
pub(crate) mod paramatizer;
pub(crate) mod util;
pub mod window;
pub mod xid;

pub use auth_info::*;
pub use colormap::*;
pub use display::*;
pub use error::*;
pub use event::Event;
pub use gcontext::*;
pub use image::Image;
pub use window::*;

/// A request.
pub trait Request: auto::AsByteSequence {
    type Reply: auto::AsByteSequence;
    // Excerpt from the X Window System Protocol:
    //
    // Every request contains an 8-bit major opcode
    const OPCODE: u8;
}

//pub use display::*;
pub use xid::{XidType, XID};

pub use auto::xproto::{Arc, EventMask, Rectangle, Segment, Visualid, Window, WindowClass};
