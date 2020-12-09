// MIT/Apache2 License

#![cfg(feature = "async")]

use std::io::{Read, Result, Write};

#[cfg(unix)]
use std::os::unix::io::{AsRawFd, RawFd};

#[cfg(windows)]
use std::os::windows::io::{AsRawSocket, RawSocket};

pub struct RefHack<'a, T>(pub &'a mut T);

impl<'a, T: Read> Read for RefHack<'a, T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.0.read(buf)
    }
}

impl<'a, T: Write> Write for RefHack<'a, T> {
    #[inline]
    fn flush(&mut self) -> Result<()> {
        self.0.flush()
    }

    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.0.write(buf)
    }
}

#[cfg(unix)]
impl<'a, T: AsRawFd> AsRawFd for RefHack<'a, T> {
    #[inline]
    fn as_raw_fd(&self) -> RawFd {
        self.0.as_raw_fd()
    }
}

#[cfg(windows)]
impl<'a, T: AsRawSocket> AsRawSocket for RefHack<'a, T> {
    #[inline]
    fn as_raw_socket(&self) -> RawSocket {
        self.0.as_raw_socket()
    }
}
