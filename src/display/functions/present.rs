// MIT/Apache2 License

#![cfg(feature = "present")]

use crate::{
    auto::present::QueryVersionRequest,
    display::{Connection, Display, RequestCookie},
    extension::ExtensionVersion,
};

impl<Conn: Connection> Display<Conn> {
    #[inline]
    pub fn query_present_version(
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
    pub async fn query_present_version_async(
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

    #[cfg(feature = "async")]
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
