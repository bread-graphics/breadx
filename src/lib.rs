/* -----------------------------------------------------------------------------------
 * src/lib.rs - Root of the Flutterbug library, for safe X11 bindings.
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

//! X11 wrapper library.

pub extern crate x11;

use euclid::default::{Point2D, Size2D};
use std::{
    ffi::CString,
    fmt, mem,
    os::raw::{c_char, c_int, c_uint},
    ptr::{self, NonNull},
    sync::{Arc, Weak},
};
use x11::xlib::{self, XID};

pub use x11::xlib::{Atom, KeySym};

pub mod color;
pub use color::*;
pub mod context;
pub use context::*;
pub mod drawable;
pub use drawable::*;
pub mod error;
pub use error::*;
pub mod event;
pub use event::*;
pub mod font;
pub use font::*;
pub mod image;
pub use image::*;
pub mod pixmap;
pub use pixmap::*;
mod screen;
pub use screen::*;
pub mod text;
pub use text::*;
pub mod window;
pub use window::*;

/// Utility function to convert a String into an ASCII *mut c_char
#[inline]
pub(crate) unsafe fn to_cstring(s: &str) -> Result<*mut c_char, FlutterbugError> {
    Ok(CString::new(s)?.into_raw())
}

/// Utility function to create a string buffer of a certain length.
#[inline]
pub(crate) fn cstring_buffer(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}

/// A trait that represents that something can be transformed into an XID.
pub trait HasXID {
    /// Get the XID for this instance.
    fn xid(&self) -> XID;
}

impl HasXID for XID {
    #[inline]
    fn xid(&self) -> XID {
        *self
    }
}

/// The X11 display. This is the context object used for the X11 window.
///
/// Note: This object is not clonable. Use the reference() method to get
/// a cheap reference to this object.
pub struct Display {
    raw: Arc<NonNull<xlib::Display>>,
}

impl PartialEq for Display {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        // check if the pointers are equal
        self.raw.as_ptr() == other.raw.as_ptr()
    }
}

impl Eq for Display {}

// make sure it can be debugged
impl fmt::Debug for Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Display Object")
    }
}

impl Drop for Display {
    fn drop(&mut self) {
        unsafe { xlib::XCloseDisplay(self.raw.as_ptr()) };
    }
}

impl Display {
    /// Create a new Display. This will call the XOpenDisplay function and
    /// store the result in an Arc. If XOpenDisplay returns null, the
    /// UnableToOpenDisplay error is returned instead.
    #[inline]
    pub fn new() -> Result<Self, FlutterbugError> {
        let display_ptr = unsafe { xlib::XOpenDisplay(ptr::null()) };
        match NonNull::new(display_ptr) {
            Some(dpy) => Ok(Self::from_raw(Arc::new(dpy))),
            None => Err(FlutterbugError::UnableToOpenDisplay),
        }
    }

    /// Since the Display object is a cheap wrapper around the Display pointer,
    /// we can use it to forward calls to the Arc<> once we upgrade a Weak<> to
    /// it. This just creates the wrapper.
    #[inline]
    pub(crate) fn from_raw(raw: Arc<NonNull<xlib::Display>>) -> Self {
        Self { raw }
    }
}

/// A reference to the X11 display. Unlike the Display object, this is
/// clonable. However, it will also decay if its parent Display object
/// is dropped.
pub struct DisplayReference {
    reference: Weak<NonNull<xlib::Display>>,
}

// make sure it can be debuged
impl fmt::Debug for DisplayReference {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Display Reference")
    }
}

impl PartialEq for DisplayReference {
    fn eq(&self, other: &Self) -> bool {
        self.reference.ptr_eq(&other.reference)
    }
}

impl Clone for DisplayReference {
    #[inline]
    fn clone(&self) -> Self {
        Self {
            reference: self.reference.clone(),
        }
    }
}

impl DisplayReference {
    #[inline]
    pub(crate) fn from_ref(reference: Weak<NonNull<xlib::Display>>) -> Self {
        Self { reference }
    }
}

/// This trait is applied to both Display and DisplayReference to ensure you can do
/// the same things with both.
///
/// Note: All methods return a Result<T, FlutterbugError> since upgrading the reference
/// to a full object can generate an error if the real Display is already dropped.
pub trait GenericDisplay: fmt::Debug {
    /// Create a reference to this object.
    fn reference(&self) -> DisplayReference;

    /// Get the pointer to the raw Display object.
    fn raw(&self) -> Result<NonNull<xlib::Display>, FlutterbugError>;

    /// Get the default screen for this instance.
    #[inline]
    fn default_screen(&self) -> Result<Screen, FlutterbugError> {
        Ok(Screen::new(unsafe {
            xlib::XDefaultScreen(self.raw()?.as_mut())
        }))
    }

    /// Get the visual for the screen.
    #[inline]
    fn visual(&self, screen: Screen) -> Result<*mut xlib::Visual, FlutterbugError> {
        Ok(unsafe { xlib::XDefaultVisual(self.raw()?.as_mut(), screen.value()) })
    }

    /// Get the default visual for the default screen.
    #[inline]
    fn default_visual(&self) -> Result<*mut xlib::Visual, FlutterbugError> {
        self.visual(self.default_screen()?)
    }

    /// Get the black pixel for the screen.
    #[inline]
    fn black_pixel(&self, screen: Screen) -> Result<Color, FlutterbugError> {
        Ok(Color::PixelID(unsafe {
            xlib::XBlackPixel(self.raw()?.as_mut(), screen.value())
        }))
    }

    /// Get the default black pixel for the default screen.
    #[inline]
    fn default_black_pixel(&self) -> Result<Color, FlutterbugError> {
        self.black_pixel(self.default_screen()?)
    }

    /// Get the white pixel for the screen.
    #[inline]
    fn white_pixel(&self, screen: Screen) -> Result<Color, FlutterbugError> {
        Ok(Color::PixelID(unsafe {
            xlib::XWhitePixel(self.raw()?.as_mut(), screen.value())
        }))
    }

    /// Get the default white pixel for the default screen.
    #[inline]
    fn default_white_pixel(&self) -> Result<Color, FlutterbugError> {
        self.white_pixel(self.default_screen()?)
    }

    /// Get the colormap for the screen.
    #[inline]
    fn colormap(&self, screen: Screen) -> Result<ColorMap, FlutterbugError> {
        let cmp = unsafe { xlib::XDefaultColormap(self.raw()?.as_mut(), screen.value()) };
        Ok(ColorMap::from_raw(cmp, self.reference(), true)?)
    }

    /// Get the default colormap for the default screen.
    #[inline]
    fn default_colormap(&self) -> Result<ColorMap, FlutterbugError> {
        self.colormap(self.default_screen()?)
    }

    /// Get the default graphics context for a screen.
    #[inline] 
    fn gc(&self, screen: Screen) -> Result<GraphicsContext, FlutterbugError> {
        let gc = unsafe { xlib::XDefaultGC(self.raw()?.as_mut(), screen.value()) };
        let gc = NonNull::new(gc).ok_or_else(|| FlutterbugError::GCWasNull)?;
        Ok(GraphicsContext::from_raw(Arc::new(gc), self.reference(), true))
    }

    /// Get the graphics context for the default screen.
    #[inline]
    fn default_gc(&self) -> Result<GraphicsContext, FlutterbugError> {
        self.gc(self.default_screen()?)
    }

    /// Create a simple window from this display.
    fn create_simple_window(
        &self,
        parent: Option<&Window>,
        origin: Point2D<i32>,
        size: Size2D<u32>,
        border_width: u32,
        border_color: Color,
        background_color: Color,
    ) -> Result<Window, FlutterbugError> {
        macro_rules! test_color {
            ($cname: ident) => {
                if $cname != self.default_black_pixel()? && $cname != self.default_white_pixel()? {
                    return Err(FlutterbugError::Msg(format!(
                        "{} must be either black or white",
                        &stringify!($cname)
                    )));
                }
            };
        }

        test_color!(border_color);
        test_color!(background_color);

        let win = unsafe {
            xlib::XCreateSimpleWindow(
                self.raw()?.as_mut(),
                match parent {
                    Some(p) => p.window(),
                    None => xlib::XRootWindow(self.raw()?.as_mut(), self.default_screen()?.value()),
                },
                origin.x as c_int,
                origin.y as c_int,
                size.width as c_uint,
                size.height as c_uint,
                border_width as c_uint,
                border_color.pixel_id(),
                background_color.pixel_id(),
            )
        };

        // create a graphics context
        let gc = unsafe { xlib::XCreateGC(self.raw()?.as_mut(), win, 0, ptr::null_mut()) };
        let gc = NonNull::new(gc).ok_or_else(|| FlutterbugError::GCWasNull)?;
        let gc = GraphicsContext::from_raw(Arc::new(gc), self.reference(), false);

        Ok(Window::from_raw(win, self.reference(), gc))
    }
    /// Create a context using this connection.
    fn create_context(&self) -> Result<Context, FlutterbugError> {
        Ok(Context::from_dpy(self.reference()))
    }
    /// Get an internal atom based upon its name.
    fn internal_atom(
        &self,
        name: &str,
        create_if_exists: bool,
    ) -> Result<xlib::Atom, FlutterbugError> {
        let txt = unsafe { to_cstring(name) }?;
        let val = Ok(unsafe {
            xlib::XInternAtom(
                self.raw()?.as_mut(),
                txt,
                if create_if_exists { 1 } else { 0 },
            )
        });
        let _ = unsafe { CString::from_raw(txt) };
        val
    }
    /// Create a new input method based on this display.
    fn input_method(&self) -> Result<InputMethod, FlutterbugError> {
        // try to get the XIM based on the environment vars
        unsafe { libc::setlocale(libc::LC_ALL, (&[0]).as_ptr()) };
        unsafe { xlib::XSetLocaleModifiers((&[0]).as_ptr()) };

        #[inline]
        fn open_im(mut dpy: NonNull<xlib::Display>) -> xlib::XIM {
            unsafe {
                xlib::XOpenIM(
                    dpy.as_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                    ptr::null_mut(),
                )
            }
        }

        let xim = NonNull::new(open_im(self.raw()?));
        Ok(InputMethod::from_raw(
            self.reference(),
            match xim {
                Some(x) => x,
                None => {
                    // try setting the locale to the internal input method
                    let txt = unsafe { to_cstring("@im=none") }?;
                    unsafe { xlib::XSetLocaleModifiers(txt) };
                    let _ = unsafe { CString::from_raw(txt) };

                    NonNull::new(open_im(self.raw()?))
                        .ok_or_else(|| FlutterbugError::InputMethodNull)?
                }
            },
        ))
    }
    /// Create a new image from this display.
    #[inline]
    fn create_image(
        &self,
        bounds: Size2D<u32>,
        depth: u32,
        data: Vec<c_char>,
    ) -> Result<Image, FlutterbugError> {
        let mut boxed = data.into_boxed_slice();
        let ptr = boxed.as_mut_ptr();
        let raw = self.raw()?;
        let img = unsafe {
            xlib::XCreateImage(
                raw.as_ptr(),
                self.default_visual()?,
                depth as c_uint,
                xlib::ZPixmap,
                0,
                ptr,
                bounds.width,
                bounds.height,
                32,
                0,
            )
        };
        let img = NonNull::new(img).ok_or_else(|| FlutterbugError::ImageWasNull)?;

        // don't dealloc ptr
        mem::forget(boxed);

        Ok(Image::from_raw(Arc::new(img)))
    }

    fn sync(&self, s: bool) -> Result<(), FlutterbugError> {
        unsafe { xlib::XSync(self.raw()?.as_mut(), if s { 1 } else { 0 }) };
        Ok(())
    }

    /// Get the depth for a particular screen.
    fn depth(&self, screen: Screen) -> Result<i32, FlutterbugError> {
        Ok(unsafe { xlib::XDefaultDepth(self.raw()?.as_mut(), screen.value()) })
    }

    /// Get the depth for the default screen.
    fn default_depth(&self) -> Result<i32, FlutterbugError> {
        self.depth(self.default_screen()?)
    }
}

impl GenericDisplay for Display {
    #[inline]
    fn reference(&self) -> DisplayReference {
        DisplayReference::from_ref(Arc::downgrade(&self.raw))
    }
    #[inline]
    fn raw(&self) -> Result<NonNull<xlib::Display>, FlutterbugError> {
        Ok(*self.raw)
    }
}

// just forward calls to the inner Display
impl GenericDisplay for DisplayReference {
    #[inline]
    fn reference(&self) -> DisplayReference {
        self.clone()
    }
    #[inline]
    fn raw(&self) -> Result<NonNull<xlib::Display>, FlutterbugError> {
        Ok(*self
            .reference
            .upgrade()
            .ok_or_else(|| FlutterbugError::PointerWasDropped(DroppableObject::Display))?)
    }
}

/// Traits that should be imported in order to ensure the function of the library.
pub mod prelude {
    pub use super::{
        DerivesAnEvent, DerivesEvent, Drawable, GenericDisplay, GenericGraphicsContext,
        GenericImage, GenericInputContext, HasXID,
    };
}
