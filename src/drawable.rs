// MIT/Apache2 License

//! This module contains functions that can be applied to nearly all drawable objects, such as Windows
//! and Pixmaps.

use crate::{
    auto::xproto::{
        CopyAreaRequest, CopyPlaneRequest, CreatePixmapRequest, Drawable, GetGeometryReply,
        GetGeometryRequest, Pixmap, Window,
    },
    Connection, Display, Gcontext, RequestCookie,
};

/// The return type of `drawable::get_geometry_immediate`.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
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
        let mut wg: Self = Default::default();
        wg.depth = ggr.depth;
        wg.root = ggr.root;
        wg.x = ggr.x;
        wg.y = ggr.y;
        wg.width = ggr.width;
        wg.height = ggr.height;
        wg.border_width = ggr.border_width;
        wg
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
