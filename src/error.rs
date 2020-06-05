/* -----------------------------------------------------------------------------------
 * src/error.rs - Errors that can occur over the course of Flutterbug operation.
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

//! Error handling module.
//!
//! This module exports types related to error handling, such as the FlutterbugError,
//! a catch-all error type, and the X11Error, which holds data derived from the
//! XErrorEvent type.

use std::{
    ffi::{IntoStringError, NulError},
    fmt,
    sync::TryLockError,
};
use thiserror::Error;
use x11::xlib::XID;

/// Objects that can be dropped with a reference.
#[derive(Debug, Clone, Copy)]
pub enum DroppableObject {
    Display,
    GC,
    IC,
    Image,
}

impl fmt::Display for DroppableObject {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match *self {
                DroppableObject::Display => "Display",
                DroppableObject::GC => "GC",
                DroppableObject::IC => "IC",
                DroppableObject::Image => "Image",
            }
        )
    }
}

/// An error that occurred during normal operation of Flutterbug.
#[derive(Debug, Error)]
pub enum FlutterbugError {
    /// A non-static error message.
    #[error("{0}")]
    Msg(String),
    /// A static error message.
    #[error("{0}")]
    StaticMsg(&'static str),
    /// A function failed.
    #[error("The X11 function {0} failed.")]
    FunctionFailed(&'static str),
    /// Can't cast pointer reference to pointer.
    #[error(
        "Unable to cast {0}Reference to {0}. This usually means that the {0} object was dropped."
    )]
    PointerWasDropped(DroppableObject),
    /// XCreateIC returned null.
    #[error("Unable to create input context.")]
    ICWasNull,
    /// XOpenDisplay returned null.
    #[error("Unable to open X11 display")]
    UnableToOpenDisplay,
    #[error("Unable to create image")]
    ImageWasNull,
    /// No ID in color map
    #[error("Pixel ID {0} was not found in the colormap")]
    ColorNotFound(std::os::raw::c_ulong),
    /// GC returned null.
    #[error("Unable to create graphics context for window")]
    GCWasNull,
    /// Invalid event type
    #[error("Event type is not a valid X11 event type")]
    InvalidEventType,
    /// RwLock failed to lock.
    #[error("Unable to create lock for RwLock")]
    LockError,
    /// Context does not contain object
    #[error("The context did not contain object {0}")]
    ContextNotFound(XID),
    /// Context contained invalid type
    #[error("Context contained invalid type object")]
    ContextInvalidType,
    #[error("Unable to access inner element of AnyEvent")]
    InnerAnyEventInaccessible,
    #[error("Display field for event is null")]
    DisplayFieldNull,
    #[error("Unable to retrieve input method")]
    InputMethodNull,
    #[error("Graphics contexts must be identical in order to copy")]
    GCMustEqual,
    /// CString creation process failed.
    #[error("CString creation process failed")]
    NulError(#[from] NulError),
    #[error("CString failed to convert to UTF-8 String")]
    IntoString(#[from] IntoStringError),
}

impl<T> From<TryLockError<T>> for FlutterbugError {
    #[inline]
    fn from(_t: TryLockError<T>) -> Self {
        Self::LockError
    }
}

impl From<FlutterbugError> for fmt::Error {
    #[inline] 
    fn from(_f: FlutterbugError) -> Self {
        Self
    }
}
