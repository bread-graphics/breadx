// MIT/Apache2 License

#![no_std]

extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[macro_use]
mod gates;

mod error;
pub use error::*;

cfg_std! {
    mod name;
    pub use name::*;
}

pub type Fd = i32;

pub mod connection;
