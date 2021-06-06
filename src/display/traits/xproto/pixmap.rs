// MIT/Apache2 License

use crate::{
    auto::xproto::{FreePixmapRequest, Pixmap},
    display::{prelude::*, Connection, Display},
};

#[cfg(feature = "async")]
use crate::display::{futures::ExchangeRequestFuture, AsyncDisplay};

impl Pixmap {
    /// Free the memory used by a pixmap.
    #[inline]
    pub fn free<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(FreePixmapRequest {
            pixmap: self,
            ..Default::default()
        })
    }

    /// Free the memory used by a pixmap, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn free_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> ExchangeRequestFuture<'_, Dpy, FreePixmapRequest> {
        self.exchange_request_async(FreePixmapRequest {
            pixmap: self,
            ..Default::default()
        })
    }
}
