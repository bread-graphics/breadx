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
            Gravity, MapState, MapSubwindowsRequest, MapWindowRequest, PropMode, QueryTreeReply,
            QueryTreeRequest, ReparentWindowRequest, SetMode, StackMode, Timestamp,
            UnmapSubwindowsRequest, UnmapWindowRequest, Visualid, Window, WindowClass,
            ATOM_WM_NAME, GetPropertyRequest,
        },
        AsByteSequence,
    },
    display::{prelude::*, Connection, Display, DisplayExt, RequestCookie, WindowParameters},
    xid::XidType,
};
use alloc::{borrow::Cow, string::ToString, vec::Vec, boxed::Box};
use core::{iter, mem};

#[cfg(feature = "async")]
use crate::display::{traits::AsyncDisplayDrawableExt, AsyncDisplay, futures::SendRequestFuture};
#[cfg(feature = "async")]
use futures_lite::future::{self, Ready};

// macro for retrieving an atom that might be cached in the display
macro_rules! retrieve_atom {
    ($dpy: expr, $dgetter: ident, $dsetter: ident, $name: expr) => {{
        match $dpy.$dgetter() {
            Some(wpa) => Atom::const_from_xid(wpa.get()),
            None => {
                let wpa = $dpy.intern_atom_immediate(($name).to_string(), false)?;
                if wpa.xid() == 0 {
                    log::error!("Unable to intern {} atom", $name);
                    return Ok(());
                }

                $dpy.$dsetter(core::num::NonZeroU32::new(wpa.xid()).unwrap());
                wpa
            }
        }
    }};
}

#[cfg(feature = "async")]
macro_rules! retrieve_atom_async {
    ($dpy: expr, $dgetter: ident, $dsetter: ident, $name: expr) => {{
        match $dpy.$dgetter() {
            Some(wpa) => Atom::const_from_xid(wpa.get()),
            None => {
                let wpa = $dpy
                    .intern_atom_immediate_async(($name).to_string(), false)
                    .await?;
                if wpa.xid() == 0 {
                    log::error!("Unable to intern {} atom", $name);
                    return Ok(());
                } else {
                    $dpy.$dsetter(core::num::NonZeroU32::new(wpa.xid()).unwrap());
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
#[derive(Debug, Clone, Copy, PartialEq, Default)]
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

/// Information regarding a window's family tree.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Hash)]
pub struct TreeInformation {
    /// Root window, i.e. the screen the window exists on.
    pub root: Window,
    /// Parent window.
    pub parent: Window,
    /// All of the window's children.
    pub children: Box<[Window]>,
}

impl<'a> From<QueryTreeReply<'a>> for TreeInformation {
    #[inline]
    fn from(qtr: QueryTreeReply) -> Self {
        let QueryTreeReply {
            root,
            parent,
            children,
            ..
        } = qtr;
        Self {
            root,
            parent,
            children: children.into_owned().into_boxed_slice(),
        }
    }
}

impl Window {
    /// Map this window to the screen.
    #[inline]
    pub fn map<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(MapWindowRequest {
            window: self,
            ..Default::default()
        })
    }

    /// Map this window to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn map_async<Dpy: AsyncDisplay + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request_async(MapWindowRequest {
            window: self,
            ..Default::default()
        })
        .await
    }

    /// Unmap this window.
    #[inline]
    pub fn unmap<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(UnmapWindowRequest {
            window: self,
            ..Default::default()
        })
    }

    /// Unmap this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn unmap_async<Dpy: AsyncDisplay + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request_async(UnmapWindowRequest {
            window: self,
            ..Default::default()
        })
        .await
    }

    /// Get a property of this window.
    #[inline]
    pub fn get_property<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        property: Atom,
        ty: PropertyType,
        delete: bool,
    ) -> crate::Result<RequestCookie<GetPropertyRequest>> {
        dpy.send_request(GetPropertyRequest {
            window: self,
            property,
            ty: Atom::const_from_xid(ty as u32),
            long_offset: 0,
            long_length: core::mem::size_of::<usize>() as u32,
            delete,
            ..Default::default()
        })
    }

    /// Get a property of this window, resolving immediately.
    #[inline]
    pub fn get_property_immediate<Dpy: Display + ?Sized, T: AsByteSequence>(
        self,
        dpy: &mut Dpy,
        property: Atom,
        ty: PropertyType,
        delete: bool,
    ) -> crate::Result<Option<T>> {
        dpy.exchange_request(GetPropertyRequest {
            window: self,
            property,
            ty: Atom::const_from_xid(ty as u32),
            long_offset: 0,
            long_length: core::mem::size_of::<usize>() as u32,
            delete,
            ..Default::default()
        })
        .map(|gpr| T::from_bytes(&gpr.value).map(|(x, _)| x))
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
    ) -> ChangePropertyRequest<'static> {
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
            data: Cow::Owned(data_bytes),
            ..Default::default()
        }
    }

    /// Change a property of the window, given an atom that identifies that property.
    #[inline]
    pub fn change_property<Dpy: Display + ?Sized, T: AsByteSequence>(
        self,
        dpy: &mut Dpy,
        property: Atom,
        property_type: PropertyType,
        format: PropertyFormat,
        mode: PropMode,
        data: &[T],
    ) -> crate::Result<()> {
        dpy.exchange_request(self.change_property_request(
            property,
            property_type,
            format,
            mode,
            data,
        ))
    }

    /// Change a property of the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_property_async<Dpy: AsyncDisplay + ?Sized, T: AsByteSequence>(
        self,
        dpy: &mut Dpy,
        property: Atom,
        property_type: PropertyType,
        format: PropertyFormat,
        mode: PropMode,
        data: &[T],
    ) -> crate::Result<()> {
        dpy.exchange_request_async(self.change_property_request(
            property,
            property_type,
            format,
            mode,
            data,
        ))
        .await
    }

    /// Delete a property of this window.
    #[inline]
    pub fn delete_property<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        property: Atom,
    ) -> crate::Result {
        dpy.exchange_request(DeletePropertyRequest {
            window: self,
            property,
            ..Default::default()
        })
    }

    /// Delete a property of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn delete_property_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        property: Atom,
    ) -> crate::Result {
        dpy.exchange_request_async(DeletePropertyRequest {
            window: self,
            property,
            ..Default::default()
        })
        .await
    }

    /// Set the protocols for the WM in regards to this window.
    #[inline]
    pub fn set_wm_protocols<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        protocols: &[Atom],
    ) -> crate::Result<()> {
        let wm_protocols_atom = retrieve_atom!(
            dpy,
            wm_protocols_atom,
            set_wm_protocols_atom,
            "WM_PROTOCOLS"
        );

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
    pub async fn set_wm_protocols_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        protocols: &[Atom],
    ) -> crate::Result<()> {
        let wm_protocols_atom = retrieve_atom_async!(
            dpy,
            wm_protocols_atom,
            set_wm_protocols_atom,
            "WM_PROTOCOLS"
        );

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
    pub fn set_title<Dpy: Display + ?Sized>(self, dpy: &mut Dpy, title: &str) -> crate::Result<()> {
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
    pub async fn set_title_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub fn window_attributes<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<RequestCookie<GetWindowAttributesRequest>> {
        dpy.send_request(self.get_window_attributes_request())
    }

    /// Get the current set of window attributes for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn window_attributes_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<RequestCookie<GetWindowAttributesRequest>> {
        dpy.send_request_async(self.get_window_attributes_request())
            .await
    }

    /// Immediately get the current set of window attributes for this window.
    #[inline]
    pub fn window_attributes_immediate<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<WindowAttributes> {
        let tok = self.window_attributes(dpy)?;
        Ok(convert_get_window_attributes_reply(
            dpy.resolve_request(tok)?,
        ))
    }

    /// Immediately get the current set of window attributes for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn window_attributes_immediate_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<WindowAttributes> {
        let tok = self.window_attributes_async(dpy).await?;
        Ok(convert_get_window_attributes_reply(
            dpy.resolve_request_async(tok).await?,
        ))
    }

    /// Get the geometry of this window.
    #[inline]
    pub fn geometry<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        dpy.get_drawable_geometry(self)
    }

    /// Get the geometry of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn geometry_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<RequestCookie<GetGeometryRequest>> {
        dpy.get_drawable_geometry_async(self).await
    }

    /// Immediately get the geometry of this window.
    #[inline]
    pub fn geometry_immediate<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<DrawableGeometry> {
        dpy.get_drawable_geometry_immediate(self)
    }

    /// Immediately get the geometry of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn geometry_immediate_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub fn change_attributes<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        props: WindowParameters,
    ) -> crate::Result {
        dpy.exchange_request(self.change_window_attrs_request(props))
    }

    /// Change the properties of this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_attributes_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        props: WindowParameters,
    ) -> crate::Result {
        dpy.exchange_request_async(self.change_window_attrs_request(props))
            .await
    }

    /// Set this window's background color.
    #[inline]
    pub fn set_background_color<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub async fn set_background_color_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub fn configure<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        props: ConfigureWindowParameters,
    ) -> crate::Result {
        dpy.exchange_request(self.configure_window_request(props))
    }

    /// Configure the window's physical properties, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn configure_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        props: ConfigureWindowParameters,
    ) -> crate::Result {
        dpy.exchange_request_async(self.configure_window_request(props))
            .await
    }

    /// Set the border of this window.
    #[inline]
    pub fn set_border_width<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub async fn set_border_width_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub fn set_colormap<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub async fn set_colormap_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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

    /// Change the save set for this window.
    #[inline]
    pub fn change_save_set<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        mode: SetMode,
    ) -> crate::Result {
        dpy.exchange_request(self.change_save_set_request(mode))
    }

    /// Change the save set for this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn change_save_set_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        mode: SetMode,
    ) -> crate::Result {
        dpy.exchange_request_async(self.change_save_set_request(mode))
            .await
    }

    /// Resize the window.
    #[inline]
    pub fn resize<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub async fn resize_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub fn move_resize<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub async fn move_resize_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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

    /// Circulate this window.
    #[inline]
    pub fn circulate<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        direction: Circulate,
    ) -> crate::Result {
        dpy.exchange_request(self.circulate_window_request(direction))
    }

    /// Circulate this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn circulate_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        direction: Circulate,
    ) -> crate::Result {
        dpy.exchange_request_async(self.circulate_window_request(direction))
            .await
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
    pub fn clear_area<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        exposures: bool,
    ) -> crate::Result {
        dpy.exchange_request(self.clear_area_request(x, y, width, height, exposures))
    }

    /// Clear an area of the window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn clear_area_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        x: i16,
        y: i16,
        width: u16,
        height: u16,
        exposures: bool,
    ) -> crate::Result {
        dpy.exchange_request_async(self.clear_area_request(x, y, width, height, exposures))
            .await
    }

    /// Clear the entire window.
    #[inline]
    pub fn clear<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        self.clear_area(dpy, 0, 0, 0, 0, false) // means: clear the whole dang thing
    }

    /// Clear the entire window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn clear_async<Dpy: AsyncDisplay + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
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

    /// Convert a selection in this window.
    #[inline]
    pub fn convert_selection<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Timestamp,
    ) -> crate::Result {
        dpy.exchange_request(self.convert_selection_request(selection, target, property, time))
    }

    /// Convert a selection in this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn convert_selection_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        selection: Atom,
        target: Atom,
        property: Atom,
        time: Timestamp,
    ) -> crate::Result {
        dpy.exchange_request_async(
            self.convert_selection_request(selection, target, property, time),
        )
        .await
    }

    /// Set the cursor used by this window.
    #[inline]
    pub fn set_cursor<Dpy: Display + ?Sized>(self, dpy: &mut Dpy, cursor: Cursor) -> crate::Result {
        let props = WindowParameters {
            cursor: Some(cursor),
            ..Default::default()
        };
        self.change_attributes(dpy, props)
    }

    /// Set the cursor used by this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn set_cursor_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub fn destroy_subwindows<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(DestroySubwindowsRequest {
            window: self,
            ..Default::default()
        })
    }

    /// Destroy this window's subwindows, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn destroy_subwindows_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result {
        dpy.exchange_request_async(DestroySubwindowsRequest {
            window: self,
            ..Default::default()
        })
        .await
    }

    /// Free this window.
    #[inline]
    pub fn free<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(DestroyWindowRequest {
            window: self,
            ..Default::default()
        })
    }

    /// Free this window, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn free_async<Dpy: AsyncDisplay + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request_async(DestroyWindowRequest {
            window: self,
            ..Default::default()
        })
        .await
    }

    /// Set the event mask.
    #[inline]
    pub fn set_event_mask<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
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
    pub async fn set_event_mask_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
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

    /// Change this window's parent and set its position within the parent.
    #[inline]
    pub fn reparent<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        parent: Window,
        x: i16,
        y: i16,
    ) -> crate::Result {
        dpy.exchange_request(ReparentWindowRequest {
            window: self,
            parent,
            x,
            y,
            ..Default::default()
        })
    }

    /// Change this window's parent and set its position within the parent, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn reparent_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        parent: Window,
        x: i16,
        y: i16,
    ) -> crate::Result {
        dpy.exchange_request_async(ReparentWindowRequest {
            window: self,
            parent,
            x,
            y,
            ..Default::default()
        })
        .await
    }

    /// Map this window's subwindows.
    #[inline]
    pub fn map_subwindows<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(MapSubwindowsRequest {
            window: self,
            ..Default::default()
        })
    }

    /// Map this window's subwindows, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn map_subwindows_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result {
        dpy.exchange_request_async(MapSubwindowsRequest {
            window: self,
            ..Default::default()
        })
        .await
    }

    /// Unmap this window's subwindows.
    #[inline]
    pub fn unmap_subwindows<Dpy: Display + ?Sized>(self, dpy: &mut Dpy) -> crate::Result {
        dpy.exchange_request(UnmapSubwindowsRequest {
            window: self,
            ..Default::default()
        })
    }

    /// Unmap this window's subwindows, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn unmap_subwindows_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result {
        dpy.exchange_request_async(UnmapSubwindowsRequest {
            window: self,
            ..Default::default()
        })
        .await
    }

    /// Query information regarding this window's family tree.
    #[inline]
    pub fn query_tree<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<RequestCookie<QueryTreeRequest>> {
        dpy.send_request(QueryTreeRequest {
            window: self,
            ..Default::default()
        })
    }

    /// Query information regarding this window's family tree and resolve immediately.
    #[inline]
    pub fn query_tree_immediate<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<TreeInformation> {
        dpy.exchange_request(QueryTreeRequest {
            window: self,
            ..Default::default()
        })
        .map(|i| i.into())
    }

    /// Query information regarding this window's family tree, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn query_tree_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<RequestCookie<QueryTreeRequest>> {
        dpy.send_request_async(QueryTreeRequest {
            window: self,
            ..Default::default()
        })
        .await
    }

    /// Query information regarding this window's family tree and resolve immediately, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn query_tree_immediate_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
    ) -> crate::Result<TreeInformation> {
        dpy.exchange_request_async(QueryTreeRequest {
            window: self,
            ..Default::default()
        })
        .await
        .map(|i| i.into())
    }

    /// Get a property of this window, async redox
    #[cfg(feature = "async")]
    #[inline]
    pub fn get_property_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        property: Atom,
        ty: PropertyType,
        delete: bool,
    ) -> SendRequestFuture<'_, Dpy, GetPropertyRequest> {
        dpy.send_request_async(GetPropertyRequest {
            window: self,
            property,
            ty: Atom::const_from_xid(ty as u32),
            long_offset: 0,
            long_length: core::mem::size_of::<usize>() as u32,
            delete,
            ..Default::default()
        })
    }

    /// Get a property of this window, resolving immediately, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn get_property_immediate_async<Dpy: AsyncDisplay + ?Sized, T: AsByteSequence>(
        self,
        dpy: &mut Dpy,
        property: Atom,
        ty: PropertyType,
        delete: bool,
    ) -> crate::Result<Option<T>> {
        dpy.exchange_request_async(GetPropertyRequest {
            window: self,
            property,
            ty: Atom::const_from_xid(ty as u32),
            long_offset: 0,
            long_length: core::mem::size_of::<usize>() as u32,
            delete,
            ..Default::default()
        }).await
        .map(|gpr| T::from_bytes(&gpr.value).map(|(x, _)| x))
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
    Primary = 1,
    Secondary = 2,
    Arc = 3,
    Atom = 4,
    Bitmap = 5,
    Cardinal = 6,
    Colormap = 7,
    Cursor = 8,
    CutBuffer0 = 9,
    CutBuffer1 = 10,
    CutBuffer2 = 11,
    CutBuffer3 = 12,
    CutBuffer4 = 13,
    CutBuffer5 = 14,
    CutBuffer6 = 15,
    CutBuffer7 = 16,
    Drawable = 17,
    Font = 18,
    Integer = 19,
    Pixmap = 20,
    Point = 21,
    Rectangle = 22,
    ResourceManager = 23,
    RgbColormap = 24,
    RgbBestmap = 25,
    RgbBluemap = 26,
    RgbDefaultmap = 27,
    RgbGraymap = 28,
    RgbGreenmap = 29,
    RgbRedmap = 30,
    String = 31,
    VisualID = 32,
    Window = 33,
    WMCommand = 34,
    WMHints = 35,
    WMClientMachine = 36,
    WMIconName = 37,
    WMIconSize = 38,
    WMName = 39,
    WMNormalHints = 40,
    WMSizeHints = 41,
    WMZoomHints = 42,
    MinSpace = 43,
    NORMSpace = 44,
    MaxSpace = 45,
    EndSpace = 46,
    SuperscriptX = 47,
    SuperscriptY = 48,
    SubscriptX = 49,
    SubscriptY = 50,
    UnderlinePosition = 51,
    UnderlineThickness = 52,
    StrikeoutAscent = 53,
    StrikeoutDescent = 54,
    ItalicAngle = 55,
    XHeight = 56,
    QuadWidth = 57,
    Weight = 58,
    PointSize = 59,
    Resolution = 60,
    Copyright = 61,
    Notice = 62,
    FontName = 63,
    FamilyName = 64,
    FullName = 65,
    CapHeight = 66,
    WMClass = 67,
    WMTransientFor = 68,
}

/// The format of the property being changed.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
pub enum PropertyFormat {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
}
