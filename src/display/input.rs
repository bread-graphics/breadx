// MIT/Apache2 License

use super::{
    Connection, Display, DisplayBase, HasConnection, PendingReply, PendingRequest,
    PendingRequestFlags, RequestWorkaround,
};
use crate::{event::Event, log_debug, log_trace, Fd, XID};
use alloc::{boxed::Box, vec, vec::Vec};
use core::iter;
use tinyvec::TinyVec;

const TYPE_ERROR: u8 = 0;
const TYPE_REPLY: u8 = 1;
const GENERIC_EVENT: u8 = 35;
const GE_MASK: u8 = 0x7f;

/// Given a set of bytes representing a reply, error, or event, convert those bytes and process them into
/// the given `DisplayBase`.
#[inline]
pub(crate) fn process_bytes<D: DisplayBase + ?Sized>(
    display: &mut D,
    mut bytes: TinyVec<[u8; 32]>,
    fds: Vec<Fd>,
) -> crate::Result {
    // get the sequence number
    let sequence = u16::from_ne_bytes([bytes[2], bytes[3]]);
    log_trace!("Found response bytes: {:?}", &bytes);

    if bytes[0] == TYPE_REPLY {
        log::debug!("Received bytes of type REPLY");

        let pereq = display
            .take_pending_request(sequence)
            .ok_or(crate::BreadError::NoMatchingRequest(sequence))?;

        // if we're discarding the reply, skip the conversion process
        if pereq.flags.discard_reply {
            log::debug!(
                "Discarding reply as per the request's instructions (likely a synchronization)"
            );
        } else {
            // convert to a PendingReply
            let reply = PendingReply {
                data: bytes,
                fds: fds.into_boxed_slice(),
            };

            display.add_pending_reply(sequence, reply);
        }
    } else if bytes[0] == TYPE_ERROR {
        // if it's all zeroes, the X connection has closed and the programmer
        // forgot to check for the close message
        // we're fine to error out here
        if !bytes.iter().any(|&x| x != 0) {
            log::error!("Request was all zeroes, assuming this means connection is closed");
            return Err(crate::BreadError::ClosedConnection);
        }

        let err = crate::BreadError::from_x_error(bytes);

        // if we have a pending request with the given sequence, remove that pending
        // request and put that in the pending requests
        match display.take_pending_request(sequence) {
            Some(_) => {
                display.add_pending_error(sequence, err);
            }
            // if there is no pending request, the display is running in unchecked mode
            // default to returning the error from the request that's currently
            // calling wait()
            None => return Err(err),
        }
    } else {
        log::debug!("Received bytes of type EVENT");
        // this is an event
        let event = Event::from_bytes(bytes)?;
        // if it doesn't fit in any of the special event queues, put it in the main one
        if let Err(event) = filter_into_special_event(display, event) {
            display.push_event(event);
        }
    }

    Ok(())
}

/// See if the specified event fits in a special events queue.
#[inline]
fn filter_into_special_event<D: DisplayBase + ?Sized>(
    display: &mut D,
    event: Event,
) -> Result<(), Event> {
    // if the event's already differentiated, it's not a special event
    let evbytes = match event.as_byte_slice() {
        Some(evbytes) => evbytes,
        None => return Err(event),
    };

    // the first byte will always indicate an XGE event
    if evbytes[0] & 0x7F != GENERIC_EVENT as _ {
        return Err(event);
    }

    let mut eid_bytes: [u8; 4] = [0; 4];
    eid_bytes.copy_from_slice(&evbytes[12..16]);
    let my_eid = u32::from_ne_bytes(eid_bytes);

    display.push_special_event(my_eid, event)
}

/// Tell whether or not we need any additional bytes.
#[inline]
pub(crate) fn additional_bytes(bytes: &[u8]) -> Option<usize> {
    if bytes[0] == TYPE_REPLY || bytes[0] & GE_MASK == GENERIC_EVENT {
        let mut len_bytes = [0; 4];
        len_bytes.copy_from_slice(&bytes[4..8]);
        Some(u32::from_ne_bytes(len_bytes) as usize * 4)
    } else {
        None
    }
}

/// Fix the GLX bug in certain requests.
#[inline]
pub(crate) fn fix_glx_workaround<F: FnOnce(u16) -> bool>(check: F, bytes: &mut [u8]) {
    // this will only ever apply to replies
    if bytes[0] == TYPE_REPLY {
        // grab the pending request
        let sequence = u16::from_ne_bytes([bytes[2], bytes[3]]);
        if check(sequence) {
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
}

/// Add a pending request to the list of pending requests.
#[inline]
pub(crate) fn expect_reply<D: DisplayBase + ?Sized>(
    display: &mut D,
    req: u16,
    flags: PendingRequestFlags,
) {
    let pereq = PendingRequest {
        request: req,
        flags,
    };
    display.add_pending_request(req, pereq);
}

/// Wait for bytes to appear on a synchronous connection.
#[inline]
pub(crate) fn wait<C: Connection + ?Sized, D: Display + ?Sized>(
    display: &mut D,
    connection: &mut C,
) -> crate::Result {
    log_trace!("Ran wait()");
    log::debug!("Beginning wait cycle");

    // replies, errors, and events are all in units of 32 bytes
    let mut bytes: TinyVec<[u8; 32]> = iter::repeat(0).take(32).collect();
    let mut fds: Vec<Fd> = vec![];
    log_trace!("Beginning read_packet()");
    connection.read_packet(&mut bytes, &mut fds)?;
    log_trace!("Ending read_packet()");

    fix_glx_workaround(
        |seq| match display.get_pending_request(seq) {
            None => false,
            Some(pereq) => matches!(pereq.flags.workaround, RequestWorkaround::GlxFbconfigBug),
        },
        &mut bytes,
    );

    // in certain cases, we may have to read more bytes
    if let Some(ab) = additional_bytes(&bytes[..8]) {
        if ab != 0 {
            log_debug!("We need to read {} additional bytes", ab);
            bytes.extend(iter::repeat(0).take(ab));
            log_trace!("Beginning read_packet()");
            connection.read_packet(&mut bytes[32..], &mut fds)?;
            log_trace!("Ending read_packet()");
        }
    }

    log::debug!("Found {} bytes; now processing them...", bytes.len());
    process_bytes(display, bytes, fds)
}
