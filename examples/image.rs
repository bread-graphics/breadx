// MIT/Apache2 License

// source of eisenhower image:
// https://commons.wikimedia.org/wiki/File:Dwight_D._Eisenhower,_official_photo_portrait,_May_29,_1959.jpg

use breadx::{rgb, BreadError, DisplayConnection, Event, EventMask, Image, ImageFormat};
use image::{io::Reader, GenericImageView};
use std::{io::Cursor, iter};

// Data of the image (.jpg)
const EISENHOWER: &[u8] = include_bytes!("../images/eisenhower.jpg");

// Helper function to create a chunk of zeroed heap memory for an image.
#[inline]
fn create_heap_memory(width: u32, height: u32) -> Box<[u8]> {
    iter::repeat(0)
        .take((width * height) as usize * 4)
        .collect()
}

fn main() -> Result<(), BreadError> {
    // load the image from the disc
    //    let reader = Reader::with_format(Cursor::new(EISENHOWER), Jpeg);
    let reader = Reader::new(Cursor::new(EISENHOWER))
        .with_guessed_format()
        .unwrap();
    let eisenhower = reader.decode().expect("Failed to decode image");

    // Width and height of image.
    let width = eisenhower.width();
    let height = eisenhower.height();

    // Open an X connection
    let mut conn = DisplayConnection::create(None, None)?;

    // Create a new window.
    let window = conn.create_simple_window(
        conn.default_root(),
        0,
        0,
        width as _,
        height as _,
        0,
        conn.default_black_pixel(),
        conn.default_white_pixel(),
    )?;
    window.set_title(&mut conn, "Eisenhower")?;
    window.map(&mut conn)?;
    window.set_event_mask(&mut conn, EventMask::EXPOSURE)?;

    // Set up a graphics context.
    let gc = conn.create_gc(window, Default::default())?;

    // Get the visual type and depth for the default screen.
    let depth = window.geometry_immediate(&mut conn)?.depth;
    let vis = conn.default_visual();

    // Create a new image.
    // TODO: eventually replace this with from_image() once I implement that
    let mut image = Image::new(
        &conn,
        Some(vis),
        depth,
        ImageFormat::ZPixmap,
        0,
        create_heap_memory(width, height),
        width as _,
        height as _,
        32,
        None,
    )
    .unwrap();

    // For each pixel in the image of Eisenhower, set the equivalent pixel in the X image
    eisenhower.pixels().for_each(|(x, y, pixel)| {
        let p = rgb((pixel.0)[0], (pixel.0)[1], (pixel.0)[2]);
        image.set_pixel(x as _, y as _, p);
    });

    // Create a pixmap based on the window.
    let pixmap = conn.create_pixmap(window, width as _, height as _, depth)?;
    conn.put_image(
        pixmap,
        gc,
        &image,
        0,
        0,
        0,
        0,
        width as _,
        height as _,
    )?;

    // Set up the exit atom.
    let wdw = conn.intern_atom_immediate("WM_DELETE_WINDOW".to_owned(), false)?;
    window.set_wm_protocols(&mut conn, &[wdw])?;

    // Event loop
    'el: loop {
        let event = conn.wait_for_event()?;

        match event {
            Event::ClientMessage(cme) => {
                if cme.data.longs()[0] == wdw.xid {
                    break 'el;
                }
            }
            Event::Expose(_) => {
                // Copy the Eisenhower image from the pixmap to this window.
                conn.copy_area(
                    pixmap,
                    window,
                    gc,
                    0,
                    0,
                    width as _,
                    height as _,
                    0,
                    0,
                )?;
            }
            _ => (),
        }
    }

    Ok(())
}
