// MIT/Apache2 License

//! This module contains I/O that can be used to build a full
//! X11 connection.

use core::ops::DerefMut;

use super::{AsyncStatus, Strategy};
use crate::{connection::IoSlice, Fd, Result};
use alloc::vec::Vec;
use x11rb_protocol::packet_reader::PacketReader;

cfg_std! {
    use crate::mutex::{Mutex, MutexGuard};
}

type ReadTuple = (Option<Vec<u8>>, Vec<Fd>);

/// The read part of a connection.
pub(crate) trait ReadPart<Conn: ?Sized> {
    /// Try to read a packet and file descriptors.
    fn read<Dpy: ?Sized>(
        &mut self,
        strategy: &mut impl Strategy<Conn, Dpy>,
    ) -> Result<AsyncStatus<ReadTuple>>;
}

/// The write part of a connection.
pub(crate) trait WritePart {
    /// Flush this buffer.
    fn flush(&mut self) -> Result<AsyncStatus<()>>;

    /// Send slices and file descriptors.
    fn send_slices_and_fds(&mut self, data: &[IoSlice<'_>], fds: Vec<Fd>) -> Result<usize>;
}

/// Get a mutable reference to something.
pub(crate) trait Resolver<'a, T: ?Sized> {
    /// The guard we have.
    type Guard: DerefMut<Target = T> + 'a;

    /// Get the resolved reference.
    fn resolve(&'a mut self) -> Self::Guard;
}

impl<'a, T: ?Sized> Resolver<'a, T> for &'a mut T {
    type Guard = &'a mut T;

    fn resolve(&'a mut self) -> Self::Guard {
        *self
    }
}

#[cfg(feature = "std")]
impl<'a, T: ?Sized> Resolver<'a, T> for &'a Mutex<T> {
    type Guard = MutexGuard<'a, T>;

    fn resolve(&'a mut self) -> Self::Guard {
        self.lock()
    }
}

pub(crate) trait ConnResolver<'a, Conn: ?Sized> {
    type ReadHalf: ReadPart<Conn>;
    type WriteHalf: WritePart;
}