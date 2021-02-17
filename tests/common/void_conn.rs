// MIT/Apache2 License

use breadx::{auto::AsByteSequence, Connection, Fd, Setup};
use std::iter;

/// A connection that stores requests.
pub struct VoidConnection {
    requests: Vec<Box<[u8]>>,
    buffer: Vec<u8>,
}

impl Connection for VoidConnection {
    #[inline]
    fn send_packet(&mut self, bytes: &[u8], fds: &mut Vec<Fd>) -> breadx::Result {
        assert!(fds.is_empty());
        self.requests.push(bytes.into());
        Ok(())
    }

    #[inline]
    fn read_packet(&mut self, bytes: &mut [u8], _fds: &mut Vec<Fd>) -> breadx::Result {
        self.buffer
            .drain(..bytes.len())
            .enumerate()
            .for_each(|(i, b)| {
                bytes[i] = b;
            });
        Ok(())
    }
}

impl VoidConnection {
    #[inline]
    pub fn from_setup(setup: Setup) -> Self {
        let mut buffer = iter::repeat(0).take(setup.size()).collect::<Vec<u8>>();
        setup.as_bytes(&mut buffer);
        buffer[0] = 1;

        Self {
            requests: Vec::new(),
            buffer,
        }
    }

    #[inline]
    pub fn buffer(&mut self, bytes: &[u8]) {
        self.buffer.extend_from_slice(bytes);
    }

    #[inline]
    pub fn audit(&self, bytes: &[Box<[u8]>]) {
        self.requests
            .iter()
            .zip(bytes.iter())
            .for_each(|(ours, theirs)| {
                if ours != theirs {
                    panic!(
                        "Byte mismatch:\n Should've been: {:?}\n Was: {:?}",
                        theirs, ours
                    );
                }
            });
    }
}
