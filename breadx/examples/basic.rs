// MIT/Apache2 License

//! Demonstration of the basic capabilities of `breadx`.

use breadx::{
    display::DisplayConnection,
    prelude::*,
    protocol::{xproto, Event},
};

fn main() -> breadx::Result<()> {
    tracing_subscriber::fmt::init();

    // Create a new display connection.
    let mut connection = DisplayConnection::connect(None)?;

    // Events that our window receives.
    let events = xproto::EventMask::EXPOSURE | xproto::EventMask::BUTTON_PRESS;

    // Background color.
    let background = connection.default_screen().white_pixel;

    // Create a new window.
    let parent = connection.default_screen().root;
    let wid = connection.generate_xid()?;

    connection.create_window(
        0, // 0 indicates inherit from parent
        wid,
        parent,
        0,   // x
        0,   // y
        600, // width
        400, // height
        0,   // border width
        xproto::WindowClass::COPY_FROM_PARENT,
        0, // borrow from parent
        xproto::CreateWindowAux::new()
            .event_mask(events)
            .background_pixel(background),
    )?;

    // Map it to the screen and set the title.
    connection.map_window_checked(wid)?;
    let title = "Hello, world!";
    connection.change_property_checked(
        xproto::PropMode::REPLACE,
        wid,
        xproto::AtomEnum::WM_NAME.into(),
        xproto::AtomEnum::STRING.into(),
        8,
        title.len() as u32,
        title,
    )?;

    // set up a GC for our window
    let gc = connection.generate_xid()?;
    connection.create_gc_checked(
        gc,
        wid,
        xproto::CreateGCAux::new()
            .foreground(connection.default_screen().black_pixel)
            .graphics_exposures(0)
            .line_width(10),
    )?;

    // create some colors
    let cmap = connection.default_screen().default_colormap;
    let red_color = connection.alloc_color(cmap, u16::MAX, 0, 0)?;
    let green_color = connection.alloc_color(cmap, 0, u16::MAX, 0)?;
    let blue_color = connection.alloc_color(cmap, 0, 0, u16::MAX)?;

    connection.flush()?;

    // resolve the colors
    let red_pixel = connection.wait_for_reply(red_color)?.pixel;
    let green_pixel = connection.wait_for_reply(green_color)?.pixel;
    let blue_pixel = connection.wait_for_reply(blue_color)?.pixel;

    // set up an exit strategy
    let wm_protocols = connection.intern_atom(false, "WM_PROTOCOLS")?;
    let wm_delete_window = connection.intern_atom(false, "WM_DELETE_WINDOW")?;
    connection.flush()?;
    let wm_protocols = connection.wait_for_reply(wm_protocols)?.atom;
    let wm_delete_window = connection.wait_for_reply(wm_delete_window)?.atom;

    connection.change_property_checked(
        xproto::PropMode::REPLACE,
        wid,
        wm_protocols,
        xproto::AtomEnum::ATOM.into(),
        32,
        1,
        &wm_delete_window,
    )?;

    // primary event loop
    loop {
        let event = connection.wait_for_event()?;

        match event {
            Event::Expose(_) => {
                // we need to repaint
                // note that we've stopped using _checked() requests
                // here for the sake of speed

                // draw two red lines for an "x"
                connection.change_gc(gc, xproto::ChangeGCAux::new().foreground(red_pixel))?;
                connection.poly_segment(
                    wid,
                    gc,
                    &[
                        xproto::Segment {
                            x1: 10,
                            y1: 10,
                            x2: 150,
                            y2: 150,
                        },
                        xproto::Segment {
                            x1: 150,
                            y1: 10,
                            x2: 10,
                            y2: 150,
                        },
                    ],
                )?;

                // draw a green circle
                // note that angles are in units of 16ths of a degree
                connection.change_gc(gc, xproto::ChangeGCAux::new().foreground(green_pixel))?;
                connection.poly_fill_arc(
                    wid,
                    gc,
                    &[xproto::Arc {
                        x: 200,
                        y: 10,
                        width: 150,
                        height: 150,
                        angle1: 0,
                        angle2: 360 * 64,
                    }],
                )?;

                // draw a blue semicircle
                connection.change_gc(gc, xproto::ChangeGCAux::new().foreground(blue_pixel))?;
                connection.poly_fill_arc(
                    wid,
                    gc,
                    &[xproto::Arc {
                        x: 200,
                        y: 10,
                        width: 150,
                        height: 150,
                        angle1: 0,
                        angle2: 270 * 64,
                    }],
                )?;

                // draw the black outline of a circle
                connection.change_gc(
                    gc,
                    xproto::ChangeGCAux::new().foreground(connection.default_screen().black_pixel),
                )?;
                connection.poly_arc(
                    wid,
                    gc,
                    &[xproto::Arc {
                        x: 200,
                        y: 10,
                        width: 150,
                        height: 150,
                        angle1: 0,
                        angle2: 360 * 64,
                    }],
                )?;

                connection.flush()?;
            }
            Event::ButtonPress(bp) => {
                // indicate that we have been clicked
                println!("Detected click at ({}, {})", bp.event_x, bp.event_y);
            }
            Event::ClientMessage(cme) => {
                // check if it's telling us to exit
                if cme.data.as_data32()[0] == wm_delete_window {
                    break;
                }
            }
            _ => {}
        }
    }

    Ok(())
}
