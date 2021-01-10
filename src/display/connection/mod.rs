// MIT/Apache2 License

#[cfg(all(feature = "std", unix))]
pub(crate) mod unix;

#[cfg(feature = "async")]
mod async_connection;
mod sync;

#[cfg(feature = "async")]
pub use async_connection::*;
pub use sync::*;
