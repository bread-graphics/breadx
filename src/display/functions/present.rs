// MIT/Apache2 License

#![cfg(feature = "present")]

use crate::{
    auto::present::QueryVersionRequest,
    display::{Connection, Display, RequestCookie},
    extension::ExtensionVersion,
    send_request,
};

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

impl<Conn: Connection> Display<Conn> {
    #[inline]
    pub fn query_present_version(
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
    pub fn query_present_version_immediate(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<ExtensionVersion> {
        let qer = self.query_present_version(major, minor)?;
        let reply = self.resolve_request(qer)?;
        Ok(ExtensionVersion {
            major: reply.major_version,
            minor: reply.minor_version,
        })
    }
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection + Send> Display<Conn> {
    #[inline]
    pub async fn query_present_version_async(
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

    #[inline]
    pub async fn query_present_version_immediate_async(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<ExtensionVersion> {
        let qer = self.query_present_version_async(major, minor).await?;
        let reply = self.resolve_request_async(qer).await?;
        Ok(ExtensionVersion {
            major: reply.major_version,
            minor: reply.minor_version,
        })
    }
}
