/* -----------------------------------------------------------------------------------
 * src/drawable.rs - Base trait for items that can be drawed upon. They are expected
 *                   to hold references to their own GCs. This file also defines a
 *                   wrapper for the GC struct.
 * beetle - Simple graphics framework for Rust
 * Copyright © 2020 not_a_seagull
 *
 * This project is licensed under either the Apache 2.0 license or the MIT license, at
 * your option. For more information, please consult the LICENSE-APACHE or LICENSE-MIT
 * files in the repository root.
 * -----------------------------------------------------------------------------------
 * MIT License:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 * -----------------------------------------------------------------------------------
 * Apache 2.0 License Declaration:
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ----------------------------------------------------------------------------------
 */

use super::{
    Color, ColorMap, DisplayReference, DroppableObject, FlutterbugError, GenericDisplay,
    GenericImage, HasXID, Image, Pixmap,
};
use alloc::{
    sync::{Arc, Weak},
    vec::Vec,
};
use core::{fmt, mem, ptr::NonNull};
use cstr_core::CString;
use cty::{c_int, c_short, c_uint, c_ulong, c_ushort};
use euclid::default::{Point2D, Size2D};
use x11::xlib::{self, _XGC};

bitflags::bitflags! {
    pub struct GraphicsContextValues : c_uint {
        const FUNCTION = xlib::GCFunction;
        const PLANE_MASK = xlib::GCPlaneMask;
        const FOREGROUND = xlib::GCForeground;
        const BACKGROUND = xlib::GCBackground;
        const LINE_WIDTH = xlib::GCLineWidth;
        const LINE_STYLE = xlib::GCLineStyle;
        const CAP_STYLE = xlib::GCCapStyle;
        const JOIN_STYLE = xlib::GCJoinStyle;
        const FILL_STYLE = xlib::GCFillStyle;
        const FILL_RULE = xlib::GCFillRule;
        const TILE = xlib::GCTile;
        const STIPPLE = xlib::GCStipple;
        const TILE_STRIP_X_ORIGIN = xlib::GCTileStipXOrigin;
        const TILE_STRIP_Y_ORIGIN = xlib::GCTileStipYOrigin;
        const FONT = xlib::GCFont;
        const SUBWINDOW_MODE = xlib::GCSubwindowMode;
        const GRAPHICS_EXPOSURES = xlib::GCGraphicsExposures;
        const CLIP_X_ORIGIN = xlib::GCClipXOrigin;
        const CLIP_Y_ORIGIN = xlib::GCClipYOrigin;
        const CLIP_MASK = xlib::GCClipMask;
        const DASH_OFFSET = xlib::GCDashOffset;
        const DASH_LIST = xlib::GCDashList;
        const ARC_MODE = xlib::GCArcMode;
    }
}

/// A graphics context.
pub struct GraphicsContext {
    raw: Arc<NonNull<_XGC>>,
    dpy: DisplayReference,
    is_default: bool,
}

impl fmt::Debug for GraphicsContext {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}X11 Graphics Context",
            if self.is_default { "Default " } else { "" }
        )
    }
}

impl Clone for GraphicsContext {
    fn clone(&self) -> Self {
        Self::from_raw(self.raw.clone(), self.dpy.clone(), self.is_default)
    }
}

impl Drop for GraphicsContext {
    fn drop(&mut self) {
        // we shouldn't call XFreeGC if we're using the default context
        if !self.is_default {
            if let Ok(mut d) = self.dpy.raw() {
                unsafe { xlib::XFreeGC(d.as_mut(), self.raw.as_ptr()) };
            }
        }
    }
}

impl GraphicsContext {
    #[inline]
    pub(crate) fn from_raw(
        raw: Arc<NonNull<_XGC>>,
        dpy: DisplayReference,
        is_default: bool,
    ) -> Self {
        Self {
            raw,
            dpy,
            is_default,
        }
    }
}

/// A reference to a graphics context.
pub struct GraphicsContextReference {
    raw: Weak<NonNull<_XGC>>,
    dpy: DisplayReference,
    is_default: bool,
}

impl PartialEq for GraphicsContextReference {
    fn eq(&self, other: &Self) -> bool {
        self.raw.ptr_eq(&other.raw)
    }
}

impl fmt::Debug for GraphicsContextReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}X11 Graphics Context Reference",
            if self.is_default { "Default " } else { "" }
        )
    }
}

impl Clone for GraphicsContextReference {
    fn clone(&self) -> Self {
        Self::from_raw(self.raw.clone(), self.dpy.clone(), self.is_default)
    }
}

impl GraphicsContextReference {
    #[inline]
    pub(crate) fn from_raw(
        raw: Weak<NonNull<_XGC>>,
        dpy: DisplayReference,
        is_default: bool,
    ) -> Self {
        Self {
            raw,
            dpy,
            is_default,
        }
    }
}

/// A trait implemented by both GraphicsContext and GraphicsContextReference,
/// in order to abstract them out.
pub trait GenericGraphicsContext: fmt::Debug + Clone {
    /// Get a reference to this graphics context.
    fn reference(&self) -> GraphicsContextReference;
    /// Get the raw pointer to the graphics context.
    fn raw(&self) -> Result<NonNull<_XGC>, FlutterbugError>;
    /// Tell if this is the default context.
    fn is_default(&self) -> bool;
    /// Get the internal display reference.
    fn dpy(&self) -> DisplayReference;

    /// Get the properties of the graphics context.
    #[inline]
    fn gc_properties(
        &self,
        mask: GraphicsContextValues,
    ) -> Result<xlib::XGCValues, FlutterbugError> {
        let mut xgc: xlib::XGCValues = unsafe { mem::zeroed() };
        unsafe {
            xlib::XGetGCValues(
                self.dpy().raw()?.as_mut(),
                self.raw()?.as_mut(),
                mask.bits() as c_ulong,
                &mut xgc,
            )
        };
        Ok(xgc)
    }
}

impl GenericGraphicsContext for GraphicsContext {
    #[inline]
    fn reference(&self) -> GraphicsContextReference {
        GraphicsContextReference::from_raw(
            Arc::downgrade(&self.raw),
            self.dpy.clone(),
            self.is_default,
        )
    }

    #[inline]
    fn raw(&self) -> Result<NonNull<_XGC>, FlutterbugError> {
        Ok(*self.raw)
    }

    #[inline]
    fn is_default(&self) -> bool {
        self.is_default
    }

    #[inline]
    fn dpy(&self) -> DisplayReference {
        self.dpy.clone()
    }
}

impl GenericGraphicsContext for GraphicsContextReference {
    #[inline]
    fn reference(&self) -> GraphicsContextReference {
        self.clone()
    }

    #[inline]
    fn raw(&self) -> Result<NonNull<_XGC>, FlutterbugError> {
        Ok(*self
            .raw
            .upgrade()
            .ok_or_else(|| FlutterbugError::PointerWasDropped(DroppableObject::GC))?)
    }

    #[inline]
    fn is_default(&self) -> bool {
        self.is_default
    }

    #[inline]
    fn dpy(&self) -> DisplayReference {
        self.dpy.clone()
    }
}

// set a GC function
macro_rules! set_gc_val {
    ($self: ident, $mask: expr, $slot: ident, $value: expr) => {{
        use maybe_uninit::MaybeUninit;

        let mut stat: MaybeUninit<xlib::XGCValues> = MaybeUninit::zeroed();
        unsafe { (*stat.as_mut_ptr()).$slot = $value };

        $self.set_gc_properties($mask, unsafe { stat.assume_init() })
    }};
}

/// The shape of a polygon.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(i32)]
pub enum PolygonShape {
    Convex = xlib::Convex,
    NonConvex = xlib::Nonconvex,
    Complex = xlib::Complex,
}

/// Objects that can be drawed upon.
pub trait Drawable: HasXID + fmt::Debug {
    /// Get a reference to the graphics context that this item has stored.
    fn gc_ref(&self) -> GraphicsContextReference;
    /// Get the display reference stored by the object. By default, this
    /// uses the one stored in the graphics context.
    #[inline]
    fn dpy(&self) -> DisplayReference {
        self.gc_ref().dpy()
    }
    /// Get a reference to the colormap. By default, this gets the colormap used
    /// by the loaded display.
    #[inline]
    fn colormap(&self) -> Result<ColorMap, FlutterbugError> {
        self.dpy().default_colormap()
    }

    /// Get the properties of the graphics context for this drawable.
    #[inline]
    fn gc_properties(
        &self,
        mask: GraphicsContextValues,
    ) -> Result<xlib::XGCValues, FlutterbugError> {
        self.gc_ref().gc_properties(mask)
    }

    /// Set the properties of the graphics context for this drawable.
    #[inline]
    fn set_gc_properties(
        &self,
        mask: GraphicsContextValues,
        mut vals: xlib::XGCValues,
    ) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XChangeGC(
                self.dpy().raw()?.as_mut(),
                self.gc_ref().raw()?.as_mut(),
                mask.bits() as c_ulong,
                &mut vals,
            )
        };
        Ok(())
    }

    /// Set the foreground color for this drawable.
    #[inline]
    fn set_foreground(&self, color: Color) -> Result<(), FlutterbugError> {
        set_gc_val!(
            self,
            GraphicsContextValues::FOREGROUND,
            foreground,
            self.colormap()?.color(color)?.pixel_id()
        )
    }

    /// Set the background color for this drawable.
    #[inline]
    fn set_background(&self, color: Color) -> Result<(), FlutterbugError> {
        set_gc_val!(
            self,
            GraphicsContextValues::BACKGROUND,
            background,
            self.colormap()?.color(color)?.pixel_id()
        )
    }

    /// Set the line width for this drawable.
    #[inline]
    fn set_line_width(&self, width: u32) -> Result<(), FlutterbugError> {
        set_gc_val!(
            self,
            GraphicsContextValues::LINE_WIDTH,
            line_width,
            width as c_int
        )
    }

    /// Create a pixmap object bound to this drawable.
    #[inline]
    fn pixmap(&self, size: Size2D<u32>, depth: u32) -> Result<Pixmap, FlutterbugError> {
        let pixmap = unsafe {
            xlib::XCreatePixmap(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                size.width,
                size.height,
                depth,
            )
        };
        Ok(Pixmap::from_raw(pixmap, self.dpy(), self.gc_ref()))
    }

    /// Draw a string on this object.
    fn draw_string(&self, origin: Point2D<i32>, text: &str) -> Result<(), FlutterbugError> {
        let gc = self.gc_ref();
        let tlen = text.len();
        let cstr = unsafe { super::to_cstring(text)? };

        unsafe {
            xlib::XDrawString(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                gc.raw()?.as_mut(),
                origin.x as c_int,
                origin.y as c_int,
                cstr,
                tlen as c_int,
            )
        };

        // make sure to deallocate the cstring
        let _ = unsafe { CString::from_raw(cstr) };

        Ok(())
    }

    /// Copy an area to this drawable from another drawable.
    fn copy_area(
        &self,
        origin: &dyn Drawable,
        origin_pt: Point2D<i32>,
        dest_pt: Point2D<i32>,
        size: Size2D<u32>,
    ) -> Result<(), FlutterbugError> {
        // assure that the GCRefs refer to the same GC
        let gc = self.gc_ref();
        if gc != origin.gc_ref() {
            return Err(FlutterbugError::GCMustEqual);
        }

        unsafe {
            xlib::XCopyArea(
                self.dpy().raw()?.as_mut(),
                origin.xid(),
                self.xid(),
                gc.raw()?.as_mut(),
                origin_pt.x as c_int,
                origin_pt.y as c_int,
                size.width as c_uint,
                size.height as c_uint,
                dest_pt.x as c_int,
                dest_pt.y as c_int,
            )
        };
        Ok(())
    }

    /// Draw a line.
    fn draw_line(&self, point1: Point2D<i32>, point2: Point2D<i32>) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XDrawLine(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                point1.x,
                point1.y,
                point2.x,
                point2.y,
            )
        };
        Ok(())
    }

    /// Draw a point.
    fn draw_point(&self, point: Point2D<i32>) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XDrawPoint(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                point.x,
                point.y,
            )
        };
        Ok(())
    }

    /// Draw several lines.
    fn draw_lines(&self, lines: &[(Point2D<i32>, Point2D<i32>)]) -> Result<(), FlutterbugError> {
        let mut points = Vec::with_capacity(lines.len() * 2);
        lines.iter().for_each(|(i, j)| {
            points.push(xlib::XPoint {
                x: i.x as c_short,
                y: i.y as c_short,
            });
            points.push(xlib::XPoint {
                x: j.x as c_short,
                y: j.y as c_short,
            });
        });

        unsafe {
            xlib::XDrawLines(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                points.as_mut_ptr(),
                points.len() as c_int,
                xlib::CoordModeOrigin,
            )
        };
        Ok(())
    }

    /// Draw several points.
    fn draw_points(&self, points: &[Point2D<i32>]) -> Result<(), FlutterbugError> {
        let mut xpoints = Vec::with_capacity(points.len());
        points.iter().for_each(|i| {
            xpoints.push(xlib::XPoint {
                x: i.x as c_short,
                y: i.y as c_short,
            })
        });

        unsafe {
            xlib::XDrawPoints(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                xpoints.as_mut_ptr(),
                xpoints.len() as c_int,
                xlib::CoordModeOrigin,
            )
        };
        Ok(())
    }

    /// Get an image from a rectangle on this drawable.
    fn image(&self, origin: Point2D<i32>, size: Size2D<u32>) -> Result<Image, FlutterbugError> {
        let img = unsafe {
            xlib::XGetImage(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                origin.x,
                origin.y,
                size.width,
                size.height,
                0,
                xlib::ZPixmap,
            )
        };
        let img = NonNull::new(img).ok_or_else(|| FlutterbugError::ImageWasNull)?;
        Ok(Image::from_raw(Arc::new(img)))
    }

    /// Draw an image onto this drawable.
    fn put_image(
        &self,
        image: &dyn GenericImage,
        src_origin: Point2D<i32>,
        dest_origin: Point2D<i32>,
        size: Size2D<u32>,
    ) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XPutImage(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                image.raw()?.as_mut(),
                src_origin.x,
                src_origin.y,
                dest_origin.x,
                dest_origin.y,
                size.width,
                size.height,
            )
        };
        Ok(())
    }

    /// Draw a rectangle onto this drawable.
    fn draw_rectangle(
        &self,
        origin: Point2D<i32>,
        size: Size2D<u32>,
    ) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XDrawRectangle(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                origin.x,
                origin.y,
                size.width,
                size.height,
            )
        };
        Ok(())
    }

    /// Draw an arc onto this drawable.
    fn draw_arc(
        &self,
        origin: Point2D<i32>,
        size: Size2D<u32>,
        angles: (i32, i32),
    ) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XDrawArc(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                origin.x,
                origin.y,
                size.width,
                size.height,
                angles.0,
                angles.1,
            )
        };
        Ok(())
    }

    /// Draw a series of rectangles onto this drawable.
    fn draw_rectangles(
        &self,
        rects: &[(Point2D<i32>, Size2D<u32>)],
    ) -> Result<(), FlutterbugError> {
        let mut xrects = Vec::with_capacity(rects.len());
        rects.iter().for_each(|(o, s)| {
            xrects.push(xlib::XRectangle {
                x: o.x as c_short,
                y: o.y as c_short,
                width: s.width as c_ushort,
                height: s.height as c_ushort,
            })
        });

        unsafe {
            xlib::XDrawRectangles(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                xrects.as_mut_ptr(),
                xrects.len() as c_int,
            )
        };
        Ok(())
    }

    /// Draw a series of arcs onto this drawable.
    fn draw_arcs(
        &self,
        arcs: &[(Point2D<i32>, Size2D<u32>, (i32, i32))],
    ) -> Result<(), FlutterbugError> {
        let mut xarcs = Vec::with_capacity(arcs.len());
        arcs.iter().for_each(|(o, s, (a1, a2))| {
            xarcs.push(xlib::XArc {
                x: o.x as c_short,
                y: o.y as c_short,
                width: s.width as c_ushort,
                height: s.height as c_ushort,
                angle1: *a1 as c_short,
                angle2: *a2 as c_short,
            })
        });

        unsafe {
            xlib::XDrawArcs(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                xarcs.as_mut_ptr(),
                xarcs.len() as c_int,
            )
        };
        Ok(())
    }

    /// Fill a rectangle in this drawable.
    fn fill_rectangle(
        &self,
        origin: Point2D<i32>,
        size: Size2D<u32>,
    ) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XFillRectangle(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                origin.x,
                origin.y,
                size.width,
                size.height,
            )
        };
        Ok(())
    }

    /// Fill a polygon in this drawable.
    fn fill_polygon(
        &self,
        points: &[Point2D<i32>],
        shape: PolygonShape,
    ) -> Result<(), FlutterbugError> {
        let mut pts = Vec::with_capacity(points.len());
        points.iter().for_each(|p| {
            pts.push(xlib::XPoint {
                x: p.x as c_short,
                y: p.y as c_short,
            })
        });

        unsafe {
            xlib::XFillPolygon(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                pts.as_mut_ptr(),
                pts.len() as c_int,
                shape as c_int,
                xlib::CoordModeOrigin,
            )
        };
        Ok(())
    }

    /// Fill an arc in this drawable.
    fn fill_arc(
        &self,
        origin: Point2D<i32>,
        size: Size2D<u32>,
        angles: (i32, i32),
    ) -> Result<(), FlutterbugError> {
        unsafe {
            xlib::XFillArc(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                origin.x,
                origin.y,
                size.width,
                size.height,
                angles.0,
                angles.1,
            )
        };
        Ok(())
    }

    /// Fill several rectangles in this drawable.
    fn fill_rectangles(
        &self,
        rects: &[(Point2D<i32>, Size2D<u32>)],
    ) -> Result<(), FlutterbugError> {
        let mut rcts = Vec::with_capacity(rects.len());
        rects.iter().for_each(|(o, s)| {
            rcts.push(xlib::XRectangle {
                x: o.x as c_short,
                y: o.y as c_short,
                width: s.width as c_ushort,
                height: s.height as c_ushort,
            })
        });

        unsafe {
            xlib::XFillRectangles(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                rcts.as_mut_ptr(),
                rcts.len() as c_int,
            )
        };
        Ok(())
    }

    /// Fill several arcs in this drawable.
    fn fill_arcs(
        &self,
        arcs: &[(Point2D<i32>, Size2D<u32>, (i32, i32))],
    ) -> Result<(), FlutterbugError> {
        let mut xarcs = Vec::with_capacity(arcs.len());
        arcs.iter().for_each(|(o, s, (a1, a2))| {
            xarcs.push(xlib::XArc {
                x: o.x as c_short,
                y: o.y as c_short,
                width: s.width as c_ushort,
                height: s.height as c_ushort,
                angle1: *a1 as c_short,
                angle2: *a2 as c_short,
            })
        });

        unsafe {
            xlib::XFillArcs(
                self.dpy().raw()?.as_mut(),
                self.xid(),
                self.gc_ref().raw()?.as_mut(),
                xarcs.as_mut_ptr(),
                xarcs.len() as c_int,
            )
        };
        Ok(())
    }
}
