// MIT/Apache2 License

use super::{Connection, Display};
use crate::{
    auto::{
        xproto::{CreateWindowRequest, Visualid, Window, WindowClass, Cw},
        AsByteSequence,
    },
    XidType, XID,
};
use alloc::vec::Vec;

/// Parameters for a window.
#[derive(PartialEq, Eq, PartialOrd, Ord, Copy, Clone)]
pub enum WindowParameter {
    BackPixmap,
    BackPixel,
    BorderPixmap,
    BorderPixel,
    BitGravity,
    WinGravity,
    BackingStore,
    BackingPlanes,
    BackingPixel,
    OverrideRedirect,
    SaveUnder,
    EventMask,
    DontPropagate,
    Colormap,
    Cursor,
}

impl Default for WindowParameter {
    #[inline]
    fn default() -> Self {
        Self::BackPixmap
    }
}

#[inline]
fn wp_map_to_mask_and_values<T>(map: T) -> (Cw, Vec<u32>)
where
    T: IntoIterator<Item = (WindowParameter, u32)>,
{
    // collect into a tinyvec and sort
    let mut values: Vec<(WindowParameter, u32)> = map.into_iter().collect();
    values.sort_by_key(|(w, v)| w.clone());

    // create the mask and list of values
    let mut cw: Cw = Default::default();
    let values = values
        .into_iter()
        .map(|(w, v)| {
            match w {
                WindowParameter::BackPixmap => {
                    cw.back_pixmap = true;
                }
                WindowParameter::BackPixel => {
                    cw.back_pixel = true;
                }
                WindowParameter::BorderPixmap => {
                    cw.border_pixmap = true;
                }
                WindowParameter::BorderPixel => {
                    cw.border_pixel = true;
                }
                WindowParameter::BitGravity => {
                    cw.bit_gravity = true;
                }
                WindowParameter::WinGravity => {
                    cw.win_gravity = true;
                }
                WindowParameter::BackingStore => {
                    cw.backing_store = true;
                }
                WindowParameter::BackingPlanes => {
                    cw.backing_planes = true;
                }
                WindowParameter::BackingPixel => {
                    cw.backing_pixel = true;
                }
                WindowParameter::OverrideRedirect => {
                    cw.override_redirect = true;
                }
                WindowParameter::SaveUnder => {
                    cw.save_under = true;
                }
                WindowParameter::EventMask => {
                    cw.event_mask = true;
                }
                WindowParameter::DontPropagate => {
                    cw.dont_propagate = true;
                }
                WindowParameter::Colormap => {
                    cw.colormap = true;
                }
                WindowParameter::Cursor => {
                    cw.cursor = true;
                }
            }

            v
        })
        .collect();

    (cw, values)
}

impl<Conn: Connection> Display<Conn> {
    #[inline]
    fn create_window_request<T>(
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
        props: T,
    ) -> CreateWindowRequest
    where
        T: IntoIterator<Item = (WindowParameter, u32)>,
    {
        const INHERITED_DEPTH: u8 = 0;
        const INHERITED_VISUAL: Visualid = 0;

        let (value_mask, value_list) = wp_map_to_mask_and_values(props);
        let parent = parent.unwrap_or_else(|| self.default_root());

        CreateWindowRequest {
            wid,
            parent,
            class: class as u16,
            visual: visual.unwrap_or(INHERITED_VISUAL),
            depth: depth.unwrap_or(INHERITED_DEPTH),
            x,
            y,
            width,
            height,
            border_width,
            value_mask,
            value_list,
        }
    }

    #[inline]
    pub fn create_window<T>(
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
        props: T,
    ) -> crate::Result<Window>
    where
        T: IntoIterator<Item = (WindowParameter, u32)>,
    {
        let wid = Window::const_from_xid(self.generate_xid()?);
        log::debug!("Generate {:#032b}", wid.xid());
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

    #[cfg(feature = "async")]
    #[inline]
    pub async fn create_window_async<T>(
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
        props: T,
    ) -> crate::Result<Window>
    where
        T: IntoIterator<Item = (WindowParameter, u32)>,
    {
        let wid = Window::const_from_xid(self.generate_xid_async().await?);
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
}
