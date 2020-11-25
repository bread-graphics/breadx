// MIT/Apache2 License

pub use crate::{
    auto::{
        xproto::{
            Atom, BackingStore, ChangePropertyRequest, ChangeSaveSetRequest,
            ChangeWindowAttributesRequest, Circulate, CirculateWindowRequest, ClearAreaRequest,
            Colormap, ConfigWindow, ConfigureWindowRequest, ConvertSelectionRequest, EventMask,
            Gcontext, GetGeometryRequest, GetWindowAttributesReply, GetWindowAttributesRequest,
            Gravity, MapState, MapWindowRequest, PropMode, SetMode, StackMode, Timestamp, Visualid,
            Window, WindowClass, ATOM_WM_NAME,
        },
        AsByteSequence,
    },
    display::{Connection, Display, RequestCookie, WindowParameters},
    drawable::{self, Geometry as DrawableGeomtry},
    xid::XidType,
};
use alloc::{string::ToString, vec::Vec};
use core::{iter, mem};

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
                } else {
                    $dpy.$dfield = core::num::NonZeroU32::new(wpa.xid());
                    wpa
                }
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
        sibling      (set_sibling,      sibling)       : Window,
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
        let mw = MapWindowRequest {
            window: self,
            ..Default::default()
        };

        log::debug!("Sending MapWindowRequest to server");
        let tok = dpy.send_request(mw)?;
        log::debug!("Sent MapWindowRequest to server");
        dpy.resolve_request(tok)
    }

    /// Map this window to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn map_async<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        let mw = MapWindowRequest {
            window: self,
            ..Default::default()
        };
        let tok = dpy.send_request_async(mw).await?;
        dpy.resolve_request_async(tok).await
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
        log::debug!("Sending ChangePropertyRequest to server");
        let tok = dpy.send_request(self.change_property_request(
            property,
            property_type,
            format,
            mode,
            data,
        ))?;
        log::debug!("Sent ChangePropertyRequest to server");
        dpy.resolve_request(tok)
    }

    /// Change a property of the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_property_async<Conn: Connection, T: AsByteSequence>(
        self,
        dpy: &mut Display<Conn>,
        property: Atom,
        property_type: PropertyType,
        format: PropertyFormat,
        mode: PropMode,
        data: &[T],
    ) -> crate::Result<()> {
        log::debug!("Sending ChangePropertyRequest to server");
        let tok = dpy
            .send_request_async(self.change_property_request(
                property,
                property_type,
                format,
                mode,
                data,
            ))
            .await?;
        log::debug!("Sent ChangePropertyRequest to server");
        dpy.resolve_request_async(tok).await
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
    pub async fn set_wm_protocols_async<Conn: Connection>(
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
    pub async fn set_title_async<Conn: Connection>(
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
        let gwar = self.get_window_attributes_request();
        log::debug!("Sending GetWindowAttributesRequest to server.");
        let tok = dpy.send_request(gwar)?;
        log::debug!("Sent GetWindowAttributesRequest to server.");
        Ok(tok)
    }

    /// Get the current set of window attributes for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn window_attributes_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<RequestCookie<GetWindowAttributesRequest>> {
        let gwar = self.get_window_attributes_request();
        log::debug!("Sending GetWindowAttributesRequest to server.");
        let tok = dpy.send_request_async(gwar).await?;
        log::debug!("Sent GetWindowAttributesRequest to server.");
        Ok(tok)
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
    pub async fn window_attributes_immediate_async<Conn: Connection>(
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
        drawable::get_geometry(dpy, self)
    }

    /// Get the geometry of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn geometry_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        drawable::get_geometry_async(dpy, self).await
    }

    /// Immediately get the geometry of this window.
    #[inline]
    pub fn geometry_immediate<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<DrawableGeomtry> {
        drawable::get_geometry_immediate(dpy, self)
    }

    /// Immediately get the geometry of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn geometry_immediate_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
    ) -> crate::Result<DrawableGeomtry> {
        drawable::get_geometry_immediate_async(dpy, self).await
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
        let cwar = self.change_window_attrs_request(props);
        log::debug!("Sending ChangeWindowAttributesRequest to server.");
        let tok = dpy.send_request(cwar)?;
        log::debug!("Sent ChangeWindowAttributesRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Change the properties of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_attributes_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        props: WindowParameters,
    ) -> crate::Result {
        let cwar = self.change_window_attrs_request(props);
        log::debug!("Sending ChangeWindowAttributesRequest to server.");
        let tok = dpy.send_request_async(cwar).await?;
        log::debug!("Sent ChangeWindowAttributesRequest to server.");
        dpy.resolve_request_async(tok).await
    }

    /// Set this window's background color.
    #[inline]
    pub fn set_background_color<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        clr: u32,
    ) -> crate::Result<()> {
        let mut props: WindowParameters = Default::default();
        props.background_pixel = Some(clr);
        self.change_attributes(dpy, props)
    }

    /// Set this window's background color, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_background_color_async<Conn: Connection>(
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
        let cwr = self.configure_window_request(props);
        log::debug!("Sending ConfigureWindowRequest to server.");
        let tok = dpy.send_request(cwr)?;
        log::debug!("Sent ConfigureWindowRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Configure the window's physical properties, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn configure_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        props: ConfigureWindowParameters,
    ) -> crate::Result {
        let cwr = self.configure_window_request(props);
        log::debug!("Sending ConfigureWindowRequest to server.");
        let tok = dpy.send_request_async(cwr).await?;
        log::debug!("Sent ConfigureWindowRequest to server.");
        dpy.resolve_request_async(tok).await
    }

    /// Set the border of this window.
    #[inline]
    pub fn set_border_width<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        width: u32,
    ) -> crate::Result {
        let mut props: ConfigureWindowParameters = Default::default();
        props.border_width = Some(width);
        self.configure(dpy, props)
    }

    /// Set the border of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_border_width_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        width: u32,
    ) -> crate::Result {
        let mut props: ConfigureWindowParameters = Default::default();
        props.border_width = Some(width);
        self.configure_async(dpy, props).await
    }

    /// Change the colormap associated with this window.
    #[inline]
    pub fn set_colormap<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        colormap: Colormap,
    ) -> crate::Result {
        let mut props: WindowParameters = Default::default();
        props.colormap = Some(colormap);
        self.change_attributes(dpy, props)
    }

    /// Change the colormap associated with this window.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_colormap_async<Conn: Connection>(
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
        let cssr = self.change_save_set_request(mode);
        log::debug!("Sending ChangeSaveSetRequest to server.");
        let tok = dpy.send_request(cssr)?;
        log::debug!("Sent ChangeSaveSetRequest to server.");
        dpy.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_save_set_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        mode: SetMode,
    ) -> crate::Result {
        let cssr = self.change_save_set_request(mode);
        log::debug!("Sending ChangeSaveSetRequest to server.");
        let tok = dpy.send_request_async(cssr).await?;
        log::debug!("Sent ChangeSaveSetRequest to server.");
        dpy.resolve_request_async(tok).await
    }

    /// Resize the window.
    #[inline]
    pub fn resize<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        width: u32,
        height: u32,
    ) -> crate::Result {
        let mut props: ConfigureWindowParameters = Default::default();
        props.width = Some(width);
        props.height = Some(height);
        self.configure(dpy, props)
    }

    /// Resize the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn resize_async<Conn: Connection>(
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
    pub async fn move_resize_async<Conn: Connection>(
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
        let cwr = self.circulate_window_request(direction);
        log::debug!("Sending CirculateWindowRequest to server.");
        let tok = dpy.send_request(cwr)?;
        log::debug!("Sent CirculateWindowRequest to server.");
        dpy.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn circulate_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        direction: Circulate,
    ) -> crate::Result {
        let cwr = self.circulate_window_request(direction);
        log::debug!("Sending CirculateWindowRequest to server.");
        let tok = dpy.send_request_async(cwr).await?;
        log::debug!("Sent CirculateWindowRequest to server.");
        dpy.resolve_request_async(tok).await
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
        let car = self.clear_area_request(x, y, width, height, exposures);
        log::debug!("Sending ClearAreaRequest to server.");
        let tok = dpy.send_request(car)?;
        log::debug!("Sent ClearAreaRequest to server.");
        dpy.resolve_request(tok)
    }

    /// Clear an area of the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn clear_area_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        exposures: bool,
    ) -> crate::Result {
        let car = self.clear_area_request(x, y, width, height, exposures);
        log::debug!("Sending ClearAreaRequest to server.");
        let tok = dpy.send_request_async(car).await?;
        log::debug!("Sent ClearAreaRequest to server.");
        dpy.resolve_request_async(tok).await
    }

    /// Clear the entire window.
    #[inline]
    pub fn clear<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
        self.clear_area(dpy, 0, 0, 0, 0, false) // means: clear the whole dang thing
    }

    /// Clear the entire window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn clear_async<Conn: Connection>(self, dpy: &mut Display<Conn>) -> crate::Result {
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
        let csr = self.convert_selection_request(selection, target, property, time);
        log::debug!("Sending ConvertSelectionRequest to server.");
        let tok = dpy.send_request(csr)?;
        log::debug!("Sent ConvertSelectionRequest to server.");
        dpy.resolve_request(tok)
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn convert_selection_async<Conn: Connection>(
        self,
        dpy: &mut Display<Conn>,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Timestamp,
    ) -> crate::Result {
        let csr = self.convert_selection_request(selection, target, property, time);
        log::debug!("Sending ConvertSelectionRequest to server.");
        let tok = dpy.send_request_async(csr).await?;
        log::debug!("Sent ConvertSelectionRequest to server.");
        dpy.resolve_request_async(tok).await
    }
}

/// Convert a `GetWindowAttributesReply` to a `WindowAttributes` struct.
#[inline]
fn convert_get_window_attributes_reply(reply: GetWindowAttributesReply) -> WindowAttributes {
    let mut wa: WindowAttributes = Default::default();
    wa.backing_store = reply.backing_store;
    wa.visual = reply.visual;
    wa.class = reply.class;
    wa.bit_gravity = reply.bit_gravity;
    wa.win_gravity = reply.win_gravity;
    wa.backing_planes = reply.backing_planes;
    wa.backing_pixel = reply.backing_pixel;
    wa.save_under = reply.save_under;
    wa.map_is_installed = reply.map_is_installed;
    wa.map_state = reply.map_state;
    wa.override_redirect = reply.override_redirect;
    wa.colormap = reply.colormap;
    wa.all_event_masks = reply.all_event_masks;
    wa.your_event_mask = reply.your_event_mask;
    wa.do_not_propagate_mask = reply.do_not_propagate_mask;
    wa
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
