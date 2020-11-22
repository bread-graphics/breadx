// MIT/Apache2 License

use super::{Connection, RequestCookie};
use crate::{util::cycled_zeroes, Request};
use core::iter;
use tinyvec::TinyVec;

impl<Conn: Connection> super::Display<Conn> {
    #[inline]
    fn encode_request<R: Request>(&mut self, req: R) -> (u64, TinyVec<[u8; 32]>) {
        let sequence = self.request_number;
        self.request_number += 1;

        // write to bytes
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(req.size());

        let mut len = req.as_bytes(&mut bytes);
        log::trace!("len is {} bytes long", len);

        // pad to a multiple of four bytes if we can
        let remainder = len % 4;
        if remainder != 0 {
            let extend_by = 4 - remainder;
            bytes.extend(iter::once(0).cycle().take(extend_by));
            len += extend_by;
            debug_assert_eq!(len % 4, 0);
            log::trace!("Extended length is now {}", len);
        }

        // First byte is opcode
        // Second byte is minor opcode (ignored for now)
        // Third and fourth are length
        log::debug!("Request has opcode {}", R::OPCODE);
        bytes[0] = R::OPCODE;

        let x_len = len / 4;
        log::trace!("xlen is {}", x_len);
        let len_bytes = x_len.to_ne_bytes();
        bytes[2] = len_bytes[0];
        bytes[3] = len_bytes[1];

        bytes.truncate(len);

        log::trace!("Request has bytes {:?}", &bytes);

        self.expect_reply(sequence, Default::default());

        (sequence, bytes)
    }

    #[inline]
    pub fn send_request_internal<R: Request>(&mut self, req: R) -> crate::Result<RequestCookie<R>> {
        let (sequence, bytes): (u64, TinyVec<[u8; 32]>) = self.encode_request(req);

        self.connection.send_packet(&bytes)?;
        Ok(RequestCookie::from_sequence(sequence))
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn send_request_internal_async<R: Request>(
        &mut self,
        req: R,
    ) -> crate::Result<RequestCookie<R>> {
        let (sequence, bytes) = self.encode_request(req);

        self.connection.send_packet_async(&bytes).await?;
        Ok(RequestCookie::from_sequence(sequence))
    }
}
