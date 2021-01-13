// MIT/Apache2 License

use crate::{
    auto::xproto::{Cursor, FreeCursorRequest},
    display::{Connection, Display},
    sr_request,
};

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

impl Cursor {
    #[inline]
    pub fn free<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        sr_request!(
            dpy,
            FreeCursorRequest {
                cursor: self,
                ..Default::default()
            }
        )
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result {
        sr_request!(
            dpy,
            FreeCursorRequest {
                cursor: self,
                ..Default::default()
            },
            async
        )
        .await
    }
}
