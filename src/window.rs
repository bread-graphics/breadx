// MIT/Apache2 License

pub use crate::{
    auto::{
        xproto::{Atom, ChangePropertyRequest, MapWindowRequest, PropMode, Window, ATOM_WM_NAME},
        AsByteSequence,
    },
    display::{Connection, Display},
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

impl Window {
    /// Map this window to the screen.
    #[inline]
    pub fn map<Conn: Connection>(&self, dpy: &mut Display<Conn>) -> crate::Result {
        let mw = MapWindowRequest {
            window: *self,
            ..Default::default()
        };

        log::debug!("Sending MapWindowRequest to server");
        let tok = dpy.send_request(mw)?;
        defer!(log::debug!("Sent MapWindowRequest to server"));
        dpy.resolve_request(tok)
    }

    /// Map this window to the screen, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn map_async<Conn: Connection>(&self, dpy: &mut Display<Conn>) -> crate::Result {
        let mw = MapWindowRequest {
            window: *self,
            ..Default::default()
        };
        let tok = dpy.send_request_async(mw).await?;
        dpy.resolve_request_async(tok).await
    }

    /// Request struct to change the property of a window.
    #[inline]
    fn change_property_request<T: AsByteSequence>(
        &self,
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
        let len = data.iter().fold(0, |mut index, item| {
            item.as_bytes(&mut data_bytes[index..]) + index
        });
        data_bytes.truncate(len);

        let format = format as u8;

        ChangePropertyRequest {
            mode,
            window: *self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
        &self,
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
}

/// The type of the property being changed.
#[repr(u32)]
pub enum PropertyType {
    Atom = 4,
    String = 31,
}

/// The format of the property being changed.
#[repr(u8)]
pub enum PropertyFormat {
    Eight = 8,
    Sixteen = 16,
    ThirtyTwo = 32,
}
