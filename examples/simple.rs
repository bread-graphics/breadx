/* -----------------------------------------------------------------------------------
 * api/flutterbug/examples/simple.rs - Simple test for framework.
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

use euclid::default::{Point2D, Size2D};
use flutterbug::{
    prelude::*, x11::keysym, Atom, Display, Event, EventMask, EventType, FlutterbugError,
};

fn main() -> Result<(), FlutterbugError> {
    // create display and window
    let display = Display::new()?;
    let window = display.create_simple_window(
        None,
        Point2D::new(0, 0),
        Size2D::new(400, 400),
        1,
        display.default_black_pixel()?,
        display.default_white_pixel()?,
    )?;
    window.select_input(EventMask::EXPOSURE_MASK | EventMask::KEY_PRESS_MASK)?;
    window.map(true)?;
    window.store_name("Test | Simple")?;

    // ensure the delete window message goes through
    let wm_delete_window = display.internal_atom("WM_DELETE_WINDOW", false)?;
    window.set_protocols(&mut [wm_delete_window])?;

    // main event loop
    'el: loop {
        let ev = Event::next(&display)?;

        match ev.kind() {
            EventType::Expose => {
                window.draw_string(Point2D::new(10, 10), "Hello world!")?;

                let xattrs = window.window_attributes()?;
                let txt = format!("Window is {}x{}", xattrs.width, xattrs.height);
                window.draw_string(Point2D::new(10, 30), &txt)?;
            }
            EventType::KeyPress => {
                if let Event::Key(k) = ev {
                    let (ks, _) = k.lookup()?;
                    if ks == keysym::XK_Escape.into() {
                        break 'el;
                    }
                }
            }
            EventType::ClientMessage => {
                if let Event::ClientMessage(xclient) = ev {
                    if AsRef::<[Atom]>::as_ref(&xclient.data())[0] == wm_delete_window {
                        break 'el;
                    }
                }
            }
            _ => { /* do nothing */ }
        }
    }

    Ok(())
}
