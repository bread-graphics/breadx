// MIT/Apache2 License

use breadx::{CreateWindowParameters, DisplayConnection, EventMask, WindowClass, BreadError};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "breadx=debug");
    env_logger::init();

    let mut conn = DisplayConnection::create(None, None).unwrap();

    // create the event mask
    let mut event_mask: EventMask = Default::default();
    event_mask.set_exposure(true);

    // window properties
    let mut cwp: CreateWindowParameters = Default::default();
    cwp.event_mask = Some(event_mask);

    let window = conn
        .create_window(
            None,
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

    // set up an exit atom
    let wm_delete_window = conn.intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false).unwrap();

    loop {
        let ev = match conn.wait_for_event() {
            Ok(ev) => ev,
            Err(BreadError::ClosedConnection) => break,
            Err(e) => {
                eprintln!("Program closed with error: {:?}", e);
                break;
            }
        };
    }
}
