// MIT/Apache2 License

#![cfg(feature = "dri3")]

use crate::{
    auto::{
        dri3::{
            BufferFromPixmapReply, BufferFromPixmapRequest, BuffersFromPixmapReply,
            BuffersFromPixmapRequest, FenceFromFdRequest, GetSupportedModifiersReply,
            GetSupportedModifiersRequest, OpenRequest, PixmapFromBufferRequest,
            PixmapFromBuffersRequest, QueryVersionRequest,
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
use crate::display::AsyncConnection;

#[derive(Debug, Clone, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Modifiers {
    pub window: Vec<u64>,
    pub screen: Vec<u64>,
}

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

    #[inline]
    pub fn get_supported_modifiers(
        &mut self,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> crate::Result<RequestCookie<GetSupportedModifiersRequest>> {
        send_request!(
            self,
            GetSupportedModifiersRequest {
                window,
                depth,
                bpp,
                ..Default::default()
            }
        )
    }

    #[inline]
    pub fn get_supported_modifiers_immediate(
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
    pub fn pixmap_from_buffers(
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
        sr_request!(
            self,
            PixmapFromBuffersRequest {
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
            }
        )?;
        Ok(xid)
    }

    #[inline]
    pub fn pixmap_from_buffer(
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
        sr_request!(
            self,
            PixmapFromBufferRequest {
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
            }
        )?;
        Ok(xid)
    }

    #[inline]
    pub fn fence_from_fd(
        &mut self,
        drawable: Drawable,
        initially_triggered: bool,
        fence_fd: Fd,
    ) -> crate::Result<Fence> {
        let xid = Fence::const_from_xid(self.generate_xid()?);
        sr_request!(
            self,
            FenceFromFdRequest {
                drawable,
                initially_triggered,
                fence: xid.xid,
                fence_fd: vec![fence_fd],
                ..Default::default()
            }
        )?;
        Ok(xid)
    }

    #[inline]
    pub fn buffer_from_pixmap(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<RequestCookie<BufferFromPixmapRequest>> {
        send_request!(
            self,
            BufferFromPixmapRequest {
                pixmap,
                ..Default::default()
            }
        )
    }

    #[inline]
    pub fn buffer_from_pixmap_immediate(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<BufferFromPixmapReply> {
        let bfp = self.buffer_from_pixmap(pixmap)?;
        self.resolve_request(bfp)
    }

    #[inline]
    pub fn buffers_from_pixmap(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<RequestCookie<BuffersFromPixmapRequest>> {
        send_request!(
            self,
            BuffersFromPixmapRequest {
                pixmap,
                ..Default::default()
            }
        )
    }

    #[inline]
    pub fn buffers_from_pixmap_immediate(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<BuffersFromPixmapReply> {
        let bfp = self.buffers_from_pixmap(pixmap)?;
        self.resolve_request(bfp)
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

    #[inline]
    pub async fn get_supported_modifiers_async(
        &mut self,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> crate::Result<RequestCookie<GetSupportedModifiersRequest>> {
        send_request!(
            self,
            GetSupportedModifiersRequest {
                window,
                depth,
                bpp,
                ..Default::default()
            },
            async
        )
        .await
    }

    #[inline]
    pub async fn get_supported_modifiers_immediate_async(
        &mut self,
        window: u32,
        depth: u8,
        bpp: u8,
    ) -> crate::Result<Modifiers> {
        let tok = self
            .get_supported_modifiers_async(window, depth, bpp)
            .await?;
        let GetSupportedModifiersReply {
            window_modifiers,
            screen_modifiers,
            ..
        } = self.resolve_request_async(tok).await?;
        Ok(Modifiers {
            window: window_modifiers,
            screen: screen_modifiers,
        })
    }

    #[inline]
    pub async fn pixmap_from_buffers_async(
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
    ) -> crate::Result<Pixmap> {
        let xid = Pixmap::const_from_xid(self.generate_xid()?);
        fds.retain(|fd| *fd != -1);
        sr_request!(
            self,
            PixmapFromBuffersRequest {
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
            },
            async
        )
        .await?;
        Ok(xid)
    }

    #[inline]
    pub async fn pixmap_from_buffer_async(
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
        sr_request!(
            self,
            PixmapFromBufferRequest {
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
            },
            async
        )
        .await?;
        Ok(xid)
    }

    #[inline]
    pub async fn fence_from_fd_async(
        &mut self,
        drawable: Drawable,
        initially_triggered: bool,
        fence_fd: Fd,
    ) -> crate::Result<Fence> {
        let xid = Fence::const_from_xid(self.generate_xid()?);
        sr_request!(
            self,
            FenceFromFdRequest {
                drawable,
                initially_triggered,
                fence: xid.xid,
                fence_fd: vec![fence_fd],
                ..Default::default()
            },
            async
        )
        .await?;
        Ok(xid)
    }

    #[inline]
    pub async fn buffer_from_pixmap_async(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<RequestCookie<BufferFromPixmapRequest>> {
        send_request!(
            self,
            BufferFromPixmapRequest {
                pixmap,
                ..Default::default()
            },
            async
        )
        .await
    }

    #[inline]
    pub async fn buffer_from_pixmap_immediate_async(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<BufferFromPixmapReply> {
        let bfp = self.buffer_from_pixmap_async(pixmap).await?;
        self.resolve_request_async(bfp).await
    }

    #[inline]
    pub async fn buffers_from_pixmap_async(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<RequestCookie<BuffersFromPixmapRequest>> {
        send_request!(
            self,
            BuffersFromPixmapRequest {
                pixmap,
                ..Default::default()
            },
            async
        )
        .await
    }

    #[inline]
    pub async fn buffers_from_pixmap_immediate_async(
        &mut self,
        pixmap: Pixmap,
    ) -> crate::Result<BuffersFromPixmapReply> {
        let bfp = self.buffers_from_pixmap_async(pixmap).await?;
        self.resolve_request_async(bfp).await
    }
}
