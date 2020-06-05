/* -----------------------------------------------------------------------------------
 * src/image.rs - A bitmap or pixmap on the X11 server.
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

use super::{DroppableObject, FlutterbugError};
use euclid::default::Point2D;
use std::{
    fmt,
    os::raw::c_ulong,
    ptr::NonNull,
    sync::{Arc, Weak},
};
use x11::xlib;

/// A 2D pixmap image.
pub struct Image {
    inner: Arc<NonNull<xlib::XImage>>,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let ptr = self.inner.as_ptr();
        let width = unsafe { (*ptr).width };
        let height = unsafe { (*ptr).height };
        write!(f, "{}x{} X11 Image", width, height)
    }
}

impl Drop for Image {
    fn drop(&mut self) {
        unsafe { xlib::XDestroyImage(self.inner.as_ptr()) };
        // note: XDestroyImage also destroys the inner data
    }
}

impl Image {
    #[inline]
    pub(crate) fn from_raw(inner: Arc<NonNull<xlib::XImage>>) -> Self {
        Self { inner }
    }
}

/// A reference to a 2D pixmap image.
pub struct ImageReference {
    inner: Weak<NonNull<xlib::XImage>>,
}

impl fmt::Debug for ImageReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Image Reference")
    }
}

impl Clone for ImageReference {
    fn clone(&self) -> Self {
        Self::from_raw(self.inner.clone())
    }
}

impl ImageReference {
    #[inline]
    pub(crate) fn from_raw(inner: Weak<NonNull<xlib::XImage>>) -> Self {
        Self { inner }
    }
}

/// Either an Image or an ImageReference
pub trait GenericImage: fmt::Debug {
    /// Get a raw pointer to the image.
    fn raw(&self) -> Result<NonNull<xlib::XImage>, FlutterbugError>;
    /// Get a reference to the image.
    fn reference(&self) -> ImageReference;

    /// Get the width of the image.
    fn width(&self) -> Result<u32, FlutterbugError> {
        let ptr = self.raw()?.as_ptr();
        Ok(unsafe { (*ptr).width } as u32)
    }

    /// Get the height of the image.
    fn height(&self) -> Result<u32, FlutterbugError> {
        let ptr = self.raw()?.as_ptr();
        Ok(unsafe { (*ptr).height } as u32)
    }

    /// Put a pixel into the image.
    fn put_pixel(&self, loc: Point2D<i32>, r: u8, g: u8, b: u8) -> Result<(), FlutterbugError> {
        let pixel = (65536 * (b as c_ulong)) + (256 * (g as c_ulong)) + (r as c_ulong);

        unsafe { xlib::XPutPixel(self.raw()?.as_mut(), loc.x, loc.y, pixel) };
        Ok(())
    }
}

impl GenericImage for Image {
    #[inline]
    fn raw(&self) -> Result<NonNull<xlib::XImage>, FlutterbugError> {
        Ok(*self.inner)
    }

    #[inline]
    fn reference(&self) -> ImageReference {
        ImageReference::from_raw(Arc::downgrade(&self.inner))
    }
}

impl GenericImage for ImageReference {
    #[inline]
    fn raw(&self) -> Result<NonNull<xlib::XImage>, FlutterbugError> {
        Ok(*self
            .inner
            .upgrade()
            .ok_or_else(|| FlutterbugError::PointerWasDropped(DroppableObject::Image))?)
    }

    #[inline]
    fn reference(&self) -> ImageReference {
        self.clone()
    }
}
