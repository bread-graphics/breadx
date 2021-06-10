// MIT/Apache2 License

use crate::BreadError;

/// Either a pending request, a pending reply, or a pending error.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Exchange {
    Request(PendingRequest),
    Reply(PendingReply),
    Error(BreadError),
}

impl Exchange {

}

/// Request information, monomorphized from the Request trait.
#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct RequestInfo {
    pub(crate) data: TinyVec<[u8; 32]>,
    pub(crate) fds: Vec<Fd>,
    pub(crate) zero_sized_reply: bool,
    pub(crate) opcode: u8,
    pub(crate) extension: Option<&'static str>,
    pub(crate) expects_fds: bool,
    pub(crate) discard_reply: bool,
    pub(crate) sequence: Option<u16>,
}

impl RequestInfo {
    /// Generate a `RequestInfo` given a specific `Request` to generate from.
    #[inline]
    pub fn from_request<R: Request>(mut req: R, use_bigreq: bool, max_request_len: usize) -> Self {
        const SHORT_REQUEST_LIMIT: usize = (u16::MAX as usize) * 4;
        debug_assert!(use_bigreq || max_request_len <= SHORT_REQUEST_LIMIT);

        // TODO: somehow write using uninitialzied data
        let mut data = iter::repeat(0)
            .take(req.size())
            .collect::<TinyVec<[u8; 32]>>();
        let mut len = req.as_bytes(&mut data);

        // make sure it's aligned to a multiple of 4
        len = (len + 3) & (!0x03);
        expand_or_truncate_to_length(&mut data, len);

        // note: we assume max_request_len is already normalized
        assert!(
            max_request_len >= len,
            "Request's size was larger than the maximum request length"
        );

        // If we fit in the short request limit, third and fourth bytes need to be length
        let x_len = len / 4;
        log::trace!("xlen is {}", x_len);
        if use_bigreq {
            let length_bytes = ((x_len + 1) as u32).to_ne_bytes();
            data = match data {
                TinyVec::Inline(data) => BigreqIterator {
                    inner: data.into_iter(),
                    length_bytes,
                    cursor: 0,
                }
                .collect(),
                TinyVec::Heap(data) => BigreqIterator {
                    inner: data.into_iter(),
                    length_bytes,
                    cursor: 0,
                }
                .collect(),
            };
        } else {
            let len_bytes = (x_len as u16).to_ne_bytes();
            data[2] = len_bytes[0];
            data[3] = len_bytes[1];
        }

        RequestInfo {
            data,
            fds: match req.file_descriptors() {
                Some(fd) => mem::take(fd),
                None => Vec::new(),
            },
            zero_sized_reply: mem::size_of::<R::Reply>() == 0,
            opcode: R::OPCODE,
            extension: R::EXTENSION,
            expects_fds: R::REPLY_EXPECTS_FDS,
            discard_reply: false,
            sequence: None,
        }
    }

    /// Set the sequence number for this `RequestInfo`.
    #[inline]
    pub(crate) fn set_sequence(&mut self, seq: u16) {
        self.sequence = Some(seq);
    }
}

struct BigreqIterator<I> {
    inner: I,
    cursor: usize,
    length_bytes: [u8; 4],
}

impl<I: Iterator<Item = u8>> Iterator for BigreqIterator<I> {
    type Item = u8;

    #[inline]
    fn next(&mut self) -> Option<u8> {
        let res = match self.cursor {
            // 2..4
            2..=3 => {
                self.inner.next();
                Some(0)
            }
            // 4..8
            4..=7 => Some(self.length_bytes[self.cursor - 4]),
            _ => self.inner.next(),
        };

        self.cursor += 1;
        res
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let (lo, hi) = self.inner.size_hint();
        (lo + 4, hi.map(|hi| hi + 4))
    }
}

impl<I: Iterator<Item = u8> + FusedIterator> FusedIterator for BigreqIterator<I> {}

impl<I: Iterator<Item = u8> + ExactSizeIterator> ExactSizeIterator for BigreqIterator<I> {}

/// A reply, pending returning from the display.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PendingReply {
    pub data: TinyVec<[u8; 32]>,
    pub fds: Box<[Fd]>,
}

/// A cookie for a request.
///
/// Requests usually take time to resolve into replies. Therefore, the `Display::send_request` method returns
/// the `RequestCookie`, which is later used to block (or await) for the request's eventual result.
#[derive(Copy, Clone, PartialOrd, Ord, PartialEq, Default, Eq, Hash)]
#[repr(transparent)]
pub struct RequestCookie<R: Request> {
    sequence: u16,
    _phantom: PhantomData<Option<R::Reply>>,
}

impl<R: Request> fmt::Debug for RequestCookie<R> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RequestCookie")
            .field("sequence", &self.sequence)
            .finish()
    }
}

impl<R: Request> RequestCookie<R> {
    #[inline]
    pub(crate) fn from_sequence(sequence: u16) -> Self {
        Self {
            sequence,
            _phantom: PhantomData,
        }
    }

    #[inline]
    #[must_use]
    pub fn sequence(self) -> u16 {
        self.sequence
    }
}

#[derive(Debug, Default, Clone, Copy)]
pub struct PendingRequest {
    pub request: u16,
    pub flags: PendingRequestFlags,
}

#[derive(Default, Debug, Copy, Clone)]
pub struct PendingRequestFlags {
    pub discard_reply: bool,
    pub checked: bool,
    pub expects_fds: bool,
    pub workaround: RequestWorkaround,
}

#[derive(Debug, Copy, Clone)]
pub enum RequestWorkaround {
    NoWorkaround,
    GlxFbconfigBug,
}

impl Default for RequestWorkaround {
    #[inline]
    fn default() -> Self {
        Self::NoWorkaround
    }
}
