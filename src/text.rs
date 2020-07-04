/* -----------------------------------------------------------------------------------
 * src/text.rs - Text and font-related structs and functions.
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

use super::{DisplayReference, DroppableObject, FlutterbugError};
use alloc::sync::{Arc, Weak};
use core::{fmt, ptr::NonNull};
use x11::xlib::{self, _XIC, _XIM};

/// An X11 input method, used to determine how text is input.
pub struct InputMethod {
    inner: NonNull<_XIM>,
}

impl fmt::Debug for InputMethod {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Input Method")
    }
}

impl InputMethod {
    #[inline]
    pub(crate) fn from_raw(_dpy: DisplayReference, inner: NonNull<_XIM>) -> Self {
        Self { inner }
    }

    /// Get the raw pointer to this InputMethod.
    #[inline]
    pub fn raw(&self) -> NonNull<_XIM> {
        self.inner
    }
}

impl Drop for InputMethod {
    fn drop(&mut self) {
        unsafe { xlib::XCloseIM(self.inner.as_mut()) };
    }
}

/// An X11 input context, used to determine how text is input.
pub struct InputContext {
    inner: Arc<NonNull<_XIC>>,
    dpy: DisplayReference,
    win: xlib::Window,
}

impl fmt::Debug for InputContext {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Input Context for window {}", self.win)
    }
}

impl Drop for InputContext {
    fn drop(&mut self) {
        unsafe { xlib::XDestroyIC(self.inner.as_ptr()) };
    }
}

impl InputContext {
    #[inline]
    pub(crate) fn from_raw(
        dpy: DisplayReference,
        inner: Arc<NonNull<_XIC>>,
        win: xlib::Window,
    ) -> Self {
        Self { dpy, inner, win }
    }
}

/// A reference to an X11 input context.
pub struct InputContextReference {
    inner: Weak<NonNull<_XIC>>,
    dpy: DisplayReference,
    win: xlib::Window,
}

impl fmt::Debug for InputContextReference {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "X11 Input Context Reference for window {}", self.win)
    }
}

impl Clone for InputContextReference {
    fn clone(&self) -> InputContextReference {
        Self::from_raw(self.dpy.clone(), self.inner.clone(), self.win)
    }
}

impl InputContextReference {
    #[inline]
    pub(crate) fn from_raw(
        dpy: DisplayReference,
        inner: Weak<NonNull<_XIC>>,
        win: xlib::Window,
    ) -> Self {
        Self { dpy, inner, win }
    }
}

/// Trait that applies to InputContext and InputContextReference.
pub trait GenericInputContext: fmt::Debug {
    /// Get a pointer to the raw input context.
    fn raw(&self) -> Result<NonNull<_XIC>, FlutterbugError>;
    /// Get a clonable reference to this context.
    fn reference(&self) -> InputContextReference;
    /// Set the input focus for this input context.
    fn set_focus(&self) -> Result<(), FlutterbugError> {
        unsafe { xlib::XSetICFocus(self.raw()?.as_mut()) };
        Ok(())
    }
}

impl GenericInputContext for InputContext {
    #[inline]
    fn raw(&self) -> Result<NonNull<_XIC>, FlutterbugError> {
        Ok(*self.inner)
    }

    #[inline]
    fn reference(&self) -> InputContextReference {
        InputContextReference::from_raw(self.dpy.clone(), Arc::downgrade(&self.inner), self.win)
    }
}

impl GenericInputContext for InputContextReference {
    #[inline]
    fn raw(&self) -> Result<NonNull<_XIC>, FlutterbugError> {
        Ok(*self
            .inner
            .upgrade()
            .ok_or_else(|| FlutterbugError::PointerWasDropped(DroppableObject::IC))?)
    }

    #[inline]
    fn reference(&self) -> InputContextReference {
        self.clone()
    }
}
