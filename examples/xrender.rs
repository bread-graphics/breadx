// MIT/Apache2 License

use breadx::{
    event::Event,
    render::{
        double_to_fixed, Color, Linefix, PictOp, Picture, Pointfix, RenderDisplay, StandardFormat,
        Trapezoid,
    },
    DisplayConnection, DisplayLike, EventMask, GcParameters, Rectangle, Result,
};
use std::{env, process};

fn main() -> Result {
    env::set_var("RUST_LOG", "breadx=warn");
    env_logger::init();

    let mut width = 500;
    let mut height = 500;

    let mut conn = DisplayConnection::create(None, None)?;
    let window = conn.create_simple_window(
        conn.default_root(),
        0,
        0,
        width,
        height,
        0,
        conn.default_black_pixel(),
        conn.default_white_pixel(),
    )?;
    window.set_title(&mut conn, "Render")?;
    window.map(&mut conn)?;
    window.set_event_mask(&mut conn, EventMask::EXPOSURE)?;
    let wdw = conn.intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false)?;
    window.set_wm_protocols(&mut conn, &[wdw])?;
    let attrs = window.window_attributes_immediate(conn.display_mut())?;

    // create a pixmap to use as a mask
    let pixmap = conn.create_pixmap(window, width, height, 8)?;

    let mut conn = RenderDisplay::new(conn, 0, 10).map_err(|(_, e)| e)?;

    // get the format for the window
    let visual = attrs.visual;
    let visual = conn.display().visual_id_to_visual(visual).unwrap();
    let window_format = conn.find_visual_format(visual).unwrap();

    // create a picture on top of the window
    let pic = conn.create_picture(window, window_format, Default::default())?;
    let mask = conn.create_picture(window, window_format, Default::default())?;
    mask.fill_rectangles(
        conn.display_mut(),
        PictOp::Src,
        Color {
            red: 0,
            green: 0,
            blue: 0,
            alpha: 0xFFFF,
        },
        &[Rectangle {
            x: 0,
            y: 0,
            width,
            height,
        }],
    )?;

    // create a linear gradient
    let stops = vec![
        double_to_fixed(0.0),
        double_to_fixed(1.0 / 3.0),
        double_to_fixed(2.0 / 3.0),
        double_to_fixed(1.0),
    ];
    let colors = vec![
        Color {
            red: 0xFFFF,
            green: 0,
            blue: 0,
            alpha: 0xFFFF,
        },
        Color {
            red: 0,
            green: 0xFFFF,
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
            red: 0xFFFF,
            green: 0,
            blue: 0,
            alpha: 0xFFFF,
        },
    ];

    let center = Pointfix {
        x: (width as i32) << 15,
        y: (height as i32) << 15,
    };

    let linear_gradient = conn.create_linear_gradient(
        Pointfix { x: 0, y: 2000 },
        Pointfix {
            x: (width as i32) << 16,
            y: ((height as i32) << 16) + 2000,
        },
        &stops,
        &colors,
    )?;
    let radial_gradient = conn.create_radial_gradient(
        center.clone(), 
        center.clone(),
        0,
        200 << 16,
        &stops,
        &colors,
    )?;
    let conical_gradient = conn.create_conical_gradient(center, 0, &stops, &colors)?;

    loop {
        let event = conn.display_mut().wait_for_event()?;

        match event {
            Event::ClientMessage(cme) => {
                if cme.data.longs()[0] == wdw.xid {
                    break;
                }
            }
            Event::Expose(e) => {
                width = e.width;
                height = e.height;

                // composite the gradient onto this window
                radial_gradient.composite(
                    conn.display_mut(),
                    PictOp::Src,
                    Picture::const_from_xid(0),
                    pic,
                    0,
                    0,
                    0,
                    0,
                    0,
                    0,
                    width,
                    height,
                )?;
            }
            _ => (),
        }
    }

    linear_gradient.free(conn.display_mut())?;
    radial_gradient.free(conn.display_mut())?;
    conical_gradient.free(conn.display_mut())?;

    Ok(())
}
