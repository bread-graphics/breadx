// MIT/Apache2 License

use crate::{
    connection::{advance_io, new_io_slice, IoSlice},
    Error, Fd, Result,
};
use alloc::{boxed::Box, vec::Vec};
use core::{convert::TryFrom, mem};
use x11rb_protocol::{
    connection::ReplyFdKind,
    x11_utils::{ReplyFDsRequest, ReplyRequest, Request, TryParseFd, VoidRequest},
    DiscardMode,
};

/// The raw request.
///
/// This structure essentially acts as a monomorphization of the
/// [`Request`] trait. It contains all of the information needed
/// to send the request, such as the raw bytes, whether it has a
/// reply, and the name of the extension.
///
/// [`Request`]: crate::x11_utils::Request
pub struct RawRequest<'target, 'req> {
    // invariants:
    //
    // - always two empty slices before any actual data
    // - third data slice has at least four bytes
    data: &'target mut [IoSlice<'req>],
    fds: Vec<Fd>,
    advanced: usize,
    variant: ReplyFdKind,
    extension_name: Option<&'static str>,
    discard_reply: bool,
    /// Scratch space buffer for use in formatting.
    buffer: Option<&'req mut [u8; 8]>,
}

/// A self-contained request.
///
/// Useful for futures.
pub(crate) struct BufferedRequest {
    data: Vec<u8>,
    fds: Vec<Fd>,
    advanced: usize,
    variant: ReplyFdKind,
    discard_reply: bool,
    extension_name: Option<&'static str>,
    buffer: [u8; 8],
}

fn from_request<R: Request, Ret>(
    request: R,
    variant: ReplyFdKind,
    discard_reply: bool,
    f: impl FnOnce(RawRequest<'_, '_>) -> Ret,
) -> Ret {
    // use u8::MAX as a placeholder
    let (serialized, fds) = request.serialize(u8::MAX);

    // slices to use
    let mut slices = [
        new_io_slice(&[]),
        new_io_slice(&[]),
        new_io_slice(&serialized),
    ];

    // buffer to use for formatting
    let mut buffer = [0; 8];

    let mut req = RawRequest::new(&mut slices, fds, variant, R::EXTENSION_NAME, &mut buffer);
    if discard_reply {
        req.discard_reply();
    }

    f(req)
}

/// Create a new `RawRequest` from a `VoidRequest` type.
pub fn from_void_request<R: VoidRequest, Ret>(
    request: R,
    discard_reply: bool,
    f: impl FnOnce(RawRequest<'_, '_>) -> Ret,
) -> Ret {
    from_request(request, ReplyFdKind::NoReply, discard_reply, f)
}

/// Create a new `RawRequest` from a `ReplyRequest` type.
pub fn from_reply_request<R: ReplyRequest, Ret>(
    request: R,
    f: impl FnOnce(RawRequest<'_, '_>) -> Ret,
) -> Ret {
    from_request(request, ReplyFdKind::ReplyWithoutFDs, false, f)
}

/// Create a new `RawRequest` from a `ReplyFDsRequest` type.
pub fn from_reply_fds_request<R: ReplyFDsRequest, Ret>(
    request: R,
    f: impl FnOnce(RawRequest<'_, '_>) -> Ret,
) -> Ret {
    from_request(request, ReplyFdKind::ReplyWithFDs, false, f)
}

impl<'target, 'req> RawRequest<'target, 'req> {
    /// Create a new `RawRequest` from its raw parts.
    ///
    /// # Panics
    ///
    /// Panics if:
    ///
    /// - There are less than three slices.
    /// - The third slice is not at least four bytes long.
    pub fn new(
        data: &'target mut [IoSlice<'req>],
        fds: Vec<Fd>,
        variant: ReplyFdKind,
        extension_name: Option<&'static str>,
        buffer: &'req mut [u8; 8],
    ) -> Self {
        assert!(data.len() >= 3);
        assert!(data[2].len() >= 4);

        Self {
            data,
            fds,
            advanced: 0,
            variant,
            extension_name,
            discard_reply: false,
            buffer: Some(buffer),
        }
    }

    /// Discard the reply.
    fn discard_reply(&mut self) {
        self.discard_reply = true;
    }

    pub(crate) fn discard_mode(&self) -> Option<DiscardMode> {
        if self.discard_reply {
            Some(DiscardMode::DiscardReply)
        } else {
            None
        }
    }

    /// Get the variant of the request.
    #[must_use]
    pub fn variant(&self) -> ReplyFdKind {
        self.variant
    }

    /// Once the `RawRequest` is finished, this function will
    /// return the parts that can be sent over the wire.
    #[must_use]
    pub fn into_raw_parts(self) -> (Box<[u8]>, Vec<Fd>) {
        let BufferedRequest { data, fds, .. } = self.into();

        (data.into_boxed_slice(), fds)
    }

    /// Compute the length of this request.
    ///
    /// # Panics
    ///
    /// - If this function was already called on this request.
    #[allow(clippy::cast_possible_truncation)]
    pub fn format(&mut self, ext_opcode: Option<u8>, max_len: usize) -> Result<()> {
        let len = self.len();

        // format!
        let x_len = len / 4;

        if x_len > max_len {
            return Err(Error::make_large_request(x_len, max_len));
        }

        // see if we can use truncated notation
        if let Ok(x_len) = u16::try_from(x_len) {
            let [l1, l2] = x_len.to_ne_bytes();

            // copy first two bytes of data into the buffer
            let buffer = self.buffer.as_mut().unwrap();
            buffer[0] = match ext_opcode {
                Some(x) => x,
                None => self.data[2][0],
            };
            buffer[1] = self.data[2][1];
            buffer[2] = l1;
            buffer[3] = l2;

            // set/advance buffer
            let buffer = self.buffer.take().unwrap();
            self.data[1] = new_io_slice(&buffer[..4]);
            advance_io(&mut self.data[2], 4);
        } else {
            let length_bytes = ((x_len + 1) as u32).to_ne_bytes();

            // copy existing data to the buffer
            let buffer = self.buffer.as_mut().unwrap();
            buffer[0] = match ext_opcode {
                Some(x) => x,
                None => self.data[2][0],
            };
            buffer[1] = self.data[2][1];
            buffer[2] = 0;
            buffer[3] = 0;
            buffer[4..8].copy_from_slice(&length_bytes);

            self.data[1] = new_io_slice(self.buffer.take().unwrap());
            advance_io(&mut self.data[2], 4);
        }

        Ok(())
    }

    /// The extension name for the opcode.
    #[must_use]
    pub fn extension(&self) -> Option<&'static str> {
        self.extension_name
    }

    /// Get the total amount of data.
    pub fn len(&self) -> usize {
        self.data
            .iter()
            .map(|buf| buf.len())
            .fold(0, usize::saturating_add)
    }

    /// Tell whether this request is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.data.iter().all(|buf| buf.is_empty())
    }

    /// Get the parts of this data that will be sent over the wire.
    #[allow(clippy::mut_mut)]
    pub fn mut_parts(&mut self) -> (&mut &'target mut [IoSlice<'req>], &mut Vec<Fd>) {
        (&mut self.data, &mut self.fds)
    }

    /// Advance this request by the given number of bytes.
    pub fn advance(&mut self, bytes: usize) {
        // keep track of how many bytes we advanced by
        self.advanced += bytes;

        // determine how many slices from the front we can
        // remove
        let mut to_remove = 0;
        let mut total_len = 0;

        for buf in self.data.iter() {
            if total_len + buf.len() > bytes {
                break;
            }
            total_len += buf.len();
            to_remove += 1;
        }

        tracing::trace!(
            "Advancing by {} bytes, removing {} slices and {} bytes",
            bytes,
            to_remove,
            bytes - total_len,
        );
        /*tracing::trace!(
            "Remaining slices: {:?}",
            self.data.iter().map(|buf| buf.len()).collect::<Vec<_>>()
        );*/

        // replace the bytes
        self.data = &mut mem::take(&mut self.data)[to_remove..];

        // advance the last buffer if we need to
        if !self.data.is_empty() {
            advance_io(&mut self.data[0], bytes - total_len);
        }
    }
}

impl BufferedRequest {
    pub(crate) fn take<Ret>(&mut self, f: impl FnOnce(RawRequest<'_, '_>) -> Ret) -> Ret {
        // create a RawRequest borrowed from this request
        let raw = RawRequest {
            buffer: Some(&mut self.buffer),
            data: &mut [
                new_io_slice(&[]),
                new_io_slice(&[]),
                new_io_slice(&self.data[self.advanced..]),
            ],
            fds: mem::take(&mut self.fds),
            advanced: self.advanced,
            variant: self.variant,
            discard_reply: self.discard_reply,
            extension_name: self.extension_name,
        };

        // run the function
        f(raw)
    }
}

cfg_async! {
    impl BufferedRequest {
        pub(crate) fn borrow<Ret>(&mut self, f: impl FnOnce(&mut RawRequest<'_, '_>) -> Ret) -> Ret {
            // create a RawRequest borrowed from this request
            tracing::trace!(data_len = self.data.len());
            let mut raw = RawRequest {
                buffer: Some(&mut self.buffer),
                data: &mut [
                    new_io_slice(&[]),
                    new_io_slice(&[]),
                    new_io_slice(&self.data[self.advanced..]),
                ],
                fds: mem::take(&mut self.fds),
                advanced: self.advanced,
                variant: self.variant,
                discard_reply: self.discard_reply,
                extension_name: self.extension_name,
            };

            // run the function
            let ret = f(&mut raw);

            // merge the buffer into the request if we've formatted it
            let merge_buffer = if raw.data.len() > 2 && raw.data[1].len() > 0 {
                Some(raw.data[1].len())
            } else {
                None
            };

            // if the request has already advanced past our cursor,
            // we're done
            self.advanced = raw.advanced;
            self.fds = mem::take(&mut raw.fds);

            if self.advanced >= self.data.len() {
                self.data.clear();
                return ret;
            }

            if let Some(merge_len) = merge_buffer {
                // splice the buffer variable to the beginning of our slice
                tracing::trace!(merge_len, "merging buffer into data");
                self.data
                    .splice(0..4, self.buffer[..merge_len].iter().copied());
            }

            tracing::trace!("{:?}", &self.data);

            ret
        }
    }
}

impl<'target, 'req> From<RawRequest<'target, 'req>> for BufferedRequest {
    fn from(raw: RawRequest<'target, 'req>) -> Self {
        // collect the data into a single buffer
        let mut data = Vec::with_capacity(raw.len() + 4);
        for buf in raw.data.iter() {
            data.extend_from_slice(buf);
        }

        Self {
            data,
            advanced: raw.advanced,
            variant: raw.variant,
            extension_name: raw.extension_name,
            fds: raw.fds,
            discard_reply: raw.discard_reply,
            buffer: match raw.buffer {
                Some(buf) => *buf,
                None => [0; 8],
            },
        }
    }
}

/// The raw reply.
pub struct RawReply {
    data: Box<[u8]>,
    fds: Vec<Fd>,
}

impl RawReply {
    #[must_use]
    pub fn new(data: Box<[u8]>, fds: Vec<Fd>) -> Self {
        Self { data, fds }
    }

    pub fn into_reply<T: TryParseFd>(mut self) -> Result<T> {
        let (val, _) =
            T::try_parse_fd(&self.data, &mut self.fds).map_err(Error::make_parse_error)?;
        Ok(val)
    }
}
