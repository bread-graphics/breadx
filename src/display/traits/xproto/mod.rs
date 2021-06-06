// MIT/Apache2 License

#![allow(clippy::similar_names)]

use crate::{
    auto::xproto::{
        AccessControl, ArcMode, Atom, AutoRepeatMode, BackingStore, BellRequest, CapStyle,
        ChangeActivePointerGrabRequest, ChangeGcRequest, ChangeKeyboardControlRequest,
        ChangePointerControlRequest, ChangeWindowAttributesRequest, CloseDown, Colormap,
        ColormapAlloc, CreateColormapRequest, CreateCursorRequest, CreateGcRequest,
        CreateWindowRequest, Cursor, Cw, Drawable, EventMask, FillRule, FillStyle, Font,
        ForceScreenSaverRequest, Gc, Gcontext, GetKeyboardMappingReply, GetKeyboardMappingRequest,
        GetModifierMappingReply, GetModifierMappingRequest, Gravity, Gx, InternAtomReply,
        InternAtomRequest, JoinStyle, Kb, Keycode, Keysym, LedMode, LineStyle, Pixmap,
        QueryExtensionReply, QueryExtensionRequest, ScreenSaver, SendEventRequest,
        SetAccessControlRequest, SetCloseDownModeRequest, SubwindowMode, Timestamp, Visualid,
        Window, WindowClass,
    },
    display::{generate_xid, Display, RequestCookie},
    util::BoxedFnOnce,
    Event, Extension,
};
use alloc::{boxed::Box, string::String};
use cty::c_char;

#[cfg(feature = "async")]
use crate::display::{
    futures::{ExchangeRequestFuture, ExchangeXidFuture, MapFuture, SendRequestFuture},
    AsyncDisplay,
};

mod colormap;
mod cursor;
mod drawable;
mod gcontext;
mod pixmap;
mod window;

pub use colormap::*;
pub use cursor::*;
pub use drawable::*;
pub use gcontext::*;
pub use pixmap::*;
pub use window::*;

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

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyboardMapping {
    pub keysyms_per_keycode: u8,
    pub keysyms: Box<[Keysym]>,
}

impl From<GetKeyboardMappingReply> for KeyboardMapping {
    #[inline]
    fn from(gkmr: GetKeyboardMappingReply) -> Self {
        Self {
            keysyms_per_keycode: gkmr.keysyms_per_keycode,
            keysyms: gkmr.keysyms.into_boxed_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModifierMapping {
    pub keycodes_per_modifier: u8,
    pub keycodes: Box<[Keycode]>,
}

impl From<GetModifierMappingReply> for ModifierMapping {
    #[inline]
    fn from(gmmr: GetModifierMappingReply) -> Self {
        Self {
            keycodes_per_modifier: gmmr.keycodes_per_modifier,
            keycodes: gmmr.keycodes.into_boxed_slice(),
        }
    }
}

#[inline]
fn create_window_request(
    wid: Window,
    parent: Window,
    class: WindowClass,
    depth: Option<u8>,
    visual: Option<Visualid>,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    props: WindowParameters,
) -> CreateWindowRequest {
    const INHERITED_DEPTH: u8 = 0;
    const INHERITED_VISUAL: Visualid = 0;

    let mut cwr = CreateWindowRequest {
        wid,
        parent,
        class,
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

/// Create a `CreateWindowRequest` but with less arguments.
#[inline]
fn create_simple_window_request(
    wid: Window,
    parent: Window,
    x: i16,
    y: i16,
    width: u16,
    height: u16,
    border_width: u16,
    border: u32,
    background: u32,
) -> CreateWindowRequest {
    let mut cwr = CreateWindowRequest {
        parent,
        x,
        y,
        width,
        height,
        border_width,
        depth: 0,
        class: WindowClass::CopyFromParent,
        visual: 0,
        wid,
        ..Default::default()
    };
    let wp = WindowParameters {
        background_pixel: Some(background),
        border_pixel: Some(border),
        ..Default::default()
    };

    let wpm = wp.convert_to_flags(&mut cwr);
    cwr.value_mask = wpm;
    cwr
}

/// Create a `CreateGcRequest`.
#[inline]
fn create_gc_request(cid: Gcontext, drawable: Drawable, props: GcParameters) -> CreateGcRequest {
    let mut gcr = CreateGcRequest {
        cid,
        drawable,
        ..Default::default()
    };

    let gcmask = props.convert_to_flags(&mut gcr);
    gcr.value_mask = gcmask;
    gcr
}

/// Create an `InternAtomRequest` for our use.
#[inline]
fn intern_atom_request(name: String, exists: bool) -> InternAtomRequest {
    InternAtomRequest {
        only_if_exists: exists,
        name,
        ..Default::default()
    }
}

/// Change Keyboard Control Request
#[inline]
fn change_keyboard_control_request(props: KbParameters) -> ChangeKeyboardControlRequest {
    let mut ckcr: ChangeKeyboardControlRequest = Default::default();
    let c = props.convert_to_flags(&mut ckcr);
    ckcr.value_mask = c;
    ckcr
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

/// Create a new cursor request.
#[inline]
fn create_cursor_request(
    cid: Cursor,
    source: Pixmap,
    mask: Pixmap,
    fg_red: u16,
    fg_green: u16,
    fg_blue: u16,
    bg_red: u16,
    bg_green: u16,
    bg_blue: u16,
    x: u16,
    y: u16,
) -> CreateCursorRequest {
    CreateCursorRequest {
        cid,
        source,
        mask,
        fore_red: fg_red,
        fore_blue: fg_blue,
        fore_green: fg_green,
        back_red: bg_red,
        back_blue: bg_blue,
        back_green: bg_green,
        x,
        y,
        ..Default::default()
    }
}

#[inline]
fn send_event_request(target: Window, em: EventMask, event: Event) -> SendEventRequest {
    let mut bytes: [u8; 32] = [0; 32];
    event.as_bytes(&mut bytes);

    bytes[0] = event.opcode();

    SendEventRequest {
        destination: target,
        event_mask: em,
        event: bytemuck::cast::<_, [c_char; 32]>(bytes),
        ..Default::default()
    }
}

/// Create a new colormap request.
#[inline]
fn create_colormap_request(
    alloc: ColormapAlloc,
    id: Colormap,
    win: Window,
    visual: Visualid,
) -> CreateColormapRequest {
    CreateColormapRequest {
        alloc,
        mid: id,
        window: win,
        visual,
        ..Default::default()
    }
}

pub trait DisplayXprotoExt: Display {
    /// Query for extension information.
    #[inline]
    fn query_extension(
        &mut self,
        name: String,
    ) -> crate::Result<RequestCookie<QueryExtensionRequest>> {
        self.send_request(QueryExtensionRequest {
            name,
            ..Default::default()
        })
    }

    /// Query for extension information, but resolve immediately. The `Error::ExtensionNotPresent` error is
    /// returned when the extension is not found.
    #[inline]
    fn query_extension_immediate(&mut self, name: String) -> crate::Result<Extension> {
        let qer = self.exchange_request(QueryExtensionRequest {
            name: name.clone(),
            ..Default::default()
        })?;
        Extension::from_reply(qer)
    }

    /// Create a new window.
    #[inline]
    fn create_window(
        &mut self,
        parent: Window,
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
        let wid = Window::const_from_xid(generate_xid(self)?);
        let cw = create_window_request(
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

        self.exchange_request(cw)?;
        Ok(wid)
    }

    /// Create a window, but assume some parameters from its parents.
    #[inline]
    fn create_simple_window(
        &mut self,
        parent: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        border: u32,
        background: u32,
    ) -> crate::Result<Window> {
        let wid = Window::const_from_xid(generate_xid(self)?);
        let cw = create_simple_window_request(
            wid,
            parent,
            x,
            y,
            width,
            height,
            border_width,
            border,
            background,
        );
        self.exchange_request(cw)?;
        Ok(wid)
    }

    /// Create a new graphics context for the specified target.
    #[inline]
    fn create_gc<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        props: GcParameters,
    ) -> crate::Result<Gcontext> {
        let gid = Gcontext::const_from_xid(generate_xid(self)?);
        let gcr = create_gc_request(gid, target.into(), props);
        self.exchange_request(gcr)?;
        Ok(gid)
    }

    /// Intern a string and get a corresponding atom for that string.
    #[inline]
    fn intern_atom(
        &mut self,
        name: String,
        only_if_exists: bool,
    ) -> crate::Result<RequestCookie<InternAtomRequest>> {
        self.send_request(intern_atom_request(name, only_if_exists))
    }

    /// Intern an atom, but try to resolve the request immediately.
    #[inline]
    fn intern_atom_immediate(&mut self, name: String, only_if_exists: bool) -> crate::Result<Atom> {
        let r = self.intern_atom(name, only_if_exists)?;
        Ok(self.resolve_request(r)?.atom)
    }

    /// Change the keyboard's control properties.
    #[inline]
    fn change_keyboard_control(&mut self, props: KbParameters) -> crate::Result<()> {
        let ckcr = change_keyboard_control_request(props);
        self.exchange_request(ckcr)
    }

    #[inline]
    fn bell(&mut self, percent: i8) -> crate::Result {
        self.exchange_request(BellRequest {
            percent,
            ..Default::default()
        })
    }

    #[inline]
    fn set_access_control(&mut self, mode: AccessControl) -> crate::Result {
        self.exchange_request(SetAccessControlRequest {
            mode,
            ..Default::default()
        })
    }

    #[inline]
    fn change_active_pointer_grab(
        &mut self,
        event_mask: EventMask,
        cursor: Cursor,
        time: Option<Timestamp>,
    ) -> crate::Result {
        self.exchange_request(change_active_pointer_grab_request(event_mask, cursor, time))
    }

    #[inline]
    fn set_close_down_mode(&mut self, mode: CloseDown) -> crate::Result {
        self.exchange_request(SetCloseDownModeRequest {
            mode,
            ..Default::default()
        })
    }

    #[inline]
    fn change_pointer_control(
        &mut self,
        accel_numerator: i16,
        accel_denominator: i16,
        threshold: i16,
        do_acceleration: bool,
        do_threshold: bool,
    ) -> crate::Result {
        self.exchange_request(change_pointer_control_request(
            accel_numerator,
            accel_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        ))
    }

    /// Create a new cursor.
    #[inline]
    fn create_cursor(
        &mut self,
        source: Pixmap,
        mask: Pixmap,
        fg_red: u16,
        fg_green: u16,
        fg_blue: u16,
        bg_red: u16,
        bg_green: u16,
        bg_blue: u16,
        x: u16,
        y: u16,
    ) -> crate::Result<Cursor> {
        let cid = Cursor::const_from_xid(generate_xid(self)?);
        self.exchange_request(create_cursor_request(
            cid, source, mask, fg_red, fg_green, fg_blue, bg_red, bg_green, bg_blue, x, y,
        ))?;
        Ok(cid)
    }

    #[inline]
    fn force_screensaver(&mut self, mode: ScreenSaver) -> crate::Result {
        self.exchange_request(ForceScreenSaverRequest {
            mode,
            ..Default::default()
        })
    }

    /// Send an event to the X server.
    #[inline]
    fn send_event(&mut self, target: Window, mask: EventMask, event: Event) -> crate::Result {
        self.exchange_request(send_event_request(target, mask, event))
    }

    /// Create a new colormap.
    #[inline]
    fn create_colormap(
        &mut self,
        window: Window,
        visual: Visualid,
        alloc: ColormapAlloc,
    ) -> crate::Result<Colormap> {
        let cid = Colormap::const_from_xid(generate_xid(self)?);
        self.exchange_request(create_colormap_request(alloc, cid, window, visual))?;
        Ok(cid)
    }

    /// Get the keyboard mapping for this display.
    #[inline]
    fn get_keyboard_mapping(&mut self) -> crate::Result<RequestCookie<GetKeyboardMappingRequest>> {
        let min_keycode = self.setup().min_keycode;
        let max_keycode = self.setup().max_keycode;

        self.send_request(GetKeyboardMappingRequest {
            first_keycode: min_keycode,
            count: max_keycode - min_keycode,
            ..Default::default()
        })
    }

    /// Immediately get the keyboard mapping for this display.
    #[inline]
    fn get_keyboard_mapping_immediate(&mut self) -> crate::Result<KeyboardMapping> {
        let tok = self.get_keyboard_mapping()?;
        let repl = self.resolve_request(tok)?;
        Ok(repl.into())
    }

    /// Get the modifier mapping for this display.
    #[inline]
    fn get_modifier_mapping(&mut self) -> crate::Result<RequestCookie<GetModifierMappingRequest>> {
        self.send_request(GetModifierMappingRequest::default())
    }

    /// Immediately get the modifier mapping for this display.
    #[inline]
    fn get_modifier_mapping_immediate(&mut self) -> crate::Result<ModifierMapping> {
        let tok = self.get_modifier_mapping()?;
        let repl = self.resolve_request(tok)?;
        Ok(repl.into())
    }
}

impl<D: Display + ?Sized> DisplayXprotoExt for D {}

#[cfg(feature = "async")]
pub trait AsyncDisplayXprotoExt: AsyncDisplay {
    /// Query for extension information redox.
    #[inline]
    fn query_extension_async(
        &mut self,
        name: String,
    ) -> SendRequestFuture<'_, Self, QueryExtensionRequest> {
        self.send_request_async(QueryExtensionRequest {
            name,
            ..Default::default()
        })
    }

    /// Query for extension information, but resolve immediately redox . The `Error::ExtensionNotPresent`
    /// error is returned when the extension is not found.
    #[inline]
    fn query_extension_immediate_async(
        &mut self,
        name: String,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, QueryExtensionRequest>,
        fn(crate::Result<QueryExtensionReply>) -> crate::Result<Extension>,
    > {
        MapFuture::run(
            self.exchange_request_async(QueryExtensionRequest {
                name,
                ..Default::default()
            }),
            |repl| repl.map(Extension::from_reply),
        )
    }

    /// Create a new window redox.
    #[inline]
    fn create_window_async(
        &mut self,
        parent: Window,
        class: WindowClass,
        depth: Option<u8>,
        visual: Option<Visualid>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        props: WindowParameters,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        CreateWindowRequest,
        Window,
        BoxedFnOnce<Window, CreateWindowRequest>,
    > {
        let mut cw = create_window_request(
            Window::const_from_xid(0),
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

        self.exchange_xid_async(Box::new(move |wid| {
            cw.wid = wid;
            cw
        }))
    }

    /// Create a window, but assume some parameters from its parents redox.
    #[inline]
    fn create_simple_window_async(
        &mut self,
        parent: Window,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        border_width: u16,
        border: u32,
        background: u32,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        CreateWindowRequest,
        Window,
        BoxedFnOnce<Window, CreateWindowRequest>,
    > {
        let mut cw = create_simple_window_request(
            Window::const_from_xid(0),
            parent,
            x,
            y,
            width,
            height,
            border_width,
            border,
            background,
        );

        self.exchange_xid_async(Box::new(move |wid| {
            cw.wid = wid;
            cw
        }))
    }

    /// Create a new graphics context redox.
    #[inline]
    fn create_gc_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        props: GcParameters,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        CreateGcRequest,
        Gcontext,
        BoxedFnOnce<Gcontext, CreateGcRequest>,
    > {
        let mut gcr = create_gc_request(Gcontext::const_from_xid(0), target.into(), props);
        self.exchange_xid_async(Box::new(move |gid| {
            gcr.gid = gid;
            gcr
        }))
    }

    /// Intern a string and get a corresponding atom for that string redox.
    #[inline]
    fn intern_atom_async(
        &mut self,
        name: String,
        only_if_exists: bool,
    ) -> SendRequestFuture<'_, Self, InternAtomRequest> {
        self.send_request_async(intern_atom_request(name, only_if_exists))
    }

    /// Intern an atom, but try to resolve the request immediately redox.
    #[inline]
    fn intern_atom_immediate_async(
        &mut self,
        name: String,
        only_if_exists: bool,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, InternAtomRequest>,
        fn(crate::Result<InternAtomReply>) -> crate::Result<Atom>,
    > {
        MapFuture::run(
            self.exchange_request_async(intern_atom_request(name, only_if_exists)),
            |repl| repl.map(|repl| repl.atom),
        )
    }

    /// Change the keyboard's control properties redox.
    #[inline]
    fn change_keyboard_control_async(
        &mut self,
        props: KbParameters,
    ) -> ExchangeRequestFuture<'_, Self, ChangeKeyboardControlRequest> {
        let ckcr = change_keyboard_control_request(props);
        self.exchange_request_async(ckcr)
    }

    #[inline]
    fn bell_async(&mut self, percent: i8) -> ExchangeRequestFuture<'_, Self, BellRequest> {
        self.exchange_request_async(BellRequest {
            percent,
            ..Default::default()
        })
    }

    #[inline]
    fn set_access_control_async(
        &mut self,
        mode: AccessControl,
    ) -> ExchangeRequestFuture<'_, Self, SetAccessControlRequest> {
        self.exchange_request_async(SetAccessControlRequest {
            mode,
            ..Default::default()
        })
    }

    #[inline]
    fn change_active_pointer_grab_async(
        &mut self,
        event_mask: EventMask,
        cursor: Cursor,
        time: Option<Timestamp>,
    ) -> ExchangeRequestFuture<'_, Self, ChangeActivePointerGrabRequest> {
        self.exchange_request(change_active_pointer_grab_request(event_mask, cursor, time))
    }

    #[inline]
    fn set_close_down_mode_async(
        &mut self,
        mode: CloseDown,
    ) -> ExchangeRequestFuture<'_, Self, SetCloseDownModeRequest> {
        self.exchange_request_async(SetCloseDownModeRequest {
            mode,
            ..Default::default()
        })
    }

    #[inline]
    fn change_pointer_control_async(
        &mut self,
        accel_numerator: i16,
        accel_denominator: i16,
        threshold: i16,
        do_acceleration: bool,
        do_threshold: bool,
    ) -> ExchangeRequestFuture<'_, Self, ChangePointerControlRequest> {
        self.exchange_request_async(change_pointer_control_request(
            accel_numerator,
            accel_denominator,
            threshold,
            do_acceleration,
            do_threshold,
        ))
    }

    /// Create a new cursor redox.
    #[inline]
    fn create_cursor_async(
        &mut self,
        source: Pixmap,
        mask: Pixmap,
        fg_red: u16,
        fg_green: u16,
        fg_blue: u16,
        bg_red: u16,
        bg_green: u16,
        bg_blue: u16,
        x: u16,
        y: u16,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        CreateCursorRequest,
        Cursor,
        BoxedFnOnce<Cursor, CreateCursorRequest>,
    > {
        let mut ccr = create_cursor_request(
            Cursor::const_from_xid(0),
            source,
            mask,
            fg_red,
            fg_green,
            fg_blue,
            bg_red,
            bg_green,
            bg_blue,
            x,
            y,
        );
        self.exchange_xid_async(Box::new(move |cid| {
            ccr.cid = cid;
            ccr
        }))
    }

    #[inline]
    fn force_screensaver_async(
        &mut self,
        mode: ScreenSaver,
    ) -> ExchangeRequestFuture<'_, Self, ForceScreenSaverRequest> {
        self.exchange_request_async(ForceScreenSaverRequest {
            mode,
            ..Default::default()
        })
    }

    /// Send an event to the X server redox.
    #[inline]
    fn send_event_async(
        &mut self,
        target: Window,
        mask: EventMask,
        event: Event,
    ) -> ExchangeRequestFuture<'_, Self, SendEventRequest> {
        self.exchange_request_async(send_event_request(target, mask, event))
    }

    /// Create a new colormap redox.
    #[inline]
    fn create_colormap_async(
        &mut self,
        window: Window,
        visual: Visualid,
        alloc: ColormapAlloc,
    ) -> ExchangeXidFuture<
        '_,
        Self,
        CreateColormapRequest,
        Colormap,
        BoxedFnOnce<Colormap, CreateColormapRequest>,
    > {
        let mut ccr = create_colormap_request(alloc, Colormap::const_from_xid(0), window, visual);
        self.exchange_xid_async(Box::new(move |cid| {
            ccr.mid = cid;
            ccr
        }))
    }

    /// Get the keyboard mapping for this display redox.
    #[inline]
    fn get_keyboard_mapping_async(
        &mut self,
    ) -> SendRequestFuture<'_, Self, GetKeyboardMappingRequest> {
        let min_keycode = self.setup().min_keycode;
        let max_keycode = self.setup().max_keycode;

        self.send_request_async(GetKeyboardMappingRequest {
            first_keycode: min_keycode,
            count: max_keycode - min_keycode,
            ..Default::default()
        })
    }

    /// Immediately get the keyboard mapping for this display redox.
    #[inline]
    fn get_keyboard_mapping_immediate_async(
        &mut self,
    ) -> ExchangeRequestFuture<'_, Self, GetKeyboardMappingRequest> {
        let min_keycode = self.setup().min_keycode;
        let max_keycode = self.setup().max_keycode;

        self.exchange_request_async(GetKeyboardMappingRequest {
            first_keycode: min_keycode,
            count: max_keycode - min_keycode,
            ..Default::default()
        })
    }

    /// Get the modifier mapping for this display redox.
    #[inline]
    fn get_modifier_mapping_async(
        &mut self,
    ) -> SendRequestFuture<'_, Self, GetModifierMappingRequest> {
        self.send_request_async(GetModifierMappingRequest::default())
    }

    /// Immediately get the modifier mapping for this display.
    #[inline]
    fn get_modifier_mapping_immediate_async(
        &mut self,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Self, GetModifierMappingRequest>,
        fn(crate::Result<GetModifierMappingReply>) -> crate::Result<ModifierMapping>,
    > {
        MapFuture::run(
            self.exchange_request_async(GetModifierMappingRequest::default()),
            |repl| repl.map(ModifierMapping::from),
        )
    }
}

#[cfg(feature = "async")]
impl<D: AsyncDisplay + ?Sized> AsyncDisplayXprotoExt for D {}
