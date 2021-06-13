// MIT/Apache2 License

use crate::{
    auto::xproto::{
        Arc, ChangeGcRequest, CoordMode, Drawable, FillPolyRequest, FreeGcRequest, Gcontext, Point,
        PolyArcRequest, PolyFillArcRequest, PolyFillRectangleRequest, PolyRectangleRequest,
        PolySegmentRequest, PolyShape, Rectangle, Segment,
    },
    display::prelude::*,
    Display, GcParameters,
};
use alloc::borrow::Cow;

#[cfg(feature = "async")]
use crate::display::{AsyncDisplay, EitherFuture, ExchangeRequestFuture, SendRequestFuture};
#[cfg(feature = "async")]
use alloc::vec;
#[cfg(feature = "async")]
use futures_lite::future::{self, Ready};

impl Gcontext {
    #[inline]
    fn change_request(self, params: GcParameters) -> ChangeGcRequest {
        let mut cgcr = ChangeGcRequest {
            gc: self,
            ..Default::default()
        };

        let g = params.mask_change_gc_request(&mut cgcr);
        cgcr.value_mask = g;
        cgcr
    }

    /// Change the properties of this GC.
    #[inline]
    pub fn change<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        params: GcParameters,
    ) -> crate::Result<()> {
        dpy.exchange_request(self.change_request(params))
    }

    /// Change the properties of this GC, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn change_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        params: GcParameters,
    ) -> ExchangeRequestFuture<'_, Dpy, ChangeGcRequest> {
        dpy.exchange_request_async(self.change_request(params))
    }

    /// Request to draw a line.
    #[inline]
    fn poly_segment_request(
        self,
        drawable: Drawable,
        segments: Cow<'_, [Segment]>,
    ) -> PolySegmentRequest<'_> {
        PolySegmentRequest {
            drawable,
            gc: self,
            segments,
            ..Default::default()
        }
    }

    /// Draw a set of lines.
    #[inline]
    pub fn draw_lines<
        'a,
        Dpy: Display + ?Sized,
        Target: Into<Drawable>,
        Lines: Into<Cow<'a, [Segment]>>,
    >(
        self,
        dpy: &mut Dpy,
        target: Target,
        lines: Lines,
    ) -> crate::Result {
        let line = lines.into();
        if line.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_segment_request(target.into(), line))
    }

    /// Draw a set of lines, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_lines_async<
        'a,
        'b,
        Dpy: AsyncDisplay + ?Sized,
        Target: Into<Drawable>,
        Lines: Into<Cow<'b, [Segment]>>,
    >(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        lines: Lines,
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolySegmentRequest>>
    where
        'b: 'a,
    {
        let line = lines.into();
        match line.is_empty() {
            true => EitherFuture::Left {
                future: future::ready(Ok(())),
            },
            false => EitherFuture::Right {
                future: dpy.exchange_request_async(self.poly_segment_request(target.into(), line)),
            },
        }
    }

    /// Draw a singular line.
    #[inline]
    pub fn draw_line<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        line: Segment,
    ) -> crate::Result {
        let line = [line];
        let line: &[Segment] = &line;
        self.draw_lines(dpy, target, line)
    }

    /// Draw a singular line, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_line_async<'a, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        line: Segment,
    ) -> ExchangeRequestFuture<'a, Dpy, PolySegmentRequest<'a>> {
        match self.draw_lines_async(dpy, target, vec![line]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Rectangle drawing request.
    #[inline]
    fn poly_rectangle_request(
        self,
        target: Drawable,
        rectangles: Cow<'_, [Rectangle]>,
    ) -> PolyRectangleRequest<'_> {
        PolyRectangleRequest {
            drawable: target,
            gc: self,
            rectangles,
            ..Default::default()
        }
    }

    /// Draw one or more rectangles to the screen.
    #[inline]
    pub fn draw_rectangles<
        'a,
        Dpy: Display + ?Sized,
        Target: Into<Drawable>,
        Rects: Into<Cow<'a, [Rectangle]>>,
    >(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangles: Rects,
    ) -> crate::Result<()> {
        let rectangles = rectangles.into();
        if rectangles.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_rectangle_request(target.into(), rectangles))
    }

    /// Draw one or more rectangles to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_rectangles_async<
        'a,
        'b,
        Dpy: AsyncDisplay + ?Sized,
        Target: Into<Drawable>,
        Rects: Into<Cow<'b, [Rectangle]>>,
    >(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        rectangles: Rects,
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolyRectangleRequest<'b>>>
    {
        let rectangles = rectangles.into();
        match rectangles.is_empty() {
            true => EitherFuture::Left {
                future: future::ready(Ok(())),
            },
            false => EitherFuture::Right {
                future: dpy
                    .exchange_request_async(self.poly_rectangle_request(target.into(), rectangles)),
            },
        }
    }

    /// Draw a rectangle to the screen.
    #[inline]
    pub fn draw_rectangle<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangle: Rectangle,
    ) -> crate::Result<()> {
        let rectangle: &[Rectangle] = &[rectangle];
        self.draw_rectangles(dpy, target, rectangle)
    }

    /// Draw a rectangle to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_rectangle_async<Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangle: Rectangle,
    ) -> ExchangeRequestFuture<'_, Dpy, PolyRectangleRequest<'static>> {
        match self.draw_rectangles_async(dpy, target, vec![rectangle]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Arc drawing request.
    #[inline]
    fn poly_arc_request(self, target: Drawable, arcs: Cow<'_, [Arc]>) -> PolyArcRequest<'_> {
        PolyArcRequest {
            drawable: target,
            gc: self,
            arcs,
            ..Default::default()
        }
    }

    /// Draw one or more arcs to the screen.
    #[inline]
    pub fn draw_arcs<
        'a,
        Dpy: Display + ?Sized,
        Target: Into<Drawable>,
        Arcs: Into<Cow<'a, [Arc]>>,
    >(
        self,
        dpy: &mut Dpy,
        target: Target,
        arcs: Arcs,
    ) -> crate::Result {
        let arcs = arcs.into();
        if arcs.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_arc_request(target.into(), arcs))
    }

    /// Draw one or more arcs to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_arcs_async<
        'a,
        'b,
        Dpy: AsyncDisplay + ?Sized,
        Target: Into<Drawable>,
        Arcs: Into<Cow<'b, [Arc]>>,
    >(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        arcs: Arcs,
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolyArcRequest<'b>>>
    where
        'b: 'a,
    {
        let arcs = arcs.into();
        match arcs.is_empty() {
            true => EitherFuture::Left {
                future: future::ready(Ok(())),
            },
            false => EitherFuture::Right {
                future: dpy.exchange_request_async(self.poly_arc_request(target.into(), arcs)),
            },
        }
    }

    /// Draw an arc to the screen.
    #[inline]
    pub fn draw_arc<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        arc: Arc,
    ) -> crate::Result {
        let arc: &[Arc] = &[arc];
        self.draw_arcs(dpy, target, arc)
    }

    /// Draw an arc to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_arc_async<'a, 'b, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        arc: Arc,
    ) -> ExchangeRequestFuture<'a, Dpy, PolyArcRequest<'b>>
    where
        'b: 'a,
    {
        match self.draw_arcs_async(dpy, target, vec![arc]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Request to fill a polygon.
    #[inline]
    fn fill_poly_request(
        self,
        drawable: Drawable,
        shape: PolyShape,
        mode: CoordMode,
        points: Cow<'_, [Point]>,
    ) -> FillPolyRequest<'_> {
        FillPolyRequest {
            drawable,
            gc: self,
            shape,
            coordinate_mode: mode,
            points,
            ..Default::default()
        }
    }

    /// Fill a polygon specified by the given points.
    #[inline]
    pub fn fill_polygon<
        'a,
        Dpy: Display + ?Sized,
        Target: Into<Drawable>,
        Pts: Into<Cow<'a, [Point]>>,
    >(
        self,
        dpy: &mut Dpy,
        target: Target,
        shape: PolyShape,
        coordinate_mode: CoordMode,
        points: Pts,
    ) -> crate::Result {
        let points = points.into();
        if points.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.fill_poly_request(target.into(), shape, coordinate_mode, points))
    }

    /// Fill a polygon specified by the given points, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_polygon_async<
        'a,
        'b,
        Dpy: AsyncDisplay + ?Sized,
        Target: Into<Drawable>,
        Pts: Into<Cow<'b, [Point]>>,
    >(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        shape: PolyShape,
        coordinate_mode: CoordMode,
        points: Pts,
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, FillPolyRequest>>
    where
        'b: 'a,
    {
        let points = points.into();
        match points.is_empty() {
            true => EitherFuture::Left {
                future: future::ready(Ok(())),
            },
            false => EitherFuture::Right {
                future: dpy.exchange_request_async(self.fill_poly_request(
                    target.into(),
                    shape,
                    coordinate_mode,
                    points,
                )),
            },
        }
    }

    /// Request to fill rectangles.
    #[inline]
    fn poly_fill_rectangle_request(
        self,
        drawable: Drawable,
        rectangles: Cow<'_, [Rectangle]>,
    ) -> PolyFillRectangleRequest<'_> {
        PolyFillRectangleRequest {
            drawable,
            gc: self,
            rectangles,
            ..Default::default()
        }
    }

    /// Fill a set of one or more rectangles.
    #[inline]
    pub fn fill_rectangles<
        'a,
        Dpy: Display + ?Sized,
        Target: Into<Drawable>,
        Rects: Into<Cow<'a, [Rectangle]>>,
    >(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangles: Rects,
    ) -> crate::Result {
        let rectangles = rectangles.into();
        if rectangles.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_fill_rectangle_request(target.into(), rectangles))
    }

    /// Fill a set of one or more rectangles, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_rectangles_async<
        'a,
        'b,
        Dpy: AsyncDisplay + ?Sized,
        Target: Into<Drawable>,
        Rects: Into<Cow<'b, [Rectangle]>>,
    >(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        rectangles: Rects,
    ) -> EitherFuture<
        Ready<crate::Result>,
        ExchangeRequestFuture<'a, Dpy, PolyFillRectangleRequest<'b>>,
    > {
        let rectangles = rectangles.into();
        match rectangles.is_empty() {
            true => EitherFuture::Left {
                future: future::ready(Ok(())),
            },
            false => EitherFuture::Right {
                future: dpy.exchange_request_async(
                    self.poly_fill_rectangle_request(target.into(), rectangles),
                ),
            },
        }
    }

    /// Fill a single rectangle.
    #[inline]
    pub fn fill_rectangle<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangle: Rectangle,
    ) -> crate::Result {
        let rectangle: &[Rectangle] = &[rectangle];
        self.fill_rectangles(dpy, target, rectangle)
    }

    /// Fill a single rectangle, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_rectangle_async<Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangle: Rectangle,
    ) -> ExchangeRequestFuture<'_, Dpy, PolyFillRectangleRequest> {
        match self.fill_rectangles_async(dpy, target, vec![rectangle]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Request to fill a series of arcs.
    #[inline]
    fn poly_fill_arc_request(
        self,
        drawable: Drawable,
        arcs: Cow<'_, [Arc]>,
    ) -> PolyFillArcRequest<'_> {
        PolyFillArcRequest {
            drawable,
            gc: self,
            arcs,
            ..Default::default()
        }
    }

    /// Fill a set of one or more arcs.
    #[inline]
    pub fn fill_arcs<
        'a,
        Dpy: Display + ?Sized,
        Target: Into<Drawable>,
        Arcs: Into<Cow<'a, [Arc]>>,
    >(
        self,
        dpy: &mut Dpy,
        target: Target,
        arcs: Arcs,
    ) -> crate::Result {
        let arcs = arcs.into();
        if arcs.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_fill_arc_request(target.into(), arcs))
    }

    /// Fill a set of one or more arcs, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_arcs_async<
        'a,
        'b,
        Dpy: AsyncDisplay + ?Sized,
        Target: Into<Drawable>,
        Arcs: Into<Cow<'b, [Arc]>>,
    >(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        arcs: Arcs,
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolyFillArcRequest<'b>>>
    {
        let arcs = arcs.into();
        match arcs.is_empty() {
            true => EitherFuture::Left {
                future: future::ready(Ok(())),
            },
            false => EitherFuture::Right {
                future: dpy.exchange_request_async(self.poly_fill_arc_request(target.into(), arcs)),
            },
        }
    }

    /// Fill an arc.
    #[inline]
    pub fn fill_arc<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        arc: Arc,
    ) -> crate::Result {
        let arc: &[Arc] = &[arc];
        self.fill_arcs(dpy, target, arc)
    }

    /// Fill an arc, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_arc_async<Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        arc: Arc,
    ) -> ExchangeRequestFuture<'_, Dpy, PolyFillArcRequest<'static>> {
        match self.fill_arcs_async(dpy, target, vec![arc]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Free the memory this GC allocates. Note that this will cause future requests involving this GC
    /// to fail.
    #[inline]
    pub fn free<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(FreeGcRequest {
            gc: self,
            ..Default::default()
        })
    }

    /// Free the memory this GC allocates, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn free_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> ExchangeRequestFuture<'_, Dpy, FreeGcRequest> {
        dpy.exchange_request_async(FreeGcRequest {
            gc: self,
            ..Default::default()
        })
    }
}
