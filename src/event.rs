/* -----------------------------------------------------------------------------------
 * src/event.rs - An event that is emitted by the X11 server. The main Event object 
 *                should be an enum that encompasses all other events.
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

use super::{FlutterbugError, GenericDisplay, GenericInputContext, Window};
use std::{
    ffi::CString,
    mem,
    os::raw::{c_int, c_long, c_uchar, c_uint, c_ulong},
    ptr::{self, NonNull},
};
use x11::xlib::{self, XID};

/// The type of an X11 event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EventType {
    KeyPress,
    KeyRelease,
    ButtonPress,
    ButtonRelease,
    MotionNotify,
    EnterNotify,
    LeaveNotify,
    FocusIn,
    FocusOut,
    KeymapNotify,
    Expose,
    GraphicsExpose,
    NoExpose,
    CirculateRequest,
    ConfigureRequest,
    MapRequest,
    ResizeRequest,
    CirculateNotify,
    ConfigureNotify,
    CreateNotify,
    DestroyNotify,
    GravityNotify,
    MapNotify,
    MappingNotify,
    ReparentNotify,
    UnmapNotify,
    VisibilityNotify,
    ColormapEvent,
    ClientMessage,
    PropertyNotify,
    SelectionClear,
    SelectionNotify,
    SelectionRequest,
}

impl EventType {
    /// Convert a C integer representing an event to an event
    pub fn from_int(t: c_int) -> Option<Self> {
        Some(match t {
            xlib::KeyPress => Self::KeyPress,
            xlib::KeyRelease => Self::KeyRelease,
            xlib::ButtonPress => Self::ButtonPress,
            xlib::ButtonRelease => Self::ButtonRelease,
            xlib::MotionNotify => Self::MotionNotify,
            xlib::EnterNotify => Self::EnterNotify,
            xlib::LeaveNotify => Self::LeaveNotify,
            xlib::FocusIn => Self::FocusIn,
            xlib::FocusOut => Self::FocusOut,
            xlib::KeymapNotify => Self::KeymapNotify,
            xlib::Expose => Self::Expose,
            xlib::GraphicsExpose => Self::GraphicsExpose,
            xlib::NoExpose => Self::NoExpose,
            xlib::CirculateRequest => Self::CirculateRequest,
            xlib::ConfigureRequest => Self::ConfigureRequest,
            xlib::MapRequest => Self::MapRequest,
            xlib::ResizeRequest => Self::ResizeRequest,
            xlib::CirculateNotify => Self::CirculateNotify,
            xlib::ConfigureNotify => Self::ConfigureNotify,
            xlib::CreateNotify => Self::CreateNotify,
            xlib::DestroyNotify => Self::DestroyNotify,
            xlib::GravityNotify => Self::GravityNotify,
            xlib::MapNotify => Self::MapNotify,
            xlib::MappingNotify => Self::MappingNotify,
            xlib::ReparentNotify => Self::ReparentNotify,
            xlib::UnmapNotify => Self::UnmapNotify,
            xlib::VisibilityNotify => Self::VisibilityNotify,
            xlib::ColormapNotify => Self::ColormapEvent,
            xlib::ClientMessage => Self::ClientMessage,
            xlib::PropertyNotify => Self::PropertyNotify,
            xlib::SelectionClear => Self::SelectionClear,
            xlib::SelectionNotify => Self::SelectionNotify,
            xlib::SelectionRequest => Self::SelectionRequest,
            _ => return None,
        })
    }

    /// Convert to a representative C integer.
    pub fn to_int(self) -> c_int {
        match self {
            Self::KeyPress => xlib::KeyPress,
            Self::KeyRelease => xlib::KeyRelease,
            Self::ButtonPress => xlib::ButtonPress,
            Self::ButtonRelease => xlib::ButtonRelease,
            Self::MotionNotify => xlib::MotionNotify,
            Self::EnterNotify => xlib::EnterNotify,
            Self::LeaveNotify => xlib::LeaveNotify,
            Self::FocusIn => xlib::FocusIn,
            Self::FocusOut => xlib::FocusOut,
            Self::KeymapNotify => xlib::KeymapNotify,
            Self::Expose => xlib::Expose,
            Self::GraphicsExpose => xlib::GraphicsExpose,
            Self::NoExpose => xlib::NoExpose,
            Self::CirculateRequest => xlib::CirculateRequest,
            Self::ConfigureRequest => xlib::ConfigureRequest,
            Self::MapRequest => xlib::MapRequest,
            Self::ResizeRequest => xlib::ResizeRequest,
            Self::CirculateNotify => xlib::CirculateNotify,
            Self::ConfigureNotify => xlib::ConfigureNotify,
            Self::CreateNotify => xlib::CreateNotify,
            Self::DestroyNotify => xlib::DestroyNotify,
            Self::GravityNotify => xlib::GravityNotify,
            Self::MapNotify => xlib::MapNotify,
            Self::MappingNotify => xlib::MappingNotify,
            Self::ReparentNotify => xlib::ReparentNotify,
            Self::UnmapNotify => xlib::UnmapNotify,
            Self::VisibilityNotify => xlib::VisibilityNotify,
            Self::ColormapEvent => xlib::ColormapNotify,
            Self::ClientMessage => xlib::ClientMessage,
            Self::PropertyNotify => xlib::PropertyNotify,
            Self::SelectionClear => xlib::SelectionClear,
            Self::SelectionNotify => xlib::SelectionNotify,
            Self::SelectionRequest => xlib::SelectionRequest,
        }
    }
}

bitflags::bitflags! {
    #[doc = "The masks that can be applied to an event listener"]
    pub struct EventMask : c_long {
        const NO_EVENT_MASK = xlib::NoEventMask;
        const KEY_PRESS_MASK = xlib::KeyPressMask;
        const KEY_RELEASE_MASK = xlib::KeyReleaseMask;
        const BUTTON_PRESS_MASK = xlib::ButtonPressMask;
        const BUTTON_RELEASE_MASK = xlib::ButtonReleaseMask;
        const ENTER_WINDOW_MASK = xlib::EnterWindowMask;
        const LEAVE_WINDOW_MASK = xlib::LeaveWindowMask;
        const POINTER_MOTION_MASK = xlib::PointerMotionMask;
        const POINTER_MOTION_HINT_MASK = xlib::PointerMotionHintMask;
        const BUTTON_1_MOTION_MASK = xlib::Button1MotionMask;
        const BUTTON_2_MOTION_MASK = xlib::Button2MotionMask;
        const BUTTON_3_MOTION_MASK = xlib::Button3MotionMask;
        const BUTTON_4_MOTION_MASK = xlib::Button4MotionMask;
        const BUTTON_5_MOTION_MASK = xlib::Button5MotionMask;
        const BUTTON_MOTION_MASK = xlib::ButtonMotionMask;
        const KEYMAP_STATE_MASK = xlib::KeymapStateMask;
        const EXPOSURE_MASK = xlib::ExposureMask;
        const VISIBILITY_CHANGE_MASK = xlib::VisibilityChangeMask;
        const STRUCTURE_NOTIFY_MASK = xlib::StructureNotifyMask;
        const RESIZE_REDIRECT_MASK = xlib::ResizeRedirectMask;
        const SUBSTRUCTURE_NOTIFY_MASK = xlib::SubstructureNotifyMask;
        const FOCUS_CHANGE_MASK = xlib::FocusChangeMask;
        const PROPERTY_CHANGE_MASK = xlib::PropertyChangeMask;
        const COLORMAP_CHANGE_MASK = xlib::ColormapChangeMask;
        const OWNER_GRAB_BUTTON_MASK = xlib::OwnerGrabButtonMask;
    }
}

/// Non-generic trait for event wrappers.
pub trait DerivesAnEvent: Sized + Clone {
    /// Convert this item to an AnyEvent
    fn as_anyevent(&self) -> AnyEvent {
        AnyEvent::from_raw(
            self.kind(),
            self.serial(),
            unsafe { self.display() },
            self.window(),
            self.from_send_event(),
        )
    }
    /// Get the type of this event.
    fn kind(&self) -> EventType;
    /// Get the serial number associated with this event.
    fn serial(&self) -> c_ulong;
    /// Get the connection associated with this event.
    ///
    /// # Safety
    ///
    /// This is unsafe because it circumvents the usual Flutterbug method of
    /// using the Display/DisplayReference structure.
    unsafe fn display(&self) -> NonNull<xlib::Display>;
    /// Get the window ID representing this event.
    fn window(&self) -> xlib::Window;
    /// Is the event sent from the SendEvent function?
    fn from_send_event(&self) -> bool;
}

/// Trait for event wrappers.
pub trait DerivesEvent<EvStruct: Copy>: DerivesAnEvent {
    /// Derive this item from the native struct.
    fn from_evstruct(xev: EvStruct) -> Result<Self, FlutterbugError>
    where
        Self: Sized;

    /// Get the raw inner event.
    fn inner(&self) -> Result<EvStruct, FlutterbugError>;
}

/// The default XEvent
#[derive(Debug, Clone)]
pub struct AnyEvent {
    kind: EventType,
    serial: c_ulong,
    display: NonNull<xlib::Display>,
    window: xlib::Window,
    from_send_event: bool,
}

impl AnyEvent {
    /// Fill out with raw details
    #[inline]
    pub(crate) fn from_raw(
        kind: EventType,
        serial: c_ulong,
        display: NonNull<xlib::Display>,
        window: xlib::Window,
        fse: bool,
    ) -> Self {
        Self {
            kind,
            serial,
            display,
            window,
            from_send_event: fse,
        }
    }
}

impl DerivesAnEvent for AnyEvent {
    #[inline]
    fn as_anyevent(&self) -> Self {
        self.clone()
    }
    #[inline]
    fn kind(&self) -> EventType {
        self.kind
    }
    #[inline]
    fn serial(&self) -> c_ulong {
        self.serial
    }
    #[inline]
    unsafe fn display(&self) -> NonNull<xlib::Display> {
        self.display
    }
    #[inline]
    fn window(&self) -> xlib::Window {
        self.window
    }
    #[inline]
    fn from_send_event(&self) -> bool {
        self.from_send_event
    }
}

// impl AnyEvent for all events
macro_rules! anyev_impl {
    ($xev: ty [ $winname: ident ]) => {
        impl DerivesEvent<$xev> for AnyEvent {
            #[inline]
            fn from_evstruct(xev: $xev) -> Result<Self, FlutterbugError> {
                Ok(Self {
                    kind: EventType::from_int(xev.type_)
                        .ok_or_else(|| FlutterbugError::InvalidEventType)?,
                    serial: xev.serial,
                    display: NonNull::new(xev.display)
                        .ok_or_else(|| FlutterbugError::DisplayFieldNull)?,
                    window: xev.$winname,
                    from_send_event: xev.send_event != 0,
                })
            }

            #[inline]
            fn inner(&self) -> Result<$xev, FlutterbugError> {
                Err(FlutterbugError::InnerAnyEventInaccessible)
            }
        }
    };
    ($xev:ty) => {
        anyev_impl! {$xev[window]}
    };
}

// manually implement the AnyEvent
impl DerivesEvent<xlib::XAnyEvent> for AnyEvent {
    #[inline]
    fn from_evstruct(xev: xlib::XAnyEvent) -> Result<Self, FlutterbugError> {
        Ok(Self {
            kind: EventType::from_int(xev.type_)
                .ok_or_else(|| FlutterbugError::InvalidEventType)?,
            serial: xev.serial,
            display: NonNull::new(xev.display).ok_or_else(|| FlutterbugError::DisplayFieldNull)?,
            window: xev.window,
            from_send_event: xev.send_event != 0,
        })
    }

    #[inline]
    fn inner(&self) -> Result<xlib::XAnyEvent, FlutterbugError> {
        Ok(xlib::XAnyEvent {
            type_: self.kind.to_int(),
            serial: self.serial,
            display: self.display.as_ptr(),
            window: self.window,
            send_event: if self.from_send_event { 1 } else { 0 },
        })
    }
}

anyev_impl! {xlib::XKeyEvent}
anyev_impl! {xlib::XButtonEvent}
anyev_impl! {xlib::XMotionEvent}
anyev_impl! {xlib::XCrossingEvent}
anyev_impl! {xlib::XFocusChangeEvent}
anyev_impl! {xlib::XExposeEvent}
anyev_impl! {xlib::XGraphicsExposeEvent[drawable]}
anyev_impl! {xlib::XNoExposeEvent[drawable]}
anyev_impl! {xlib::XVisibilityEvent}
anyev_impl! {xlib::XCreateWindowEvent[parent]}
anyev_impl! {xlib::XDestroyWindowEvent}
anyev_impl! {xlib::XUnmapEvent}
anyev_impl! {xlib::XMapEvent}
anyev_impl! {xlib::XMapRequestEvent}
anyev_impl! {xlib::XReparentEvent}
anyev_impl! {xlib::XConfigureEvent}
anyev_impl! {xlib::XGravityEvent}
anyev_impl! {xlib::XResizeRequestEvent}
anyev_impl! {xlib::XConfigureRequestEvent}
anyev_impl! {xlib::XCirculateEvent}
anyev_impl! {xlib::XCirculateRequestEvent}
anyev_impl! {xlib::XPropertyEvent}
anyev_impl! {xlib::XSelectionClearEvent}
anyev_impl! {xlib::XSelectionRequestEvent[owner]}
anyev_impl! {xlib::XSelectionEvent[requestor]}
anyev_impl! {xlib::XColormapEvent}
anyev_impl! {xlib::XClientMessageEvent}
anyev_impl! {xlib::XMappingEvent[event]}
anyev_impl! {xlib::XKeymapEvent}

// manually do one for XErrorEvent
/*impl DerivesEvent<xlib::XErrorEvent> for AnyEvent {
    #[inline]
    fn from_evstruct(xev: xlib::XErrorEvent) -> Result<Self, FlutterbugError> {
        Ok(Self {
            kind: EventType::from_int(xev.type_)
                .ok_or_else(|| FlutterbugError::InvalidEventType)?,
            window: xev.resourceid,
            from_send_event: false,
        })
    }

    #[inline]
    fn inner(&self) -> Result<xlib::XErrorEvent, FlutterbugError> {
        Err(FlutterbugError::StaticMsg(
            "Unable to access inner element of AnyEvent",
        ))
    }
}*/

// macro to create a new event type
macro_rules! event_type {
    ($(#[$attr: meta])* $vis: vis struct $sname: ident : $bname: ty [ $winname: ident ] {
        $($fvis: vis $fname: ident : $ftname: ty = $sfname: ident),*
        $(,)?
    }: $mname: ident) => {
        $vis mod $mname {
            #![allow(clippy::all)]
            #![allow(unused_imports)]

            use crate::{FlutterbugError, GenericDisplay, Window};
            use super::{DerivesAnEvent, DerivesEvent, EventType};
            use std::{convert::TryInto, os::raw::{c_char, c_uint, c_ulong, c_int}, ptr::NonNull};
            use x11::xlib;

            #[derive(Debug, Clone)]
            $(#[$attr])*
            pub struct $sname {
                kind: EventType,
                serial: c_ulong,
                display: NonNull<xlib::Display>,
                window: xlib::Window,
                from_send_event: bool,
                inner: $bname,
                $($fvis $fname: $ftname),*
            }

            impl DerivesAnEvent for $sname {
                #[inline]
                fn kind(&self) -> EventType { self.kind }
                #[inline]
                fn serial(&self) -> c_ulong { self.serial }
                #[inline]
                unsafe fn display(&self) -> NonNull<xlib::Display> { self.display }
                #[inline]
                fn window(&self) -> xlib::Window { self.window }
                #[inline]
                fn from_send_event(&self) -> bool { self.from_send_event }
            }

            impl DerivesEvent<$bname> for $sname {
                #[inline]
                fn from_evstruct(ev: $bname) -> Result<Self, FlutterbugError> {
                    Ok(Self {
                        kind: EventType::from_int(ev.type_).ok_or_else(|| FlutterbugError::InvalidEventType)?,
                        serial: ev.serial,
                        display: NonNull::new(ev.display).ok_or_else(|| FlutterbugError::DisplayFieldNull)?,
                        window: ev.$winname,
                        from_send_event: ev.send_event != 0,
                        inner: ev,
                        $($fname: ev.$sfname.try_into().unwrap_or_else(|_|
                            panic!("Tried to convert {} and failed", stringify!($fname))
                        )),*
                    })
                }

                #[inline]
                fn inner(&self) -> Result<$bname, FlutterbugError> { Ok(self.inner) }
            }

            type OurStruct = $bname;

            impl $sname {
               $(#[inline] pub fn $fname(&self) -> $ftname { self.$fname })*

                #[inline]
                pub fn new(
                    kind: EventType,
                    serial: c_ulong,
                    display: &dyn GenericDisplay,
                    sender_window: &Window,
                    fse: bool,
                    $($fname: $ftname),*
                ) -> Result<Self, FlutterbugError> {
                    let inner = OurStruct {
                        type_: kind.to_int(),
                        serial,
                        display: display.raw()?.as_ptr(),
                        $winname: sender_window.window(),
                        send_event: if fse { 1 } else { 0 },
                        $($sfname: $fname.try_into().unwrap_or_else(|_|
                            panic!("Tried to convert {} and failed", stringify!($fname))
                        )),*
                    };

                    Ok(Self {
                        kind, serial, display: display.raw()?, window: sender_window.window(),
                        from_send_event: fse,
                        inner,
                        $($fname),*
                    })
                }
            }
        }

        $vis use $mname::*;
    };
    ($(#[$attr: meta])* $vis: vis struct $sname: ident : $bname: ty {
        $($fvis: vis $fname: ident : $ftname: ty = $sfname: ident),*
        $(,)?
    }: $mname: ident) => {
        event_type! {
            $(#[$attr])*
            $vis struct $sname : $bname [ window ] {
                $($fvis $fname: $ftname = $sfname),*
            } : $mname
        }
    };
}

event_type! {
    pub struct KeyEvent : xlib::XKeyEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: i32 = x,
        y: i32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        pub state: c_uint = state,
        keycode: c_uint = keycode,
        same_screen: xlib::Bool = same_screen,
    }: key_event
}

bitflags::bitflags! {
    #[doc = "Represents function keys that can be depressed"]
    pub struct FunctionKeys : c_uint {
        const CONTROL = xlib::ControlMask;
        const ALT = 8;
        const SHIFT = xlib::ShiftMask;
        const CAPS_LOCK = 2;
    }
}

impl KeyEvent {
    /// Tell if a function is in the state.
    #[inline]
    pub fn has_function(&self, f: FunctionKeys) -> bool {
        (self.state & f.bits()) != 0
    }

    /// Add or remove a function key.
    #[inline]
    pub fn set_function(&mut self, f: FunctionKeys, is_in: bool) {
        if !is_in {
            self.state &= !f.bits();
        } else {
            self.state |= f.bits();
        }
    }

    /// Lookup the keysym and text that this symbol corresponds to.
    #[inline]
    pub fn lookup(&self) -> Result<(xlib::KeySym, String), FlutterbugError> {
        const BUFFER_SIZE: usize = 50;
        let mut inner = self.inner()?;
        let buffer = crate::cstring_buffer(BUFFER_SIZE);
        let buffer = buffer.into_raw();
        let mut ks = 0;

        let _bsize = unsafe {
            xlib::XLookupString(
                &mut inner,
                buffer,
                BUFFER_SIZE as c_int - 1,
                &mut ks,
                ptr::null_mut(),
            )
        };
        let res = unsafe { CString::from_raw(buffer) }.into_string()?;

        Ok((ks, res))
    }

    /// Lookup the keysym and text that this symbols corresponds to, with full UTF-8 support.
    #[inline]
    pub fn lookup_utf8(
        &self,
        ic: &dyn GenericInputContext,
    ) -> Result<(Option<xlib::KeySym>, Option<String>), FlutterbugError> {
        const BUFFER_SIZE: usize = 50;
        let mut inner = self.inner()?;
        let buffer = crate::cstring_buffer(BUFFER_SIZE);
        let buffer = buffer.into_raw();
        let mut status = 0;
        let mut ks = 0;

        let _bsize = unsafe {
            xlib::Xutf8LookupString(
                ic.raw()?.as_mut(),
                &mut inner,
                buffer,
                BUFFER_SIZE as c_int - 1,
                &mut ks,
                &mut status,
            )
        };

        let mut res_str = None;
        let mut res_ks = None;

        match status {
            xlib::XBufferOverflow => {
                return Err(FlutterbugError::StaticMsg("Did not allocate enough memory"));
            }
            xlib::XLookupBoth | xlib::XLookupChars => {
                res_str = Some(unsafe { CString::from_raw(buffer) }.into_string()?);
            }
            _ => { /* do nothing */ }
        }

        match status {
            xlib::XLookupBoth | xlib::XLookupKeySym => {
                res_ks = Some(ks);
            }
            _ => { /* do nothing */ }
        }

        Ok((res_ks, res_str))
    }
}

event_type! {
    pub struct ButtonEvent : xlib::XButtonEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: i32 = x,
        y: i32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        state: c_uint = state,
        button: c_uint = button,
        same_screen: xlib::Bool = same_screen,
    }: button_event
}

event_type! {
    pub struct MotionEvent : xlib::XMotionEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: i32 = x,
        y: i32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        state: c_uint = state,
        is_hint: c_char = is_hint,
        same_screen: xlib::Bool = same_screen,
    }: motion_event
}

event_type! {
    pub struct CrossingEvent : xlib::XCrossingEvent {
        root: xlib::Window = root,
        subwindow: xlib::Window = subwindow,
        time: xlib::Time = time,
        x: i32 = x,
        y: i32 = y,
        x_root: u32 = x_root,
        y_root: u32 = y_root,
        state: c_uint = state,
        mode: c_int = mode,
        detail: c_int = detail,
        focus: xlib::Bool = focus,
        same_screen: xlib::Bool = same_screen,
    }: crossing_event
}

event_type! {
    pub struct FocusChangeEvent : xlib::XFocusChangeEvent {
        mode: c_int = mode,
        detail: c_int = detail,
    }: focus_change_event
}

event_type! {
    pub struct ExposeEvent : xlib::XExposeEvent {
        x: i32 = x,
        y: i32 = y,
        width: u32 = width,
        height: u32 = height,
        count: i32 = count,
    }: expose_event
}

event_type! {
   pub struct NoExposeEvent : xlib::XNoExposeEvent[drawable] {
       major_code: c_int = major_code,
       minor_code: c_int = minor_code,
   }: no_expose_event
}

event_type! {
    pub struct GraphicsExposeEvent : xlib::XGraphicsExposeEvent[drawable] {
        x: i32 = x,
        y: i32 = y,
        width: u32 = width,
        height: u32 = height,
        count: i32 = count,
        major_code: c_int = major_code,
        minor_code: c_int = minor_code,
    }: graphics_expose_event
}

event_type! {
    pub struct ConfigureEvent : xlib::XConfigureEvent[event] {
        child: xlib::Window = window,
        x: i32 = x,
        y: i32 = y,
        width: u32 = width,
        height: u32 = height,
        border_width: u32 = border_width,
        above: xlib::Window = above,
        override_redirect: xlib::Bool = override_redirect,
    }: configure_event
}

event_type! {
    pub struct VisibilityEvent : xlib::XVisibilityEvent {
        state: c_int = state,
    }: visibility_event
}

event_type! {
    pub struct CreateWindowEvent : xlib::XCreateWindowEvent[parent] {
        child: xlib::Window = window,
        x: i32 = x,
        y: i32 = y,
        width: u32 = width,
        height: u32 = height,
        border_width: u32 = border_width,
        override_redirect: xlib::Bool = override_redirect,
    }: create_window_event
}

event_type! {
    pub struct DestroyWindowEvent : xlib::XDestroyWindowEvent[event] {
        child: xlib::Window = window,
    }: destroy_window_event
}

event_type! {
    pub struct UnmapEvent : xlib::XUnmapEvent[event] {
        child: xlib::Window = window,
        from_configure: xlib::Bool = from_configure,
    }: unmap_event
}

event_type! {
    pub struct MapEvent : xlib::XMapEvent[event] {
        child: xlib::Window = window,
        override_redirect: xlib::Bool = override_redirect,
    }: map_event
}

event_type! {
    pub struct MapRequestEvent : xlib::XMapRequestEvent[parent] {
        child: xlib::Window = window,
    }: map_request_event
}

event_type! {
    pub struct ReparentEvent : xlib::XReparentEvent[event] {
        child: xlib::Window = window,
        parent: xlib::Window = parent,
        x: i32 = x,
        y: i32 = y,
        override_redirect: xlib::Bool = override_redirect,
    }: reparent_event
}

event_type! {
    pub struct GravityEvent : xlib::XGravityEvent[event] {
        child: xlib::Window = window,
        x: i32 = x,
        y: i32 = y,
    }: gravity_event
}

event_type! {
    pub struct ResizeRequestEvent : xlib::XResizeRequestEvent {
        width: u32 = width,
        height: u32 = height,
    }: resize_request_event
}

event_type! {
    pub struct ConfigureRequestEvent : xlib::XConfigureRequestEvent {
        parent: xlib::Window = parent,
        x: i32 = x,
        y: i32 = y,
        width: u32 = width,
        height: u32 = height,
        border_width: u32 = border_width,
        above: xlib::Window = above,
        detail: c_int = detail,
        value_mask: c_uint = value_mask,
    }: configure_request_event
}

event_type! {
    pub struct CirculateEvent : xlib::XCirculateEvent {
        event: xlib::Window = event,
        place: c_int = place,
    }: circulate_event
}

event_type! {
    pub struct CirculateRequestEvent : xlib::XCirculateRequestEvent {
        parent: xlib::Window = parent,
        place: c_int = place,
    }: circulate_request_event
}

event_type! {
    pub struct PropertyEvent : xlib::XPropertyEvent {
        atom: xlib::Atom = atom,
        time: xlib::Time = time,
        state: c_int = state,
    }: property_event
}

event_type! {
    pub struct SelectionClearEvent : xlib::XSelectionClearEvent {
        selection: xlib::Atom = selection,
        time: xlib::Time = time,
    }: selection_clear_event
}

event_type! {
    pub struct SelectionRequestEvent : xlib::XSelectionRequestEvent[owner] {
        requestor: xlib::Window = requestor,
        selection: xlib::Atom = selection,
        target: xlib::Atom = target,
        property: xlib::Atom = property,
        time: xlib::Time = time,
    }: selection_request_event
}

event_type! {
    pub struct SelectionEvent : xlib::XSelectionEvent[requestor] {
        selection: xlib::Atom = selection,
        target: xlib::Atom = target,
        property: xlib::Atom = property,
        time: xlib::Time = time,
    }: selection_event
}

event_type! {
    pub struct ColormapEvent : xlib::XColormapEvent {
        colormap: xlib::Colormap = colormap,
        state: c_int = state,
        is_new_map: xlib::Bool = new,
    }: colormap_event
}

event_type! {
    pub struct ClientMessageEvent : xlib::XClientMessageEvent {
        message_type: xlib::Atom = message_type,
        format: c_int = format,
        data: xlib::ClientMessageData = data,
    }: client_message_event
}

event_type! {
    pub struct MappingEvent : xlib::XMappingEvent[event] {
        request: c_int = request,
        first_keycode: c_int = first_keycode,
        count: i32 = count,
    }: mapping_event
}

event_type! {
    pub struct KeymapEvent : xlib::XKeymapEvent {
        keys: [c_char; 32] = key_vector
    }: keymap_event
}

/// The event signifying an X11 error.
#[derive(Clone)]
pub struct ErrorEvent {
    kind: EventType,
    display: NonNull<xlib::Display>,
    resource_id: XID,
    serial: c_ulong,
    error_code: c_uchar,
    minor_code: c_uchar,
    request: c_uchar,
    err_text: String,
}

impl DerivesAnEvent for ErrorEvent {
    #[inline]
    fn kind(&self) -> EventType {
        self.kind
    }
    #[inline]
    fn window(&self) -> xlib::Window {
        self.resource_id
    }
    #[inline]
    fn from_send_event(&self) -> bool {
        false
    }
    #[inline]
    unsafe fn display(&self) -> NonNull<xlib::Display> {
        self.display
    }
    #[inline]
    fn serial(&self) -> c_ulong {
        self.serial
    }
}

/// An X11 event that can be received from the event loop.
#[derive(Debug, Clone)]
pub enum Event {
    Any(AnyEvent),
    Key(KeyEvent),
    Button(ButtonEvent),
    Motion(MotionEvent),
    Crossing(CrossingEvent),
    FocusChange(FocusChangeEvent),
    Expose(ExposeEvent),
    GraphicsExpose(GraphicsExposeEvent),
    NoExpose(NoExposeEvent),
    Visibility(VisibilityEvent),
    CreateWindow(CreateWindowEvent),
    DestroyWindow(DestroyWindowEvent),
    Unmap(UnmapEvent),
    Map(MapEvent),
    MapRequest(MapRequestEvent),
    Reparent(ReparentEvent),
    Configure(ConfigureEvent),
    Gravity(GravityEvent),
    ResizeRequest(ResizeRequestEvent),
    ConfigureRequest(ConfigureRequestEvent),
    Circulate(CirculateEvent),
    CirculateRequest(CirculateRequestEvent),
    Property(PropertyEvent),
    SelectionClear(SelectionClearEvent),
    SelectionRequest(SelectionRequestEvent),
    Selection(SelectionEvent),
    Colormap(ColormapEvent),
    ClientMessage(ClientMessageEvent),
    Mapping(MappingEvent),
    Keymap(KeymapEvent),
}

macro_rules! get_inner_property {
    ($s: ident, $prop: ident) => {
        match *$s {
            Event::Any(ref a) => a.$prop(),
            Event::Key(ref k) => k.$prop(),
            Event::Button(ref b) => b.$prop(),
            Event::Motion(ref m) => m.$prop(),
            Event::Crossing(ref c) => c.$prop(),
            Event::FocusChange(ref fc) => fc.$prop(),
            Event::Expose(ref e) => e.$prop(),
            Event::GraphicsExpose(ref ge) => ge.$prop(),
            Event::NoExpose(ref ne) => ne.$prop(),
            Event::Visibility(ref v) => v.$prop(),
            Event::CreateWindow(ref cw) => cw.$prop(),
            Event::DestroyWindow(ref dw) => dw.$prop(),
            Event::Unmap(ref u) => u.$prop(),
            Event::Map(ref m) => m.$prop(),
            Event::MapRequest(ref mr) => mr.$prop(),
            Event::Reparent(ref r) => r.$prop(),
            Event::Configure(ref c) => c.$prop(),
            Event::Gravity(ref g) => g.$prop(),
            Event::ResizeRequest(ref rr) => rr.$prop(),
            Event::ConfigureRequest(ref cr) => cr.$prop(),
            Event::Circulate(ref c) => c.$prop(),
            Event::CirculateRequest(ref cr) => cr.$prop(),
            Event::Property(ref p) => p.$prop(),
            Event::SelectionClear(ref sc) => sc.$prop(),
            Event::SelectionRequest(ref sr) => sr.$prop(),
            Event::Selection(ref s) => s.$prop(),
            Event::Colormap(ref cm) => cm.$prop(),
            Event::ClientMessage(ref cm) => cm.$prop(),
            Event::Mapping(ref m) => m.$prop(),
            Event::Keymap(ref k) => k.$prop(),
        }
    };
}

impl DerivesEvent<xlib::XEvent> for Event {
    fn from_evstruct(x: xlib::XEvent) -> Result<Self, FlutterbugError> {
        macro_rules! evt {
            ($bname: ident, $sname: ty, $evfield: ident) => {
                Ok(Event::$bname(<$sname>::from_evstruct(unsafe {
                    x.$evfield
                })?))
            };
        }

        let kind = unsafe { x.type_ };
        let kind = match EventType::from_int(kind) {
            Some(k) => k,
            None => return evt!(Any, AnyEvent, any),
        };

        match kind {
            EventType::KeyPress | EventType::KeyRelease => evt!(Key, KeyEvent, key),
            EventType::ButtonPress | EventType::ButtonRelease => evt!(Button, ButtonEvent, button),
            EventType::MotionNotify => evt!(Motion, MotionEvent, motion),
            EventType::FocusIn | EventType::FocusOut => {
                evt!(FocusChange, FocusChangeEvent, focus_change)
            }
            EventType::EnterNotify | EventType::LeaveNotify => {
                evt!(Crossing, CrossingEvent, crossing)
            }
            EventType::KeymapNotify => evt!(Keymap, KeymapEvent, keymap),
            EventType::Expose => evt!(Expose, ExposeEvent, expose),
            EventType::GraphicsExpose => evt!(GraphicsExpose, GraphicsExposeEvent, graphics_expose),
            EventType::NoExpose => evt!(NoExpose, NoExposeEvent, no_expose),
            EventType::CirculateRequest => {
                evt!(CirculateRequest, CirculateRequestEvent, circulate_request)
            }
            EventType::ConfigureRequest => {
                evt!(ConfigureRequest, ConfigureRequestEvent, configure_request)
            }
            EventType::MapRequest => evt!(MapRequest, MapRequestEvent, map_request),
            EventType::ResizeRequest => evt!(ResizeRequest, ResizeRequestEvent, resize_request),
            EventType::CirculateNotify => evt!(Circulate, CirculateEvent, circulate),
            EventType::ConfigureNotify => evt!(Configure, ConfigureEvent, configure),
            EventType::CreateNotify => evt!(CreateWindow, CreateWindowEvent, create_window),
            EventType::DestroyNotify => evt!(DestroyWindow, DestroyWindowEvent, destroy_window),
            EventType::GravityNotify => evt!(Gravity, GravityEvent, gravity),
            EventType::MapNotify => evt!(Map, MapEvent, map),
            EventType::MappingNotify => evt!(Mapping, MappingEvent, mapping),
            EventType::ReparentNotify => evt!(Reparent, ReparentEvent, reparent),
            EventType::UnmapNotify => evt!(Unmap, UnmapEvent, unmap),
            EventType::VisibilityNotify => evt!(Visibility, VisibilityEvent, visibility),
            EventType::ColormapEvent => evt!(Colormap, ColormapEvent, colormap),
            EventType::ClientMessage => evt!(ClientMessage, ClientMessageEvent, client_message),
            EventType::PropertyNotify => evt!(Property, PropertyEvent, property),
            EventType::SelectionClear => evt!(SelectionClear, SelectionClearEvent, selection_clear),
            EventType::SelectionRequest => {
                evt!(SelectionRequest, SelectionRequestEvent, selection_request)
            }
            EventType::SelectionNotify => evt!(Selection, SelectionEvent, selection),
        }
    }

    #[allow(unused_unsafe)]
    fn inner(&self) -> Result<xlib::XEvent, FlutterbugError> {
        let mut xev: xlib::XEvent = unsafe { mem::zeroed() };

        macro_rules! set_evt {
            ($item: ident, $field: ident) => {
                unsafe { xev.$field = $item.inner()? }
            };
        }

        match *self {
            Event::Any(ref a) => set_evt!(a, any),
            Event::Button(ref b) => set_evt!(b, button),
            Event::Key(ref k) => set_evt!(k, key),
            Event::Motion(ref m) => set_evt!(m, motion),
            Event::FocusChange(ref f) => set_evt!(f, focus_change),
            Event::Crossing(ref c) => set_evt!(c, crossing),
            Event::Keymap(ref km) => set_evt!(km, keymap),
            Event::Expose(ref e) => set_evt!(e, expose),
            Event::GraphicsExpose(ref ge) => set_evt!(ge, graphics_expose),
            Event::NoExpose(ref ne) => set_evt!(ne, no_expose),
            Event::CirculateRequest(ref ce) => set_evt!(ce, circulate_request),
            Event::ConfigureRequest(ref ce) => set_evt!(ce, configure_request),
            Event::MapRequest(ref me) => set_evt!(me, map_request),
            Event::ResizeRequest(ref rr) => set_evt!(rr, resize_request),
            Event::Circulate(ref cn) => set_evt!(cn, circulate),
            Event::Configure(ref cn) => set_evt!(cn, configure),
            Event::CreateWindow(ref cn) => set_evt!(cn, create_window),
            Event::DestroyWindow(ref dn) => set_evt!(dn, destroy_window),
            Event::Gravity(ref gn) => set_evt!(gn, gravity),
            Event::Map(ref m) => set_evt!(m, map),
            Event::Mapping(ref m) => set_evt!(m, mapping),
            Event::Reparent(ref r) => set_evt!(r, reparent),
            Event::Unmap(ref u) => set_evt!(u, unmap),
            Event::Visibility(ref v) => set_evt!(v, visibility),
            Event::Colormap(ref c) => set_evt!(c, colormap),
            Event::ClientMessage(ref cm) => set_evt!(cm, client_message),
            Event::Property(ref p) => set_evt!(p, property),
            Event::SelectionClear(ref sc) => set_evt!(sc, selection_clear),
            Event::SelectionRequest(ref sr) => set_evt!(sr, selection_request),
            Event::Selection(ref sn) => set_evt!(sn, selection),
        }

        Ok(xev)
    }
}

impl Event {
    /// Get the next event from the event loop.
    pub fn next(dpy: &dyn GenericDisplay) -> Result<Event, FlutterbugError> {
        let mut xev: xlib::XEvent = unsafe { mem::zeroed() };
        unsafe { xlib::XNextEvent(dpy.raw()?.as_mut(), &mut xev) };

        Self::from_evstruct(xev)
    }

    /// Send this event into an event loop.
    #[inline]
    pub fn send(
        self,
        dpy: &dyn GenericDisplay,
        target: &Window,
        propogate: bool,
        mask: EventMask,
    ) -> Result<(), FlutterbugError> {
        let mut ev = self.inner()?;
        unsafe {
            xlib::XSendEvent(
                dpy.raw()?.as_mut(),
                target.window(),
                if propogate { 1 } else { 0 },
                mask.bits(),
                &mut ev,
            )
        };
        Ok(())
    }

    /// Wait for a map event.
    #[inline]
    pub fn wait_for_map(display: &dyn GenericDisplay) -> Result<(), FlutterbugError> {
        'mapwait: loop {
            if let Self::Map(_m) = Self::next(display)? {
                break 'mapwait;
            }
        }

        Ok(())
    }

    /// Filter this event if it is a raw event.
    #[inline]
    pub fn filter(&self, window: Option<&Window>) -> Result<bool, FlutterbugError> {
        let mut inner = self.inner()?;
        Ok(unsafe {
            xlib::XFilterEvent(
                &mut inner,
                match window {
                    Some(w) => w.window(),
                    None => 0,
                },
            )
        } != 0)
    }
}

impl DerivesAnEvent for Event {
    fn kind(&self) -> EventType {
        get_inner_property!(self, kind)
    }

    fn window(&self) -> xlib::Window {
        get_inner_property!(self, window)
    }

    fn from_send_event(&self) -> bool {
        get_inner_property!(self, from_send_event)
    }

    fn serial(&self) -> c_ulong {
        get_inner_property!(self, serial)
    }
    unsafe fn display(&self) -> NonNull<xlib::Display> {
        get_inner_property!(self, display)
    }
}
