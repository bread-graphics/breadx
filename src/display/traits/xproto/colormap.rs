// MIT/Apache2 License

//! Provides functionality and structures used to interface with the colormap.

use crate::{
    auto::xproto::{AllocColorReply, AllocColorRequest, Colormap},
    display::prelude::*,
    Connection, Display, RequestCookie,
};

#[cfg(feature = "async")]
use crate::{
    display::{
        futures::{ExchangeRequestFuture, MapFuture, SendRequestFuture},
        AsyncDisplay,
    },
    util::BoxedFnOnce,
};
#[cfg(feature = "async")]
use alloc::boxed::Box;

/// Convenience function for producing an RGB pixel value for supported monitors.
#[inline]
#[must_use]
pub const fn rgb(r: u8, g: u8, b: u8) -> u32 {
    let r = r as u32;
    let g = g as u32;
    let b = b as u32;
    b + (g << 8) + (r << 16)
}

/// The result of a color allocation call.
#[derive(Debug, Clone, Copy)]
pub enum ColorAllocation {
    /// The desired color was allocated without issue.
    NoChange(u32),
    /// The color was allocated, but it had to be changed to another color.
    Changed {
        pixel: u32,
        red: u16,
        green: u16,
        blue: u16,
    },
}

impl ColorAllocation {
    /// Get the pixel for this result.
    #[inline]
    #[must_use]
    pub fn pixel(&self) -> u32 {
        match self {
            Self::NoChange(pixel) | Self::Changed { pixel, .. } => *pixel,
        }
    }

    /// Convert an alloc color reply to the result.
    #[inline]
    #[must_use]
    pub fn from_alloc_color_reply(acr: AllocColorReply, r: u16, g: u16, b: u16) -> Self {
        if acr.red == r && acr.green == g && acr.blue == b {
            Self::NoChange(acr.pixel)
        } else {
            Self::Changed {
                red: acr.red,
                green: acr.green,
                blue: acr.blue,
                pixel: acr.pixel,
            }
        }
    }
}

impl From<ColorAllocation> for u32 {
    #[inline]
    fn from(ca: ColorAllocation) -> u32 {
        ca.pixel()
    }
}

impl Colormap {
    /// Alloc color request.
    #[inline]
    fn alloc_color_request(self, r: u16, g: u16, b: u16) -> AllocColorRequest {
        AllocColorRequest {
            cmap: self,
            red: r,
            green: g,
            blue: b,
            ..Default::default()
        }
    }

    /// Allocate a new color in the colormap.
    #[inline]
    pub fn alloc_color<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        r: u16,
        g: u16,
        b: u16,
    ) -> crate::Result<RequestCookie<AllocColorRequest>> {
        dpy.send_request(self.alloc_color_request(r, g, b))
    }

    /// Allocate a new color in the colormap, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn alloc_color_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        r: u16,
        g: u16,
        b: u16,
    ) -> SendRequestFuture<'_, Dpy, AllocColorRequest> {
        dpy.send_request_async(self.alloc_color_request(r, g, b))
    }

    /// Immediately allocate a new color in the colormap.
    #[inline]
    pub fn alloc_color_immediate<Dpy: Display + ?Sized>(
        self,
        dpy: &mut Dpy,
        r: u16,
        g: u16,
        b: u16,
    ) -> crate::Result<ColorAllocation> {
        let tok = self.alloc_color(dpy, r, g, b)?;
        Ok(ColorAllocation::from_alloc_color_reply(
            dpy.resolve_request(tok)?,
            r,
            g,
            b,
        ))
    }

    /// Immediately allocate a new color in the colormap, async redox.
    #[cfg(feature = "async")]
    #[inline]
    pub async fn alloc_color_immediate_async<Dpy: AsyncDisplay + ?Sized>(
        self,
        dpy: &mut Dpy,
        r: u16,
        g: u16,
        b: u16,
    ) -> MapFuture<
        ExchangeRequestFuture<'_, Dpy, AllocColorRequest>,
        BoxedFnOnce<AllocColorReply, ColorAllocation>,
    > {
        MapFuture::run(
            dpy.exchange_request_async(self.alloc_color_request(r, g, b)),
            Box::new(move |acr| ColorAllocation::from_alloc_color_reply(acr, r, g, b)),
        )
    }
}
