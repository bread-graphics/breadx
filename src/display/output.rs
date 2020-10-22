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

        let mut bytes: TinyVec<[u8; 32]> = cycled_zeroes(R::size() + 4);

        // First byte is opcode
        // Second byte is minor opcode (ignored for now)
        // Third and fourth are length
        bytes[0] = req.opcode();
        let len = req.as_bytes(&mut bytes[4..]);
        bytes.truncate(len + 5);
        let xlen = (len / 4) + 1;
        let len_bytes = xlen.to_ne_bytes();
        bytes[2] = len_bytes[0];
        bytes[3] = len_bytes[1];

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
