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

#[cfg(feature = "async")]
use crate::display::{AsyncDisplay, EitherFuture, ExchangeRequestFuture, SendRequestFuture};
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
    fn poly_segment_request(self, drawable: Drawable, line: &[Segment]) -> PolySegmentRequest {
        PolySegmentRequest {
            drawable,
            gc: self,
            segments: line.to_vec(),
            ..Default::default()
        }
    }

    /// Draw a set of lines.
    #[inline]
    pub fn draw_lines<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        line: &[Segment],
    ) -> crate::Result {
        if line.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_segment_request(target.into(), line))
    }

    /// Draw a set of lines, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_lines_async<'a, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        line: &[Segment],
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolySegmentRequest>>
    {
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
        self.draw_lines(dpy, target, &[line])
    }

    /// Draw a singular line, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_line_async<Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        line: Segment,
    ) -> ExchangeRequestFuture<'_, Dpy, PolySegmentRequest> {
        match self.draw_lines_async(dpy, target, &[line]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Rectangle drawing request.
    #[inline]
    fn poly_rectangle_request(
        self,
        target: Drawable,
        rectangles: &[Rectangle],
    ) -> PolyRectangleRequest {
        PolyRectangleRequest {
            drawable: target,
            gc: self,
            rectangles: rectangles.to_vec(),
            ..Default::default()
        }
    }

    /// Draw one or more rectangles to the screen.
    #[inline]
    pub fn draw_rectangles<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangles: &[Rectangle],
    ) -> crate::Result<()> {
        if rectangles.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_rectangle_request(target.into(), rectangles))
    }

    /// Draw one or more rectangles to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_rectangles_async<'a, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        rectangles: &[Rectangle],
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolyRectangleRequest>>
    {
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
        self.draw_rectangles(dpy, target, &[rectangle])
    }

    /// Draw a rectangle to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_rectangle_async<Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangle: Rectangle,
    ) -> ExchangeRequestFuture<'_, Dpy, PolyRectangleRequest> {
        match self.draw_rectangles_async(dpy, target, &[rectangle]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Arc drawing request.
    #[inline]
    fn poly_arc_request(self, target: Drawable, arcs: &[Arc]) -> PolyArcRequest {
        PolyArcRequest {
            drawable: target,
            gc: self,
            arcs: arcs.to_vec(),
            ..Default::default()
        }
    }

    /// Draw one or more arcs to the screen.
    #[inline]
    pub fn draw_arcs<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        arcs: &[Arc],
    ) -> crate::Result {
        if arcs.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_arc_request(target.into(), arcs))
    }

    /// Draw one or more arcs to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_arcs_async<'a, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        arcs: &[Arc],
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolyArcRequest>> {
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
        self.draw_arcs(dpy, target, &[arc])
    }

    /// Draw an arc to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn draw_arc_async<Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        arc: Arc,
    ) -> ExchangeRequestFuture<'_, Dpy, PolyArcRequest> {
        match self.draw_arcs_async(dpy, target, &[arc]) {
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
        points: &[Point],
    ) -> FillPolyRequest {
        FillPolyRequest {
            drawable,
            gc: self,
            shape,
            coordinate_mode: mode,
            points: points.to_vec(),
            ..Default::default()
        }
    }

    /// Fill a polygon specified by the given points.
    #[inline]
    pub fn fill_polygon<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        shape: PolyShape,
        coordinate_mode: CoordMode,
        points: &[Point],
    ) -> crate::Result {
        if points.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.fill_poly_request(target.into(), shape, coordinate_mode, points))
    }

    /// Fill a polygon specified by the given points, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_polygon_async<'a, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        shape: PolyShape,
        coordinate_mode: CoordMode,
        points: &[Point],
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, FillPolyRequest>> {
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
        rectangles: &[Rectangle],
    ) -> PolyFillRectangleRequest {
        PolyFillRectangleRequest {
            drawable,
            gc: self,
            rectangles: rectangles.to_vec(),
            ..Default::default()
        }
    }

    /// Fill a set of one or more rectangles.
    #[inline]
    pub fn fill_rectangles<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        rectangles: &[Rectangle],
    ) -> crate::Result {
        if rectangles.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_fill_rectangle_request(target.into(), rectangles))
    }

    /// Fill a set of one or more rectangles, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_rectangles_async<'a, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        rectangles: &[Rectangle],
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolyFillRectangleRequest>>
    {
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
        self.fill_rectangles(dpy, target, &[rectangle])
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
        match self.fill_rectangles_async(dpy, target, &[rectangle]) {
            EitherFuture::Right { future } => future,
            EitherFuture::Left { .. } => unreachable!(),
        }
    }

    /// Request to fill a series of arcs.
    #[inline]
    fn poly_fill_arc_request(self, drawable: Drawable, arcs: &[Arc]) -> PolyFillArcRequest {
        PolyFillArcRequest {
            drawable,
            gc: self,
            arcs: arcs.to_vec(),
            ..Default::default()
        }
    }

    /// Fill a set of one or more arcs.
    #[inline]
    pub fn fill_arcs<Dpy: Display + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        arcs: &[Arc],
    ) -> crate::Result {
        if arcs.is_empty() {
            return Ok(());
        }

        dpy.exchange_request(self.poly_fill_arc_request(target.into(), arcs))
    }

    /// Fill a set of one or more arcs, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_arcs_async<'a, Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &'a mut Dpy,
        target: Target,
        arcs: &[Arc],
    ) -> EitherFuture<Ready<crate::Result>, ExchangeRequestFuture<'a, Dpy, PolyFillArcRequest>>
    {
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
        self.fill_arcs(dpy, target, &[arc])
    }

    /// Fill an arc, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub fn fill_arc_async<Dpy: AsyncDisplay + ?Sized, Target: Into<Drawable>>(
        self,
        dpy: &mut Dpy,
        target: Target,
        arc: Arc,
    ) -> ExchangeRequestFuture<'_, Dpy, PolyFillArcRequest> {
        match self.fill_arcs_async(dpy, target, &[arc]) {
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
