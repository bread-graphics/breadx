// MIT/Apache2 License

use breadx::Result;

#[cfg(feature = "async-std-support")]
mod inner {
    use async_std::io::prelude::*;
    use breadx::{
        prelude::*,
        protocol::{xproto, Event},
        rt_support::async_std_support,
        Result,
    };

    pub async fn real_main() -> Result<()> {
        // almost identical to the tokio example, see that for comments
        let mut connection = async_std_support::connect(None).await?;

        // the events our windows receives.
        let events = xproto::EventMask::EXPOSURE | xproto::EventMask::BUTTON_PRESS;

        // the background color
        let background = connection.default_screen().white_pixel;

        // create the new window
        let parent = connection.default_screen().root;
        let wid = connection.generate_xid().await?;

        connection
            .create_window_checked(
                0,
                wid,
                parent,
                0,
                0,
                600,
                400,
                0,
                xproto::WindowClass::COPY_FROM_PARENT,
                0,
                xproto::CreateWindowAux::new()
                    .event_mask(events)
                    .background_pixel(background),
            )
            .await?;

        // map to screen and set title
        connection.map_window_checked(wid).await?;
        let title = "Hello from async-std!";
        connection
            .change_property_checked(
                xproto::PropMode::REPLACE,
                wid,
                xproto::AtomEnum::WM_NAME.into(),
                xproto::AtomEnum::STRING.into(),
                8,
                title.len() as u32,
                title,
            )
            .await?;

        // set up a GC for drawing
        let gc = connection.generate_xid().await?;
        connection
            .create_gc_checked(
                gc,
                wid,
                xproto::CreateGCAux::new()
                    .foreground(connection.default_screen().black_pixel)
                    .graphics_exposures(0)
                    .line_width(10),
            )
            .await?;

        // create some colors
        let cmap = connection.default_screen().default_colormap;
        let red_color = connection.alloc_color(cmap, 0xffff, 0, 0).await?;
        let green_color = connection.alloc_color(cmap, 0, 0xffff, 0).await?;
        let blue_color = connection.alloc_color(cmap, 0, 0, 0xffff).await?;

        connection.flush().await?;

        // resolve the colors
        let red_pixel = connection.wait_for_reply(red_color).await?.pixel;
        let green_pixel = connection.wait_for_reply(green_color).await?.pixel;
        let blue_pixel = connection.wait_for_reply(blue_color).await?.pixel;

        // setup exit strategy
        let wm_protocols = connection.intern_atom(false, "WM_PROTOCOLS").await?;
        let wm_delete_window = connection.intern_atom(false, "WM_DELETE_WINDOW").await?;
        connection.flush().await?;
        let wm_protocols = connection.wait_for_reply(wm_protocols).await?.atom;
        let wm_delete_window = connection.wait_for_reply(wm_delete_window).await?.atom;

        connection
            .change_property_checked(
                xproto::PropMode::REPLACE,
                wid,
                wm_protocols,
                xproto::AtomEnum::ATOM.into(),
                32,
                1,
                &wm_delete_window,
            )
            .await?;

        // primary event loop
        loop {
            let event = connection.wait_for_event().await?;

            match event {
                Event::Expose(_) => {
                    // begin the repaint

                    // draw a red "X"
                    connection
                        .change_gc(gc, xproto::ChangeGCAux::new().foreground(red_pixel))
                        .await?;
                    connection
                        .poly_segment(
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
                        )
                        .await?;

                    // draw a green circle
                    connection
                        .change_gc(gc, xproto::ChangeGCAux::new().foreground(green_pixel))
                        .await?;
                    connection
                        .poly_fill_arc(
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
                        )
                        .await?;

                    // draw a blue semicircle
                    connection
                        .change_gc(gc, xproto::ChangeGCAux::new().foreground(blue_pixel))
                        .await?;
                    connection
                        .poly_fill_arc(
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
                        )
                        .await?;

                    // draw the black outline of a circle
                    connection
                        .change_gc(
                            gc,
                            xproto::ChangeGCAux::new()
                                .foreground(connection.default_screen().black_pixel),
                        )
                        .await?;
                    connection
                        .poly_arc(
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
                        )
                        .await?;

                    // end the repaint
                    connection.flush().await?;
                }
                Event::ButtonPress(bp) => {
                    // indicate the button press
                    let mut stdout = async_std::io::stdout();
                    writeln!(stdout, "Detected click at ({}, {})", bp.event_x, bp.event_y)
                        .await
                        .unwrap();
                }
                Event::ClientMessage(cme) => {
                    // check if exit msg
                    if cme.data.as_data32()[0] == wm_delete_window {
                        break;
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }
}

#[async_std::main]
#[cfg(feature = "async-std-support")]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    inner::real_main().await
}

#[cfg(not(feature = "async-std-support"))]
fn main() {
    println!("This example requires the `async-std-support` feature to be enabled.");
}
