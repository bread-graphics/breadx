// MIT/Apache2 License

pub use crate::{
    auto::xproto::{MapWindowRequest, Window},
    display::{Connection, Display},
};

impl Window {
    /// Map this window to the screen.
    #[inline]
    pub fn map<Conn: Connection>(&self, dpy: &mut Display<Conn>) -> crate::Result {
        let mw = MapWindowRequest {
            window: *self,
            ..Default::default()
        };

        log::debug!("Mapping: {:?}", &mw);

        log::debug!("Sending MapWindowRequest to server");
        let tok = dpy.send_request(mw)?;
        defer!(log::debug!("Sent MapWindowRequest to server"));
        dpy.resolve_request(tok)
    }

    /// Map this window to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn map_async<Conn: Connection>(&self, dpy: &mut Display<Conn>) -> crate::Result {
        let mw = MapWindowRequest {
            window: *self,
            ..Default::default()
        };
        let tok = dpy.send_request_async(mw).await?;
        dpy.resolve_request_async(tok).await
    }
}
