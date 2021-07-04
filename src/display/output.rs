// MIT/Apache2 License

use super::{
    decode_reply, input, Connection, Display, DisplayBase, PendingReply, PendingRequestFlags,
    RequestInfo, RequestWorkaround, EXT_KEY_SIZE,
};
use crate::{auto::xproto::QueryExtensionRequest, log_debug, log_trace};
use alloc::borrow::Cow;
use core::mem;

#[inline]
pub(crate) fn preprocess_request<D: DisplayBase + ?Sized>(
    display: &mut D,
    mut pr: RequestInfo,
) -> RequestInfo {
    log_trace!("Entering preprocess_request()");
    let sequence = display.next_request_number();
    // truncate to u16
    let sequence = sequence as u16;

    pr.set_sequence(sequence);
    pr
}

#[inline]
pub(crate) fn finish_request<D: DisplayBase + ?Sized>(display: &mut D, mut pr: RequestInfo) -> u16 {
    log_trace!("Entering finish_request() with request info: {:?}", &pr);

    // data has already been sent over the bandwaves, make sure we acknowledge it
    let mut flags = PendingRequestFlags {
        expects_fds: pr.expects_fds,
        discard_reply: pr.discard_reply,
        checked: pr.zero_sized_reply && display.checked(),
        ..Default::default()
    };

    match (
        pr.extension,
        pr.opcode,
        pr.data.get(32..36).map(|a| {
            let mut arr: [u8; 4] = [0; 4];
            arr.copy_from_slice(a);
            u32::from_ne_bytes(arr)
        }),
    ) {
        (Some("GLX"), 17, Some(0x10004)) | (Some("GLX"), 21, _) => {
            log_debug!("Applying GLX FbConfig workaround to request");
            flags.workaround = RequestWorkaround::GlxFbconfigBug;
        }
        _ => (),
    }

    let seq = pr.sequence.take().expect("Failed to set sequence number");
    log_debug!("Got sequence number {}", seq);

    if !pr.zero_sized_reply || display.checked() {
        log::trace!(
            "Request is neither zero-sized nor is the display not checked, so we expect a reply"
        );
        input::expect_reply(display, seq, flags);
    }

    seq
}

#[inline]
pub(crate) fn modify_for_opcode(bytes: &mut [u8], request_opcode: u8, ext_opcode: Option<u8>) {
    match ext_opcode {
        None => {
            // First byte is opcode
            // Second byte is minor opcode (ignored for now)
            bytes[0] = request_opcode;
        }
        Some(extension) => {
            // First byte is extension opcode
            // Second byte is regular opcode
            bytes[0] = extension;
            bytes[1] = request_opcode;
        }
    }
}

#[inline]
pub(crate) fn send_request<D: Display + ?Sized, C: Connection + ?Sized>(
    display: &mut D,
    connection: &mut C,
    request_info: RequestInfo,
) -> crate::Result<u16> {
    log_trace!("Entering output::send_request()");

    // figure out the extension opcode
    let ext_opcode = match request_info.extension {
        None => None,
        Some(extension) => {
            let key = str_to_key(extension);
            if let Some(opcode) = display.get_extension_opcode(&key) {
                Some(opcode)
            } else {
                let opcode = get_ext_opcode(display, connection, extension)?;
                display.set_extension_opcode(key, opcode);
                Some(opcode)
            }
        }
    };

    // figure out sequence, et al
    let mut req = preprocess_request(display, request_info);

    let request_opcode = req.opcode;
    modify_for_opcode(&mut req.data, request_opcode, ext_opcode);
    log_trace!("We are sending the following request: {:?}", &req);

    // send the packet
    log_debug!("Request is ready to send, beginning send_packet()");
    let mut fds = mem::take(&mut req.fds);
    connection.send_packet(&req.data, &mut fds)?;
    log_debug!("Finished send_packet()");

    Ok(finish_request(display, req))
}

#[inline]
pub(crate) fn get_ext_opcode<D: Display + ?Sized, C: Connection + ?Sized>(
    display: &mut D,
    conn: &mut C,
    extension: &'static str,
) -> crate::Result<u8> {
    log_trace!("Entering get_ext_opcode with extension: {}", extension);
    log_debug!(
        "Could not find extension opcode in display's database; sending request to server..."
    );

    let qer = QueryExtensionRequest {
        name: Cow::Borrowed(extension),
        ..Default::default()
    };

    log_trace!("Sending QER..");
    let tok = send_request(
        display,
        conn,
        RequestInfo::from_request(qer, display.bigreq_enabled(), display.max_request_len()),
    )?;
    log_trace!("Resolving QER...");
    let repl = loop {
        match display.take_pending_reply(tok) {
            Some(PendingReply { data, fds }) => {
                break decode_reply::<QueryExtensionRequest>(&data, fds)?;
            }
            None => input::wait(display, conn)?,
        }
    };

    if !repl.present {
        return Err(crate::BreadError::ExtensionNotPresent(extension.into()));
    }

    log_debug!("Found opcode for extension: {}", &repl.major_opcode);
    let key = str_to_key(extension);
    display.set_extension_opcode(key, repl.major_opcode);
    log::info!(
        "Loaded extension \"{}\" with first event {} and first error {}",
        extension,
        repl.first_event,
        repl.first_error
    );
    // TODO: first_event, first_error
    Ok(repl.major_opcode)
}

#[inline]
pub(crate) fn str_to_key(s: &str) -> [u8; EXT_KEY_SIZE] {
    let mut key = [0_u8; EXT_KEY_SIZE];
    let b = s.as_bytes();

    if b.len() > EXT_KEY_SIZE {
        key.copy_from_slice(&b[..EXT_KEY_SIZE]);
    } else {
        (&mut key[..b.len()]).copy_from_slice(b);
    }

    key
}
