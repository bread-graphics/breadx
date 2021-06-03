// MIT/Apache2 License

#![allow(unused_qualifications)]

mod xproto;

#[cfg(feature = "dri3")]
mod dri3;
#[cfg(feature = "glx")]
mod glx;
#[cfg(feature = "present")]
mod present;
#[cfg(feature = "sync")]
mod sync;
#[cfg(feature = "fixes")]
mod xfixes;

#[cfg(feature = "dri3")]
pub use dri3::*;
#[cfg(feature = "glx")]
pub use glx::*;
#[cfg(feature = "present")]
pub use present::*;
#[cfg(feature = "sync")]
pub use sync::*;
#[cfg(feature = "fixes")]
pub use xfixes::*;
pub use xproto::*;
