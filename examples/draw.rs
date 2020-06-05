/* -----------------------------------------------------------------------------------
 * api/flutterbug/examples/draw.rs - Test of drawing capabilities.
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
    prelude::*, x11::keysym, Atom, Color, Display, Event, EventMask, EventType, ExposeEvent,
    FlutterbugError,
};

fn main() -> Result<(), FlutterbugError> {
    let display = Display::new()?;
    let mut window = display.create_simple_window(
        None,
        Point2D::new(0, 0),
        Size2D::new(200, 200),
        1,
        display.default_black_pixel()?,
        display.default_white_pixel()?,
    )?;
    window.select_input(EventMask::EXPOSURE_MASK | EventMask::KEY_PRESS_MASK)?;
    window.map(true)?;
    window.set_standard_properties(
        Some("Test | Draw"),
        Some("Draw Test"),
        None,
        false,
    )?;

    let wdw = display.internal_atom("WM_DELETE_WINDOW", false)?;
    window.set_protocols(&mut [wdw])?;

    let colormap = display.default_colormap()?;
    let colors = [
        colormap.color(Color::from_rgb(255, 0, 0))?,
        colormap.color(Color::from_rgb(0, 255, 0))?,
        colormap.color(Color::from_rgb(0, 0, 255))?,
    ];
    let mut curr_index = 0;

    window.set_line_width(6)?;

    'el: loop {
        let ev = Event::next(&display)?;
        let ty = ev.kind();

        match ev {
            Event::Expose(_e) => {
                window.clear_area(Point2D::new(0, 0), Size2D::new(200, 200), false)?;

                // draw a circle with a diameter of 150 pixels
                const DIAMETER: i32 = 150;
                const CENTER_X: i32 = 100;
                const CENTER_Y: i32 = 100;
                let origin = Point2D::new(CENTER_X - (DIAMETER / 2), CENTER_Y - (DIAMETER / 2));
                let size = Size2D::new(DIAMETER as u32, DIAMETER as u32);
                let angles = (0, 360 * 64);

                window.set_foreground(display.default_black_pixel()?)?;
                window.draw_arc(origin, size, angles)?;
                window.set_foreground(colors[curr_index])?;
                window.fill_arc(origin, size, angles)?;
            }
            Event::Key(ke) => {
                if ty != EventType::KeyPress {
                    continue 'el;
                }

                let (ks, _) = ke.lookup()?;

                if ks == keysym::XK_R.into() || ks == keysym::XK_r.into() {
                    curr_index = 0;
                } else if ks == keysym::XK_G.into() || ks == keysym::XK_g.into() {
                    curr_index = 1;
                } else if ks == keysym::XK_B.into() || ks == keysym::XK_b.into() {
                    curr_index = 2;
                } else {
                    continue 'el;
                }

                // send expose event
                let ev = ExposeEvent::new(
                    EventType::Expose,
                    0,
                    &display,
                    &window,
                    true,
                    0,
                    0,
                    200,
                    200,
                    1,
                )?;
                let ev = Event::Expose(ev);
                ev.send(&display, &window, true, EventMask::EXPOSURE_MASK)?;
            }
            Event::ClientMessage(cm) => {
                if AsRef::<[Atom]>::as_ref(&cm.data())[0] == wdw {
                    break 'el;
                }
            }
            _ => { /* do nothing */ }
        }
    }

    Ok(())
}
