// MIT/Apache2 License

use super::{Connection, PendingRequest, PendingRequestFlags, RequestWorkaround};
use crate::{event::Event, util::cycled_zeroes, Fd};
use alloc::{boxed::Box, vec, vec::Vec};
use core::iter;
use tinyvec::TinyVec;

const TYPE_ERROR: u8 = 0;
const TYPE_REPLY: u8 = 1;
const GENERIC_EVENT: u8 = 32;
const GE_MASK: u8 = 0x7f;

impl<Conn: Connection> super::Display<Conn> {
    // process a set of 32 bytes into the system
    #[inline]
    fn process_bytes(&mut self, mut bytes: TinyVec<[u8; 32]>, fds: Box<[Fd]>) -> crate::Result {
        // get the sequence number
        let sequence = u16::from_ne_bytes([bytes[2], bytes[3]]);
        log::trace!("Found response bytes: {}", &bytes);

        if bytes[0] == TYPE_REPLY {
            log::debug!("Received bytes of type REPLY");

            let _pereq = self
                .pending_requests
                .remove(&sequence)
                .ok_or(crate::BreadError::NoMatchingRequest(sequence))?;

            // convert bytes to a boxed slice
            bytes.move_to_the_heap();
            let bytes = match bytes {
                TinyVec::Heap(v) => v.into_boxed_slice(),
                TinyVec::Inline(_) => unreachable!(),
            };

            self.pending_replies.insert(sequence, (bytes, fds));
        } else if bytes[0] == TYPE_ERROR {
            // if it's all zeroes, the X connection has closed and the programmer
            // forgot to check for the close message
            // we're fine to error out here
            if !bytes.iter().copied().any(|x| x != 0) {
                return Err(crate::BreadError::ClosedConnection);
            }

            // XCB has some convoluted machinery for errors
            // thank God Rust has better error handling
            return Err(crate::BreadError::from_x_error(bytes));
        } else {
            log::debug!("Received bytes of type EVENT");
            // this is an event
            let event = Event::from_bytes(bytes)?;
            self.event_queue.push_back(event);
        }

        Ok(())
    }

    // if necessary, fix the GLX FbConfigs bug
    // I already ranted about this in output.rs
    #[inline]
    fn fix_glx_workaround(&self, bytes: &mut TinyVec<[u8; 32]>) -> crate::Result<()> {
        // this will only ever apply to replies
        if bytes[0] == TYPE_REPLY {
            // grab the pending request
            let sequence = u16::from_ne_bytes([bytes[2], bytes[3]]);
            let pereq = self
                .pending_requests
                .get(&sequence)
                .ok_or(crate::BreadError::NoMatchingRequest(sequence))?;

            if let RequestWorkaround::GlxFbconfigBug = pereq.flags.workaround {
                log::debug!("Applying GLX FbConfig workaround to reply");

                // length is the 1st u32, numVisuals is the 2nd u32, numProps in the 3rd u32
                // numVisuals is 8..12, numProps is 12..16, length is 4..8
                let (mut visuals, mut props): ([u8; 4], [u8; 4]) = ([0; 4], [0; 4]);
                visuals.copy_from_slice(&bytes[8..12]);
                props.copy_from_slice(&bytes[12..16]);

                let (visuals, props) = (u32::from_ne_bytes(visuals), u32::from_ne_bytes(props));

                let length = (visuals * props * 2).to_ne_bytes();
                (&mut bytes[4..8]).copy_from_slice(&length);
            }
        }

        Ok(())
    }

    // add an entry to the pending elements linked list
    #[allow(clippy::unused_self)]
    #[inline]
    pub(crate) fn expect_reply(&mut self, req: u64, flags: PendingRequestFlags) {
        let pereq = PendingRequest {
            request: req,
            flags,
        };
        if self.pending_requests.insert(req as _, pereq).is_some() {
            panic!("Sequence number overlap; too many requests!");
        }
    }

    // wait for bytes to appear
    #[inline]
    pub(crate) fn wait(&mut self) -> crate::Result {
        log::debug!("Running wait cycle");
        // replies, errors, and events are all in units of 32 bytes
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(32);
        let mut fds: Vec<Fd> = vec![];
        self.connection.read_packet(&mut bytes, &mut fds)?;

        self.fix_glx_workaround(&mut bytes)?;

        // in certain cases, we may have to read more bytes
        if let Some(ab) = additional_bytes(&bytes[..8]) {
            if ab != 0 {
                bytes.extend(iter::once(0).cycle().take(ab * 4));

                log::debug!("Waiting for {} additional bytes", ab * 4);
                self.connection.read_packet(&mut bytes[32..], &mut fds)?;
                log::debug!("Ending wait with {} additional bytes", ab * 4);
            }
        }

        self.process_bytes(bytes, fds.into_boxed_slice())
    }

    // wait for bytes to appear, async redox
    #[cfg(feature = "async")]
    #[inline]
    pub(crate) async fn wait_async(&mut self) -> crate::Result {
        // see above function for more information
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(32);
        let mut fds: Vec<Fd> = vec![];
        self.connection
            .read_packet_async(&mut bytes, &mut fds)
            .await?;

        self.fix_glx_workaround(&mut bytes)?;

        if let Some(ab) = additional_bytes(&bytes[..8]) {
            bytes.extend(iter::once(0).cycle().take(ab * 4));
            self.connection
                .read_packet_async(&mut bytes[32..], &mut fds)
                .await?;
        }

        self.process_bytes(bytes, fds.into_boxed_slice())
    }
}

#[inline]
fn additional_bytes(bytes: &[u8]) -> Option<usize> {
    if bytes[0] == TYPE_REPLY || bytes[0] & GE_MASK == GENERIC_EVENT {
        let mut len_bytes = [0; 4];
        len_bytes.copy_from_slice(&bytes[4..8]);
        Some(u32::from_ne_bytes(len_bytes) as usize)
    } else {
        None
    }
}
