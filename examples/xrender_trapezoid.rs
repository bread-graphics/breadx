// MIT/Apache2 License

#[cfg(feature = "render")]
use breadx::{
    auto::{
        render::Repeat,
        xproto::{ExposeEvent, Rectangle},
    },
    keyboard::KeyboardState,
    prelude::*,
    render::{
        double_to_fixed, Color, Fixed, Linefix, PictOp, Picture, PictureParameters, Pointfix,
        RenderDisplay, Trapezoid,
    },
    DisplayConnection, Event, EventMask, Result,
};

#[cfg(feature = "render")]
use gluten_keyboard::Key;

#[cfg(feature = "render")]
fn main() -> Result {
    env_logger::init();

    let mut conn = DisplayConnection::create(None, None)?;
    let win = conn.create_simple_window(
        conn.default_root(),
        0,
        0,
        600,
        480,
        0,
        conn.default_black_pixel(),
        conn.default_white_pixel(),
    )?;
    win.set_title(&mut conn, "XRender Trapezoids")?;
    win.map(&mut conn)?;
    win.set_event_mask(&mut conn, EventMask::EXPOSURE | EventMask::KEY_PRESS)?;

    let wdw = conn.intern_atom_immediate("WM_DELETE_WINDOW", false)?;
    win.set_wm_protocols(&mut conn, &[wdw])?;

    let mut keys = KeyboardState::new(&mut conn)?;

    let mut conn = RenderDisplay::new(conn, 0, 10).map_err(|(_, e)| e)?;

    let attrs = win.window_attributes_immediate(&mut conn)?;
    let visual = attrs.visual;
    let visual = conn.visual_id_to_visual(visual).unwrap();
    let window_format = conn.find_visual_format(visual).unwrap();

    let pic = conn.create_picture(win, window_format, Default::default())?;
    let mut t = Trapezoid {
        top: 50 << 16,
        bottom: 150 << 16,
        left: Linefix {
            p1: Pointfix {
                x: 150 << 16,
                y: 50 << 16,
            },
            p2: Pointfix {
                x: 50 << 16,
                y: 150 << 16,
            },
        },
        right: Linefix {
            p1: Pointfix {
                x: 175 << 16,
                y: 50 << 16,
            },
            p2: Pointfix {
                x: 200 << 16,
                y: 150 << 16,
            },
        },
    };

    // create a brush
    let stops: &[Fixed] = &[
        double_to_fixed(0.0),
        double_to_fixed(0.5),
        double_to_fixed(1.0),
    ];
    let colors: &[Color] = &[
        Color {
            red: 0xFFFF,
            green: 0,
            blue: 0,
            alpha: 0xFFFF,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0xFFFF,
            alpha: 0xFFFF,
        },
        Color {
            red: 0,
            green: 0xFFFF,
            blue: 0,
            alpha: 0xFFFF,
        },
    ];
    let geom = win.geometry_immediate(&mut conn)?;
    let solid_p = conn.create_pixmap(win, 1, 1, geom.depth)?;
    let solid = conn.create_picture(
        solid_p,
        window_format,
        PictureParameters {
            repeat: Some(Repeat::Normal),
            ..Default::default()
        },
    )?;
    solid.fill_rectangles(
        &mut conn,
        PictOp::Over,
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0xFFFF,
        },
        [Rectangle {
            x: 0,
            y: 0,
            width: 1,
            height: 1,
        }]
        .as_ref(),
    )?;

    loop {
        match conn.wait_for_event()? {
            Event::ClientMessage(cme) => {
                if cme.data.longs()[0] == wdw.xid {
                    break;
                }
            }
            Event::Expose(_) => {
                pic.fill_rectangles(
                    &mut conn,
                    PictOp::Src,
                    Color {
                        red: 0xFFFF,
                        green: 0xFFFF,
                        blue: 0xFFFF,
                        alpha: 0xFFFF,
                    },
                    [Rectangle {
                        x: 0,
                        y: 0,
                        width: 600,
                        height: 480,
                    }]
                    .as_ref(),
                )?;
                pic.trapezoids(
                    &mut conn,
                    PictOp::Over,
                    solid,
                    window_format,
                    0,
                    0,
                    std::slice::from_ref(&t),
                )?;
            }
            Event::KeyPress(kp) => {
                let mut redraw = true;
                match keys.process_keycode(kp.detail, kp.state) {
                    Some(Key::T) => {
                        t.top += 0xFFFF;
                    }
                    Some(Key::Y) => {
                        t.top -= 0xFFFF;
                    }
                    Some(Key::B) => {
                        t.bottom += 0xFFFF;
                    }
                    Some(Key::N) => {
                        t.bottom -= 0xFFFF;
                    }
                    Some(Key::Q) => {
                        t.left.p1.x += 0xFFFF;
                    }
                    Some(Key::W) => {
                        t.left.p1.x -= 0xFFFF;
                    }
                    Some(Key::E) => {
                        t.left.p2.x += 0xFFFF;
                    }
                    Some(Key::R) => {
                        t.left.p2.x -= 0xFFFF;
                    }
                    Some(Key::U) => {
                        t.left.p1.y += 0xFFFF;
                    }
                    Some(Key::I) => {
                        t.left.p1.y -= 0xFFFF;
                    }
                    Some(Key::O) => {
                        t.left.p2.y += 0xFFFF;
                    }
                    Some(Key::P) => {
                        t.left.p2.y -= 0xFFFF;
                    }

                    Some(Key::Z) => {
                        t.right.p1.x += 0xFFFF;
                    }
                    Some(Key::X) => {
                        t.right.p1.x -= 0xFFFF;
                    }
                    Some(Key::C) => {
                        t.right.p2.x += 0xFFFF;
                    }
                    Some(Key::V) => {
                        t.right.p2.x -= 0xFFFF;
                    }
                    Some(Key::G) => {
                        t.right.p1.y += 0xFFFF;
                    }
                    Some(Key::H) => {
                        t.right.p1.y -= 0xFFFF;
                    }
                    Some(Key::M) => {
                        t.right.p2.y += 0xFFFF;
                    }
                    Some(Key::Comma) => {
                        t.right.p2.y -= 0xFFFF;
                    }
                    _ => {
                        redraw = false;
                    }
                }
                if redraw {
                    conn.send_event(
                        win,
                        EventMask::EXPOSURE,
                        Event::Expose(ExposeEvent {
                            window: win,
                            x: 0,
                            y: 0,
                            width: 600,
                            height: 480,
                            count: 0,
                            ..Default::default()
                        }),
                    )?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[cfg(not(feature = "render"))]
fn main() {
    println!("Cannot run xrender example unless xrender is enabled");
}
