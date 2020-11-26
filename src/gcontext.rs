// MIT/Apache2 License

use super::{
    auto::xproto::{
        Arc, ChangeGcRequest, CoordMode, Drawable, FillPolyRequest, FreeGcRequest, Gcontext, Point,
        PolyArcRequest, PolyFillArcRequest, PolyFillRectangleRequest, PolyRectangleRequest,
        PolySegmentRequest, PolyShape, Rectangle, Segment,
    },
    Connection, Display, GcParameters,
};

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
    pub fn change<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        params: GcParameters,
    ) -> crate::Result<()> {
        log::debug!("Sending ChangeGcRequest to server");
        let req = self.change_request(params);
        let tok = dpy.send_request(req)?;
        log::debug!("Send ChangeGcRequest to server");
        dpy.resolve_request(tok)
    }

    /// Change the properties of this GC, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        params: GcParameters,
    ) -> crate::Result<()> {
        log::debug!("Sending ChangeGcRequest to server");
        let req = self.change_request(params);
        let tok = dpy.send_request_async(req).await?;
        log::debug!("Send ChangeGcRequest to server");
        dpy.resolve_request_async(tok).await
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
    pub fn draw_lines<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        line: &[Segment],
    ) -> crate::Result {
        if line.is_empty() {
            return Ok(());
        }

        let psr = self.poly_segment_request(target.into(), line);
        log::debug!("Sending PolySegmentRequest to server");
        let tok = dpy.send_request(psr)?;
        log::debug!("Sent PolySegmentRequest to server");
        dpy.resolve_request(tok)
    }

    /// Draw a set of lines, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn draw_lines_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        line: &[Segment],
    ) -> crate::Result {
        if line.is_empty() {
            return Ok(());
        }

        let psr = self.poly_segment_request(target.into(), line);
        log::debug!("Sending PolySegmentRequest to server");
        let tok = dpy.send_request_async(psr).await?;
        log::debug!("Sent PolySegmentRequest to server");
        dpy.resolve_request_async(tok).await
    }

    /// Draw a singular line.
    #[inline]
    pub fn draw_line<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        line: Segment,
    ) -> crate::Result {
        self.draw_lines(dpy, target, &[line])
    }

    /// Draw a singular line, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn draw_line_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        line: Segment,
    ) -> crate::Result {
        self.draw_lines_async(dpy, target, &[line]).await
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
    pub fn draw_rectangles<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangles: &[Rectangle],
    ) -> crate::Result<()> {
        if rectangles.is_empty() {
            return Ok(());
        }

        let prr = self.poly_rectangle_request(target.into(), rectangles);
        log::debug!("Sending PolyRectangleRequest to the server.");
        let tok = dpy.send_request(prr)?;
        log::debug!("Sent PolyRectangleRequest to the server.");
        dpy.resolve_request(tok)
    }

    /// Draw one or more rectangles to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn draw_rectangles_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangles: &[Rectangle],
    ) -> crate::Result<()> {
        if rectangles.is_empty() {
            return Ok(());
        }

        let prr = self.poly_rectangle_request(target.into(), rectangles);
        log::debug!("Sending PolyRectangleRequest to the server.");
        let tok = dpy.send_request_async(prr).await?;
        log::debug!("Sent PolyRectangleRequest to the server.");
        dpy.resolve_request_async(tok).await
    }

    /// Draw a rectangle to the screen.
    #[inline]
    pub fn draw_rectangle<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangle: Rectangle,
    ) -> crate::Result<()> {
        self.draw_rectangles(dpy, target, &[rectangle])
    }

    /// Draw a rectangle to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn draw_rectangle_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangle: Rectangle,
    ) -> crate::Result<()> {
        self.draw_rectangles_async(dpy, target, &[rectangle]).await
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
    pub fn draw_arcs<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arcs: &[Arc],
    ) -> crate::Result {
        if arcs.is_empty() {
            return Ok(());
        }

        let par = self.poly_arc_request(target.into(), arcs);
        log::debug!("Sending PolyArcRequest to the server.");
        let tok = dpy.send_request(par)?;
        log::debug!("Send PolyArcRequest to the server.");
        dpy.resolve_request(tok)
    }

    /// Draw one or more arcs to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn draw_arcs_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arcs: &[Arc],
    ) -> crate::Result {
        if arcs.is_empty() {
            return Ok(());
        }

        let par = self.poly_arc_request(target.into(), arcs);
        log::debug!("Sending PolyArcRequest to the server.");
        let tok = dpy.send_request_async(par).await?;
        log::debug!("Send PolyArcRequest to the server.");
        dpy.resolve_request_async(tok).await
    }

    /// Draw an arc to the screen.
    #[inline]
    pub fn draw_arc<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arc: Arc,
    ) -> crate::Result {
        self.draw_arcs(dpy, target, &[arc])
    }

    /// Draw an arc to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn draw_arc_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arc: Arc,
    ) -> crate::Result {
        self.draw_arcs_async(dpy, target, &[arc]).await
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
    pub fn fill_polygon<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        shape: PolyShape,
        coordinate_mode: CoordMode,
        points: &[Point],
    ) -> crate::Result {
        if points.is_empty() {
            return Ok(());
        }

        let fpr = self.fill_poly_request(target.into(), shape, coordinate_mode, points);
        log::debug!("Sending FillPolyRequest to server.");
        let tok = dpy.send_request(fpr)?;
        log::debug!("Sent FillPolyRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Fill a polygon specified by the given points, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn fill_polygon_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        shape: PolyShape,
        coordinate_mode: CoordMode,
        points: &[Point],
    ) -> crate::Result {
        if points.is_empty() {
            return Ok(());
        }

        let fpr = self.fill_poly_request(target.into(), shape, coordinate_mode, points);
        log::debug!("Sending FillPolyRequest to server.");
        let tok = dpy.send_request_async(fpr).await?;
        log::debug!("Sent FillPolyRequest to server.");
        dpy.resolve_request_async(tok).await
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
    pub fn fill_rectangles<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangles: &[Rectangle],
    ) -> crate::Result {
        if rectangles.is_empty() {
            return Ok(());
        }

        let pfrr = self.poly_fill_rectangle_request(target.into(), rectangles);
        log::debug!("Sending PolyFillRectangleRequest to server.");
        let tok = dpy.send_request(pfrr)?;
        log::debug!("Sent PolyFillRectangleRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Fill a set of one or more rectangles, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn fill_rectangles_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangles: &[Rectangle],
    ) -> crate::Result {
        if rectangles.is_empty() {
            return Ok(());
        }

        let pfrr = self.poly_fill_rectangle_request(target.into(), rectangles);
        log::debug!("Sending PolyFillRectangleRequest to server.");
        let tok = dpy.send_request_async(pfrr).await?;
        log::debug!("Sent PolyFillRectangleRequest to server.");
        dpy.resolve_request_async(tok).await
    }

    /// Fill a single rectangle.
    #[inline]
    pub fn fill_rectangle<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangle: Rectangle,
    ) -> crate::Result {
        self.fill_rectangles(dpy, target, &[rectangle])
    }

    /// Fill a single rectangle, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn fill_rectangle_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        rectangle: Rectangle,
    ) -> crate::Result {
        self.fill_rectangles_async(dpy, target, &[rectangle]).await
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
    pub fn fill_arcs<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arcs: &[Arc],
    ) -> crate::Result {
        if arcs.is_empty() {
            return Ok(());
        }

        let pfar = self.poly_fill_arc_request(target.into(), arcs);
        log::debug!("Sending PolyFillArcRequest to server.");
        let tok = dpy.send_request(pfar)?;
        log::debug!("Sent PolyFillArcRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Fill a set of one or more arcs, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn fill_arcs_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arcs: &[Arc],
    ) -> crate::Result {
        if arcs.is_empty() {
            return Ok(());
        }

        let pfar = self.poly_fill_arc_request(target.into(), arcs);
        log::debug!("Sending PolyFillArcRequest to server.");
        let tok = dpy.send_request_async(pfar).await?;
        log::debug!("Sent PolyFillArcRequest to server.");
        dpy.resolve_request_async(tok).await
    }

    /// Fill an arc.
    #[inline]
    pub fn fill_arc<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arc: Arc,
    ) -> crate::Result {
        self.fill_arcs(dpy, target, &[arc])
    }

    /// Fill an arc, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn fill_arc_async<Conn: Connection, Target: Into<Drawable>>(
        self,
        dpy: &mut Display<Conn>,
        target: Target,
        arc: Arc,
    ) -> crate::Result {
        self.fill_arcs_async(dpy, target, &[arc]).await
    }

    /// Free the memory this GC allocates. Note that this will cause future requests involving this GC
    /// to fail.
    #[inline]
    pub fn free<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        let req = FreeGcRequest {
            gc: self,
            ..Default::default()
        };
        log::debug!("Sending FreeGcRequest to server.");
        let tok = dpy.send_request(req)?;
        log::debug!("Sent FreeGcRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Free the memory this GC allocates, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        let req = FreeGcRequest {
            gc: self,
            ..Default::default()
        };
        log::debug!("Sending FreeGcRequest to server.");
        let tok = dpy.send_request_async(req).await?;
        log::debug!("Sent FreeGcRequest to server.");
        dpy.resolve_request_async(tok).await
    }
}
