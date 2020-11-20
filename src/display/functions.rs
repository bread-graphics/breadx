// MIT/Apache2 License

use super::{Connection, Display, RequestCookie};
use crate::{
    auto::{
        xproto::{
            Atom, BackingStore, Colormap, CreateWindowRequest, Cursor, Cw, EventMask, Gravity,
            InternAtomReply, InternAtomRequest, Pixmap, Visualid, Window, WindowClass,
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
