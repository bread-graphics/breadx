/* -----------------------------------------------------------------------------------
 * src/font.rs - Wrapper around X11 fonts. These don't tend to be that useful.
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

use super::{DisplayReference, FlutterbugError, GenericDisplay};
use alloc::string::String;
use core::fmt;
use core::ptr::NonNull;
use cstr_core::CStr;
use cty::c_void;
use x11::xlib;

pub struct Font {
    storage: NonNull<xlib::XFontStruct>,
    dpy: DisplayReference,
}

impl fmt::Debug for Font {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "X11 Font with name \"{}\"",
            match self.name() {
                Ok(s) => s,
                Err(_) => String::from("UNKNOWN"),
            }
        )
    }
}

impl Font {
    /// Get this font from a font ID.
    #[inline]
    pub fn from_id(
        dpy: &dyn GenericDisplay,
        id: xlib::Font,
    ) -> Result<Option<Font>, FlutterbugError> {
        let fnt = unsafe { xlib::XQueryFont(dpy.raw()?.as_mut(), id) };
        Ok(NonNull::new(fnt).map(|storage| Self {
            dpy: dpy.reference(),
            storage,
        }))
    }

    /// Get the name of this font.
    #[inline]
    pub fn name(&self) -> Result<String, FlutterbugError> {
        let mut atom: crate::Atom = 0;
        if unsafe { xlib::XGetFontProperty(self.storage.as_ptr(), xlib::XA_FONT, &mut atom) } == 0 {
            return Err(FlutterbugError::FunctionFailed("XGetFontProperty"));
        }

        // shenanigans to convert a C string to Rust
        let ptr = unsafe { xlib::XGetAtomName(self.dpy.raw()?.as_mut(), atom) };
        let res = unsafe { CStr::from_ptr(ptr) };
        let res = String::from(res.to_string_lossy());
        unsafe { xlib::XFree(ptr as *mut c_void) };
        Ok(res)
    }
}
