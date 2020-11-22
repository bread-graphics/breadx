// MIT/Apache2 License

use super::{Connection, Display, RequestCookie};
use crate::{
    auto::{
        xproto::{
            AccessControl, ArcMode, Atom, AutoRepeatMode, BackingStore, BellRequest, CapStyle,
            ChangeActivePointerGrabRequest, ChangeGcRequest, ChangeKeyboardControlRequest,
            ChangePointerControlRequest, ChangeWindowAttributesRequest, CloseDown, Colormap,
            CreateGcRequest, CreateWindowRequest, Cursor, Cw, Drawable, EventMask, FillRule,
            FillStyle, Font, Gc, Gcontext, Gravity, Gx, InternAtomReply, InternAtomRequest,
            JoinStyle, Kb, LedMode, LineStyle, Pixmap, SetAccessControlRequest,
            SetCloseDownModeRequest, SubwindowMode, Timestamp, Visualid, Window, WindowClass,
        },
        AsByteSequence,
    },
    XidType, XID,
};
use alloc::{string::String, vec::Vec};

crate::create_paramaterizer! {
    pub struct WindowParameters : (Cw, CreateWindowRequest) {
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

impl WindowParameters {
    #[inline]
    pub(crate) fn mask_to_change_attrs_request(&self, o: &mut ChangeWindowAttributesRequest) -> Cw {
        let mut c: CreateWindowRequest = Default::default();
        let mask = self.convert_to_flags(&mut c);

        o.background_pixmap = c.background_pixmap;
        o.background_pixel = c.background_pixel;
        o.border_pixmap = c.border_pixmap;
        o.border_pixel = c.border_pixel;
        o.bit_gravity = c.bit_gravity;
        o.win_gravity = c.win_gravity;
        o.backing_store = c.backing_store;
        o.backing_planes = c.backing_planes;
        o.backing_pixel = c.backing_pixel;
        o.override_redirect = c.override_redirect;
        o.save_under = c.save_under;
        o.event_mask = c.event_mask;
        o.do_not_propogate_mask = c.do_not_propogate_mask;
        o.colormap = c.colormap;
        o.cursor = c.cursor;

        mask
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

crate::create_paramaterizer! {
    pub struct KbParameters : (Kb, ChangeKeyboardControlRequest) {
        key_click_percent (set_key_click_percent, key_click_percent): i32,
        bell_percent      (set_bell_percent,      bell_percent):      i32,
        bell_pitch        (set_bell_pitch,        bell_pitch):        i32,
        bell_duration     (set_bell_duration,     bell_duration):     i32,
        led               (set_led,               led):               u32,
        led_mode          (set_led_mode,          led_mode):          LedMode,
        key               (set_key,               key):               u32,
        auto_repeat_mode  (set_auto_repeat_mode,  auto_repeat_mode):  AutoRepeatMode
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
        mut props: WindowParameters,
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
        props: WindowParameters,
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
        props: WindowParameters,
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
    #[cfg(feature = "async")]
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

    /// Change Keyboard Control Request
    #[inline]
    fn change_keyboard_control_request(props: KbParameters) -> ChangeKeyboardControlRequest {
        let mut ckcr: ChangeKeyboardControlRequest = Default::default();
        let c = props.convert_to_flags(&mut ckcr);
        ckcr.value_mask = c;
        ckcr
    }

    /// Change the keyboard's control properties.
    #[inline]
    pub fn change_keyboard_control(&mut self, props: KbParameters) -> crate::Result<()> {
        let ckcr = Self::change_keyboard_control_request(props);
        log::debug!("Sending ChangeKeyboardControlRequest to server.");
        let tok = self.send_request(ckcr)?;
        log::debug!("Sent ChangeKeyboardControlRequest to server.");
        self.resolve_request(tok)
    }

    /// Change the keyboard's control properties, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_keyboard_control_async(
        &mut self,
        props: KbParameters,
    ) -> crate::Result<()> {
        let ckcr = Self::change_keyboard_control_request(props);
        log::debug!("Sending ChangeKeyboardControlRequest to server.");
        let tok = self.send_request_async(ckcr).await?;
        log::debug!("Sent ChangeKeyboardControlRequest to server.");
        self.resolve_request_async(tok).await
    }

    #[inline]
    pub fn bell(&mut self, percent: i8) -> crate::Result {
        let bell = BellRequest {
            percent,
            ..Default::default()
        };
        log::debug!("Sending BellRequest to server.");
        let tok = self.send_request(bell)?;
        log::debug!("Sent BellRequest to server.");
        self.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn bell_async(&mut self, percent: i8) -> crate::Result {
        let bell = BellRequest {
            percent,
            ..Default::default()
        };
        log::debug!("Sending BellRequest to server.");
        let tok = self.send_request_async(bell).await?;
        log::debug!("Sent BellRequest to server.");
        self.resolve_request_async(tok).await
    }

    #[inline]
    pub fn set_access_control(&mut self, mode: AccessControl) -> crate::Result {
        let sacr = SetAccessControlRequest {
            mode,
            ..Default::default()
        };
        log::debug!("Sending SetAccessControlRequest to server.");
        let tok = self.send_request(sacr)?;
        log::debug!("Sent SetAccessControlRequest to server.");
        self.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_access_control_async(&mut self, mode: AccessControl) -> crate::Result {
        let sacr = SetAccessControlRequest {
            mode,
            ..Default::default()
        };
        log::debug!("Sending SetAccessControlRequest to server.");
        let tok = self.send_request_async(sacr).await?;
        log::debug!("Sent SetAccessControlRequest to server.");
        self.resolve_request_async(tok).await
    }

    #[inline]
    fn change_active_pointer_grab_request(
        event_mask: EventMask,
        cursor: Cursor,
        time: Option<Timestamp>,
    ) -> ChangeActivePointerGrabRequest {
        ChangeActivePointerGrabRequest {
            cursor,
            event_mask,
            time: time.unwrap_or(0),
            ..Default::default()
        }
    }

    #[inline]
    pub fn change_active_pointer_grab(
        &mut self,
        event_mask: EventMask,
        cursor: Cursor,
        time: Option<Timestamp>,
    ) -> crate::Result {
        let capgr = Self::change_active_pointer_grab_request(event_mask, cursor, time);
        log::debug!("Sending ChangeActivePointerGrabRequest to server.");
        let tok = self.send_request(capgr)?;
        log::debug!("Sent ChangeActivePointerGrabRequest to server.");
        self.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_active_pointer_grab_async(
        &mut self,
        event_mask: EventMask,
        cursor: Cursor,
        time: Option<Timestamp>,
    ) -> crate::Result {
        let capgr = Self::change_active_pointer_grab_request(event_mask, cursor, time);
        log::debug!("Sending ChangeActivePointerGrabRequest to server.");
        let tok = self.send_request_async(capgr).await?;
        log::debug!("Sent ChangeActivePointerGrabRequest to server.");
        self.resolve_request_async(tok).await
    }

    #[inline]
    pub fn set_close_down_mode(&mut self, mode: CloseDown) -> crate::Result {
        let scdmr = SetCloseDownModeRequest {
            mode,
            ..Default::default()
        };
        log::debug!("Sending SetCloseDownModeRequest to server.");
        let tok = self.send_request(scdmr)?;
        log::debug!("Sent SetCloseDownModeRequest to server.");
        self.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_close_down_mode_async(&mut self, mode: CloseDown) -> crate::Result {
        let scdmr = SetCloseDownModeRequest {
            mode,
            ..Default::default()
        };
        log::debug!("Sending SetCloseDownModeRequest to server.");
        let tok = self.send_request_async(scdmr).await?;
        log::debug!("Sent SetCloseDownModeRequest to server.");
        self.resolve_request_async(tok).await
    }

    #[inline]
    fn change_pointer_control_request(
        accel_numer: i16,
        accel_denom: i16,
        threshold: i16,
        do_accel: bool,
        do_threshold: bool,
    ) -> ChangePointerControlRequest {
        ChangePointerControlRequest {
            acceleration_numerator: accel_numer,
            acceleration_denominator: accel_denom,
            threshold,
            do_acceleration: do_accel,
            do_threshold,
            ..Default::default()
        }
    }

    #[inline]
    pub fn change_pointer_control(
        &mut self,
        accel_numerator: i16,
        accel_denominator: i16,
        threshold: i16,
        do_acceleration: bool,
        do_threshold: bool,
    ) -> crate::Result {
        let cpcr = Self::change_pointer_control_request(
            accel_numerator,
            accel_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        );
        log::debug!("Sending ChangePointerControlRequest to server.");
        let tok = self.send_request(cpcr)?;
        log::debug!("Sent ChangePointerControlRequest to server.");
        self.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_pointer_control_async(
        &mut self,
        accel_numerator: i16,
        accel_denominator: i16,
        threshold: i16,
        do_acceleration: bool,
        do_threshold: bool,
    ) -> crate::Result {
        let cpcr = Self::change_pointer_control_request(
            accel_numerator,
            accel_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        );
        log::debug!("Sending ChangePointerControlRequest to server.");
        let tok = self.send_request_async(cpcr).await?;
        log::debug!("Sent ChangePointerControlRequest to server.");
        self.resolve_request_async(tok).await
    }
}
