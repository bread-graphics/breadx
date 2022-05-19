// MIT/Apache2 License

#![cfg(test)]

use alloc::vec::Vec;
use core::{cmp, mem};

use super::{Connection, IoSlice, IoSliceMut};
use crate::Fd;

/// It is useful, for testing, to just have a `Connection` that reads and writes from
/// internal buffers.
pub(crate) struct TestConnection<'a> {
    /// Bytes that are passed to the reader.
    read_bytes: &'a [u8],
    /// File descriptors that are passed to the reader.
    read_fds: &'a mut Vec<Fd>,

    /// Bytes that are passed from the writer.
    written_bytes: &'a mut Vec<u8>,
    /// File descriptors that are passed from the writer.
    written_fds: &'a mut Vec<i32>,
}

impl<'a> TestConnection<'a> {
    /// Create a new `TestConnection`.
    pub fn new(
        read_bytes: &'a [u8],
        read_fds: &'a mut Vec<Fd>,
        written_bytes: &'a mut Vec<u8>,
        written_fds: &'a mut Vec<i32>,
    ) -> Self {
        Self {
            read_bytes,
            read_fds,
            written_bytes,
            written_fds,
        }
    }
}

impl<'a> Connection for TestConnection<'a> {
    fn send_slices_and_fds(
        &mut self,
        slices: &[IoSlice<'_>],
        fds: &mut Vec<Fd>,
    ) -> crate::Result<usize> {
        let mut len = 0;

        // push every slice onto the written_bytes vector
        for slice in slices {
            len += slice.len();
            self.written_bytes.extend_from_slice(&*slice);
        }

        self.written_fds
            .extend(mem::take(fds).into_iter().map(Fd::into_raw_fd));
        Ok(len)
    }

    fn recv_slices_and_fds(
        &mut self,
        buffer: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> crate::Result<usize> {
        // copy bytes from read_bytes for as long as we can
        let mut len = 0;
        for slice in buffer {
            let amt = cmp::min(self.read_bytes.len(), slice.len());
            slice[..amt].copy_from_slice(&self.read_bytes[..amt]);
            len += amt;
            self.read_bytes = &self.read_bytes[amt..];
        }

        fds.append(self.read_fds);

        Ok(len)
    }

    fn flush(&mut self) -> crate::Result<()> {
        Ok(())
    }

    fn shutdown(&self) -> crate::Result<()> {
        Ok(())
    }

    fn non_blocking_recv_slices_and_fds(
        &mut self,
        slices: &mut [IoSliceMut<'_>],
        fds: &mut Vec<Fd>,
    ) -> crate::Result<usize> {
        self.recv_slices_and_fds(slices, fds)
    }
}

pub(crate) fn with_test_connection(
    read_bytes: &[u8],
    read_fds: Vec<i32>,
    test: impl FnOnce(TestConnection<'_>),
    asserts: impl FnOnce(Vec<u8>, Vec<i32>),
) {
    let mut written_bytes = Vec::new();
    let mut written_fds = Vec::new();
    let mut read_fds = read_fds.into_iter().map(Fd::from).collect::<Vec<_>>();

    let conn = TestConnection::new(
        read_bytes,
        &mut read_fds,
        &mut written_bytes,
        &mut written_fds,
    );

    test(conn);
    asserts(written_bytes, written_fds);
}

#[cfg(all(feature = "std", test))]
mod tests {
    use super::*;
    use crate::setup_tracing;
    use alloc::vec;
    use std::io::IoSlice;

    #[test]
    fn test_send_slices_and_fds() {
        setup_tracing();

        with_test_connection(
            &[],
            vec![],
            |mut conn| {
                let slices = vec![
                    IoSlice::new(&[1, 2, 3]),
                    IoSlice::new(&[4, 5, 6]),
                    IoSlice::new(&[7, 8, 9]),
                ];
                let mut fds = (&[1, 2, 3, 4, 5])
                    .iter()
                    .copied()
                    .map(Fd::from)
                    .collect::<Vec<_>>();
                let mut total_len = 9;

                while total_len > 0 {
                    total_len -= conn.send_slices_and_fds(&slices, &mut fds).unwrap();
                }
            },
            |written_bytes, written_fds| {
                assert_eq!(written_bytes, &[1, 2, 3, 4, 5, 6, 7, 8, 9]);
                assert_eq!(&written_fds, &[1, 2, 3, 4, 5]);
            },
        );
    }

    #[test]
    fn test_recv_slices_and_fds() {
        setup_tracing();

        with_test_connection(
            &[1, 2, 3, 4, 5, 6, 7, 8, 9],
            vec![1, 2, 3, 4, 5],
            |mut conn| {
                let mut buffer = vec![0; 9];

                // split buffer borrows up
                let bborrow = &mut buffer[..];
                let (bborrow1, bborrow_r) = bborrow.split_at_mut(3);
                let (bborrow2, bborrow3) = bborrow_r.split_at_mut(3);

                let mut iov = [
                    IoSliceMut::new(bborrow1),
                    IoSliceMut::new(bborrow2),
                    IoSliceMut::new(bborrow3),
                ];
                let mut fds = vec![];
                let mut total_len = 9;

                while total_len > 0 {
                    total_len -= conn.recv_slices_and_fds(&mut iov, &mut fds).unwrap();
                }

                assert_eq!(buffer, [1, 2, 3, 4, 5, 6, 7, 8, 9]);
                let fds = fds.into_iter().map(Fd::into_raw_fd).collect::<Vec<_>>();
                assert_eq!(fds, vec![1, 2, 3, 4, 5]);
            },
            |_, _| {},
        );
    }
}
