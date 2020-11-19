// MIT/Apache2 License

#![forbid(unsafe_code)]
#![no_std]

#[cfg(feature = "std")]
extern crate std;

extern crate alloc;
extern crate core;

#[macro_use]
extern crate scopeguard;

pub mod auth_info;
pub mod auto;
pub mod client_message_data;
pub mod display;
pub mod error;
pub mod event;
pub(crate) mod paramatizer;
pub(crate) mod util;
pub mod window;
pub mod xid;

pub use display::*;
pub use error::*;
pub use window::*;

pub(crate) use paramatizer::*;

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

pub use auto::xproto::{Visualid, Window, WindowClass};

/// An error.
pub trait Error: auto::AsByteSequence {
    const OPCODE: u8;
}

/// An event.
pub trait Event: auto::AsByteSequence {
    const OPCODE: u8;
}
