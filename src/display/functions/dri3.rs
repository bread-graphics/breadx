// MIT/Apache2 License

#![cfg(feature = "dri3")]

use crate::{
    auto::{
        dri3::{OpenRequest, QueryVersionRequest},
        xproto::Drawable,
        AsByteSequence,
    },
    display::{Connection, Display, RequestCookie},
    extension::ExtensionVersion,
};
use cty::c_int;

impl<Conn: Connection> Display<Conn> {
    #[inline]
    fn open_dri3_request(drawable: Drawable, provider: u32) -> OpenRequest {
        OpenRequest {
            drawable,
            provider,
            ..Default::default()
        }
    }

    /// Open the DRI3 interface.
    #[inline]
    pub fn open_dri3<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<RequestCookie<OpenRequest>> {
        let or = Self::open_dri3_request(drawable.into(), provider);
        self.send_request(or)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn open_dri3_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<RequestCookie<OpenRequest>> {
        let or = Self::open_dri3_request(drawable.into(), provider);
        self.send_request_async(or).await
    }

    #[inline]
    pub fn open_dri3_immediate<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<c_int> {
        let tok = self.open_dri3(drawable, provider)?;
        let mut repl = self.resolve_request(tok)?;
        Ok(repl.file_descriptors().unwrap()[0])
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn open_dri3_immediate_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<c_int> {
        let tok = self.open_dri3_async(drawable, provider).await?;
        let mut repl = self.resolve_request_async(tok).await?;
        Ok(repl.file_descriptors().unwrap()[0])
    }

    #[inline]
    pub fn query_dri3_version(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        let qer = QueryVersionRequest {
            major_version: major,
            minor_version: minor,
            ..Default::default()
        };
        self.send_request(qer)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn query_dri3_version_async(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        let qer = QueryVersionRequest {
            major_version: major,
            minor_version: minor,
            ..Default::default()
        };
        self.send_request_async(qer).await
    }

    #[inline]
    pub fn query_dri3_version_immediate(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<ExtensionVersion> {
        let tok = self.query_dri3_version(major, minor)?;
        let repl = self.resolve_request(tok)?;
        Ok(ExtensionVersion {
            major: repl.major_version,
            minor: repl.minor_version,
        })
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn query_dri3_version_immediate_async(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<ExtensionVersion> {
        let tok = self.query_dri3_version_async(major, minor).await?;
        let repl = self.resolve_request_async(tok).await?;
        Ok(ExtensionVersion {
            major: repl.major_version,
            minor: repl.minor_version,
        })
    }
}
