use breadx::{BreadError, DisplayConnection};

fn main() -> Result<(), BreadError> {
    let mut conn = DisplayConnection::create(None, None)?;
    let window = conn.create_simple_window(
        conn.default_root(),
        0,
        0,
        640,
        400,
        0,
        conn.default_black_pixel(),
        conn.default_white_pixel(),
    )?;
    window.set_title(&mut conn, "Hello world!")?;
    window.map(&mut conn)?;

    // begin the event loop
    loop {
        let _ev = conn.wait_for_event()?;
        // TODO: do things with the event
    }

    Ok(())
}
