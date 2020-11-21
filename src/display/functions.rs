// MIT/Apache2 License

use super::{Connection, Display, RequestCookie};
use crate::{
    auto::{
        xproto::{
            ArcMode, Atom, BackingStore, CapStyle, ChangeGcRequest, Colormap, CreateGcRequest,
            CreateWindowRequest, Cursor, Cw, Drawable, EventMask, FillRule, FillStyle, Font, Gc,
            Gcontext, Gravity, Gx, InternAtomReply, InternAtomRequest, JoinStyle, LineStyle,
            Pixmap, SubwindowMode, Visualid, Window, WindowClass,
        },
        AsByteSequence,
    },
    XidType, XID,
};
use alloc::{string::String, vec::Vec};

crate::create_paramaterizer! {
    pub struct CreateWindowParameters : (Cw, CreateWindowRequest) {
        background_pixmap (set_back_pixmap,       background_pixmap):     Pixmap,
        background_pixel  (set_back_pixel,        background_pixel):      u32,
        border_pixmap     (set_border_pixmap,     border_pixmap):         Pixmap,
        border_pixel      (set_border_pixel,      border_pixel):          u32,
        bit_gravity       (set_bit_gravity,       bit_gravity):           Gravity,
        win_gravity       (set_win_gravity,       win_gravity):           Gravity,
        backing_store     (set_backing_store,     backing_store):         BackingStore,
        backing_planes    (set_backing_planes,    backing_planes):        u32,
        backing_pixel     (set_backing_pixel,     backing_pixel):         u32,
        override_redirect (set_override_redirect, override_redirect):     u32,
        save_under        (set_save_under,        save_under):            u32,
        event_mask        (set_event_mask,        event_mask):            EventMask,
        dont_propogate    (set_dont_propagate,    do_not_propogate_mask): EventMask,
        colormap          (set_colormap,          colormap):              Colormap,
        cursor            (set_cursor,            cursor):                Cursor
    }
}

crate::create_paramaterizer! {
    pub struct GcParameters : (Gc, CreateGcRequest) {
        function              (set_function,              function):              Gx,
        plane_mask            (set_plane_mask,            plane_mask):            u32,
        foreground            (set_foreground,            foreground):            u32,
        background            (set_background,            background):            u32,
        line_width            (set_line_width,            line_width):            u32,
        line_style            (set_line_style,            line_style):            LineStyle,
        cap_style             (set_cap_style,             cap_style):             CapStyle,
        join_style            (set_join_style,            join_style):            JoinStyle,
        fill_style            (set_fill_style,            fill_style):            FillStyle,
        fill_rule             (set_fill_rule,             fill_rule):             FillRule,
        tile                  (set_tile,                  tile):                  Pixmap,
        stipple               (set_stipple,               stipple):               Pixmap,
        tile_stipple_x_origin (set_tile_stipple_origin_x, tile_stipple_x_origin): i32,
        tile_stipple_y_origin (set_tile_stipple_origin_y, tile_stipple_y_origin): i32,
        font                  (set_font,                  font):                  Font,
        subwindow_mode        (set_subwindow_mode,        subwindow_mode):        SubwindowMode,
        graphics_exposures    (set_graphics_exposures,    graphics_exposures):    u32,
        clip_x_origin         (set_clip_origin_x,         clip_x_origin):         i32,
        clip_y_origin         (set_clip_origin_y,         clip_y_origin):         i32,
        clip_mask             (set_clip_mask,             clip_mask):             Pixmap,
        dash_offset           (set_dash_offset,           dash_offset):           u32,
        dashes                (set_dash_list,             dashes):                u32,
        arc_mode              (set_arc_mode,              arc_mode):              ArcMode
    }
}

impl GcParameters {
    #[inline]
    pub(crate) fn mask_change_gc_request(&self, req: &mut ChangeGcRequest) -> Gc {
        let mut create_req: CreateGcRequest = Default::default();
        let gc = self.convert_to_flags(&mut create_req);

        // hopefully this gets optimized out
        req.function = create_req.function;
        req.plane_mask = create_req.plane_mask;
        req.foreground = create_req.foreground;
        req.background = create_req.background;
        req.line_width = create_req.line_width;
        req.line_style = create_req.line_style;
        req.cap_style = create_req.cap_style;
        req.join_style = create_req.join_style;
        req.fill_style = create_req.fill_style;
        req.fill_rule = create_req.fill_rule;
        req.tile = create_req.tile;
        req.stipple = create_req.stipple;
        req.tile_stipple_x_origin = create_req.tile_stipple_x_origin;
        req.tile_stipple_y_origin = create_req.tile_stipple_y_origin;
        req.font = create_req.font;
        req.subwindow_mode = create_req.subwindow_mode;
        req.graphics_exposures = create_req.graphics_exposures;
        req.clip_x_origin = create_req.clip_y_origin;
        req.clip_mask = create_req.clip_mask;
        req.dash_offset = create_req.dash_offset;
        req.dashes = create_req.dashes;
        req.arc_mode = create_req.arc_mode;

        gc
    }
}

impl<Conn: Connection> Display<Conn> {
    #[inline]
    fn create_window_request(
        &mut self,
        wid: Window,
        parent: Option<Window>,
        class: WindowClass,
        depth: Option<u8>,
        visual: Option<Visualid>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        mut props: CreateWindowParameters,
    ) -> CreateWindowRequest {
        const INHERITED_DEPTH: u8 = 0;
        const INHERITED_VISUAL: Visualid = 0;

        if let None = props.background_pixel {
            props.background_pixel = Some(self.default_white_pixel());
        }

        let mut cwr = CreateWindowRequest {
            wid,
            parent: parent.unwrap_or_else(|| self.default_root()),
            class: class,
            visual: visual.unwrap_or(INHERITED_VISUAL),
            depth: depth.unwrap_or(INHERITED_DEPTH),
            x,
            y,
            width,
            height,
            border_width,
            ..Default::default()
        };

        let cw = props.convert_to_flags(&mut cwr);
        cwr.value_mask = cw;
        cwr
    }

    /// Create a new window.
    #[inline]
    pub fn create_window(
        &mut self,
        parent: Option<Window>,
        class: WindowClass,
        depth: Option<u8>,
        visual: Option<Visualid>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        props: CreateWindowParameters,
    ) -> crate::Result<Window> {
        let wid = Window::const_from_xid(self.generate_xid()?);
        log::debug!("Generate {}", wid.xid());
        let cw = self.create_window_request(
            wid,
            parent,
            class,
            depth,
            visual,
            x,
            y,
            width,
            height,
            border_width,
            props,
        );

        log::debug!("Sending CreateWindowRequest to server");
        let tok = self.send_request(cw)?;
        self.resolve_request(tok)?;
        log::debug!("Sent CreateWindowRequest to server");
        Ok(wid)
    }

    /// Create a new window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_window_async(
        &mut self,
        parent: Option<Window>,
        class: WindowClass,
        depth: Option<u8>,
        visual: Option<Visualid>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        props: CreateWindowParameters,
    ) -> crate::Result<Window> {
        let wid = Window::const_from_xid(self.generate_xid()?);
        let cw = self.create_window_request(
            wid,
            parent,
            class,
            depth,
            visual,
            x,
            y,
            width,
            height,
            border_width,
            props,
        );

        let tok = self.send_request_async(cw).await?;
        self.resolve_request_async(tok).await?;
        Ok(wid)
    }

    /// Create a CreateGcRequest.
    #[inline]
    fn create_gc_request<Target: Into<Drawable>>(
        &mut self,
        cid: Gcontext,
        drawable: Target,
        props: GcParameters,
    ) -> CreateGcRequest {
        let mut gcr = CreateGcRequest {
            cid,
            drawable: drawable.into(),
            ..Default::default()
        };

        let gcmask = props.convert_to_flags(&mut gcr);
        gcr.value_mask = gcmask;
        gcr
    }

    /// Create a new graphics context for the specified target.
    #[inline]
    pub fn create_gc<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        props: GcParameters,
    ) -> crate::Result<Gcontext> {
        let gid = Gcontext::const_from_xid(self.generate_xid()?);
        let gcr = self.create_gc_request(gid, target, props);
        let tok = self.send_request(gcr)?;
        self.resolve_request(tok)?;
        Ok(gid)
    }

    /// Create a new graphics context, async redox.
    #[inline]
    pub async fn create_gc_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        props: GcParameters,
    ) -> crate::Result<Gcontext> {
        let gid = Gcontext::const_from_xid(self.generate_xid()?);
        let gcr = self.create_gc_request(gid, target, props);
        let tok = self.send_request_async(gcr).await?;
        self.resolve_request_async(tok).await?;
        Ok(gid)
    }

    /// Create an InternAtomRequest for our use.
    #[inline]
    fn intern_atom_request(name: String, exists: bool) -> InternAtomRequest {
        InternAtomRequest {
            only_if_exists: exists,
            name,
            ..Default::default()
        }
    }

    /// Intern a string and get a corresponding atom for that string.
    #[inline]
    pub fn intern_atom(
        &mut self,
        name: String,
        only_if_exists: bool,
    ) -> crate::Result<RequestCookie<InternAtomRequest>> {
        log::debug!("Sending InternAtomRequest to server");
        let tok = self.send_request(Self::intern_atom_request(name, only_if_exists))?;
        log::debug!("Sent InternAtomRequest to server");
        Ok(tok)
    }

    /// Intern an atom, async redox. See function `intern_atom` for more information.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn intern_atom_async(
        &mut self,
        name: String,
        only_if_exists: bool,
    ) -> crate::Result<RequestCookie<InternAtomRequest>> {
        self.send_request_async(Self::intern_atom_request(name, only_if_exists))
            .await
    }

    /// Intern an atom, but try to resolve the request immediately.
    #[inline]
    pub fn intern_atom_immediate(
        &mut self,
        name: String,
        only_if_exists: bool,
    ) -> crate::Result<Atom> {
        let r = self.intern_atom(name, only_if_exists)?;
        Ok(self.resolve_request(r)?.atom)
    }

    /// Intern an atom, but try to resolve the request immediately, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn intern_atom_immediate_async(
        &mut self,
        name: String,
        only_if_exists: bool,
    ) -> crate::Result<Atom> {
        let r = self.intern_atom_async(name, only_if_exists).await?;
        Ok(self.resolve_request_async(r).await?.atom)
    }
}
