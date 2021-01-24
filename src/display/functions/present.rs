// MIT/Apache2 License

#![cfg(feature = "present")]

use crate::{
    auto::present::{
        Event, EventMask, QueryCapabilitiesRequest, QueryVersionRequest, SelectInputRequest,
    },
    display::{Connection, Display, RequestCookie},
    extension::ExtensionVersion,
    send_request, sr_request, Drawable, Window, XID,
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

    #[inline]
    pub fn present_capabilities<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> crate::Result<RequestCookie<QueryCapabilitiesRequest>> {
        send_request!(
            self,
            QueryCapabilitiesRequest {
                target: drawable.into().xid,
                ..Default::default()
            }
        )
    }

    #[inline]
    pub fn present_capabilities_immediate<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> crate::Result<u32> {
        let tok = self.present_capabilities(drawable)?;
        let pc = self.resolve_request(tok)?;
        Ok(pc.capabilities)
    }

    #[inline]
    pub fn present_select_input(
        &mut self,
        eid: XID,
        window: Window,
        em: EventMask,
    ) -> crate::Result<()> {
        sr_request!(
            self,
            SelectInputRequest {
                eid: Event::const_from_xid(eid),
                window,
                event_mask: em,
                ..Default::default()
            }
        )
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

    #[inline]
    pub async fn present_capabilities_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> crate::Result<RequestCookie<QueryCapabilitiesRequest>> {
        send_request!(
            self,
            QueryCapabilitiesRequest {
                target: drawable.into().xid,
                ..Default::default()
            },
            async
        )
        .await
    }

    #[inline]
    pub async fn present_capabilities_immediate_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> crate::Result<u32> {
        let tok = self.present_capabilities_async(drawable).await?;
        let pc = self.resolve_request_async(tok).await?;
        Ok(pc.capabilities)
    }

    #[inline]
    pub async fn present_select_input_async(
        &mut self,
        eid: XID,
        window: Window,
        em: EventMask,
    ) -> crate::Result<()> {
        sr_request!(
            self,
            SelectInputRequest {
                eid: Event::const_from_xid(eid),
                window,
                event_mask: em,
                ..Default::default()
            },
            async
        )
        .await
    }
}
