// MIT/Apache2 License

#[cfg(feature = "render")]
use breadx::{
    auto::xproto::{ExposeEvent, Rectangle},
    keyboard::KeyboardState,
    prelude::*,
    render::{
        fixed_to_double, double_to_fixed, tesselate_shape, Color, Fixed, Linefix, PictOp, Picture, Pointfix,
        RenderDisplay, Trapezoid, Triangle, Pictformat,
    },
    DisplayConnection, Event, EventMask, Result,
};

#[cfg(feature = "render")]
use gluten_keyboard::Key;

#[cfg(feature = "render")]
use std::io::{prelude::*, self, stdout};

#[cfg(feature = "render")]
fn main() -> Result {
    env_logger::init();

    let width = 600i32;
    let height = 480i32;
    let mut conn = DisplayConnection::create(None, None)?;
    let win = conn.create_simple_window(
        conn.default_root(),
        0,
        0,
        width as u16,
        height as u16,
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
    let grad = conn.create_linear_gradient(
        Pointfix { x: 0, y: 0 },
        Pointfix {
            x: width << 16,
            y: height << 16,
        },
        stops,
        colors,
    )?;

    // create an arbitrary polygon and convert it to trapezoids
    let t: Vec<_> = tesselate_shape(vec![
        Pointfix {
            x: double_to_fixed(100.0),
            y: double_to_fixed(100.0),
        },
        Pointfix {
            x: double_to_fixed(200.0),
            y: double_to_fixed(150.0),
        },
        Pointfix {
            x: double_to_fixed(150.0),
            y: double_to_fixed(200.0),
        },
        Pointfix {
            x: double_to_fixed(50.0),
            y: double_to_fixed(200.0),
        },
        Pointfix {
            x: double_to_fixed(0.0),
            y: double_to_fixed(150.0),
        },
    ])
    .collect();

    dump_triangles(&t).unwrap();

    let mut mode = t.len();

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
                    PictOp::Over,
                    Color {
                        red: 0xFFFF,
                        green: 0xFFFF,
                        blue: 0xFFFF,
                        alpha: 0xFFFF,
                    },
                    [Rectangle {
                        x: 0,
                        y: 0,
                        width: width as _,
                        height: height as _,
                    }]
                    .as_ref(),
                )?;

                let t = if mode >= t.len() {
                    &*t
                } else {
                    std::slice::from_ref(&t[mode])
                }; 
                pic.triangles(&mut conn, PictOp::Over, grad, Pictformat::const_from_xid(0), 0, 0, t)?;
            }
            Event::KeyPress(kp) => {
                if let Some(Key::N) = keys.process_keycode(kp.detail, kp.state) {
                    mode += 1;
                    if mode > t.len() {
                        mode = 0;
                    }

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

#[cfg(feature = "render")]
#[inline]
fn dump_triangles(triangles: &[Triangle]) -> io::Result<()> {
    #[inline]
    fn dump_fixed(cout: &mut impl Write, f: Fixed) -> io::Result<()> {
        write!(cout, "{}", fixed_to_double(f))
    }

    let o = stdout();
    let mut cout = o.lock();    

    writeln!(cout, "[")?;

    for triangle in triangles {
        write!(cout, "  [(")?;
        dump_fixed(&mut cout, triangle.p1.x)?;
        write!(cout, ", ")?;
        dump_fixed(&mut cout, triangle.p1.y)?;
        write!(cout, "), (")?;
        dump_fixed(&mut cout, triangle.p2.x)?;
        write!(cout, ", ")?;
        dump_fixed(&mut cout, triangle.p2.y)?;
        write!(cout, "), (")?;
        dump_fixed(&mut cout, triangle.p3.x)?;
        write!(cout, ", ")?;
        dump_fixed(&mut cout, triangle.p3.y)?;
        writeln!(cout, ")],")?;
    }

    writeln!(cout, "]")
}

#[cfg(not(feature = "render"))]
fn main() {
    println!("Cannot run xrender example unless xrender is enabled");
}
