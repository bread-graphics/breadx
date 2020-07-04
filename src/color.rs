/* -----------------------------------------------------------------------------------
 * src/window.rs - File for handling colors and colormaps.
 * beetle - Simple graphics framework for Rust
 * Copyright © 2020 not_a_seagull
 *
 * This project is licensed under either the Apache 2.0 license or the MIT license, at
 * your option. For more information, please consult the LICENSE-APACHE or LICENSE-MIT
 * files in the repository root.
 * -----------------------------------------------------------------------------------
 * MIT License:
 *
 * Permission is hereby granted, free of charge, to any person obtaining a copy
 * of this software and associated documentation files (the “Software”), to deal
 * in the Software without restriction, including without limitation the rights
 * to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
 * copies of the Software, and to permit persons to whom the Software is
 * furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED “AS IS”, WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
 * AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
 * OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
 * THE SOFTWARE.
 * -----------------------------------------------------------------------------------
 * Apache 2.0 License Declaration:
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 * ----------------------------------------------------------------------------------
 */

use super::{DisplayReference, FlutterbugError, GenericDisplay, HasXID};
use core::{fmt, mem};
use cty::{c_char, c_ulong, c_ushort};
use x11::xlib;

/// A color that can be used in an X11 setting.
#[derive(Debug, Copy, Clone)]
pub enum Color {
    /// A color consisting of the amount of red, green, and blue.
    Rgb {
        r: c_ushort,
        g: c_ushort,
        b: c_ushort,
    },
    /// A monochrome color consisting of only one value.
    Monochrome(c_ushort),
    /// Refers to the actual pixel ID in the color map.
    PixelID(c_ulong),
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        match *self {
            Color::Rgb { r, g, b } => match *other {
                Color::Rgb {
                    r: r1,
                    g: g1,
                    b: b1,
                } => r == r1 && g == g1 && b == b1,
                _ => false,
            },
            Color::Monochrome(m) => match *other {
                Color::Monochrome(m1) => m == m1,
                _ => false,
            },
            Color::PixelID(i) => match *other {
                Color::PixelID(i1) => i == i1,
                _ => false,
            },
        }
    }
}

impl Eq for Color {}

impl Color {
    /// Create a new color from the RGB values.
    #[inline]
    pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
        Self::Rgb {
            r: r as c_ushort * 256,
            g: g as c_ushort * 256,
            b: b as c_ushort * 256,
        }
    }

    /// Create a color from a pixel in a color map.
    #[inline]
    pub fn from_pixel_id(id: c_ulong) -> Self {
        Self::PixelID(id)
    }

    /// Get the pixel ID of this color. This will only work if it is of the
    /// PixelID variant; otherwise, it will panic.
    #[inline]
    pub fn pixel_id(&self) -> c_ulong {
        match *self {
            Color::PixelID(p) => p,
            _ => panic!("Color was not of the PixelID variant: {:?}", self),
        }
    }
}

/// A color map that is installed in a window. Use this to create colors.
pub struct ColorMap {
    internal: xlib::Colormap,
    dpy: DisplayReference,
    // also store the black and white pixels
    black_pixel: c_ulong,
    white_pixel: c_ulong,
    is_default: bool,
}

impl fmt::Debug for ColorMap {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Colormap")
    }
}

impl HasXID for ColorMap {
    #[inline]
    fn xid(&self) -> xlib::XID {
        self.internal
    }
}

impl ColorMap {
    #[inline]
    pub(crate) fn from_raw(
        internal: xlib::Colormap,
        dpy: DisplayReference,
        is_default: bool,
    ) -> Result<Self, FlutterbugError> {
        Ok(Self {
            internal,
            dpy: dpy.clone(),
            black_pixel: dpy.default_black_pixel()?.pixel_id(),
            white_pixel: dpy.default_white_pixel()?.pixel_id(),
            is_default,
        })
    }

    /*
     /// Returns a Vec<> containing all of the colors in this ColorMap. This is a call to
     /// XQueryColors() in a way that fills up a vector.
     fn color_list(&self) -> Result<Vec<xlib::XColor>, FlutterbugError> {
         let cap = self.known_pix_ids.try_read()?.len();
         let mut colors = Vec::<xlib::XColor>::with_capacity(cap);
         unsafe {
             xlib::XQueryColors(
                 self.dpy.raw()?.as_mut(),
                 self.internal,
                 colors.as_mut_ptr(),
                 cap as c_int,
             )
         };
         unsafe { colors.set_len(cap) };
         Ok(colors)
     }
    */

    /// Normalize a color into a PixelID variant.
    pub fn color(&self, clr: Color) -> Result<Color, FlutterbugError> {
        const DO_ALL_COLORS: c_char = xlib::DoRed | xlib::DoGreen | xlib::DoBlue;

        // if clr is a color id already and we're in debug mode, check to see if it's
        // in the pixel list before returning
        let tuple: (c_ushort, c_ushort, c_ushort) = match clr {
            Color::PixelID(_p) => {
                /*                #[cfg(debug_assertions)] // don't bother checking if we're not debugging
                {
                    if p != self.black_pixel
                        || p != self.white_pixel
                        || !(self.known_pix_ids.try_read()?.contains(&p))
                    {
                        return Err(FlutterbugError::ColorNotFound(p));
                    }
                }*/

                return Ok(clr);
            }
            Color::Monochrome(m) => (m, m, m),
            Color::Rgb { r, g, b } => (r, g, b),
        };
        let (r, g, b) = tuple;

        // iterate over the list of colors and check for equality
        if (r == 0) && (g == 0) && (b == 0) {
            return Ok(Color::PixelID(self.black_pixel));
        } else if (r == core::u16::MAX) && (g == core::u16::MAX) && (b == core::u16::MAX) {
            return Ok(Color::PixelID(self.white_pixel));
        }

        // insert the color into this color map
        let mut xcolor = xlib::XColor {
            red: r,
            green: g,
            blue: b,
            flags: DO_ALL_COLORS,
            ..unsafe { mem::zeroed() }
        };

        unsafe { xlib::XAllocColor(self.dpy.raw()?.as_mut(), self.internal, &mut xcolor) };

        Ok(Color::PixelID(xcolor.pixel))
    }
}

impl Drop for ColorMap {
    fn drop(&mut self) {
        if let Ok(mut d) = self.dpy.raw() {
            if !self.is_default {
                unsafe { xlib::XFreeColormap(d.as_mut(), self.internal) };
            }
        }
    }
}
