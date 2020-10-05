// MIT/Apache2 License

use alloc::boxed::Box;
use std::io::prelude::*;

#[cfg(feature = "async")]
use tokio::io::AsyncWrite;

#[cfg(not(feature = "pl"))]
use std::sync::Mutex;
#[cfg(feature = "pl")]
use parking_lot::Mutex;

/// Writer that writes to an X11 server.
enum Connection {
    Blocking(Box<dyn Write>),
    #[cfg(feature = "async")]
    Async(Box<dyn AsyncWrite>),
}

/// Connection to the X11 server.
pub struct Display {
    connection: Connection,
}

impl Display {
    #[inline]
    fn write_str_blocking(&mut self, s: &[u8]) -> crate::Result {
        let lock = self.lock.lock();
        match self.connection {
            Connection::Blocking(ref w) => w.write(
        }
        Ok(())
    }
}
