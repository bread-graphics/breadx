/* -----------------------------------------------------------------------------------
 * api/flutterbug/examples/font_text.rs - Draw text using a font and the font-kit
 *                                        crate.
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
    FunctionKeys, Pixmap, Window,
};
use font_kit::{
    canvas::{Canvas, Format, RasterizationOptions},
    family_name::FamilyName,
    font::Font,
    hinting::HintingOptions,
    properties::Properties,
    source::SystemSource,
};
use pathfinder_geometry::{
    transform2d::Transform2F,
    vector::{Vector2F, Vector2I},
};
use std::{collections::HashMap, convert::TryInto, env};

// load a char into the pixmap hashmap
fn load_char(
    display: &Display,
    window: &Window,
    font: &Font,
    pixmaps: &mut HashMap<char, Pixmap>,
    c: char,
) -> Result<(), anyhow::Error> {
    if let Some(_c) = pixmaps.get(&c) {
        return Ok(());
    }

    // rasterize the canvas
    let mut canvas = Canvas::new(Vector2I::splat(32), Format::A8);
    let glyph = font
        .glyph_for_char(c)
        .ok_or_else(|| FlutterbugError::StaticMsg("Key not found"))?;
    font.rasterize_glyph(
        &mut canvas,
        glyph,
        32.0,
        Transform2F::from_translation(Vector2F::new(0.0, 32.0)),
        HintingOptions::None,
        RasterizationOptions::GrayscaleAa,
    )?;

    let depth = display.default_depth()?.try_into().unwrap();

    // convert the canvas into a pixmap
    let pix = window.pixmap(Size2D::new(32, 32), depth)?;
    let img = pix.image(Point2D::zero(), Size2D::new(32, 32))?;

    for y in 0i32..32 {
        for x in 0i32..32 {
            let loc = (32 * y as usize) + x as usize;
            let bright = std::u8::MAX - canvas.pixels[loc];
            img.put_pixel(Point2D::new(x, y), bright, bright, bright)?;
        }
    }

    pix.put_image(&img, Point2D::zero(), Point2D::zero(), Size2D::new(32, 32))?;
    pixmaps.insert(c, pix);
    Ok(())
}

fn main() -> Result<(), anyhow::Error> {
    let display = Display::new()?;
    let win_size = Size2D::new(400, 200);
    let mut window = display.create_simple_window(
        None,
        Point2D::new(0, 0),
        win_size,
        1,
        display.default_black_pixel()?,
        display.default_white_pixel()?,
    )?;

    window.select_input(EventMask::EXPOSURE_MASK | EventMask::KEY_PRESS_MASK)?;
    window.map(true)?;
    window.set_standard_properties(Some("Test | Font"), None, None, false)?;

    let wdw = display.internal_atom("WM_DELETE_WINDOW", false)?;
    window.set_protocols(&mut [wdw])?;

    // needed for key inputs
    let im = display.input_method()?;
    let ic = window.input_context(&im)?;
    let mut key: Option<String> = None;

    // get the string to render from args
    let text = env::args()
        .nth(1)
        .or_else(|| Some(String::from("Hello world!")))
        .unwrap();

    let mut chars = HashMap::new();

    // load from sources
    let font = SystemSource::new()
        .select_best_match(
            &[FamilyName::Title(String::from("ProFont for Powerline"))],
            &Properties::new(),
        )?
        .load()?;

    // load all of the required chars
    text.chars()
        .map(|c| load_char(&display, &window, &font, &mut chars, c))
        .collect::<Result<_, _>>()?;

    'el: loop {
        let ev = Event::next(&display)?;

        match ev {
            Event::Expose(_e) => {
                window.clear_area(Point2D::zero(), Size2D::new(400, 200), false)?;

                // draw the text
                text.chars()
                    .enumerate()
                    .map(|(i, c)| {
                        let pix = chars.get(&c).unwrap();
                        window.copy_area(
                            pix,
                            Point2D::new(0, 0),
                            Point2D::new(20 + (i as i32 * 32), 20),
                            Size2D::new(32, 32),
                        )
                    })
                    .collect::<Result<_, _>>()?;

                // draw the currently pressed key
                if let Some(ref k) = &key {
                    let c = k.chars().nth(0).unwrap();

                    if let Err(_e) = load_char(&display, &window, &font, &mut chars, c) {
                        continue 'el;
                    }

                    let pix = chars.get(&c).unwrap();
                    window.copy_area(
                        pix,
                        Point2D::new(0, 0),
                        Point2D::new(20, 52),
                        Size2D::new(32, 32),
                    )?
                }
            }
            Event::Key(mut k) => {
                k.set_function(FunctionKeys::CONTROL, false);
                let (_, kstr) = k.lookup_utf8(&ic)?;
                key = kstr;

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
