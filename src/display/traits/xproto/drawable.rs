// MIT/Apache2 License

//! This module contains functions that can be applied to nearly all drawable objects, such as Windows
//! and Pixmaps.

use crate::{
    auto::xproto::{
        CopyAreaRequest, CopyPlaneRequest, CreatePixmapRequest, Drawable, GetGeometryReply,
        GetGeometryRequest, Pixmap, PutImageRequest, Window,
    },
    display::{generate_xid, prelude::*},
    image::{put::put_image_req, Image},
    Connection, Display, Gcontext, RequestCookie,
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
        depth,
        pid,
        drawable,
        width,
        height,
        ..Default::default()
    }
}

pub trait DisplayDrawableExt<'a>: Display<'a> {
    /// Get the geometry of a drawable object.
    #[inline]
    fn get_drawable_geometry<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        self.send_request(get_geometry_request(target.into()))
    }

    /// Immediately resolve the geometry of a drawable object.
    #[inline]
    fn get_drawable_geometry_immediate<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> crate::Result<Geometry> {
        let tok = self.get_drawable_geometry(target)?;
        Ok(self.resolve_request(tok)?.into())
    }

    /// Copy pixels from one area of the drawable to another.
    #[inline]
    fn copy_area<Source: Into<Drawable>, Destination: Into<Drawable>>(
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
        self.exchange_request(copy_area_request(
            source.into(),
            destination.into(),
            gc,
            src_x,
            src_y,
            width,
            height,
            dest_x,
            dest_y,
        ))
    }

    /// Copy a plane from one drawable to another.
    #[inline]
    fn copy_plane<Source: Into<Drawable>, Destination: Into<Drawable>>(
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
        self.exchange_request(copy_plane_request(
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
        ))
    }

    /// Create a new pixmap.
    #[inline]
    fn create_pixmap<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        width: u16,
        height: u16,
        depth: u8,
    ) -> crate::Result<Pixmap> {
        let pixmap = Pixmap::const_from_xid(generate_xid(self)?);
        self.exchange_request(create_pixmap_request(
            target.into(),
            pixmap,
            width,
            height,
            depth,
        ))?;
        Ok(pixmap)
    }

    /// Write an image to a drawable.
    #[inline]
    fn put_image<Target: Into<Drawable>, Data: Deref<Target = [u8]>>(
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
            .map(|r| self.send_request(r))
            .collect::<crate::Result<Vec<_>>>()?;

        toks.into_iter().try_for_each(|t| self.resolve_request(t))
    }

    /// Create a pixmap from an image.
    #[inline]
    fn create_pixmap_from_image<Target: Clone + Into<Drawable>, Data: Deref<Target = [u8]>>(
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

impl<'a, D: Display<'a> + ?Sized> DisplayDrawableExt<'a> for D {}

#[cfg(feature = "async")]
pub trait AsyncDisplayDrawableExt: AsyncDisplay {
    /// Get the geometry of a drawable object, async redox.
    #[inline]
    fn get_drawable_geometry_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> SendRequestFuture<'_, Self, GetGeometryRequest> {
        self.send_request_async(get_geometry_request(target.into()))
    }

    /// Immediately resolve the geometry of a drawable object, async redox.
    #[inline]
    fn get_drawable_geometry_immediate_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, GetGeometryRequest>,
        fn(crate::Result<GetGeometryReply>) -> crate::Result<Geometry>,
    > {
        MapFuture::run(
            self.exchange_future_async(get_geometry_request(target.into())),
            |repl| repl.map(Geometry::from),
        )
    }

    /// Copy pixels from one area of the drawable to another, async redox.
    #[inline]
    fn copy_area_async<Source: Into<Drawable>, Destination: Into<Drawable>>(
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
    ) -> ExchangeRequestFuture<'_, Self, CopyAreaRequest> {
        self.exchange_request_async(copy_area_request(
            source.into(),
            destination.into(),
            gc,
            src_x,
            src_y,
            width,
            height,
            dest_x,
            dest_y,
        ))
    }

    /// Copy a plane from one drawable to another, async redox.
    #[inline]
    fn copy_plane_async<Source: Into<Drawable>, Destination: Into<Drawable>>(
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
    ) -> ExchangeRequestFuture<'_, Self, CopyPlaneRequest> {
        self.exchange_request_async(copy_plane_request(
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
        ))
    }

    /// Create a new pixmap, async redox.
    #[inline]
    fn create_pixmap_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        width: u16,
        height: u16,
        depth: u8,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        CreatePixmapRequest,
        Pixmap,
        BoxedFnOnce<Pixmap, CreatePixmapRequest>,
    > {
        let mut cpr = create_pixmap_request(
            target.into(),
            Pixmap::const_from_xid(0),
            width,
            height,
            depth,
        );
        self.exchange_xid_future(Box::new(move |pid| {
            cpr.pid = pid;
            cpr
        }))
    }

    /// Write an image to a drawable, async redox.
    #[inline]
    fn put_image_async<Target: Into<Drawable>, Data: Deref<Target = [u8]>>(
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
    ) -> PutImageFuture<'_, Self, Vec<PutImageRequest>> {
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

        PutImageFuture::run(self, reqs)
    }

    // TODO: too lazy to fix this
    /*
    /// Create a pixmap from an image, async redox.
    #[inline]
    fn create_pixmap_from_image_async<
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
    */
}
