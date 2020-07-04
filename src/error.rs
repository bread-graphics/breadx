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

use alloc::string::String;
use core::fmt;
use cstr_core::{IntoStringError, NulError};
use cty::c_ulong;
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
#[derive(Debug, Clone)]
pub enum FlutterbugError {
    /// A non-static error message.
    Msg(String),
    /// A static error message.
    StaticMsg(&'static str),
    /// A function failed.
    FunctionFailed(&'static str),
    /// Can't cast pointer reference to pointer.
    PointerWasDropped(DroppableObject),
    /// XCreateIC returned null.
    ICWasNull,
    /// XOpenDisplay returned null.
    UnableToOpenDisplay,
    ImageWasNull,
    /// No ID in color map
    ColorNotFound(c_ulong),
    /// GC returned null.
    GCWasNull,
    /// Invalid event type
    InvalidEventType,
    /// Context does not contain object
    ContextNotFound(XID),
    /// Context contained invalid type
    ContextInvalidType,
    InnerAnyEventInaccessible,
    DisplayFieldNull,
    InputMethodNull,
    GCMustEqual,
    Nul(NulError),
    IntoString(IntoStringError),
}

impl fmt::Display for FlutterbugError {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Msg(ref s) => f.pad(s),
            Self::StaticMsg(s) => f.pad(s),
            Self::FunctionFailed(s) => f.pad(s),
            Self::PointerWasDropped(d) => write!(
                f,
                "Attempted to cast {0}Reference to {0}. It is possible that the {0} was dropped.",
                d
            ),
            Self::ICWasNull => f.pad("Unable to create input context"),
            Self::GCWasNull => f.pad("Unable to create graphics context"),
            Self::InvalidEventType => f.pad("Invalid event type"),
            Self::ContextNotFound(x) => write!(f, "Unable to find ID {} within XContext", x),
            Self::UnableToOpenDisplay => f.pad("Unable to open display"),
            Self::ImageWasNull => f.pad("Unable to create image"),
            Self::ColorNotFound(c) => write!(f, "The color ID {} did not return a color", c),
            Self::ContextInvalidType => f.pad("Attempted to retrieve invalid type from context"),
            Self::InnerAnyEventInaccessible => f.pad("Unable to access inner AnyEvent"),
            Self::DisplayFieldNull => f.pad("Display field was null"),
            Self::InputMethodNull => f.pad("Input method was null"),
            Self::GCMustEqual => f.pad("Graphics context for operations were not equal"),
            Self::Nul(ref n) => fmt::Display::fmt(n, f),
            Self::IntoString(ref i) => fmt::Display::fmt(i, f),
        }
    }
}

impl From<FlutterbugError> for fmt::Error {
    #[inline]
    fn from(_f: FlutterbugError) -> Self {
        Self
    }
}

#[cfg(feature = "std")]
impl std::error::Error for FlutterbugError {}

impl From<NulError> for FlutterbugError {
    #[inline]
    fn from(n: NulError) -> FlutterbugError {
        Self::Nul(n)
    }
}

impl From<IntoStringError> for FlutterbugError {
    #[inline]
    fn from(i: IntoStringError) -> FlutterbugError {
        Self::IntoString(i)
    }
}
