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
        xproto::{Drawable, Visualtype},
    },
    display::{Connection, Display, DisplayLike},
    send_request, sr_request,
};
use alloc::vec::Vec;

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

/// A wrapper around the `Display` that contains XRender-specific data.
#[derive(Debug)]
pub struct RenderDisplay<Dpy> {
    inner: Dpy,
    formats: Vec<Pictforminfo>,
    screens: Vec<Pictscreen>,
    subpixels: Vec<u32>,
    major_version: u32,
    minor_version: u32,
}

impl<Dpy: DisplayLike> DisplayLike for RenderDisplay<Dpy> {
    type Connection = Dpy::Connection;

    #[inline]
    fn display(&self) -> &Display<Dpy::Connection> {
        self.inner.display()
    }

    #[inline]
    fn display_mut(&mut self) -> &mut Display<Dpy::Connection> {
        self.inner.display_mut()
    }
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

impl<Dpy: DisplayLike> RenderDisplay<Dpy>
where
    Dpy::Connection: Connection,
{
    /// Initialize a RenderDisplay with the appropriate information.
    #[inline]
    pub fn new(
        mut dpy: Dpy,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> crate::Result<Self> {
        // run QueryVersion and QueryPictFormats simultaneously
        let qvtok = send_request!(
            dpy.display_mut(),
            QueryVersionRequest {
                client_major_version,
                client_minor_version,
                ..Default::default()
            }
        )?;
        let qpftok = send_request!(dpy.display_mut(), QueryPictFormatsRequest::default())?;

        let QueryVersionReply {
            major_version,
            minor_version,
            ..
        } = dpy.display_mut().resolve_request(qvtok)?;
        let QueryPictFormatsReply {
            formats,
            screens,
            subpixels,
            ..
        } = dpy.display_mut().resolve_request(qpftok)?;

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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let cpr = Self::create_picture_request(pic, target.into(), format, properties);
        sr_request!(self.display_mut(), cpr)?;
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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let clgr = Self::create_linear_gradient_request(pic, p1, p2, stops, colors);
        sr_request!(self.display_mut(), clgr)?;
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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let crgr = Self::create_radial_gradient_request(
            pic,
            inner,
            outer,
            inner_radius,
            outer_radius,
            stops,
            colors,
        );
        sr_request!(self.display_mut(), crgr)?;
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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let ccgr = Self::create_conical_gradient_request(pic, center, angle, stops, colors);
        sr_request!(self.display_mut(), ccgr)?;
        Ok(pic)
    }
}

#[cfg(feature = "async")]
impl<Dpy: DisplayLike> RenderDisplay<Dpy>
where
    Dpy::Connection: AsyncConnection + Send,
{
    /// Initialize a RenderDisplay with the appropriate information, async redox.
    #[inline]
    pub async fn new_async(
        mut dpy: Dpy,
        client_major_version: u32,
        client_minor_version: u32,
    ) -> crate::Result<Self> {
        let qvtok = send_request!(
            dpy.display_mut(),
            QueryVersionRequest {
                client_major_version,
                client_minor_version,
                ..Default::default()
            },
            async
        )
        .await?;
        let qpftok =
            send_request!(dpy.display_mut(), QueryPictFormatsRequest::default(), async).await?;

        let QueryVersionReply {
            major_version,
            minor_version,
            ..
        } = dpy.display_mut().resolve_request_async(qvtok).await?;
        let QueryPictFormatsReply {
            formats,
            screens,
            subpixels,
            ..
        } = dpy.display_mut().resolve_request_async(qpftok).await?;

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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let cpr = Self::create_picture_request(pic, target.into(), format, properties);
        sr_request!(self.display_mut(), cpr, async).await?;
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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let clgr = Self::create_linear_gradient_request(pic, p1, p2, stops, colors);
        sr_request!(self.display_mut(), clgr, async).await?;
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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let crgr = Self::create_radial_gradient_request(
            pic,
            inner,
            outer,
            inner_radius,
            outer_radius,
            stops,
            colors,
        );
        sr_request!(self.display_mut(), crgr, async).await?;
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
        let pic = Picture::const_from_xid(self.display_mut().generate_xid()?);
        let ccgr = Self::create_conical_gradient_request(pic, center, angle, stops, colors);
        sr_request!(self.display_mut(), ccgr, async).await?;
        Ok(pic)
    }
}
