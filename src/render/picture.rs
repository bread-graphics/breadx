// MIT/Apache2 License

use crate::{
    auto::{
        render::{
            ChangePictureRequest, Color, CompositeRequest, Cp, CreatePictureRequest,
            FillRectanglesRequest, FreePictureRequest, PictOp, Pictformat, Picture, PolyEdge,
            PolyMode, Repeat, Trapezoid, TrapezoidsRequest,
        },
        xproto::{Atom, Pixmap, Rectangle, SubwindowMode},
    },
    display::{Connection, Display},
    sr_request,
};

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

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
    pub fn change<Conn: Connection>(
        self,
        display: &mut Display<Conn>,
        params: PictureParameters,
    ) -> crate::Result {
        sr_request!(display, self.change_request(params))
    }

    /// Change an attribute of this picture, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_async<Conn: AsyncConnection + Send>(
        self,
        display: &mut Display<Conn>,
        params: PictureParameters,
    ) -> crate::Result {
        sr_request!(display, self.change_request(params), async).await
    }

    /// Composite this picture with another.
    #[inline]
    pub fn composite<Conn: Connection>(
        self,
        display: &mut Display<Conn>,
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
        sr_request!(
            display,
            CompositeRequest {
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
            }
        )
    }

    /// Composite this picture with another, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn composite_async<Conn: AsyncConnection + Send>(
        self,
        display: &mut Display<Conn>,
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
        sr_request!(
            display,
            CompositeRequest {
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
            },
            async
        )
        .await
    }

    /// Fill a series of solid color rectangles on this surface.
    #[inline]
    pub fn fill_rectangles<Conn: Connection>(
        self,
        display: &mut Display<Conn>,
        op: PictOp,
        color: Color,
        rects: &[Rectangle],
    ) -> crate::Result {
        sr_request!(
            display,
            FillRectanglesRequest {
                dst: self,
                op,
                color,
                rects: rects.to_vec(),
                ..Default::default()
            }
        )
    }

    /// Fill a series of solid color rectangles on this surface, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn fill_rectangles_async<Conn: AsyncConnection + Send>(
        self,
        display: &mut Display<Conn>,
        op: PictOp,
        color: Color,
        rects: &[Rectangle],
    ) -> crate::Result {
        sr_request!(
            display,
            FillRectanglesRequest {
                dst: self,
                op,
                color,
                rects: rects.to_vec(),
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Free this picture.
    #[inline]
    pub fn free<Conn: Connection>(self, display: &mut Display<Conn>) -> crate::Result {
        sr_request!(
            display,
            FreePictureRequest {
                picture: self,
                ..Default::default()
            }
        )
    }

    /// Free this picture, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Conn: AsyncConnection + Send>(
        self,
        display: &mut Display<Conn>,
    ) -> crate::Result {
        sr_request!(
            display,
            FreePictureRequest {
                picture: self,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Draw a set of trapezoids.
    #[inline]
    pub fn trapezoids<Conn: Connection>(
        self,
        display: &mut Display<Conn>,
        op: PictOp,
        src: Picture,
        mask_format: Pictformat,
        srcx: i16,
        srcy: i16,
        trapezoids: &[Trapezoid],
    ) -> crate::Result {
        sr_request!(
            display,
            TrapezoidsRequest {
                src,
                dst: self,
                op,
                mask_format,
                src_x: srcx,
                src_y: srcy,
                traps: trapezoids.to_vec(),
                ..Default::default()
            }
        )
    }

    /// Draw a set of trapezoids, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn trapezoids_async<Conn: AsyncConnection + Send>(
        self,
        display: &mut Display<Conn>,
        op: PictOp,
        src: Picture,
        mask_format: Pictformat,
        srcx: i16,
        srcy: i16,
        trapezoids: &[Trapezoid],
    ) -> crate::Result {
        sr_request!(
            display,
            TrapezoidsRequest {
                src,
                dst: self,
                op,
                mask_format,
                src_x: srcx,
                src_y: srcy,
                traps: trapezoids.to_vec(),
                ..Default::default()
            },
            async
        )
        .await
    }
}