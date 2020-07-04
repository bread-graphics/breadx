/* -----------------------------------------------------------------------------------
 * src/context.rs - A wrapper around the XContext object.
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
use alloc::boxed::Box;
use core::{
    any::{Any, TypeId},
    fmt, ptr,
};
use cty::c_char;
use hashbrown::HashMap;
use x11::xlib::{self, XID};

mod xctx {
    use x11::xlib::{self, XContext};

    /// XUniqueContext is just an alias to XrmUniqueQuark
    #[allow(non_snake_case)]
    #[inline]
    pub unsafe fn XUniqueContext() -> XContext {
        xlib::XrmUniqueQuark() as XContext
    }
}

/// A wrapper around the XContext object.
pub struct Context {
    inner: xlib::XContext,
    dpy: DisplayReference,
    current_types: HashMap<XID, TypeId>,
}

impl fmt::Debug for Context {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "X11 Context Manager {{ Types: {:?} }}",
            &self.current_types
        )
    }
}

impl Drop for Context {
    fn drop(&mut self) {
        if let Ok(d) = self.dpy.raw() {
            self.current_types.iter().for_each(|(i, _)| {
                let mut ptr: *mut dyn Any = ptr::null_mut::<i32>(); // need to coerce from a sized type
                unsafe {
                    xlib::XFindContext(
                        d.as_ptr(),
                        *i,
                        self.inner,
                        &mut ptr as *mut *mut dyn Any as *mut *mut c_char,
                    )
                };
                let _b = unsafe { Box::<dyn Any>::from_raw(ptr) }; // let the Box destructor destroy the memory
                unsafe { xlib::XDeleteContext(d.as_ptr(), *i, self.inner) };
            });
        }
    }
}

impl Context {
    /// Create a new context object.
    #[inline]
    pub(crate) fn from_dpy(dpy: DisplayReference) -> Self {
        Self {
            inner: unsafe { xctx::XUniqueContext() },
            dpy,
            current_types: HashMap::new(),
        }
    }

    /// Insert a boxed item into the context.
    pub fn insert_boxed<T: 'static>(
        &mut self,
        h: &dyn HasXID,
        b: Box<T>,
    ) -> Result<(), FlutterbugError> {
        self.current_types.insert(h.xid(), TypeId::of::<T>());
        let ptr = Box::<T>::into_raw(b) as *const T;
        unsafe {
            xlib::XSaveContext(
                self.dpy.raw()?.as_mut(),
                h.xid(),
                self.inner,
                ptr as *const c_char,
            )
        };
        Ok(())
    }

    /// Insert an item into the context.
    #[inline]
    pub fn insert<T: 'static>(&mut self, h: &dyn HasXID, b: T) -> Result<(), FlutterbugError> {
        self.insert_boxed(h, Box::new(b))
    }

    /// Remove an object from a map and return it.
    pub fn remove<T: 'static>(&mut self, h: &dyn HasXID) -> Result<Box<T>, FlutterbugError> {
        // check to ensure the type is proper
        let ty = self
            .current_types
            .remove(&h.xid())
            .ok_or_else(|| FlutterbugError::ContextNotFound(h.xid()))?;
        if TypeId::of::<T>() != ty {
            self.current_types.insert(h.xid(), ty);
            return Err(FlutterbugError::ContextInvalidType);
        }

        let mut ptr: *mut T = ptr::null_mut();
        unsafe {
            xlib::XFindContext(
                self.dpy.raw()?.as_mut(),
                h.xid(),
                self.inner,
                &mut ptr as *mut *mut T as *mut *mut c_char,
            )
        };
        unsafe { xlib::XDeleteContext(self.dpy.raw()?.as_mut(), h.xid(), self.inner) };
        Ok(unsafe { Box::from_raw(ptr) })
    }
}
