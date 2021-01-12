// MIT/Apache2 License

#[cfg(all(feature = "std", unix))]
pub(crate) mod unix;

#[cfg(feature = "async")]
mod async_connection;
mod sync;

#[cfg(feature = "async")]
pub use async_connection::*;
pub use sync::*;

#[cfg(not(unix))]
use alloc::vec::Vec;

#[cfg(not(unix))]
pub(crate) fn standard_fd_warning(fds: &mut Vec<crate::Fd>) {
    if !fds.is_empty() {
        log::warn!("Cannot pass file descriptors into request on non-Unix operating systems.");
    }
}
