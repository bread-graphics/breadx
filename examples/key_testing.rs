// MIT/Apache2 License

use breadx::{DisplayConnection, Event, EventMask, KeyboardState};
use std::env;

fn main() -> breadx::Result {
    env::set_var("RUST_LOG", "breadx=info");
    env_logger::init();

    let mut conn = DisplayConnection::create(None, None)?;
    let window = conn.create_simple_window(
        conn.default_screen().root,
        0,
        0,
        640,
        400,
        0,
        conn.default_black_pixel(),
        conn.default_white_pixel(),
    )?;

    window.map(&mut conn)?;
    window.set_title(&mut conn, "Hello world!")?;
    window.set_event_mask(&mut conn, EventMask::KEY_PRESS | EventMask::KEY_RELEASE)?;

    let wm_delete_window = conn.intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false)?;
    window.set_wm_protocols(&mut conn, &[wm_delete_window])?;

    let mut keystate = KeyboardState::new(&mut conn)?;

    loop {
        let ev = conn.wait_for_event()?;
        match ev {
            Event::ClientMessage(cme) => {
                if cme.data.longs()[0] == wm_delete_window.xid {
                    return Ok(());
                }
            }
            Event::KeyPress(kp) => {
                println!("{:?}", keystate.process_keycode_press(kp.detail));
                //println!("{:?}", keystate.lookup_keysyms(kp.detail));
            }
            Event::KeyRelease(kr) => {
                keystate.process_keycode_release(kr.detail);
            }
            _ => (),
        }
    }
}
