// MIT/Apache2 License

//! This module contains runtime support for the most common runtimes:
//! [`tokio`] and [`async-std`].
//!
//! [`tokio`]: crate::rt_support::tokio_support
//! [`async-std`]: crate::rt_support::async_std_support

#[cfg(feature = "async-std-support")]
pub mod async_std_support;
#[cfg(all(feature = "tokio-support", unix))]
pub mod tokio_support;
