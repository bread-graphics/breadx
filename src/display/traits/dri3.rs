// MIT/Apache2 License

#![cfg(feature = "dri3")]

use crate::{
    auto::{
        dri3::{
            BufferFromPixmapReply, BufferFromPixmapRequest, BuffersFromPixmapReply,
            BuffersFromPixmapRequest, FenceFromFdRequest, GetSupportedModifiersReply,
            GetSupportedModifiersRequest, OpenReply, OpenRequest, PixmapFromBufferRequest,
            PixmapFromBuffersRequest, QueryVersionReply, QueryVersionRequest,
        },
        sync::Fence,
        xproto::Drawable,
        AsByteSequence,
    },
    display::{Connection, Display, RequestCookie},
    extension::ExtensionVersion,
    send_request, sr_request, Fd, Pixmap, Window,
};
use alloc::{vec, vec::Vec};
use cty::c_int;

#[cfg(feature = "async")]
use crate::display::AsyncDisplay;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Modifiers {
    pub window: Vec<u64>,
    pub screen: Vec<u64>,
}

#[inline]
fn open_dri3_request(drawable: Drawable, provider: u32) -> OpenRequest {
    OpenRequest {
        drawable,
        provider,
        ..Default::default()
    }
}

pub trait DisplayDri3Ext: Display {
    /// Open the DRI3 interface.
    #[inline]
    fn open_dri3<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<RequestCookie<OpenRequest>> {
        self.send_request(open_dri3_request(drawable.into(), provider))
    }

    #[inline]
    fn open_dri3_immediate<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> crate::Result<c_int> {
        let tok = self.open_dri3(drawable, provider)?;
        let mut repl = self.resolve_request(tok)?;
        Ok(repl.file_descriptors().unwrap()[0])
    }

    #[inline]
    fn query_dri3_version(
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
    fn query_dri3_version_immediate(
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

    #[inline]
    fn get_supported_modifiers(
        &mut self,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> crate::Result<RequestCookie<GetSupportedModifiersRequest>> {
        self.send_request(GetSupportedModifiersRequest {
            window,
            depth,
            bpp,
            ..Default::default()
        })
    }

    #[inline]
    fn get_supported_modifiers_immediate(
        &mut self,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> crate::Result<Modifiers> {
        let tok = self.get_supported_modifiers(window, depth, bpp)?;
        let GetSupportedModifiersReply {
            window_modifiers,
            screen_modifiers,
            ..
        } = self.resolve_request(tok)?;
        Ok(Modifiers {
            window: window_modifiers,
            screen: screen_modifiers,
        })
    }

    #[inline]
    fn pixmap_from_buffers(
        &mut self,
        window: Window,
        width: u16,
        height: u16,
        depth: u8,
        strides: [u32; 4],
        offsets: [u32; 4],
        bpp: u8,
        modifier: u64,
        mut fds: Vec<Fd>,
    ) -> crate::Result<Pixmap> {
        let xid = Pixmap::const_from_xid(self.generate_xid()?);
        fds.retain(|fd| *fd != -1);
        self.exchange_request(PixmapFromBuffersRequest {
            pixmap: xid,
            window,
            width,
            height,
            depth,
            bpp,
            modifier,
            stride0: strides[0],
            stride1: strides[1],
            stride2: strides[2],
            stride3: strides[3],
            offset0: offsets[0],
            offset1: offsets[1],
            offset2: offsets[2],
            offset3: offsets[3],
            buffers: fds,
            ..Default::default()
        })?;
        Ok(xid)
    }

    #[inline]
    fn pixmap_from_buffer(
        &mut self,
        drawable: Drawable,
        size: u32,
        width: u16,
        height: u16,
        stride: u16,
        depth: u8,
        bpp: u8,
        fd: Fd,
    ) -> crate::Result<Pixmap> {
        let xid = Pixmap::const_from_xid(self.generate_xid()?);
        self.exchange_request(PixmapFromBufferRequest {
            pixmap: xid,
            drawable,
            size,
            width,
            height,
            stride,
            depth,
            bpp,
            pixmap_fd: vec![fd],
            ..Default::default()
        })?;
        Ok(xid)
    }

    #[inline]
    fn fence_from_fd(
        &mut self,
        drawable: Drawable,
        initially_triggered: bool,
        fence_fd: Fd,
    ) -> crate::Result<Fence> {
        let xid = Fence::const_from_xid(self.generate_xid()?);
        self.exchange_request(FenceFromFdRequest {
            drawable,
            initially_triggered,
            fence: xid.xid,
            fence_fd: vec![fence_fd],
            ..Default::default()
        })?;
        Ok(xid)
    }

    #[inline]
    fn buffer_from_pixmap(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<RequestCookie<BufferFromPixmapRequest>> {
        self.send_request(BufferFromPixmapRequest {
            pixmap,
            ..Default::default()
        })
    }

    #[inline]
    fn buffer_from_pixmap_immediate(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<BufferFromPixmapReply> {
        let bfp = self.buffer_from_pixmap(pixmap)?;
        self.resolve_request(bfp)
    }

    #[inline]
    fn buffers_from_pixmap(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<RequestCookie<BuffersFromPixmapRequest>> {
        self.send_request(BuffersFromPixmapRequest {
            pixmap,
            ..Default::default()
        })
    }

    #[inline]
    fn buffers_from_pixmap_immediate(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<BuffersFromPixmapReply> {
        let bfp = self.buffers_from_pixmap(pixmap)?;
        self.resolve_request(bfp)
    }
}

impl<D: Display + ?Sized> DisplayDri3Ext for D {}

#[cfg(feature = "async")]
pub trait AsyncDisplayDri3Ext: AsyncDisplay {
    #[inline]
    fn open_dri3_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> SendRequestFuture<'_, Self, OpenRequest> {
        self.send_request_async(open_dri3_request(drawable.into(), provider))
    }

    #[inline]
    fn open_dri3_immediate_async<Target: Into<Drawable>>(
        &mut self,
        drawable: Target,
        provider: u32,
    ) -> MapFuture<ExchangeRequestFuture<'_, Self, OpenRequest>, fn(OpenReply) -> c_int> {
        MapFuture::run(
            self.exchange_request_async(open_dri3_request(drawable.into(), provider)),
            |repl| repl.file_descriptors().unwrap()[0],
        )
    }

    #[inline]
    fn query_dri3_version_async(
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
    fn query_dri3_version_immediate_async(
        &mut self,
        major: u32,
        minor: u32,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, QueryVersionRequest>,
        fn(QueryVersionReply) -> ExtensionVersion,
    > {
        MapFuture::run(
            self.exchange_request_async(QueryVersionRequest {
                major_version: major,
                minor_version: minor,
                ..Default::default()
            }),
            |repl| ExtensionVersion {
                major: repl.major_version,
                minor: repl.minor_version,
            },
        )
    }

    #[inline]
    fn get_supported_modifiers_async(
        &mut self,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> SendFuture<'_, Self, GetSupportedModifiersRequest> {
        self.send_request_async(GetSupportedModifiersRequest {
            window,
            depth,
            bpp,
            ..Default::default()
        })
    }

    #[inline]
    fn get_supported_modifiers_immediate_async(
        &mut self,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, GetSupportedModifiersRequest>,
        fn(GetSupportedModifiersReply) -> Modifiers,
    > {
        MapFuture::run(
            self.exchange_request_async(GetSupportedModifiersRequest {
                window,
                depth,
                bpp,
                ..Default::default()
            }),
            |GetSupportedModifiersReply {
                 window_modifiers,
                 screen_modifiers,
                 ..
             }| Modifiers {
                window: window_modifiers,
                screen: screen_modifiers,
            },
        )
    }

    #[inline]
    fn pixmap_from_buffers_async(
        &mut self,
        window: Window,
        width: u16,
        height: u16,
        strides: [u32; 4],
        offsets: [u32; 4],
        depth: u8,
        bpp: u8,
        modifier: u64,
        mut fds: Vec<Fd>,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        PixmapFromBuffersRequest,
        Pixmap,
        BoxedFnOnce<Pixmap, PixmapFromBuffersRequest>,
    > {
        fds.retain(|fd| *fd != -1);
        let mut pfbr = PixmapFromBuffersRequest {
            pixmap: Pixmap::const_from_xid(0),
            window,
            width,
            height,
            depth,
            bpp,
            modifier,
            stride0: strides[0],
            stride1: strides[1],
            stride2: strides[2],
            stride3: strides[3],
            offset0: offsets[0],
            offset1: offsets[1],
            offset2: offsets[2],
            offset3: offsets[3],
            buffers: fds,
            ..Default::default()
        };
        self.exchange_xid_async(Box::new(move |xid| {
            pfbr.pixmap = xid;
            pfbr
        }))
    }

    #[inline]
    fn pixmap_from_buffer_async(
        &mut self,
        drawable: Drawable,
        size: u32,
        width: u16,
        height: u16,
        stride: u16,
        depth: u8,
        bpp: u8,
        fd: Fd,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        PixmapFromBufferRequest,
        Pixmap,
        BoxedFnOnce<Pixmap, PixmapFromBufferRequest>,
    > {
        let mut pfbr = PixmapFromBufferRequest {
            pixmap: Pixmap::const_from_xid(0),
            drawable,
            size,
            width,
            height,
            stride,
            depth,
            bpp,
            pixmap_fd: vec![fd],
            ..Default::default()
        };
        self.exchange_xid_async(Box::new(move |xid| {
            pfbr.pixmap = xid;
            pfbr
        }))
    }

    #[inline]
    fn fence_from_fd_async(
        &mut self,
        drawable: Drawable,
        initially_triggered: bool,
        fence_fd: Fd,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        FenceFromFdRequest,
        Fence,
        BoxedFnOnce<Fence, FenceFromFdRequest>,
    > {
        let mut fffr = FenceFromFdRequest {
            drawable,
            initially_triggered,
            fence: xid.xid,
            fence_fd: vec![fence_fd],
            ..Default::default()
        };
        self.exchange_xid_async(Box::new(move |xid| {
            fffr.fence = xid.xid;
            fffr
        }))
    }

    #[inline]
    fn buffer_from_pixmap_async(
        &mut self,
        pixmap: Pixmap,
    ) -> SendRequestFuture<'_, Self, BufferFromPixmapRequest> {
        self.send_request_async(BufferFromPixmapRequest {
            pixmap,
            ..Default::default()
        })
    }

    #[inline]
    fn buffer_from_pixmap_immediate_async(
        &mut self,
        pixmap: Pixmap,
    ) -> ExchangeRequestFuture<'_, Self, BufferFromPixmapReply> {
        self.exchange_request_async(BufferFromPixmapRequest {
            pixmap,
            ..Default::default()
        })
    }

    #[inline]
    fn buffers_from_pixmap_async(
        &mut self,
        pixmap: Pixmap,
    ) -> SendRequestFuture<'_, Self, BuffersFromPixmapRequest> {
        self.send_request_async(BuffersFromPixmapRequest {
            pixmap,
            ..Default::default()
        })
    }

    #[inline]
    fn buffers_from_pixmap_immediate_async(
        &mut self,
        pixmap: Pixmap,
    ) -> ExchangeRequestFuruer<'_, Self, BuffersFromPixmapRequest> {
        self.exchange_request_async(BuffersFromPixmapRequest {
            pixmap,
            ..Default::default()
        })
    }
}

impl<D: AsyncDisplay + ?Sized> AsyncDisplayDri3Ext for D {}
