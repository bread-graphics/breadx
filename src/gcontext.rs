// MIT/Apache2 License

use super::{
    auto::xproto::{ChangeGcRequest, Drawable, Gcontext},
    Connection, Display, GcParameters,
};

impl Gcontext {
    #[inline]
    fn change_request<Conn: Connection>(
        &self,
        dpy: &mut Display<Conn>,
        params: GcParameters,
    ) -> ChangeGcRequest {
        let mut cgcr = ChangeGcRequest {
            gc: *self,
            ..Default::default()
        };

        let g = params.mask_change_gc_request(&mut cgcr);
        cgcr.value_mask = g;
        cgcr
    }

    /// Change the properties of this GC.
    #[inline]
    pub fn change<Conn: Connection>(
        &self,
        dpy: &mut Display<Conn>,
        params: GcParameters,
    ) -> crate::Result<()> {
        log::debug!("Sending ChangeGcRequest to server");
        let req = self.change_request(dpy, params);
        let tok = dpy.send_request(req)?;
        log::debug!("Send ChangeGcRequest to server");
        dpy.resolve_request(tok)
    }

    /// Change the properties of this GC, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_async<Conn: Connection, Target: Into<Drawable>>(
        &self,
        dpy: &mut Display<Conn>,
        params: GcParameters,
    ) -> crate::Result<()> {
        log::debug!("Sending ChangeGcRequest to server");
        let req = self.change_request(dpy, params);
        let tok = dpy.send_request_async(req).await?;
        log::debug!("Send ChangeGcRequest to server");
        dpy.resolve_request_async(tok).await
    }
}
