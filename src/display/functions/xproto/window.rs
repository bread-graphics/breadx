// MIT/Apache2 License

#![allow(clippy::similar_names)]

use super::Geometry as DrawableGeometry;
pub use crate::{
    auto::{
        xproto::{
            Atom, BackingStore, ChangePropertyRequest, ChangeSaveSetRequest,
            ChangeWindowAttributesRequest, Circulate, CirculateWindowRequest, ClearAreaRequest,
            Colormap, ConfigWindow, ConfigureWindowRequest, ConvertSelectionRequest, Cursor,
            DeletePropertyRequest, DestroySubwindowsRequest, DestroyWindowRequest, EventMask,
            Gcontext, GetGeometryRequest, GetWindowAttributesReply, GetWindowAttributesRequest,
            Gravity, MapState, MapWindowRequest, PropMode, SetMode, StackMode, Timestamp, Visualid,
            Window, WindowClass, ATOM_WM_NAME,
        },
        AsByteSequence,
    },
    display::{Connection, Display, RequestCookie, WindowParameters},
    send_request, sr_request,
    xid::XidType,
};
use alloc::{string::ToString, vec::Vec};
use core::{iter, mem};

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

// macro for retrieving an atom that might be cached in the display
macro_rules! retrieve_atom {
    ($dpy: expr, $dfield: ident, $name: expr) => {{
        match $dpy.$dfield {
            Some(wpa) => Atom::const_from_xid(wpa.get()),
            None => {
                let wpa = $dpy.intern_atom_immediate(($name).to_string(), false)?;
                if wpa.xid() == 0 {
                    log::error!("Unable to intern {} atom", $name);
                    return Ok(());
                }

                $dpy.$dfield = core::num::NonZeroU32::new(wpa.xid());
                wpa
            }
        }
    }};
}

#[cfg(feature = "async")]
macro_rules! retrieve_atom_async {
    ($dpy: expr, $dfield: ident, $name: expr) => {{
        match $dpy.$dfield {
            Some(wpa) => Atom::const_from_xid(wpa.get()),
            None => {
                let wpa = $dpy
                    .intern_atom_immediate_async(($name).to_string(), false)
                    .await?;
                if wpa.xid() == 0 {
                    log::error!("Unable to intern {} atom", $name);
                    return Ok(());
                } else {
                    $dpy.$dfield = core::num::NonZeroU32::new(wpa.xid());
                    wpa
                }
            }
        }
    }};
}

crate::create_paramaterizer! {
    pub struct ConfigureWindowParameters : (ConfigWindow, ConfigureWindowRequest) {
        x            (set_x,            x)            : i32,
        y            (set_y,            y)            : i32,
        width        (set_width,        width)        : u32,
        height       (set_height,       height)       : u32,
        border_width (set_border_width, border_width) : u32,
        sibling      (set_sibling,      sibling)      : Window,
        stack_mode   (set_stack_mode,   stack_mode)   : StackMode
    }
}

/// The return type of `Window::window_attributes_immediate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WindowAttributes {
    pub backing_store: BackingStore,
    pub visual: Visualid,
    pub class: WindowClass,
    pub bit_gravity: Gravity,
    pub win_gravity: Gravity,
    pub backing_planes: u32,
    pub backing_pixel: u32,
    pub save_under: bool,
    pub map_is_installed: bool,
    pub map_state: MapState,
    pub override_redirect: bool,
    pub colormap: Colormap,
    pub all_event_masks: EventMask,
    pub your_event_mask: EventMask,
    pub do_not_propagate_mask: EventMask,
}

impl From<GetWindowAttributesReply> for WindowAttributes {
    #[inline]
    fn from(gwar: GetWindowAttributesReply) -> Self {
        convert_get_window_attributes_reply(gwar)
    }
}

/// The return type of `Window::geometry_immediate`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct WindowGeometry {
    pub depth: u8,
    pub root: Window,
    pub x: i16,
    pub y: i16,
    pub width: u16,
    pub height: u16,
    pub border_width: u16,
}

impl Window {
    /// Map this window to the screen.
    #[inline]
    pub fn map<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        sr_request!(
            dpy,
            MapWindowRequest {
                window: self,
                ..Default::default()
            }
        )
    }

    /// Map this window to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn map_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result {
        sr_request!(
            dpy,
            MapWindowRequest {
                window: self,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Request struct to change the property of a window.
    #[inline]
    fn change_property_request<T: AsByteSequence>(
        self,
        property: Atom,
        property_type: PropertyType,
        format: PropertyFormat,
        mode: PropMode,
        data: &[T],
    ) -> ChangePropertyRequest {
        // convert to a u8 collection
        let mut data_bytes: Vec<u8> = iter::repeat(0)
            .take(mem::size_of::<T>() * data.len())
            .collect();
        let len = data.iter().fold(0, |index, item| {
            item.as_bytes(&mut data_bytes[index..]) + index
        });
        data_bytes.truncate(len);

        let format = format as u8;

        ChangePropertyRequest {
            mode,
            window: self,
            property,
            ty: Atom::const_from_xid(property_type as u32),
            format,
            data_len: data.len() as u32,
            data: data_bytes,
            ..Default::default()
        }
    }

    /// Change a property of the window, given an atom that identifies that property.
    #[inline]
    pub fn change_property<Conn: Connection, T: AsByteSequence>(
        self,
        dpy: &mut Display<Conn>,
        property: Atom,
        property_type: PropertyType,
        format: PropertyFormat,
        mode: PropMode,
        data: &[T],
    ) -> crate::Result<()> {
        sr_request!(
            dpy,
            self.change_property_request(property, property_type, format, mode, data,)
        )
    }

    /// Change a property of the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_property_async<Conn: AsyncConnection + Send, T: AsByteSequence>(
        self,
        dpy: &mut Display<Conn>,
        property: Atom,
        property_type: PropertyType,
        format: PropertyFormat,
        mode: PropMode,
        data: &[T],
    ) -> crate::Result<()> {
        sr_request!(
            dpy,
            self.change_property_request(property, property_type, format, mode, data,),
            async
        )
        .await
    }

    /// Delete a property of this window.
    #[inline]
    pub fn delete_property<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        property: Atom,
    ) -> crate::Result {
        sr_request!(
            dpy,
            DeletePropertyRequest {
                window: self,
                property,
                ..Default::default()
            }
        )
    }

    /// Delete a property of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn delete_property_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        property: Atom,
    ) -> crate::Result {
        sr_request!(
            dpy,
            DeletePropertyRequest {
                window: self,
                property,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Set the protocols for the WM in regards to this window.
    #[inline]
    pub fn set_wm_protocols<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        protocols: &[Atom],
    ) -> crate::Result<()> {
        let wm_protocols_atom = retrieve_atom!(dpy, wm_protocols_atom, "WM_PROTOCOLS");

        self.change_property(
            dpy,
            wm_protocols_atom,
            PropertyType::Atom,
            PropertyFormat::ThirtyTwo,
            PropMode::Replace,
            protocols,
        )
    }

    /// Set the WM protocols for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_wm_protocols_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        protocols: &[Atom],
    ) -> crate::Result<()> {
        let wm_protocols_atom = retrieve_atom_async!(dpy, wm_protocols_atom, "WM_PROTOCOLS");

        self.change_property_async(
            dpy,
            wm_protocols_atom,
            PropertyType::Atom,
            PropertyFormat::ThirtyTwo,
            PropMode::Replace,
            protocols,
        )
        .await
    }

    /// Set the title for this window.
    #[inline]
    pub fn set_title<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        title: &str,
    ) -> crate::Result<()> {
        self.change_property(
            dpy,
            ATOM_WM_NAME,
            PropertyType::String,
            PropertyFormat::Eight,
            PropMode::Replace,
            title.as_bytes(),
        )
    }

    /// Set the title for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_title_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        title: &str,
    ) -> crate::Result<()> {
        self.change_property_async(
            dpy,
            ATOM_WM_NAME,
            PropertyType::String,
            PropertyFormat::Eight,
            PropMode::Replace,
            title.as_bytes(),
        )
        .await
    }

    /// `GetWindowAttributesRequest`
    #[inline]
    fn get_window_attributes_request(self) -> GetWindowAttributesRequest {
        GetWindowAttributesRequest {
            window: self,
            ..Default::default()
        }
    }

    /// Get the current set of window attributes for this window.
    #[inline]
    pub fn window_attributes<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<RequestCookie<GetWindowAttributesRequest>> {
        send_request!(dpy, self.get_window_attributes_request())
    }

    /// Get the current set of window attributes for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn window_attributes_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<RequestCookie<GetWindowAttributesRequest>> {
        send_request!(dpy, self.get_window_attributes_request(), async).await
    }

    /// Immediately get the current set of window attributes for this window.
    #[inline]
    pub fn window_attributes_immediate<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<WindowAttributes> {
        let tok = self.window_attributes(dpy)?;
        Ok(convert_get_window_attributes_reply(
            dpy.resolve_request(tok)?,
        ))
    }

    /// Immediately get the current set of window attributes for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn window_attributes_immediate_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<WindowAttributes> {
        let tok = self.window_attributes_async(dpy).await?;
        Ok(convert_get_window_attributes_reply(
            dpy.resolve_request_async(tok).await?,
        ))
    }

    /// Get the geometry of this window.
    #[inline]
    pub fn geometry<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        dpy.get_drawable_geometry(self)
    }

    /// Get the geometry of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn geometry_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        dpy.get_drawable_geometry_async(self).await
    }

    /// Immediately get the geometry of this window.
    #[inline]
    pub fn geometry_immediate<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<DrawableGeometry> {
        dpy.get_drawable_geometry_immediate(self)
    }

    /// Immediately get the geometry of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn geometry_immediate_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<DrawableGeometry> {
        dpy.get_drawable_geometry_immediate_async(self).await
    }

    /// Request to change this window's parameters.
    #[inline]
    fn change_window_attrs_request(self, props: WindowParameters) -> ChangeWindowAttributesRequest {
        let mut cwar = ChangeWindowAttributesRequest {
            window: self,
            ..Default::default()
        };

        let c = props.mask_to_change_attrs_request(&mut cwar);
        cwar.value_mask = c;
        cwar
    }

    /// Change the properties of this window.
    #[inline]
    pub fn change_attributes<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        props: WindowParameters,
    ) -> crate::Result {
        sr_request!(dpy, self.change_window_attrs_request(props))
    }

    /// Change the properties of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_attributes_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        props: WindowParameters,
    ) -> crate::Result {
        sr_request!(dpy, self.change_window_attrs_request(props), async).await
    }

    /// Set this window's background color.
    #[inline]
    pub fn set_background_color<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        clr: u32,
    ) -> crate::Result<()> {
        let props = WindowParameters {
            background_pixel: Some(clr),
            ..Default::default()
        };
        self.change_attributes(dpy, props)
    }

    /// Set this window's background color, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_background_color_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        clr: u32,
    ) -> crate::Result<()> {
        let mut props: WindowParameters = Default::default();
        props.background_pixel = Some(clr);
        self.change_attributes_async(dpy, props).await
    }

    /// Request to configure window.
    #[inline]
    fn configure_window_request(self, props: ConfigureWindowParameters) -> ConfigureWindowRequest {
        let mut cwr = ConfigureWindowRequest {
            window: self,
            ..Default::default()
        };
        let c = props.convert_to_flags(&mut cwr);
        cwr.value_mask = c;
        cwr
    }

    /// Configure the window's physical properties.
    #[inline]
    pub fn configure<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        props: ConfigureWindowParameters,
    ) -> crate::Result {
        sr_request!(dpy, self.configure_window_request(props))
    }

    /// Configure the window's physical properties, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn configure_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        props: ConfigureWindowParameters,
    ) -> crate::Result {
        sr_request!(dpy, self.configure_window_request(props), async).await
    }

    /// Set the border of this window.
    #[inline]
    pub fn set_border_width<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        width: u32,
    ) -> crate::Result {
        let props = ConfigureWindowParameters {
            border_width: Some(width),
            ..Default::default()
        };
        self.configure(dpy, props)
    }

    /// Set the border of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_border_width_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        width: u32,
    ) -> crate::Result {
        let props = ConfigureWindowParameters {
            border_width: Some(width),
            ..Default::default()
        };
        self.configure_async(dpy, props).await
    }

    /// Change the colormap associated with this window.
    #[inline]
    pub fn set_colormap<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        colormap: Colormap,
    ) -> crate::Result {
        let props = WindowParameters {
            colormap: Some(colormap),
            ..Default::default()
        };
        self.change_attributes(dpy, props)
    }

    /// Change the colormap associated with this window.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_colormap_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        colormap: Colormap,
    ) -> crate::Result {
        let mut props: WindowParameters = Default::default();
        props.colormap = Some(colormap);
        self.change_attributes_async(dpy, props).await
    }

    #[inline]
    fn change_save_set_request(self, mode: SetMode) -> ChangeSaveSetRequest {
        ChangeSaveSetRequest {
            mode,
            window: self,
            ..Default::default()
        }
    }

    #[inline]
    pub fn change_save_set<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        mode: SetMode,
    ) -> crate::Result {
        sr_request!(dpy, self.change_save_set_request(mode))
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_save_set_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        mode: SetMode,
    ) -> crate::Result {
        sr_request!(dpy, self.change_save_set_request(mode), async).await
    }

    /// Resize the window.
    #[inline]
    pub fn resize<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        width: u32,
        height: u32,
    ) -> crate::Result {
        let props = ConfigureWindowParameters {
            width: Some(width),
            height: Some(height),
            ..Default::default()
        };
        self.configure(dpy, props)
    }

    /// Resize the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn resize_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        width: u32,
        height: u32,
    ) -> crate::Result {
        let mut props: ConfigureWindowParameters = Default::default();
        props.width = Some(width);
        props.height = Some(height);
        self.configure_async(dpy, props).await
    }

    /// Move and resize the window.
    #[inline]
    pub fn move_resize<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> crate::Result {
        let props = ConfigureWindowParameters {
            x: Some(x),
            y: Some(y),
            width: Some(width),
            height: Some(height),
            ..Default::default()
        };
        self.configure(dpy, props)
    }

    /// Move and resize the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn move_resize_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        x: i32,
        y: i32,
        width: u32,
        height: u32,
    ) -> crate::Result {
        let props = ConfigureWindowParameters {
            x: Some(x),
            y: Some(y),
            width: Some(width),
            height: Some(height),
            ..Default::default()
        };
        self.configure_async(dpy, props).await
    }

    #[inline]
    fn circulate_window_request(self, direction: Circulate) -> CirculateWindowRequest {
        CirculateWindowRequest {
            window: self,
            direction,
            ..Default::default()
        }
    }

    #[inline]
    pub fn circulate<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        direction: Circulate,
    ) -> crate::Result {
        sr_request!(dpy, self.circulate_window_request(direction))
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn circulate_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        direction: Circulate,
    ) -> crate::Result {
        sr_request!(dpy, self.circulate_window_request(direction), async).await
    }

    /// Clear Window Request
    #[inline]
    fn clear_area_request(
        self,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        exposures: bool,
    ) -> ClearAreaRequest {
        ClearAreaRequest {
            exposures,
            x,
            y,
            width,
            height,
            window: self,
            ..Default::default()
        }
    }

    /// Clear an area of the window.
    #[inline]
    pub fn clear_area<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        exposures: bool,
    ) -> crate::Result {
        sr_request!(dpy, self.clear_area_request(x, y, width, height, exposures))
    }

    /// Clear an area of the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn clear_area_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        exposures: bool,
    ) -> crate::Result {
        sr_request!(
            dpy,
            self.clear_area_request(x, y, width, height, exposures),
            async
        )
        .await
    }

    /// Clear the entire window.
    #[inline]
    pub fn clear<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        self.clear_area(dpy, 0, 0, 0, 0, false) // means: clear the whole dang thing
    }

    /// Clear the entire window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn clear_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result {
        self.clear_area_async(dpy, 0, 0, 0, 0, false).await
    }

    #[inline]
    fn convert_selection_request(
        self,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Timestamp,
    ) -> ConvertSelectionRequest {
        ConvertSelectionRequest {
            requestor: self,
            selection,
            target,
            property,
            time,
            ..Default::default()
        }
    }

    #[inline]
    pub fn convert_selection<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Timestamp,
    ) -> crate::Result {
        sr_request!(
            dpy,
            self.convert_selection_request(selection, target, property, time)
        )
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn convert_selection_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Timestamp,
    ) -> crate::Result {
        sr_request!(
            dpy,
            self.convert_selection_request(selection, target, property, time),
            async
        )
        .await
    }

    /// Set the cursor used by this window.
    #[inline]
    pub fn set_cursor<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        cursor: Cursor,
    ) -> crate::Result {
        let props = WindowParameters {
            cursor: Some(cursor),
            ..Default::default()
        };
        self.change_attributes(dpy, props)
    }

    /// Set the cursor used by this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_cursor_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        cursor: Cursor,
    ) -> crate::Result {
        let props = WindowParameters {
            cursor: Some(cursor),
            ..Default::default()
        };
        self.change_attributes_async(dpy, props).await
    }

    /// Destroy this window's subwindows.
    #[inline]
    pub fn destroy_subwindows<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        sr_request!(
            dpy,
            DestroySubwindowsRequest {
                window: self,
                ..Default::default()
            }
        )
    }

    /// Destroy this window's subwindows, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn destroy_subwindows_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result {
        sr_request!(
            dpy,
            DestroySubwindowsRequest {
                window: self,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Free this window.
    #[inline]
    pub fn free<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        sr_request!(
            dpy,
            DestroyWindowRequest {
                window: self,
                ..Default::default()
            }
        )
    }

    /// Free this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result {
        sr_request!(
            dpy,
            DestroyWindowRequest {
                window: self,
                ..Default::default()
            },
            async
        )
        .await
    }

    /// Set the event mask.
    #[inline]
    pub fn set_event_mask<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        em: EventMask,
    ) -> crate::Result {
        self.change_attributes(
            dpy,
            WindowParameters {
                event_mask: Some(em),
                ..Default::default()
            },
        )
    }

    /// Set the event mask, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_event_mask_async<Conn: AsyncConnection + Send>(
        self,
        dpy: &mut Display<Conn>,
        em: EventMask,
    ) -> crate::Result {
        self.change_attributes_async(
            dpy,
            WindowParameters {
                event_mask: Some(em),
                ..Default::default()
            },
        )
        .await
    }
}

/// Convert a `GetWindowAttributesReply` to a `WindowAttributes` struct.
#[inline]
fn convert_get_window_attributes_reply(reply: GetWindowAttributesReply) -> WindowAttributes {
    WindowAttributes {
        backing_store: reply.backing_store,
        visual: reply.visual,
        class: reply.class,
        bit_gravity: reply.bit_gravity,
        win_gravity: reply.win_gravity,
        backing_planes: reply.backing_planes,
        backing_pixel: reply.backing_pixel,
        save_under: reply.save_under,
        map_is_installed: reply.map_is_installed,
        map_state: reply.map_state,
        override_redirect: reply.override_redirect,
        colormap: reply.colormap,
        all_event_masks: reply.all_event_masks,
        your_event_mask: reply.your_event_mask,
        do_not_propagate_mask: reply.do_not_propagate_mask,
    }
}

/// The type of the property being changed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u32)]
pub enum PropertyType {
    Atom = 4,
    String = 31,
}

/// The format of the property being changed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PropertyFormat {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
}
