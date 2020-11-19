// MIT/Apache2 License

use breadx::{DisplayConnection, WindowClass};
use std::env;

fn main() {
    env::set_var("RUST_LOG", "breadx=debug");
    env_logger::init();

    let mut conn = DisplayConnection::create(None, None).unwrap();
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
            Default::default(),
        )
        .unwrap();
    window.map(&mut conn).unwrap();

    loop {
        let ev = conn.wait_for_event().unwrap();
    }
}
