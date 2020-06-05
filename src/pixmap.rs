/* -----------------------------------------------------------------------------------
 * src/pixmap.rs - An array of pixels that can be drawn upon.
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

use super::{DisplayReference, Drawable, GenericDisplay, GraphicsContextReference, HasXID};
use x11::xlib;

/// A pixmap that can be drawn upon.
#[derive(Debug)]
pub struct Pixmap {
    inner: xlib::Pixmap,
    dpy: DisplayReference,
    gc: GraphicsContextReference,
}

impl Drop for Pixmap {
    fn drop(&mut self) {
        if let Ok(mut d) = self.dpy.raw() {
            unsafe { xlib::XFreePixmap(d.as_mut(), self.inner) };
        }
    }
}

impl Pixmap {
    #[inline]
    pub(crate) fn from_raw(
        inner: xlib::Pixmap,
        dpy: DisplayReference,
        gc: GraphicsContextReference,
    ) -> Self {
        Self { inner, dpy, gc }
    }
}

impl Drawable for Pixmap {
    #[inline]
    fn gc_ref(&self) -> GraphicsContextReference {
        self.gc.clone()
    }
    #[inline]
    fn dpy(&self) -> DisplayReference {
        self.dpy.clone()
    }
}

impl HasXID for Pixmap {
    #[inline]
    fn xid(&self) -> xlib::XID {
        self.inner
    }
}
