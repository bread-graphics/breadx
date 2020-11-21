// MIT/Apache2 License

//! This module contains functions that can be applied to nearly all drawable objects, such as Windows
//! and Pixmaps.

use crate::{
    auto::xproto::{Drawable, GetGeometryReply, GetGeometryRequest, Window},
    Connection, Display, RequestCookie,
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
