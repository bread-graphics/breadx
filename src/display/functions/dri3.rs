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
    send_request,
};
use cty::c_int;

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

impl<Conn> Display<Conn> {
    #[inline]
    fn open_dri3_request(drawable: Drawable, provider: u32) -> OpenRequest {
        OpenRequest {
            drawable,
            provider,
            ..Default::default()
        }
    }
}

impl<Conn: Connection> Display<Conn> {
    /// Open the DRI3 interface.
    #[inline]
    pub fn open_dri3<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<RequestCookie<OpenRequest>> {
        send_request!(self, Self::open_dri3_request(drawable.into(), provider))
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

    #[inline]
    pub fn query_dri3_version(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        send_request!(
            self,
            QueryVersionRequest {
                major_version: major,
                minor_version: minor,
                ..Default::default()
            }
        )
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
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection + Send> Display<Conn> {
    #[inline]
    pub async fn open_dri3_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<RequestCookie<OpenRequest>> {
        send_request!(
            self,
            Self::open_dri3_request(drawable.into(), provider),
            async
        )
        .await
    }

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

    #[cfg(feature = "async")]
    #[inline]
    pub async fn query_dri3_version_async(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        send_request!(
            self,
            QueryVersionRequest {
                major_version: major,
                minor_version: minor,
                ..Default::default()
            },
            async
        )
        .await
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
