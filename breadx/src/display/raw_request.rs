// MIT/Apache2 License

use crate::{Error, Fd, Result};
use alloc::{boxed::Box, vec::Vec};
use core::ops::Range;
use x11rb_protocol::{
    connection::ReplyFdKind,
    x11_utils::{ReplyFDsRequest, ReplyRequest, Request, TryParseFd, VoidRequest},
};

/// The raw request.
///
/// This structure essentially acts as a monomorphization of the
/// [`Request`] trait. It contains all of the information needed
/// to send the request, such as the raw bytes, whether it has a
/// reply, and the name of the extension.
///
/// [`Request`]: crate::x11_utils::Request
pub struct RawRequest {
    data: Vec<u8>,
    fds: Vec<Fd>,
    valid_region: Range<usize>,
    variant: ReplyFdKind,
    extension_name: Option<&'static str>,
}

impl Default for RawRequest {
    fn default() -> Self {
        Self {
            data: Vec::new(),
            fds: Vec::new(),
            valid_region: 0..0,
            variant: ReplyFdKind::NoReply,
            extension_name: None,
        }
    }
}

impl RawRequest {
    /// Create a new `RawRequest` from a `Request`.
    fn from_request<R: Request>(req: R, variant: ReplyFdKind) -> Self {
        // use u8::MAX as a placeholder
        let (data, fds) = req.serialize(u8::MAX);

        Self {
            valid_region: 0..data.len(),
            data,
            fds,
            variant,
            extension_name: R::EXTENSION_NAME,
        }
    }

    /// Create a new `RawRequest` from a `VoidRequest`.
    pub fn from_request_void(req: impl VoidRequest) -> Self {
        Self::from_request(req, ReplyFdKind::NoReply)
    }

    /// Create a new `RawRequest` from a `ReplyRequest`.
    pub fn from_request_reply(req: impl ReplyRequest) -> Self {
        Self::from_request(req, ReplyFdKind::ReplyWithoutFDs)
    }

    /// Create a new `RawRequest` from a `ReplyFDsRequest`.
    pub fn from_request_reply_fds(req: impl ReplyFDsRequest) -> Self {
        Self::from_request(req, ReplyFdKind::ReplyWithFDs)
    }

    /// Get the variant of the request.
    pub fn variant(&self) -> ReplyFdKind {
        self.variant
    }

    /// Once the `RawRequest` is finished, this function will
    /// return the parts that can be sent over the wire.
    pub fn into_raw_parts(self) -> (Box<[u8]>, Vec<Fd>) {
        let Self {
            mut data,
            fds,
            valid_region,
            ..
        } = self;

        // take the valid region
        data.truncate(valid_region.end);
        let data = data.split_off(valid_region.start);

        (data.into_boxed_slice(), fds)
    }

    /// Compute the length of this request.
    pub fn compute_length(&mut self, bigreq: bool) {
        let mut len = self.data.len();

        // make sure the length is a multiple of 4
        len = (len + 3) & (!0x03);

        // pad the data to the new length
        self.data.resize(len, 0);

        // format as per bigreq
        let x_len = len / 4;
        if bigreq {
            // TODO: probably more efficient to use i/o
            // slices in this case
            let length_bytes = ((x_len + 1) as u32).to_ne_bytes();
            let [l1, l2, l3, l4] = length_bytes;

            // splice data into the correct place
            let spliced_data = [0, 0, l1, l2, l3, l4];

            self.data.splice(2..4, spliced_data);
            // we need to expand the valid range
            self.valid_region.end += 4;
        } else {
            let length_bytes = (x_len as u16).to_ne_bytes();
            self.data[2..4].copy_from_slice(&length_bytes);
        }
    }

    /// The extension name for the opcode.
    pub fn extension(&self) -> Option<&'static str> {
        self.extension_name
    }

    /// Set the opcode for the extension.
    pub fn set_extension_opcode(&mut self, ext_opcode: u8) {
        self.data[0] = ext_opcode;
    }

    /// Get the mutable parts of the data.
    pub fn mut_parts(&mut self) -> (&mut [u8], &mut Vec<Fd>) {
        (&mut self.data[self.valid_region.clone()], &mut self.fds)
    }

    /// Advance this request by the given number of bytes.
    pub fn advance(&mut self, bytes: usize) {
        self.valid_region.start += bytes;
    }
}

/// The raw reply.
pub struct RawReply {
    data: Box<[u8]>,
    fds: Vec<Fd>,
}

impl RawReply {
    pub fn new(data: Box<[u8]>, fds: Vec<Fd>) -> Self {
        Self { data, fds }
    }

    pub fn into_reply<T: TryParseFd>(mut self) -> Result<T> {
        let (val, _) =
            T::try_parse_fd(&self.data, &mut self.fds).map_err(|e| Error::make_parse_error(e))?;
        Ok(val)
    }
}