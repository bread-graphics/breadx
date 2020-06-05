/* -----------------------------------------------------------------------------------
 * api/flutterbug/examples/keytest.rs - Display which keys are being pressed.
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
    prelude::*, Atom, Display, Event, EventMask, EventType, ExposeEvent, FlutterbugError,
    FunctionKeys, KeySym,
};

fn main() -> Result<(), FlutterbugError> {
    let display = Display::new()?;
    let window = display.create_simple_window(
        None,
        Point2D::new(0, 0),
        Size2D::new(400, 200),
        1,
        display.default_black_pixel()?,
        display.default_white_pixel()?,
    )?;
    window.select_input(EventMask::EXPOSURE_MASK | EventMask::KEY_PRESS_MASK)?;
    window.map(true)?;
    window.store_name("Test | Key Pressing")?;

    let wdw = display.internal_atom("WM_DELETE_WINDOW", false)?;
    window.set_protocols(&mut [wdw])?;

    let im = display.input_method()?;
    let ic = window.input_context(&im)?;

    let mut keycode = 0;
    let mut state = 0;
    let mut key = String::new();
    let mut ks: Option<KeySym> = None;

    'el: loop {
        let ev = Event::next(&display)?;

        match ev.kind() {
            EventType::Expose => {
                window.clear_area(Point2D::new(0, 0), Size2D::new(400, 200), false)?;
                let xattrs = window.window_attributes()?;
                window.draw_string(
                    Point2D::new(10, 20),
                    &format!("Window size is {}x{}", xattrs.width, xattrs.height),
                )?;

                window.draw_string(Point2D::new(10, 40), &format!("Key Code is {}", keycode))?;
                window.draw_string(Point2D::new(10, 60), &format!("Key State is {}", state))?;
                window.draw_string(Point2D::new(10, 80), &format!("Key is {}", key))?;
                match ks {
                    Some(ks) => {
                        window.draw_string(
                            Point2D::new(10, 100),
                            &format!("Key symbol is {:X}", ks),
                        )?;
                        window.draw_string(
                            Point2D::new(10, 120),
                            &format!(
                                "Escape? {:?}",
                                ks as u32 == flutterbug::x11::keysym::XK_Escape
                            ),
                        )?;
                    }
                    None => {
                        window.draw_string(Point2D::new(10, 100), "No key symbol")?
                    }
                }
            }
            EventType::KeyPress => {
                if let Event::Key(mut k) = ev {
                    keycode = k.keycode();
                    state = k.state();
                    k.set_function(FunctionKeys::CONTROL, false);
                    let (ks1, key1) = k.lookup_utf8(&ic)?;
                    key = key1.unwrap_or_else(|| String::new());
                    ks = ks1;

                    //println!("{} ; {}", keycode, state);

                    // send expose event
                    let ev = ExposeEvent::new(
                        EventType::Expose,
                        0,
                        &display,
                        &window,
                        true,
                        0,
                        0,
                        400,
                        200,
                        1,
                    )?;
                    let ev = Event::Expose(ev);
                    ev.send(&display, &window, true, EventMask::EXPOSURE_MASK)?;
                }
            }
            EventType::ClientMessage => {
                if let Event::ClientMessage(c) = ev {
                    if AsRef::<[Atom]>::as_ref(&c.data())[0] == wdw {
                        break 'el;
                    }
                }
            }
            _ => { /* do nothing */ }
        }
    }

    Ok(())
}
