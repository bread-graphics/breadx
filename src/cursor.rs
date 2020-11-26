// MIT/Apache2 License

use crate::{
    auto::xproto::{Cursor, FreeCursorRequest},
    display::{Connection, Display},
};

impl Cursor {
    #[inline]
    pub fn free<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        let fcr = FreeCursorRequest {
            cursor: self,
            ..Default::default()
        };
        log::debug!("Sending FreeCursorRequest to server.");
        let tok = dpy.send_request(fcr)?;
        log::debug!("Sent FreeCursorRequest to server.");
        dpy.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        let fcr = FreeCursorRequest {
            cursor: self,
            ..Default::default()
        };
        log::debug!("Sending FreeCursorRequest to server.");
        let tok = dpy.send_request_async(fcr).await?;
        log::debug!("Sent FreeCursorRequest to server.");
        dpy.resolve_request_async(tok).await
    }
}
