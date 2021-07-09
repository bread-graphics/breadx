// MIT/Apache2 License

use crate::{
    auto::{
        render::{
            ChangePictureRequest, Color, CompositeRequest, Cp, CreatePictureRequest,
            FillRectanglesRequest, FreePictureRequest, PictOp, Pictformat, Picture, PolyEdge,
            PolyMode, Repeat, Trapezoid, TrapezoidsRequest, Triangle, TrianglesRequest,
        },
        xproto::{Atom, Pixmap, Rectangle, SubwindowMode},
    },
    display::{Display, DisplayExt},
};
use alloc::borrow::Cow;

#[cfg(feature = "async")]
use crate::display::{AsyncDisplay, AsyncDisplayExt};

crate::create_paramaterizer! {
    pub struct PictureParameters : (Cp, CreatePictureRequest) {
        repeat             (set_repeat,            repeat):           Repeat,
        alphamap           (set_alpha_map,         alphamap):         Picture,
        alpha_x_origin     (set_alpha_x_origin,    alphaxorigin):     i32,
        alpha_y_origin     (set_alpha_y_origin,    alphayorigin):     i32,
        clip_x_origin      (set_clip_x_origin,     clipxorigin):      i32,
        clip_y_origin      (set_clip_y_origin,     clipyorigin):      i32,
        clipmask           (set_clip_mask,         clipmask):         Pixmap,
        graphics_exposure  (set_graphics_exposure, graphicsexposure): u32,
        subwindow_mode     (set_subwindow_mode,    subwindowmode):    SubwindowMode,
        poly_edge          (set_poly_edge,         polyedge):         PolyEdge,
        poly_mode          (set_poly_mode,         polymode):         PolyMode,
        dither             (set_dither,            dither):           Atom,
        component_alpha    (set_component_alpha,   componentalpha):   u32
    }
}

impl PictureParameters {
    #[inline]
    pub(crate) fn mask_to_change_picture_request(&self, o: &mut ChangePictureRequest) -> Cp {
        let mut c: CreatePictureRequest = Default::default();
        let mask = self.convert_to_flags(&mut c);

        o.repeat = c.repeat;
        o.alphamap = c.alphamap;
        o.alphaxorigin = c.alphaxorigin;
        o.alphayorigin = c.alphayorigin;
        o.clipxorigin = c.clipxorigin;
        o.clipyorigin = c.clipyorigin;
        o.clipmask = c.clipmask;
        o.graphicsexposure = c.graphicsexposure;
        o.subwindowmode = c.subwindowmode;
        o.polyedge = c.polyedge;
        o.polymode = c.polymode;
        o.dither = c.dither;
        o.componentalpha = c.componentalpha;

        mask
    }
}

impl Picture {
    #[inline]
    fn change_request(self, params: PictureParameters) -> ChangePictureRequest {
        let mut cr = ChangePictureRequest {
            picture: self,
            ..Default::default()
        };
        let mask = params.mask_to_change_picture_request(&mut cr);
        cr.value_mask = mask;
        cr
    }

    /// Change an attribute of this picture.
    #[inline]
    pub fn change<Dpy: Display + ?Sized>(
        self,
        display: &mut Dpy,
        params: PictureParameters,
    ) -> crate::Result {
        display.exchange_request(self.change_request(params))
    }

    /// Change an attribute of this picture, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        display: &mut Dpy,
        params: PictureParameters,
    ) -> crate::Result {
        display
            .exchange_request_async(self.change_request(params))
            .await
    }

    /// Composite this picture with another.
    #[inline]
    pub fn composite<Dpy: Display + ?Sized>(
        self,
        display: &mut Dpy,
        op: PictOp,
        mask: Picture,
        dst: Picture,
        srcx: i16,
        srcy: i16,
        maskx: i16,
        masky: i16,
        dstx: i16,
        dsty: i16,
        width: u16,
        height: u16,
    ) -> crate::Result {
        display.exchange_request(CompositeRequest {
            op,
            src: self,
            mask,
            dst,
            src_x: srcx,
            src_y: srcy,
            mask_x: maskx,
            mask_y: masky,
            dst_x: dstx,
            dst_y: dsty,
            width,
            height,
            ..Default::default()
        })
    }

    /// Composite this picture with another, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn composite_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        display: &mut Dpy,
        op: PictOp,
        mask: Picture,
        dst: Picture,
        srcx: i16,
        srcy: i16,
        maskx: i16,
        masky: i16,
        dstx: i16,
        dsty: i16,
        width: u16,
        height: u16,
    ) -> crate::Result {
        display
            .exchange_request_async(CompositeRequest {
                op,
                src: self,
                mask,
                dst,
                src_x: srcx,
                src_y: srcy,
                mask_x: maskx,
                mask_y: masky,
                dst_x: dstx,
                dst_y: dsty,
                width,
                height,
                ..Default::default()
            })
            .await
    }

    /// Fill a series of solid color rectangles on this surface.
    #[inline]
    pub fn fill_rectangles<'a, Dpy: Display + ?Sized, Rects: Into<Cow<'a, [Rectangle]>>>(
        self,
        display: &mut Dpy,
        op: PictOp,
        color: Color,
        rects: Rects,
    ) -> crate::Result {
        let rects = rects.into();
        display.exchange_request(FillRectanglesRequest {
            dst: self,
            op,
            color,
            rects,
            ..Default::default()
        })
    }

    /// Fill a series of solid color rectangles on this surface, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn fill_rectangles_async<
        'a,
        Dpy: AsyncDisplay + ?Sized,
        Rects: Into<Cow<'a, [Rectangle]>>,
    >(
        self,
        display: &mut Dpy,
        op: PictOp,
        color: Color,
        rects: Rects,
    ) -> crate::Result {
        display
            .exchange_request_async(FillRectanglesRequest {
                dst: self,
                op,
                color,
                rects: rects.into(),
                ..Default::default()
            })
            .await
    }

    /// Free this picture.
    #[inline]
    pub fn free<Dpy: Display + ?Sized>(self, display: &mut Dpy) -> crate::Result {
        display.exchange_request(FreePictureRequest {
            picture: self,
            ..Default::default()
        })
    }

    /// Free this picture, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Dpy: AsyncDisplay + ?Sized>(self, display: &mut Dpy) -> crate::Result {
        display
            .exchange_request_async(FreePictureRequest {
                picture: self,
                ..Default::default()
            })
            .await
    }

    /// Draw a set of trapezoids.
    #[inline]
    pub fn trapezoids<'a, Dpy: Display + ?Sized, Tzds: Into<Cow<'a, [Trapezoid]>>>(
        self,
        display: &mut Dpy,
        op: PictOp,
        src: Picture,
        mask_format: Pictformat,
        srcx: i16,
        srcy: i16,
        trapezoids: Tzds,
    ) -> crate::Result {
        display.exchange_request(TrapezoidsRequest {
            src,
            dst: self,
            op,
            mask_format,
            src_x: srcx,
            src_y: srcy,
            traps: trapezoids.into(),
            ..Default::default()
        })
    }

    /// Draw a set of triangles.
    #[inline]
    pub fn triangles<'a, Dpy: Display + ?Sized, Trgs: Into<Cow<'a, [Triangle]>>>(
        self,
        display: &mut Dpy,
        op: PictOp,
        src: Picture,
        mask_format: Pictformat,
        srcx: i16,
        srcy: i16,
        triangles: Trgs,
    ) -> crate::Result {
        display.exchange_request(TrianglesRequest {
            src,
            dst: self,
            op,
            mask_format,
            src_x: srcx,
            src_y: srcy,
            triangles: triangles.into(),
            ..Default::default()
        })
    }

    /// Draw a set of trapezoids, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn trapezoids_async<
        'a,
        Dpy: AsyncDisplay + ?Sized,
        Tzds: Into<Cow<'a, [Trapezoid]>>,
    >(
        self,
        display: &mut Dpy,
        op: PictOp,
        src: Picture,
        mask_format: Pictformat,
        srcx: i16,
        srcy: i16,
        trapezoids: Tzds,
    ) -> crate::Result {
        display
            .exchange_request_async(TrapezoidsRequest {
                src,
                dst: self,
                op,
                mask_format,
                src_x: srcx,
                src_y: srcy,
                traps: trapezoids.into(),
                ..Default::default()
            })
            .await
    }

    /// Draw a set of triangles, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn triangles_async<
        'a,
        Dpy: AsyncDisplay + ?Sized,
        Trgs: Into<Cow<'a, [Triangle]>>,
    >(
        self,
        display: &mut Dpy,
        op: PictOp,
        src: Picture,
        mask_format: Pictformat,
        srcx: i16,
        srcy: i16,
        triangles: Trgs,
    ) -> crate::Result {
        display
            .exchange_request_async(TrianglesRequest {
                src,
                dst: self,
                op,
                mask_format,
                src_x: srcx,
                src_y: srcy,
                triangles: triangles.into(),
                ..Default::default()
            })
            .await
    }
}
