// MIT/Apache2 License

//! This module contains functions that can be applied to nearly all drawable objects, such as Windows
//! and Pixmaps.

use crate::{
    auto::xproto::{
        CopyAreaRequest, CopyPlaneRequest, CreatePixmapRequest, Drawable, GetGeometryReply,
        GetGeometryRequest, GetImageReply, GetImageRequest, ImageFormat, Pixmap, Window,
    },
    image::{put::put_image_req, Image},
    Connection, Display, Gcontext, RequestCookie,
};
use alloc::vec::Vec;
use core::{convert::TryInto, ops::Deref};

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

/// Get the geometry of a drawable object.
#[inline]
pub fn get_geometry<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    target: Target,
) -> crate::Result<RequestCookie<GetGeometryRequest>> {
    let ggr = get_geometry_request(target.into());
    log::debug!("Sending GetGeometryRequest to server.");
    let tok = dpy.send_request(ggr)?;
    log::debug!("Sent GetGeometryRequest to server.");
    Ok(tok)
}

/// Get the geometry of a drawable object, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn get_geometry_async<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    target: Target,
) -> crate::Result<RequestCookie<GetGeometryRequest>> {
    let ggr = get_geometry_request(target.into());
    log::debug!("Sending GetGeometryRequest to server.");
    let tok = dpy.send_request_async(ggr).await?;
    log::debug!("Sent GetGeometryRequest to server.");
    Ok(tok)
}

/// Immediately resolve the geometry of a drawable object.
#[inline]
pub fn get_geometry_immediate<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    target: Target,
) -> crate::Result<Geometry> {
    let tok = get_geometry(dpy, target)?;
    Ok(dpy.resolve_request(tok)?.into())
}

/// Immediately resolve the geometry of a drawable object, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn get_geometry_immediate_async<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    target: Target,
) -> crate::Result<Geometry> {
    let tok = get_geometry_async(dpy, target).await?;
    Ok(dpy.resolve_request_async(tok).await?.into())
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

/// Copy pixels from one area of the drawable to another.
#[inline]
pub fn copy_area<Conn: Connection, Source: Into<Drawable>, Destination: Into<Drawable>>(
    dpy: &mut Display<Conn>,
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
    let car = copy_area_request(
        source.into(),
        destination.into(),
        gc,
        src_x,
        src_y,
        width,
        height,
        dest_x,
        dest_y,
    );
    log::debug!("Sending CopyAreaRequest to server.");
    let tok = dpy.send_request(car)?;
    log::debug!("Sent CopyAreaRequest to server.");
    dpy.resolve_request(tok)
}

/// Copy pixels from one area of the drawable to another, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn copy_area_async<
    Conn: Connection,
    Source: Into<Drawable>,
    Destination: Into<Drawable>,
>(
    dpy: &mut Display<Conn>,
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
    let car = copy_area_request(
        source.into(),
        destination.into(),
        gc,
        src_x,
        src_y,
        width,
        height,
        dest_x,
        dest_y,
    );
    log::debug!("Sending CopyAreaRequest to server.");
    let tok = dpy.send_request_async(car).await?;
    log::debug!("Sent CopyAreaRequest to server.");
    dpy.resolve_request_async(tok).await
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

/// Copy a plane from one drawable to another.
#[inline]
pub fn copy_plane<Conn: Connection, Source: Into<Drawable>, Destination: Into<Drawable>>(
    dpy: &mut Display<Conn>,
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
    let cpr = copy_plane_request(
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
    );
    log::debug!("Sending CopyPlaneRequest to server.");
    let tok = dpy.send_request(cpr)?;
    log::debug!("Sent CopyPlaneRequest to server.");
    dpy.resolve_request(tok)
}

/// Copy a plane from one drawable to another, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn copy_plane_async<
    Conn: Connection,
    Source: Into<Drawable>,
    Destination: Into<Drawable>,
>(
    dpy: &mut Display<Conn>,
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
    let cpr = copy_plane_request(
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
    );
    log::debug!("Sending CopyPlaneRequest to server.");
    let tok = dpy.send_request_async(cpr).await?;
    log::debug!("Sent CopyPlaneRequest to server.");
    dpy.resolve_request_async(tok).await
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

/// Create a new pixmap.
#[inline]
pub fn create_pixmap<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    target: Target,
    width: u16,
    height: u16,
    depth: u8,
) -> crate::Result<Pixmap> {
    let pixmap = Pixmap::const_from_xid(dpy.generate_xid()?);
    let cpr = create_pixmap_request(target.into(), pixmap, width, height, depth);
    log::debug!("Sending CreatePixmapRequest to server.");
    let tok = dpy.send_request(cpr)?;
    log::debug!("Sent CreatePixmapRequest to server.");
    dpy.resolve_request(tok)?;
    Ok(pixmap)
}

/// Create a new pixmap, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn create_pixmap_async<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    target: Target,
    width: u16,
    height: u16,
    depth: u8,
) -> crate::Result<Pixmap> {
    let pixmap = Pixmap::const_from_xid(dpy.generate_xid()?);
    let cpr = create_pixmap_request(target.into(), pixmap, width, height, depth);
    log::debug!("Sending CreatePixmapRequest to server.");
    let tok = dpy.send_request_async(cpr).await?;
    log::debug!("Sent CreatePixmapRequest to server.");
    dpy.resolve_request_async(tok).await?;
    Ok(pixmap)
}

#[inline]
fn get_image_req(
    drawable: Drawable,
    x: isize,
    y: isize,
    width: usize,
    height: usize,
    plane_mask: usize,
    format: ImageFormat,
) -> GetImageRequest {
    GetImageRequest {
        drawable,
        x: x as _,
        y: y as _,
        width: width as _,
        height: height as _,
        plane_mask: plane_mask as _,
        format,
        ..Default::default()
    }
}

#[inline]
fn create_image_from_get_image_reply<Conn: Connection>(
    dpy: &mut Display<Conn>,
    width: usize,
    height: usize,
    plane_mask: usize,
    format: ImageFormat,
    reply: GetImageReply,
) -> Image<Vec<u8>> {
    if format == ImageFormat::XyPixmap {
        let depth = (plane_mask & (0xffff_ffff >> (32 - reply.depth))).count_ones();

        let image = Image::new(
            dpy,
            dpy.visual_id_to_visual(reply.visual),
            depth as _,
            format,
            0,
            reply.data,
            width,
            height,
            u32::from(dpy.setup.bitmap_format_scanline_pad),
            None,
        )
        .unwrap();

        image
    } else {
        let image = Image::new(
            dpy,
            dpy.visual_id_to_visual(reply.visual),
            reply.depth as _,
            ImageFormat::ZPixmap,
            0,
            reply.data,
            width,
            height,
            dpy.get_scanline_pad(reply.depth as _) as _,
            None,
        )
        .unwrap();

        image
    }
}

// Get an image from a drawable.
#[inline]
pub fn get_image<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    drawable: Target,
    x: isize,
    y: isize,
    width: usize,
    height: usize,
    plane_mask: usize,
    format: ImageFormat,
) -> crate::Result<Image<Vec<u8>>> {
    let req = get_image_req(drawable.into(), x, y, width, height, plane_mask, format);

    log::debug!("Sending GetImageRequest to server.");
    let tok = dpy.send_request(req)?;
    log::debug!("Sent GetImageRequest to server.");
    let reply = dpy.resolve_request(tok)?;

    Ok(create_image_from_get_image_reply(
        dpy, width, height, plane_mask, format, reply,
    ))
}

// Get an image from a drawable, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn get_image_async<Conn: Connection, Target: Into<Drawable>>(
    dpy: &mut Display<Conn>,
    drawable: Target,
    x: isize,
    y: isize,
    width: usize,
    height: usize,
    plane_mask: usize,
    format: ImageFormat,
) -> crate::Result<Image<Vec<u8>>> {
    let req = get_image_req(drawable.into(), x, y, width, height, plane_mask, format);

    log::debug!("Sending GetImageRequest to server.");
    let tok = dpy.send_request_async(req).await?;
    log::debug!("Sent GetImageRequest to server.");
    let reply = dpy.resolve_request_async(tok).await?;

    Ok(create_image_from_get_image_reply(
        dpy, width, height, plane_mask, format, reply,
    ))
}

/// Write an image to a drawable.
#[inline]
pub fn put_image<Conn: Connection, Target: Into<Drawable>, Data: Deref<Target = [u8]>>(
    dpy: &mut Display<Conn>,
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
        dpy,
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
        .map(|r| {
            log::debug!("Sending PutImageRequest to server.");
            let tok = dpy.send_request(r)?;
            log::debug!("Sent PutImageRequest to server.");
            Ok(tok)
        })
        .collect::<crate::Result<Vec<_>>>()?;

    toks.into_iter().try_for_each(|t| dpy.resolve_request(t))
}

/// Write an image to a drawable, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn put_image_async<
    Conn: Connection,
    Target: Into<Drawable>,
    Data: Deref<Target = [u8]>,
>(
    dpy: &mut Display<Conn>,
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
        dpy,
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
    let mut toks = Vec::with_capacity(reqs.len());
    for req in reqs {
        log::debug!("Sending PutImageRequest to server.");
        let tok = dpy.send_request_async(req).await?;
        log::debug!("Sent PutImageRequest to server.");
        toks.push(tok);
    }

    for tok in toks {
        dpy.resolve_request_async(tok).await?;
    }

    Ok(())
}

/// Create a pixmap from an image.
#[inline]
pub fn create_pixmap_from_image<
    Conn: Connection,
    Target: Clone + Into<Drawable>,
    Data: Deref<Target = [u8]>,
>(
    dpy: &mut Display<Conn>,
    target: Target,
    image: &Image<Data>,
) -> crate::Result<Pixmap> {
    let pixmap = create_pixmap(
        dpy,
        target.clone(),
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.depth,
    )?;
    let gc = match dpy.create_gc(target.clone(), Default::default()) {
        Ok(gc) => gc,
        Err(e) => {
            pixmap.free(dpy)?;
            return Err(e);
        }
    };

    put_image(
        dpy,
        target,
        gc,
        image,
        0,
        0,
        0,
        0,
        image.width,
        image.height,
    )?;
    gc.free(dpy)?;
    Ok(pixmap)
}

/// Create a pixmap from an image, async redox.
#[cfg(feature = "async")]
#[inline]
pub async fn create_pixmap_from_image_async<
    Conn: Connection,
    Target: Clone + Into<Drawable>,
    Data: Deref<Target = [u8]>,
>(
    dpy: &mut Display<Conn>,
    target: Target,
    image: &Image<Data>,
) -> crate::Result<Pixmap> {
    let pixmap = create_pixmap_async(
        dpy,
        target.clone(),
        image.width.try_into().unwrap(),
        image.height.try_into().unwrap(),
        image.depth,
    )
    .await?;
    let gc = match dpy
        .create_gc_async(target.clone(), Default::default())
        .await
    {
        Ok(gc) => gc,
        Err(e) => {
            pixmap.free_async(dpy).await?;
            return Err(e);
        }
    };

    put_image_async(
        dpy,
        target,
        gc,
        image,
        0,
        0,
        0,
        0,
        image.width,
        image.height,
    )
    .await?;
    gc.free_async(dpy).await?;
    Ok(pixmap)
}
