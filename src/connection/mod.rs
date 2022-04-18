// MIT/Apache2 License

use crate::{Error, Fd, Result, Unsupported};
use alloc::vec::Vec;

mod buffered;
pub use buffered::BufConnection;

cfg_test! {
    mod test;
    pub(crate) use test::TestConnection;
}

cfg_std_unix! {
    mod sendmsg;
    pub use sendmsg::SendmsgConnection;
}

cfg_std! {
    use std::io::{Read, Write};

    pub type IoSlice<'a> = std::io::IoSlice<'a>;
}

cfg_no_std! {
    pub type IoSlice<'a> = &'a [u8];
}

/// A "suitable byte stream" where communication with the X11 server can occur.
/// 
/// This is implemented by default for all types that implement both `Read` and
/// `Write`.
pub trait Connection {
    /// Write a series of I/O slices and file descriptors to the connection.
    fn write_slices_with_fds(&mut self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize>;

    /// Write a series of I/O slices to the connection.
    ///
    /// This degenerates into `write_slices_with_fds` without file descriptors by default.
    fn write_slices(&mut self, iov: &[IoSlice<'_>]) -> Result<usize> {
        self.write_slices_with_fds(iov, &mut Vec::new())
    }

    /// Write a single slice to the connection.
    fn write_slice(&mut self, buffer: &[u8]) -> Result<usize> {
        #[cfg(not(feature = "std"))]
        let iov = [buffer];
        #[cfg(feature = "std")]
        let iov = [IoSlice::new(buffer)];

        self.write_slices(&iov)
    }

    /// Does the writer take advantage of vectored writes?
    fn is_write_vectored(&self) -> bool {
        false
    }

    /// Read into a buffer, along with file descriptors.
    fn read_to_buffer_with_fds(&mut self, buffer: &mut [u8], fds: &mut Vec<Fd>) -> Result<usize>;

    /// Flush bytes down this connection.
    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

/// A [`Connection`] that can be read from in a non-blocking manner.
pub trait NbConnection: Connection {
    /// Read into a buffer, but this function will never block.
    fn read_to_buffer_non_blocking(
        &mut self,
        buffer: &mut [u8],
        fds: &mut Vec<Fd>,
    ) -> Result<usize>;
}

cfg_std! {
    impl<Conn: Read + Write + ?Sized> Connection for Conn {
        fn write_slices_with_fds(&mut self, iov: &[IoSlice<'_>], fds: &mut Vec<Fd>) -> Result<usize> {
            if !fds.is_empty() {
                return Err(Error::unsupported(Unsupported::Fds));
            }

            self.write_vectored(iov).map_err(Error::io)
        }

        fn write_slices(&mut self, iov: &[IoSlice<'_>]) -> Result<usize> {
            self.write_vectored(iov).map_err(Error::io)
        }

        fn write_slice(&mut self, data: &[u8]) -> Result<usize> {
            self.write(data).map_err(Error::io)
        }

        fn is_write_vectored(&self) -> bool {
            //Write::is_write_vectored(self)
            true
        }

        fn read_to_buffer_with_fds(&mut self, buffer: &mut [u8], _fds: &mut Vec<Fd>) -> Result<usize> {
            self.read(buffer).map_err(Error::io)
        }

        fn flush(&mut self) -> Result<()> {
            Write::flush(self).map_err(Error::io)
        }
    }
}
