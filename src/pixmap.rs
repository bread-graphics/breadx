// MIT/Apache2 License

use crate::{
    auto::xproto::{FreePixmapRequest, Pixmap},
    display::{Connection, Display},
};

impl Pixmap {
    /// Free the memory used by a pixmap.
    #[inline]
    pub fn free<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        let fpr = FreePixmapRequest {
            pixmap: self,
            ..Default::default()
        };
        log::debug!("Sending FreePixmapRequest to server.");
        let tok = dpy.send_request(fpr)?;
        log::debug!("Sent FreePixmapRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Free the memory used by a pixmap, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        let fpr = FreePixmapRequest {
            pixmap: self,
            ..Default::default()
        };
        log::debug!("Sending FreePixmapRequest to server.");
        let tok = dpy.send_request_async(fpr).await?;
        log::debug!("Sent FreePixmapRequest to server.");
        dpy.resolve_request_async(tok).await
    }
}
