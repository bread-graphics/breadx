// MIT/Apache2 License

#![no_std]
#![forbid(future_incompatible, unsafe_code, rust_2018_idioms)]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod gates;

mod error;
pub use error::*;

mod utils;
pub(crate) use utils::*;

cfg_std! {
    mod name;
    pub use name::*;
}

cfg_sync! {
    mod mutex;
    pub(crate) use mutex::*;
}

pub type Fd = x11rb_protocol::RawFdContainer;

pub mod connection;
pub mod display;
