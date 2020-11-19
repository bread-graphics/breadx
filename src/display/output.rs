// MIT/Apache2 License

use super::{Connection, RequestCookie};
use crate::{util::cycled_zeroes, Request};
use core::mem;
use tinyvec::TinyVec;

impl<Conn: Connection> super::Display<Conn> {
    #[inline]
    fn encode_request<R: Request>(&mut self, req: R) -> (u64, TinyVec<[u8; 32]>) {
        self.request_number += 1;
        let sequence = self.request_number;

        // write to bytes
        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(req.size());
        let len = req.as_bytes(&mut bytes);

        // First byte is opcode
        // Second byte is minor opcode (ignored for now)
        // Third and fourth are length
        log::debug!("Request has opcode {}", R::OPCODE);
        bytes[0] = R::OPCODE;

        let xlen = len / 4;
        let len_bytes = xlen.to_ne_bytes();
        bytes[2] = len_bytes[0];
        bytes[3] = len_bytes[1];
        bytes.truncate(len);

        self.expect_reply(sequence, Default::default());

        (sequence, bytes)
    }

    #[inline]
    pub fn send_request_internal<R: Request>(&mut self, req: R) -> crate::Result<RequestCookie<R>> {
        let (sequence, bytes): (u64, TinyVec<[u8; 32]>) = self.encode_request(req);
        log::debug!("Request has opcode {}", bytes[0]);

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
