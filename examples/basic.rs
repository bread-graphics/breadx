// MIT/Apache2 License

use breadx::{DisplayConnection, WindowClass, EventMask, CreateWindowParameters};
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
            100,
            100,
            0,
            cwp,
        )
        .unwrap();
    window.map(&mut conn).unwrap();

    loop {
        let ev = conn.wait_for_event().unwrap();
    }
}
