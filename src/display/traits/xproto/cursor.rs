// MIT/Apache2 License

use crate::{
    auto::xproto::{Cursor, FreeCursorRequest},
    display::{prelude::*, Display},
};

#[cfg(feature = "async")]
use crate::display::{futures::ExchangeRequestFuture, AsyncDisplay};

impl Cursor {
    #[inline]
    pub fn free<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(FreeCursorRequest {
            cursor: self,
            ..Default::default()
        })
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> ExchangeRequestFuture<'_, Dpy, FreeCursorRequest> {
        dpy.exchange_request_async(FreeCursorRequest {
            cursor: self,
            ..Default::default()
        })
    }
}
