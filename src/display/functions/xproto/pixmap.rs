// MIT/Apache2 License

use crate::{
    auto::xproto::{FreePixmapRequest, Pixmap},
    display::{Connection, Display},
    sr_request,
};

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

impl Pixmap {
    /// Free the memory used by a pixmap.
    #[inline]
    pub fn free<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        sr_request!(
            dpy,
            FreePixmapRequest {
                pixmap: self,
                ..Default::default()
            }
        )
    }

    /// Free the memory used by a pixmap, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Conn: AsyncConnection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        sr_request!(
            dpy,
            FreePixmapRequest {
                pixmap: self,
                ..Default::default()
            },
            async
        )
        .await
    }
}
