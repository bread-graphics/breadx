// MIT/Apache2 License

use super::PictureParameters;
use crate::{
    auto::{
        render::{
            Color, CreateConicalGradientRequest, CreateLinearGradientRequest, CreatePictureRequest,
            CreateRadialGradientRequest, Fixed, PictType, Pictformat, Pictforminfo, Pictscreen,
            Picture, Pictvisual, Pointfix, QueryPictFormatsReply, QueryPictFormatsRequest,
            QueryVersionReply, QueryVersionRequest,
        },
        xproto::{Drawable, Setup, Visualtype},
    },
    display::{
        generate_xid, prelude::*, Display, DisplayBase, DisplayExt, PendingItem,  RequestInfo, EXT_KEY_SIZE,
    },
    event::Event,
    BreadError, XID,
};
use alloc::boxed::Box;
use core::num::NonZeroU32;

#[cfg(feature = "async")]
use crate::display::{PollOr, AsyncDisplay};
#[cfg(feature = "async")]
use core::task::{Context, Poll};

/// A wrapper around the `Display` that contains XRender-specific data.
#[derive(Debug)]
pub struct RenderDisplay<Dpy: ?Sized> {
    formats: Box<[Pictforminfo]>,
    screens: Box<[Pictscreen]>,
    subpixels: Box<[u32]>,
    major_version: u32,
    minor_version: u32,
    inner: Dpy,
}

impl<Dpy> RenderDisplay<Dpy> {
    /// Get a reference to the inner object.
    #[inline]
    pub fn inner(&self) -> &Dpy {
        &self.inner
    }

    /// Get a mutable reference to the inner object.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut Dpy {
        &mut self.inner
    }

    /// Destroy this objet and return the inner display.
    #[inline]
    pub fn into_inner(self) -> Dpy {
        self.inner
    }

    #[inline]
    fn fold_for_visformat<F: FnMut(&Pictvisual) -> bool>(&self, mut f: F) -> Option<Pictformat> {
        self.screens
            .iter()
            .flat_map(|s| s.depths.iter())
            .flat_map(|d| d.visuals.iter())
            .find_map(|v| if f(v) { Some(v.format) } else { None })
    }

    /// Get a `Pictformat` object that matches the given `Visualtype`.
    #[inline]
    pub fn find_visual_format(&self, visual: &Visualtype) -> Option<Pictformat> {
        self.fold_for_visformat(|v| v.visual == visual.visual_id)
    }

    /// Get a `Pictformat` based on a standard format.
    #[inline]
    pub fn find_standard_format(&self, standard: StandardFormat) -> Option<Pictformat> {
        const STANDARDS: &[fn(&Pictforminfo) -> bool] = &[
            |p| {
                p.ty == PictType::Direct
                    && p.depth == 32
                    && p.direct.red_shift == 16
                    && p.direct.red_mask == 0xFF
                    && p.direct.green_shift == 8
                    && p.direct.green_mask == 0xFF
                    && p.direct.blue_shift == 0
                    && p.direct.blue_mask == 0xFF
                    && p.direct.alpha_shift == 24
                    && p.direct.alpha_mask == 0xFF
            },
            |p| {
                p.ty == PictType::Direct
                    && p.depth == 24
                    && p.direct.red_shift == 16
                    && p.direct.red_mask == 0xFF
                    && p.direct.green_shift == 8
                    && p.direct.green_mask == 0xFF
                    && p.direct.blue_shift == 0
                    && p.direct.blue_mask == 0xFF
                    && p.direct.alpha_mask == 0
            },
            |p| {
                p.ty == PictType::Direct
                    && p.depth == 8
                    && p.direct.red_mask == 0x00
                    && p.direct.green_mask == 0x00
                    && p.direct.blue_mask == 0x00
                    && p.direct.alpha_shift == 0
                    && p.direct.alpha_mask == 0xFF
            },
            |p| {
                p.ty == PictType::Direct
                    && p.depth == 4
                    && p.direct.red_mask == 0x00
                    && p.direct.green_mask == 0x00
                    && p.direct.blue_mask == 0x00
                    && p.direct.alpha_shift == 0
                    && p.direct.alpha_mask == 0x0F
            },
            |p| {
                p.ty == PictType::Direct
                    && p.depth == 1
                    && p.direct.red_mask == 0x00
                    && p.direct.green_mask == 0x00
                    && p.direct.blue_mask == 0x00
                    && p.direct.alpha_shift == 0
                    && p.direct.alpha_mask == 0x01
            },
        ];
        let standard_identifier = STANDARDS[standard as usize];

        self.formats.iter().find_map(|p| {
            if standard_identifier(p) {
                Some(p.id)
            } else {
                None
            }
        })
    }

    #[inline]
    fn create_picture_request(
        pid: Picture,
        drawable: Drawable,
        format: Pictformat,
        props: PictureParameters,
    ) -> CreatePictureRequest {
        let mut cpr = CreatePictureRequest {
            pid,
            drawable,
            format,
            ..Default::default()
        };
        let cp = props.convert_to_flags(&mut cpr);
        cpr.value_mask = cp;
        cpr
    }

    #[inline]
    fn create_linear_gradient_request(
        pid: Picture,
        p1: Pointfix,
        p2: Pointfix,
        stops: &[Fixed],
        colors: &[Color],
    ) -> CreateLinearGradientRequest {
        assert_eq!(stops.len(), colors.len());
        CreateLinearGradientRequest {
            picture: pid,
            p1,
            p2,
            num_stops: stops.len() as u32,
            stops: stops.to_vec(),
            colors: colors.to_vec(),
            ..Default::default()
        }
    }

    #[inline]
    fn create_radial_gradient_request(
        pid: Picture,
        inner: Pointfix,
        outer: Pointfix,
        inner_radius: Fixed,
        outer_radius: Fixed,
        stops: &[Fixed],
        colors: &[Color],
    ) -> CreateRadialGradientRequest {
        assert_eq!(stops.len(), colors.len());
        CreateRadialGradientRequest {
            picture: pid,
            inner,
            outer,
            inner_radius,
            outer_radius,
            num_stops: stops.len() as u32,
            stops: stops.to_vec(),
            colors: colors.to_vec(),
            ..Default::default()
        }
    }

    #[inline]
    fn create_conical_gradient_request(
        pid: Picture,
        center: Pointfix,
        angle: Fixed,
        stops: &[Fixed],
        colors: &[Color],
    ) -> CreateConicalGradientRequest {
        assert_eq!(stops.len(), colors.len());
        CreateConicalGradientRequest {
            picture: pid,
            center,
            angle,
            num_stops: stops.len() as u32,
            stops: stops.to_vec(),
            colors: colors.to_vec(),
            ..Default::default()
        }
    }
}

/// Standard formats.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(usize)]
pub enum StandardFormat {
    Argb32 = 0,
    Rgb24 = 1,
    A8 = 2,
    A4 = 3,
    A1 = 4,
}

struct XrenderInfo {
    formats: Box<[Pictforminfo]>,
    screens: Box<[Pictscreen]>,
    subpixels: Box<[u32]>,
    major_version: u32,
    minor_version: u32,
}

impl<Dpy: DisplayBase> DisplayBase for RenderDisplay<Dpy> {
    #[inline]
    fn setup(&self) -> &Setup {
        self.inner.setup()
    }

    #[inline]
    fn default_screen_index(&self) -> usize {
        self.inner.default_screen_index()
    }

    #[inline]
    fn next_request_number(&mut self) -> u64 {
        self.inner.next_request_number()
    }

    #[inline]
    fn push_event(&mut self, event: Event) {
        self.inner.push_event(event)
    }

    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        self.inner.pop_event()
    }

    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        self.inner.generate_xid()
    }

    #[inline]
    fn add_pending_item(&mut self, req_id: u16, item: PendingItem) {
        self.inner.add_pending_item(req_id, item)
    }

    #[inline]
    fn get_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.inner.get_pending_item(req_id)
    }

    #[inline]
    fn take_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.inner.take_pending_item(req_id)
    }

    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        self.inner.create_special_event_queue(xid);
    }

    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        self.inner.push_special_event(xid, event)
    }

    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        self.inner.pop_special_event(xid)
    }

    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.inner.delete_special_event_queue(xid);
    }

    #[inline]
    fn checked(&self) -> bool {
        self.inner.checked()
    }

    #[inline]
    fn set_checked(&mut self, checked: bool) {
        self.inner.set_checked(checked);
    }

    #[inline]
    fn bigreq_enabled(&self) -> bool {
        self.inner.bigreq_enabled()
    }
    #[inline]
    fn max_request_len(&self) -> usize {
        self.inner.max_request_len()
    }

    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        self.inner.get_extension_opcode(key)
    }

    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        self.inner.set_extension_opcode(key, opcode);
    }

    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        self.inner.wm_protocols_atom()
    }

    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        self.inner.set_wm_protocols_atom(a)
    }
}

impl<'a, Dpy: DisplayBase> DisplayBase for &'a RenderDisplay<Dpy>
where
    &'a Dpy: DisplayBase,
{
    #[inline]
    fn setup(&self) -> &Setup {
        self.inner.setup()
    }

    #[inline]
    fn default_screen_index(&self) -> usize {
        self.inner().default_screen_index()
    }

    #[inline]
    fn next_request_number(&mut self) -> u64 {
        self.inner().next_request_number()
    }

    #[inline]
    fn push_event(&mut self, event: Event) {
        self.inner().push_event(event)
    }

    #[inline]
    fn pop_event(&mut self) -> Option<Event> {
        self.inner().pop_event()
    }

    #[inline]
    fn generate_xid(&mut self) -> Option<XID> {
        self.inner().generate_xid()
    }

    #[inline]
    fn add_pending_item(&mut self, req_id: u16, pereq: PendingItem) {
        self.inner().add_pending_item(req_id, pereq)
    }

    #[inline]
    fn get_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.inner().get_pending_item(req_id)
    }

    #[inline]
    fn take_pending_item(&mut self, req_id: u16) -> Option<PendingItem> {
        self.inner().take_pending_item(req_id)
    }

    #[inline]
    fn create_special_event_queue(&mut self, xid: XID) {
        self.inner().create_special_event_queue(xid);
    }

    #[inline]
    fn push_special_event(&mut self, xid: XID, event: Event) -> Result<(), Event> {
        self.inner().push_special_event(xid, event)
    }

    #[inline]
    fn pop_special_event(&mut self, xid: XID) -> Option<Event> {
        self.inner().pop_special_event(xid)
    }

    #[inline]
    fn delete_special_event_queue(&mut self, xid: XID) {
        self.inner().delete_special_event_queue(xid);
    }

    #[inline]
    fn checked(&self) -> bool {
        self.inner().checked()
    }

    #[inline]
    fn set_checked(&mut self, checked: bool) {
        self.inner().set_checked(checked);
    }

    #[inline]
    fn bigreq_enabled(&self) -> bool {
        self.inner.bigreq_enabled()
    }
    #[inline]
    fn max_request_len(&self) -> usize {
        self.inner.max_request_len()
    }

    #[inline]
    fn get_extension_opcode(&mut self, key: &[u8; EXT_KEY_SIZE]) -> Option<u8> {
        self.inner().get_extension_opcode(key)
    }

    #[inline]
    fn set_extension_opcode(&mut self, key: [u8; EXT_KEY_SIZE], opcode: u8) {
        self.inner().set_extension_opcode(key, opcode);
    }

    #[inline]
    fn wm_protocols_atom(&self) -> Option<NonZeroU32> {
        self.inner().wm_protocols_atom()
    }

    #[inline]
    fn set_wm_protocols_atom(&mut self, a: NonZeroU32) {
        self.inner().set_wm_protocols_atom(a)
    }
}

impl<Dpy: Display> Display for RenderDisplay<Dpy> {
    #[inline]
    fn wait(&mut self) -> crate::Result {
        self.inner.wait()
    }

    #[inline]
    fn send_request_raw(&mut self, request: RequestInfo) -> crate::Result<u16> {
        self.inner.send_request_raw(request)
    }
}

impl<'a, Dpy: DisplayBase> Display for &'a RenderDisplay<Dpy>
where
    &'a Dpy: Display,
{
    #[inline]
    fn wait(&mut self) -> crate::Result {
        self.inner().wait()
    }

    #[inline]
    fn send_request_raw(&mut self, request: RequestInfo) -> crate::Result<u16> {
        self.inner().send_request_raw(request)
    }
}

#[cfg(feature = "async")]
impl<Dpy: AsyncDisplay> AsyncDisplay for RenderDisplay<Dpy> {
    #[inline]
    fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result> {
        self.inner.poll_wait(cx)
    }

    #[inline]
    fn begin_send_request_raw(
        &mut self,
        req: RequestInfo,
        cx: &mut Context<'_>,
    ) -> PollOr<(), RequestInfo> {
        self.inner.begin_send_request_raw(req, cx)
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        self.inner.poll_send_request_raw(cx)
    }
}

#[cfg(feature = "async")]
impl<'a, Dpy: DisplayBase> AsyncDisplay for &'a RenderDisplay<Dpy>
where
    &'a Dpy: AsyncDisplay,
{
    #[inline]
    fn poll_wait(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result> {
        self.inner().poll_wait(cx)
    }

    #[inline]
    fn begin_send_request_raw(
        &mut self,
        req: RequestInfo,
        cx: &mut Context<'_>,
    ) -> PollOr<(), RequestInfo> {
        self.inner().begin_send_request_raw(req, cx)
    }

    #[inline]
    fn poll_send_request_raw(&mut self, cx: &mut Context<'_>) -> Poll<crate::Result<u16>> {
        self.inner().poll_send_request_raw(cx)
    }
}

impl<Dpy: Display> RenderDisplay<Dpy> {
    /// Initialize a RenderDisplay with the appropriate information.
    #[inline]
    pub fn new(
        mut dpy: Dpy,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> Result<Self, (Dpy, BreadError)> {
        #[inline]
        fn xrender_info<Dpy: Display>(
            dpy: &mut Dpy,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> crate::Result<XrenderInfo> {
            // run QueryVersion and QueryPictFormats simultaneously
            let qvtok = dpy.send_request(QueryVersionRequest {
                client_major_version,
                client_minor_version,
                ..Default::default()
            })?;
            let qpftok = dpy.send_request(QueryPictFormatsRequest::default())?;

            let QueryVersionReply {
                major_version,
                minor_version,
                ..
            } = dpy.resolve_request(qvtok)?;
            let QueryPictFormatsReply {
                formats,
                screens,
                subpixels,
                ..
            } = dpy.resolve_request(qpftok)?;

            Ok(XrenderInfo {
                major_version,
                minor_version,
                formats: formats.into_boxed_slice(),
                screens: screens.into_boxed_slice(),
                subpixels: subpixels.into_boxed_slice(),
            })
        }

        let XrenderInfo {
            formats,
            screens,
            subpixels,
            major_version,
            minor_version,
        } = match xrender_info(&mut dpy, client_major_version, client_minor_version) {
            Ok(x) => x,
            Err(e) => return Err((dpy, e)),
        };

        Ok(Self {
            inner: dpy,
            major_version,
            minor_version,
            formats,
            screens,
            subpixels,
        })
    }

    /// Create a new Picture.
    #[inline]
    pub fn create_picture<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        format: Pictformat,
        properties: PictureParameters,
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let cpr = Self::create_picture_request(pic, target.into(), format, properties);
        self.send_request(cpr)?;
        Ok(pic)
    }

    /// Create a new linear gradient.
    #[inline]
    pub fn create_linear_gradient(
        &mut self,
        p1: Pointfix,
        p2: Pointfix,
        stops: &[Fixed],
        colors: &[Color],
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let clgr = Self::create_linear_gradient_request(pic, p1, p2, stops, colors);
        self.exchange_request(clgr)?;
        Ok(pic)
    }

    /// Create a new radial gradient.
    #[inline]
    pub fn create_radial_gradient(
        &mut self,
        inner: Pointfix,
        outer: Pointfix,
        inner_radius: Fixed,
        outer_radius: Fixed,
        stops: &[Fixed],
        colors: &[Color],
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let crgr = Self::create_radial_gradient_request(
            pic,
            inner,
            outer,
            inner_radius,
            outer_radius,
            stops,
            colors,
        );
        self.exchange_request(crgr)?;
        Ok(pic)
    }

    #[inline]
    pub fn create_conical_gradient(
        &mut self,
        center: Pointfix,
        angle: Fixed,
        stops: &[Fixed],
        colors: &[Color],
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let ccgr = Self::create_conical_gradient_request(pic, center, angle, stops, colors);
        self.exchange_request(ccgr)?;
        Ok(pic)
    }
}

#[cfg(feature = "async")]
impl<Dpy: AsyncDisplay> RenderDisplay<Dpy> {
    /// Initialize a RenderDisplay with the appropriate information, async redox.
    #[inline]
    pub async fn new_async(
        mut dpy: Dpy,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> Result<Self, (Dpy, crate::BreadError)> {
        #[inline]
        async fn xrender_info<Dpy: AsyncDisplay>(
            dpy: &mut Dpy,
            client_major_version: u32,
            client_minor_version: u32,
        ) -> crate::Result<XrenderInfo> {
            let qvtok = dpy
                .send_request_async(QueryVersionRequest {
                    client_major_version,
                    client_minor_version,
                    ..Default::default()
                })
                .await?;
            let qpftok = dpy
                .send_request_async(QueryPictFormatsRequest::default())
                .await?;

            let QueryVersionReply {
                major_version,
                minor_version,
                ..
            } = dpy.resolve_request_async(qvtok).await?;
            let QueryPictFormatsReply {
                formats,
                screens,
                subpixels,
                ..
            } = dpy.resolve_request_async(qpftok).await?;

            Ok(XrenderInfo {
                major_version,
                minor_version,
                formats: formats.into_boxed_slice(),
                screens: screens.into_boxed_slice(),
                subpixels: subpixels.into_boxed_slice(),
            })
        }

        let XrenderInfo {
            major_version,
            minor_version,
            formats,
            screens,
            subpixels,
        } = match xrender_info(&mut dpy, client_major_version, client_minor_version).await {
            Ok(x) => x,
            Err(e) => return Err((dpy, e)),
        };

        Ok(Self {
            inner: dpy,
            major_version,
            minor_version,
            formats,
            screens,
            subpixels,
        })
    }

    /// Create a new Picture, async redox.
    #[inline]
    pub async fn create_picture_async<Target: Into<Drawable>>(
        &mut self,
        target: Target,
        format: Pictformat,
        properties: PictureParameters,
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let cpr = Self::create_picture_request(pic, target.into(), format, properties);
        self.exchange_request_async(cpr).await?;
        Ok(pic)
    }

    /// Create a new linear gradient, async redox.
    #[inline]
    pub async fn create_linear_gradient_async(
        &mut self,
        p1: Pointfix,
        p2: Pointfix,
        stops: &[Fixed],
        colors: &[Color],
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let clgr = Self::create_linear_gradient_request(pic, p1, p2, stops, colors);
        self.exchange_request_async(clgr).await?;
        Ok(pic)
    }

    /// Create a new radial gradient, async redox.
    #[inline]
    pub async fn create_radial_gradient_async(
        &mut self,
        inner: Pointfix,
        outer: Pointfix,
        inner_radius: Fixed,
        outer_radius: Fixed,
        stops: &[Fixed],
        colors: &[Color],
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let crgr = Self::create_radial_gradient_request(
            pic,
            inner,
            outer,
            inner_radius,
            outer_radius,
            stops,
            colors,
        );
        self.exchange_request_async(crgr).await?;
        Ok(pic)
    }

    #[inline]
    pub async fn create_conical_gradient_async(
        &mut self,
        center: Pointfix,
        angle: Fixed,
        stops: &[Fixed],
        colors: &[Color],
    ) -> crate::Result<Picture> {
        let pic = Picture::const_from_xid(generate_xid(self)?);
        let ccgr = Self::create_conical_gradient_request(pic, center, angle, stops, colors);
        self.exchange_request_async(ccgr).await?;
        Ok(pic)
    }
}
