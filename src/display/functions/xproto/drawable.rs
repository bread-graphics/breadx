// MIT/Apache2 License

//! This module contains functions that can be applied to nearly all drawable objects, such as Windows
//! and Pixmaps.

use crate::{
    auto::xproto::{
        CopyAreaRequest, CopyPlaneRequest, CreatePixmapRequest, Drawable, GetGeometryReply,
        GetGeometryRequest, Pixmap, Window,
    },
    image::{put::put_image_req, Image},
    send_request, sr_request, Connection, Display, Gcontext, RequestCookie,
};
use alloc::vec::Vec;
use core::{convert::TryInto, ops::Deref};

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

/// The return type of `drawable::get_geometry_immediate`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub struct Geometry {
    pub depth: u8,
    pub root: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
}

impl From<GetGeometryReply> for Geometry {
    #[inline]
    fn from(ggr: GetGeometryReply) -> Self {
        Self {
            depth: ggr.depth,
            root: ggr.root,
            x: ggr.x,
            y: ggr.y,
            width: ggr.width,
            height: ggr.height,
            border_width: ggr.border_width,
        }
    }
}

/// Get geometry request.
#[inline]
fn get_geometry_request(target: Drawable) -> GetGeometryRequest {
    GetGeometryRequest {
        drawable: target,
        ..Default::default()
    }
}

/// Copy area request.
#[inline]
fn copy_area_request(
    src_drawable: Drawable,
    dst_drawable: Drawable,
    gc: Gcontext,
    src_x: i16,
    src_y: i16,
    width: u16,
    height: u16,
    dst_x: i16,
    dst_y: i16,
) -> CopyAreaRequest {
    CopyAreaRequest {
        src_drawable,
        dst_drawable,
        gc,
        src_x,
        src_y,
        dst_x,
        dst_y,
        width,
        height,
        ..Default::default()
    }
}

/// Copy plane request.
#[inline]
fn copy_plane_request(
    src_drawable: Drawable,
    dst_drawable: Drawable,
    gc: Gcontext,
    src_x: i16,
    src_y: i16,
    width: u16,
    height: u16,
    dst_x: i16,
    dst_y: i16,
    bit_plane: u32,
) -> CopyPlaneRequest {
    CopyPlaneRequest {
        src_drawable,
        dst_drawable,
        gc,
        src_x,
        src_y,
        dst_x,
        dst_y,
        width,
        height,
        bit_plane,
        ..Default::default()
    }
}

/// Create pixmap request.
#[inline]
fn create_pixmap_request(
    drawable: Drawable,
    pid: Pixmap,
    width: u16,
    height: u16,
    depth: u8,
) -> CreatePixmapRequest {
    CreatePixmapRequest {
        drawable,
        pid,
        width,
        height,
        depth,
        ..Default::default()
    }
}

impl<Conn: Connection> Display<Conn> {
    /// Get the geometry of a drawable object.
    #[inline]
    pub fn get_drawable_geometry<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        send_request!(self, get_geometry_request(target.into()))
    }

    /// Immediately resolve the geometry of a drawable object.
    #[inline]
    pub fn get_drawable_geometry_immediate<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> crate::Result<Geometry> {
        let tok = self.get_drawable_geometry(target)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Copy pixels from one area of the drawable to another.
    #[inline]
    pub fn copy_area<Source: Into<Drawable>, Destination: Into<Drawable>>(
        &mut self,
        source: Source,
        destination: Destination,
        gc: Gcontext,
        src_x: i16,
        src_y: i16,
        width: u16,
        height: u16,
        dest_x: i16,
        dest_y: i16,
    ) -> crate::Result {
        sr_request!(
            self,
            copy_area_request(
                source.into(),
                destination.into(),
                gc,
                src_x,
                src_y,
                width,
                height,
                dest_x,
                dest_y,
            )
        )
    }

    /// Copy a plane from one drawable to another.
    #[inline]
    pub fn copy_plane<Source: Into<Drawable>, Destination: Into<Drawable>>(
        &mut self,
        source: Source,
        destination: Destination,
        gc: Gcontext,
        src_x: i16,
        src_y: i16,
        width: u16,
        height: u16,
        dest_x: i16,
        dest_y: i16,
        bit_plane: u32,
    ) -> crate::Result {
        sr_request!(
            self,
            copy_plane_request(
                source.into(),
                destination.into(),
                gc,
                src_x,
                src_y,
                width,
                height,
                dest_x,
                dest_y,
                bit_plane,
            )
        )
    }

    /// Create a new pixmap.
    #[inline]
    pub fn create_pixmap<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        width: u16,
        height: u16,
        depth: u8,
    ) -> crate::Result<Pixmap> {
        let pixmap = Pixmap::const_from_xid(self.generate_xid()?);
        sr_request!(
            self,
            create_pixmap_request(target.into(), pixmap, width, height, depth)
        )?;
        Ok(pixmap)
    }

    /// Write an image to a drawable.
    #[inline]
    pub fn put_image<Target: Into<Drawable>, Data: Deref<Target = [u8]>>(
        &mut self,
        target: Target,
        gc: Gcontext,
        image: &Image<Data>,
        src_x: isize,
        src_y: isize,
        dest_x: isize,
        dest_y: isize,
        width: usize,
        height: usize,
    ) -> crate::Result<()> {
        let reqs = put_image_req(
            self,
            target.into(),
            gc,
            image,
            src_x,
            src_y,
            dest_x,
            dest_y,
            width,
            height,
        );
        let toks = reqs
            .into_iter()
            .map(|r| send_request!(self, r))
            .collect::<crate::Result<Vec<_>>>()?;

        toks.into_iter().try_for_each(|t| self.resolve_request(t))
    }

    /// Create a pixmap from an image.
    #[inline]
    pub fn create_pixmap_from_image<Target: Clone + Into<Drawable>, Data: Deref<Target = [u8]>>(
        &mut self,
        target: Target,
        image: &Image<Data>,
    ) -> crate::Result<Pixmap> {
        let pixmap = self.create_pixmap(
            target.clone(),
            image.width.try_into().unwrap(),
            image.height.try_into().unwrap(),
            image.depth,
        )?;
        let gc = match self.create_gc(target.clone(), Default::default()) {
            Ok(gc) => gc,
            Err(e) => {
                pixmap.free(self)?;
                return Err(e);
            }
        };

        self.put_image(target, gc, image, 0, 0, 0, 0, image.width, image.height)?;
        gc.free(self)?;
        Ok(pixmap)
    }
}

#[cfg(feature = "async")]
impl<Conn: AsyncConnection + Send> Display<Conn> {
    /// Get the geometry of a drawable object, async redox.
    #[inline]
    pub async fn get_drawable_geometry_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        send_request!(self, get_geometry_request(target.into()), async).await
    }

    /// Immediately resolve the geometry of a drawable object, async redox.
    #[inline]
    pub async fn get_drawable_geometry_immediate_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> crate::Result<Geometry> {
        let tok = self.get_drawable_geometry_async(target).await?;
        Ok(self.resolve_request_async(tok).await?.into())
    }

    /// Copy pixels from one area of the drawable to another, async redox.
    #[inline]
    pub async fn copy_area_async<Source: Into<Drawable>, Destination: Into<Drawable>>(
        &mut self,
        source: Source,
        destination: Destination,
        gc: Gcontext,
        src_x: i16,
        src_y: i16,
        width: u16,
        height: u16,
        dest_x: i16,
        dest_y: i16,
    ) -> crate::Result {
        sr_request!(
            self,
            copy_area_request(
                source.into(),
                destination.into(),
                gc,
                src_x,
                src_y,
                width,
                height,
                dest_x,
                dest_y,
            ),
            async
        )
        .await
    }

    /// Copy a plane from one drawable to another, async redox.
    #[inline]
    pub async fn copy_plane_async<Source: Into<Drawable>, Destination: Into<Drawable>>(
        &mut self,
        source: Source,
        destination: Destination,
        gc: Gcontext,
        src_x: i16,
        src_y: i16,
        width: u16,
        height: u16,
        dest_x: i16,
        dest_y: i16,
        bit_plane: u32,
    ) -> crate::Result {
        sr_request!(
            self,
            copy_plane_request(
                source.into(),
                destination.into(),
                gc,
                src_x,
                src_y,
                width,
                height,
                dest_x,
                dest_y,
                bit_plane,
            ),
            async
        )
        .await
    }

    /// Create a new pixmap, async redox.
    #[inline]
    pub async fn create_pixmap_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        width: u16,
        height: u16,
        depth: u8,
    ) -> crate::Result<Pixmap> {
        let pixmap = Pixmap::const_from_xid(self.generate_xid()?);
        sr_request!(
            self,
            create_pixmap_request(target.into(), pixmap, width, height, depth),
            async
        )
        .await?;
        Ok(pixmap)
    }

    /// Write an image to a drawable, async redox.
    #[inline]
    pub async fn put_image_async<Target: Into<Drawable>, Data: Deref<Target = [u8]>>(
        &mut self,
        target: Target,
        gc: Gcontext,
        image: &Image<Data>,
        src_x: isize,
        src_y: isize,
        dest_x: isize,
        dest_y: isize,
        width: usize,
        height: usize,
    ) -> crate::Result<()> {
        let reqs = put_image_req(
            self,
            target.into(),
            gc,
            image,
            src_x,
            src_y,
            dest_x,
            dest_y,
            width,
            height,
        );

        // TODO: use streams to do this once async closures are stable
        let mut toks = Vec::with_capacity(reqs.len());
        for req in reqs {
            toks.push(send_request!(self, req, async).await?);
        }

        for tok in toks {
            self.resolve_request_async(tok).await?;
        }

        Ok(())
    }

    /// Create a pixmap from an image, async redox.
    #[inline]
    pub async fn create_pixmap_from_image_async<
        Target: Clone + Into<Drawable>,
        Data: Deref<Target = [u8]>,
    >(
        &mut self,
        target: Target,
        image: &Image<Data>,
    ) -> crate::Result<Pixmap> {
        let pixmap = self
            .create_pixmap_async(
                target.clone(),
                image.width.try_into().unwrap(),
                image.height.try_into().unwrap(),
                image.depth,
            )
            .await?;
        let gc = match self
            .create_gc_async(target.clone(), Default::default())
            .await
        {
            Ok(gc) => gc,
            Err(e) => {
                pixmap.free_async(self).await?;
                return Err(e);
            }
        };

        self.put_image_async(target, gc, image, 0, 0, 0, 0, image.width, image.height)
            .await?;
        gc.free_async(self).await?;
        Ok(pixmap)
    }
}
