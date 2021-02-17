// MIT/Apache2 License

use breadx::{Setup, AsyncConnection, GenericConnFuture, Fd, auto::AsByteSequence};
use std::future::ready;

/// An async connection that stores requests.
#[derive(Default)]
pub struct AsyncVoidConnection(
    requests: Vec<Box<[u8]>>,
    setup: Option<Setup>,
    setup_auth: bool,
}

impl AsyncConnection for AsyncVoidConnection {
    #[inline]
    fn send_packet<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b [u8],
        fds: &'c mut Vec<Fd>,
    ) -> GenericConnFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future {
        assert!(fds.is_empty());
        self.requests.push(bytes.into());
        Box::pin(ready(Ok(())));
    }

    #[inline]
    fn read_packet<'future, 'a, 'b, 'c>(
        &'a mut self,
        bytes: &'b mut [u8],
        _fds: &'c mut Vec<Fd>,
    ) -> GenericConnFuture<'future>
    where
        'a: 'future,
        'b: 'future,
        'c: 'future {
        let setup = self.setup().take().expect("Should not be reading bytes from a void connection");
        let mut buffer = Vec::with_capacity(setup.size());
        setup.as_bytes(&mut buffer);        
        buffer[0] = 1;

        if !self.setup_auth {
            self.setup = Some(setup);
            self.setup_auth = true;
            (&mut bytes[..8]).copy_from_slice(&buffer[..8]);
        } else {
            (&mut bytes[8..]).copy_from_slice(&buffer[8..]);
        }

        Box::pin(ready(Ok(())))
    }
}

impl VoidConnection {
    #[inline]
    pub fn from_setup(setup: Setup) -> Self { Self { requests: Vec::new(), setup: Some(setup), setup_auth: false } }

    #[inline]
    pub fn audit(&self, bytes: &[Box<[u8]>]) {
        self.requests.iter().zip(bytes.iter()).for_each(|(ours, theirs)| {
            if ours != theirs {
                panic!("Byte mismatch:\n Should've been: {:?}\n Was: {:?}", theirs, ours);
            }
        });
    }
}
