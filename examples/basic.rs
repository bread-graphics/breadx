// MIT/Apache2 License

use breadx::{
    event::Event, Arc, BreadError, DisplayConnection, EventMask, GcParameters, Segment,
    WindowClass, WindowParameters, XidType,
};
use std::{env, process};

fn main() {
    env::set_var("RUST_LOG", "breadx=warn");
    env_logger::init();

    let mut conn = DisplayConnection::create(None, None).unwrap();

    // create the event mask
    let mut event_mask: EventMask = Default::default();
    event_mask.set_exposure(true);
    event_mask.set_button_press(true);
    event_mask.set_structure_notify(true);

    // window properties
    let mut cwp: WindowParameters = Default::default();
    cwp.event_mask = Some(event_mask);

    let root = conn.default_screen().root;
    let window = conn
        .create_window(
            root,
            WindowClass::CopyFromParent,
            None,
            None,
            0,
            0,
            600,
            400,
            0,
            cwp,
        )
        .unwrap();

    window.map(&mut conn).unwrap();
    window.set_title(&mut conn, "breadx Example").unwrap();

    // set up a graphics context for our window
    let mut gc_parameters: GcParameters = Default::default();
    gc_parameters.foreground = Some(conn.default_black_pixel());
    gc_parameters.graphics_exposures = Some(0);
    gc_parameters.line_width = Some(10);
    let gc = conn.create_gc(window, gc_parameters).unwrap();

    // allocate a red color
    let red_clr = conn
        .default_colormap()
        .alloc_color_immediate(&mut conn, u16::MAX, 0, 0)
        .unwrap()
        .pixel();
    let blue_clr = conn
        .default_colormap()
        .alloc_color_immediate(&mut conn, 0, 0, u16::MAX)
        .unwrap()
        .pixel();
    let green_clr = conn
        .default_colormap()
        .alloc_color_immediate(&mut conn, 0, u16::MAX, 0)
        .unwrap()
        .pixel();

    // set up an exit atom
    let wm_delete_window = conn
        .intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false)
        .unwrap();
    window
        .set_wm_protocols(&mut conn, &[wm_delete_window])
        .unwrap();

    loop {
        let ev = match conn.wait_for_event() {
            Ok(ev) => ev,
            Err(BreadError::ClosedConnection) => break,
            Err(e) => {
                eprintln!("Program closed with error: {:?}", e);
                process::exit(1);
            }
        };

        println!("{:?}", &ev);

        match ev {
            Event::ClientMessage(cme) => {
                if cme.data.longs()[0] == wm_delete_window.xid() {
                    process::exit(0);
                }
            }
            Event::Expose(_) => {
                let geometry = window.geometry_immediate(&mut conn).unwrap();
                println!("Window is [{} x {}]", geometry.width, geometry.height);

                let mut gc_params: GcParameters = Default::default();
                gc_params.foreground = Some(red_clr);
                gc.change(&mut conn, gc_params.clone()).unwrap();

                gc.draw_lines(
                    &mut conn,
                    window,
                    &[
                        Segment {
                            x1: 10,
                            y1: 10,
                            x2: 150,
                            y2: 150,
                        },
                        Segment {
                            x1: 150,
                            y1: 10,
                            x2: 10,
                            y2: 150,
                        },
                    ],
                )
                .unwrap();

                gc_params.foreground = Some(green_clr);
                gc.change(&mut conn, gc_params.clone()).unwrap();

                gc.fill_arc(
                    &mut conn,
                    window,
                    Arc {
                        x: 200,
                        y: 10,
                        width: 150,
                        height: 150,
                        angle1: 0,
                        angle2: 360 * 64,
                    },
                )
                .unwrap();

                gc_params.foreground = Some(blue_clr);
                gc.change(&mut conn, gc_params.clone()).unwrap();

                gc.fill_arc(
                    &mut conn,
                    window,
                    Arc {
                        x: 200,
                        y: 10,
                        width: 150,
                        height: 150,
                        angle1: 0,
                        angle2: 270 * 64,
                    },
                )
                .unwrap();

                gc_params.foreground = Some(conn.default_black_pixel());
                gc.change(&mut conn, gc_params.clone()).unwrap();

                gc.draw_arc(
                    &mut conn,
                    window,
                    Arc {
                        x: 200,
                        y: 10,
                        width: 150,
                        height: 150,
                        angle1: 0,
                        angle2: 360 * 64,
                    },
                )
                .unwrap();
            }
            _ => (),
        }
    }
}
