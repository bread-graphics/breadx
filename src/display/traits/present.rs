// MIT/Apache2 License

#![cfg(feature = "present")]

use crate::{
    auto::{
        present::{
            Event, EventMask, Notify, PixmapRequest, QueryCapabilitiesRequest, QueryVersionRequest,
            SelectInputRequest,
        },
        randr::Crtc,
        sync::Fence,
        xfixes::Region,
        xproto::Pixmap,
    },
    display::{Connection, Display, RequestCookie},
    extension::ExtensionVersion,
    send_request, sr_request, Drawable, Window, XID,
};
use alloc::vec::Vec;

#[cfg(feature = "async")]
use crate::display::AsyncDisplay;

pub trait DisplayPresentExt: Display {
    #[inline]
    fn query_present_version(
        &mut self,
        major: u32,
        minor: u32,
    ) -> crate::Result<RequestCookie<QueryVersionRequest>> {
        self.send_request(QueryVersionRequest {
            major_version: major,
            minor_version: minor,
            ..Default::default()
        })
    }

    #[inline]
    fn query_present_version_immediate(
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
    fn present_capabilities<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> crate::Result<RequestCookie<QueryCapabilitiesRequest>> {
        self.send_request(QueryCapabilitiesRequest {
            target: drawable.into().xid,
            ..Default::default()
        })
    }

    #[inline]
    fn present_capabilities_immediate<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> crate::Result<u32> {
        let tok = self.present_capabilities(drawable)?;
        let pc = self.resolve_request(tok)?;
        Ok(pc.capabilities)
    }

    #[inline]
    fn present_select_input(
        &mut self,
        eid: XID,
        window: Window,
        em: EventMask,
    ) -> crate::Result<()> {
        self.exchange_request(SelectInputRequest {
            eid: Event::const_from_xid(eid),
            window,
            event_mask: em,
            ..Default::default()
        })
    }

    #[inline]
    fn present_pixmap(
        &mut self,
        window: Window,
        pixmap: Pixmap,
        serial: u32,
        valid: Region,
        update: Region,
        xoff: i16,
        yoff: i16,
        target_crtc: Crtc,
        wait_fence: Fence,
        idle_fence: Fence,
        options: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
        notifies: Vec<Notify>,
    ) -> crate::Result {
        self.exchange_request(PixmapRequest {
            window,
            pixmap,
            serial,
            valid,
            update,
            x_off: xoff,
            y_off: yoff,
            target_crtc,
            wait_fence,
            idle_fence,
            options,
            target_msc,
            divisor,
            remainder,
            notifies,
            ..Default::default()
        })
    }
}

impl<'a, D: Display<'a> + ?Sized> DisplayPresentExt for D {}

#[cfg(feature = "async")]
pub trait AsyncDisplayPresentExt: AsyncDisplay {
    #[inline]
    fn query_present_version_async(
        &mut self,
        major: u32,
        minor: u32,
    ) -> SendRequestFuture<'_, Self, QueryVersionRequest> {
        self.send_request_async(QueryVersionRequest {
            major_version: major,
            minor_version: minor,
            ..Default::default()
        })
    }

    #[inline]
    fn query_present_version_immediate_async(
        &mut self,
        major: u32,
        minor: u32,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, QueryVersionRequest>,
        fn(crate::Result<QueryVersionReply>) -> crate::Result<ExtensionVersion>,
    > {
        MapFuture::run(
            self.exchange_request_async(QueryVersionRequest {
                major_version: major,
                minor_version: minor,
                ..Default::default()
            }),
            |repl| {
                repl.map(
                    |QueryVersionReply {
                         major_version,
                         minor_version,
                         ..
                     }| ExtensionVersion {
                        major: major_version,
                        minor: minor_version,
                    },
                )
            },
        )
    }

    #[inline]
    fn present_capabilities_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> SendRequestFuture<'_, Self, QueryCapabilitiesRequest> {
        self.send_request_async(QueryCapabilitiesRequest {
            target: drawable.into().xid,
            ..Default::default()
        })
    }

    #[inline]
    fn present_capabilities_immediate_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, QueryCapabilitiesRequest>,
        fn(crate::Result<QueryCapabilitiesReply>) -> crate::Result<u32>,
    > {
        MapFuture::run(
            self.exchange_request_async(QueryCapabilitiesRequest {
                target: drawable.into().xid,
                ..Default::default()
            }),
            |repl| repl.map(|repl| repl.capabilities),
        )
    }

    #[inline]
    fn present_select_input_async(
        &mut self,
        eid: XID,
        window: Window,
        em: EventMask,
    ) -> ExchangeRequestFuture<'_, Self, SelectInputRequest> {
        self.exchange_request_async(SelectInputRequest {
            eid: Event::const_from_xid(eid),
            window,
            event_mask: em,
            ..Default::default()
        })
    }

    #[inline]
    fn present_pixmap_async(
        &mut self,
        window: Window,
        pixmap: Pixmap,
        serial: u32,
        valid: Region,
        update: Region,
        xoff: i16,
        yoff: i16,
        target_crtc: Crtc,
        wait_fence: Fence,
        idle_fence: Fence,
        options: u32,
        target_msc: u64,
        divisor: u64,
        remainder: u64,
        notifies: Vec<Notify>,
    ) -> ExchangeRequestFuture<'_, Self, PixmapRequest> {
        self.exchange_request_async(PixmapRequest {
            window,
            pixmap,
            serial,
            valid,
            update,
            x_off: xoff,
            y_off: yoff,
            target_crtc,
            wait_fence,
            idle_fence,
            options,
            target_msc,
            divisor,
            remainder,
            notifies,
            ..Default::default()
        })
    }
}
