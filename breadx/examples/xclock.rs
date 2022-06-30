// MIT/Apache2 Licens

//! This is a `breadx` port of the `xclock` utility.
//!
//! `tokio` is used as a backing runtime to coordinate everything.
//! X11 events are sent into a channel along with events from a timer,
//! where they are processed in a single event loop. The `chrono` crate
//! is used to keep an accurate time, and the `euclid` crate is used
//! for geometry. The XRender extension is used for drawing.
//!
//! For the time being, the `tokio` portion of this program restricts it
//! to being `Unix`-exclusive.

#![allow(clippy::too_many_arguments)]

#[cfg(all(feature = "tokio-support", feature = "render", unix))]
#[path = "util/cancel.rs"]
mod cancel;

#[cfg(all(feature = "tokio-support", feature = "render", unix))]
use breadx::Result;

#[cfg(all(feature = "tokio-support", feature = "render", unix))]
mod inner {
    use breadx::{
        prelude::*,
        protocol::{render, xproto, Event},
        rt_support::tokio_support,
        Result,
    };
    use chrono::{DateTime, Local, SubsecRound, TimeZone, Timelike};
    use euclid::{Angle, Vector2D};
    use tokio::{
        sync::mpsc,
        time::{interval_at, Duration, Instant},
    };

    /// Units of floating-point pixels.
    struct FloatPixel;
    type FlVector = Vector2D<f32, FloatPixel>;

    /// The position of clock hands.
    ///
    /// Values are from 0 to 2*pi, to indicate the rotation of the
    /// hand in radians.
    #[derive(Copy, Clone)]
    struct ClockHands {
        second_hand: Angle<f32>,
        minute_hand: Angle<f32>,
        hour_hand: Angle<f32>,
    }

    /// Either the X11 event or the clock hand request.
    enum Action {
        Event(Event),
        NewHands(ClockHands),
    }

    impl<Tz: TimeZone> From<DateTime<Tz>> for ClockHands {
        fn from(dt: DateTime<Tz>) -> Self {
            let hour = dt.hour() % 12;
            let minute = dt.minute();
            let second = dt.second();

            // calculate where it should go on the circle
            macro_rules! posn {
                ($val: expr, $base: expr) => {{
                    (Angle::two_pi() * ($val as f32 / $base)) - Angle::frac_pi_2()
                }};
            }

            ClockHands {
                second_hand: posn!(second, 60.0),
                minute_hand: posn!(minute, 60.0),
                hour_hand: posn!(hour, 12.0),
            }
        }
    }

    impl ClockHands {
        fn current() -> Self {
            Self::from(Local::now())
        }
    }

    // we do this on a single thread for demonstration, although it would
    // probably go faster if we used multi-threading
    #[tokio::main(flavor = "current_thread")]
    pub async fn real_main() -> Result<()> {
        super::cancel::spawn_kill_thread();

        // spawn two tasks: one for X11, one for the timer
        // keep them connected using an MPSC channel
        let (send, recv) = mpsc::channel(1);
        let x11_task = tokio::spawn(x11_gui_task(recv));
        let timer_task = tokio::spawn(timekeeper_task(send));

        // select between the two tasks to wait for the x11 one to end
        tokio::select! {
            res = x11_task => { res.unwrap()?; },
            _ = timer_task => {},
        }

        Ok(())
    }

    /// Task for running the X11 window.
    ///
    /// The channel provided is used to signal a change in time.
    async fn x11_gui_task(mut changes: mpsc::Receiver<ClockHands>) -> Result<()> {
        const INITIAL_WIDTH: u16 = 350;
        const INITIAL_HEIGHT: u16 = 350;

        // open the x11 connection
        let mut dpy = tokio_support::connect(None).await?;
        let mut width = INITIAL_WIDTH;
        let mut height = INITIAL_HEIGHT;

        // set up the basic window, and pull down pictformats while we're at it
        let wm_protocol = dpy.intern_atom(false, "WM_PROTOCOLS").await?;
        let wm_delete_window = dpy.intern_atom(false, "WM_DELETE_WINDOW").await?;
        let formats = dpy.render_query_pict_formats().await?;
        let wm_protocol = dpy.wait_for_reply(wm_protocol).await?.atom;
        let wm_delete_window = dpy.wait_for_reply(wm_delete_window).await?.atom;
        let formats = dpy.wait_for_reply(formats).await?;

        let (window, _, pict) = create_window(
            &mut dpy,
            INITIAL_WIDTH,
            INITIAL_HEIGHT,
            "breadxclock",
            wm_protocol,
            wm_delete_window,
            &formats,
        )
        .await?;

        super::cancel::spawn_close_thread(window);

        // create solid pictures we can use as fills
        let gray = render::Color {
            red: 0x9999,
            green: 0x0,
            blue: 0x0,
            alpha: 0xFFFF,
        };
        let cyan = render::Color {
            red: 0x34AF,
            green: 0x0568,
            blue: 0xFFFF,
            alpha: 0x9999,
        };
        let white = render::Color {
            red: 0xFFFF,
            green: 0xFFFF,
            blue: 0xFFFF,
            alpha: 0xFFFF,
        };

        let gray_picture = dpy.generate_xid().await?;
        let cyan_picture = dpy.generate_xid().await?;
        let white_picture = dpy.generate_xid().await?;
        dpy.render_create_solid_fill(gray_picture, gray).await?;
        dpy.render_create_solid_fill(cyan_picture, cyan).await?;
        dpy.render_create_solid_fill(white_picture, white).await?;

        // create the mask
        let mut draw_mask = create_window_mask(
            &mut dpy,
            window,
            width,
            height,
            &formats,
            render::Color {
                red: 0,
                green: 0,
                blue: 0,
                alpha: 0,
            },
        )
        .await?;

        // the current clock hands to draw
        let mut clock_hands = ClockHands::current();

        // begin the event loop
        loop {
            // select on either the incoming event or the channel that
            // the timekeeper task operates on
            let event = tokio::select! {
                event = dpy.wait_for_event() => {
                    Action::Event(event?)
                },
                hands = changes.recv() => {
                    match hands {
                        Some(hands) => Action::NewHands(hands),
                        _ => continue,
                    }
                }
            };

            let mut needs_redraw = false;

            match event {
                Action::Event(Event::ClientMessage(cme)) => {
                    if cme.data.as_data32()[0] == wm_delete_window {
                        break;
                    }
                }
                Action::Event(Event::Expose(_)) => {
                    // redraw the clock
                    needs_redraw = true;
                }
                Action::Event(Event::ConfigureNotify(cne)) => {
                    if cne.window != window {
                        continue;
                    }

                    let new_width = cne.width;
                    let new_height = cne.height;

                    if new_width != width || new_height != height {
                        width = new_width;
                        height = new_height;

                        // regenerate the mask
                        dpy.render_free_picture(draw_mask).await?;
                        draw_mask = create_window_mask(
                            &mut dpy,
                            window,
                            width,
                            height,
                            &formats,
                            render::Color {
                                red: 0,
                                green: 0,
                                blue: 0,
                                alpha: 0,
                            },
                        )
                        .await?;
                    }
                }
                Action::NewHands(hands) => {
                    // set the hands, and then redraw the clock
                    clock_hands = hands;
                    needs_redraw = true;
                }
                _ => {}
            }

            if needs_redraw {
                // redraw the clock
                redraw_clock(
                    &mut dpy,
                    width,
                    height,
                    clock_hands,
                    pict,
                    draw_mask,
                    gray_picture,
                    cyan_picture,
                    white_picture,
                )
                .await?;
            }
        }

        Ok(())
    }

    /// Create the window used for display.
    ///
    /// Returned values are the window ID, the
    /// pictformat ID, and an ID for the window's picture.
    async fn create_window(
        dpy: &mut impl AsyncDisplay,
        width: u16,
        height: u16,
        title: &str,
        wm_protocols: xproto::Atom,
        wm_delete_window: xproto::Atom,
        formats: &render::QueryPictFormatsReply,
    ) -> Result<(xproto::Window, u32, render::Picture)> {
        let events = xproto::EventMask::EXPOSURE | xproto::EventMask::STRUCTURE_NOTIFY;
        let background = dpy.default_screen().white_pixel;
        let parent = dpy.default_screen().root;

        // create the window proper
        let wid = dpy.generate_xid().await?;
        dpy.create_window(
            0,
            wid,
            parent,
            0,
            0,
            width,
            height,
            0,
            xproto::WindowClass::COPY_FROM_PARENT,
            0,
            xproto::CreateWindowAux::new()
                .event_mask(events)
                .background_pixel(background),
        )
        .await?;

        // map to the string and set the title
        dpy.map_window(wid).await?;
        dpy.change_property(
            xproto::PropMode::REPLACE,
            wid,
            xproto::AtomEnum::WM_NAME.into(),
            xproto::AtomEnum::STRING.into(),
            8,
            title.len() as u32,
            title,
        )
        .await?;

        // setup exit strategy
        dpy.change_property(
            xproto::PropMode::REPLACE,
            wid,
            wm_protocols,
            xproto::AtomEnum::ATOM.into(),
            32,
            1,
            &wm_delete_window,
        )
        .await?;

        // get the picture format for this window
        let pictformat = pictformat_for_window(dpy, wid, formats).await?;

        // create a new picture for this window
        let picture = dpy.generate_xid().await?;
        dpy.render_create_picture(
            picture,
            wid,
            pictformat,
            render::CreatePictureAux::new().graphicsexposure(1),
        )
        .await?;

        Ok((wid, pictformat, picture))
    }

    /// Create a new picture to be used as a mask.
    async fn create_window_mask(
        dpy: &mut impl AsyncDisplay,
        base: xproto::Window,
        width: u16,
        height: u16,
        formats: &render::QueryPictFormatsReply,
        color: render::Color,
    ) -> Result<render::Picture> {
        // create a new pixmap with the given dimensions
        let pixmap = dpy.generate_xid().await?;
        dpy.create_pixmap(8, pixmap, base, width, height).await?;

        // determine the best format for an 8-bit alpha
        let a8format = formats
            .formats
            .iter()
            .find(|format| {
                format.type_ == render::PictType::DIRECT
                    && format.depth == 8
                    && format.direct.red_mask == 0
                    && format.direct.green_mask == 0
                    && format.direct.blue_mask == 0
                    && format.direct.alpha_mask == 0xFF
            })
            .ok_or_else(|| breadx::Error::make_msg("unable to find A8 format"))?;

        // create a new picture based on the pixmap
        let picture = dpy.generate_xid().await?;
        dpy.render_create_picture(
            picture,
            pixmap,
            a8format.id,
            render::CreatePictureAux::new(),
        )
        .await?;

        // fill w/ color
        dpy.render_fill_rectangles(
            render::PictOp::SRC,
            picture,
            color,
            &[xproto::Rectangle {
                x: 0,
                y: 0,
                width,
                height,
            }],
        )
        .await?;

        // free the pixmap, we don't need it anymore
        dpy.free_pixmap(pixmap).await?;

        Ok(picture)
    }

    /// Get the pictformat ID for a window.
    async fn pictformat_for_window(
        dpy: &mut impl AsyncDisplay,
        window: xproto::Window,
        formats: &render::QueryPictFormatsReply,
    ) -> Result<u32> {
        // load the window's visual
        let visual = dpy.get_window_attributes_immediate(window).await?.visual;

        // find the entry in "formats" that matches the visual
        formats
            .screens
            .iter()
            .flat_map(|format| &format.depths)
            .flat_map(|depth| &depth.visuals)
            .find(|v| v.visual == visual)
            .map(|v| v.format)
            .ok_or_else(|| breadx::Error::make_msg("format not found for window"))
    }

    /// Draw the clock on the window.
    async fn redraw_clock(
        dpy: &mut impl AsyncDisplay,
        width: u16,
        height: u16,
        hands: ClockHands,
        dest: render::Picture,
        mask: render::Picture,
        grey: render::Picture,
        cyan: render::Picture,
        white: render::Picture,
    ) -> Result<()> {
        // create a compilation of trapezoids representing the
        // notches
        let (w, h) = (width as f32, height as f32);
        let notches = (0..60)
            .flat_map(|notch| {
                // how long should it be?
                let length = if notch % 5 == 0 { 0.2 } else { 0.075 };

                // get the angle this line should go at
                let angle = Angle::two_pi() * (notch as f32 / 60.0);

                // get the coordinates of the points
                let (psin, pcos) = angle.sin_cos();
                let default_factor = 0.95;
                let x_diff = w / 2.0;
                let y_diff = h / 2.0;
                let p1x = (pcos * default_factor * w / 2.0) + x_diff;
                let p2x = (pcos * default_factor * (1.0 - length) * w / 2.0) + x_diff;
                let p1y = (psin * default_factor * h / 2.0) + y_diff;
                let p2y = (psin * default_factor * (1.0 - length) * h / 2.0) + y_diff;

                let p1 = FlVector::new(p1x, p1y);
                let p2 = FlVector::new(p2x, p2y);

                let tris = cvt_line(p2, p1, 5.0);
                // below line is required to keep 1.49.0 MSRV
                // in a saner program I'd just return the tris array
                Vec::from(tris)
            })
            .collect::<Vec<_>>();

        // generate triangles representing the minute/hour/second hands
        let (ssin, scos) = hands.second_hand.sin_cos();
        let (sx, sy) = (
            scos * width as f32 / 2.0 * 0.9,
            ssin * height as f32 / 2.0 * 0.9,
        );
        let seconds_hands = cvt_line(
            FlVector::new(w / 2.0, h / 2.0),
            FlVector::new(sx + (w / 2.0), sy + (h / 2.0)),
            4.0,
        );
        let minutes_hands = hand_triangle(hands.minute_hand, 0.75, 0.035, w, h);
        let hours_hands = hand_triangle(hands.hour_hand, 0.65, 0.05, w, h);
        let hands = [
            seconds_hands[0],
            seconds_hands[1],
            minutes_hands[0],
            minutes_hands[1],
            hours_hands[0],
            hours_hands[1],
        ];

        // clear the picture to have a white background
        // also clear the mask
        dpy.render_fill_rectangles(
            render::PictOp::SRC,
            dest,
            render::Color {
                red: 0xFFFF,
                green: 0xFFFF,
                blue: 0xFFFF,
                alpha: 0xFFFF,
            },
            &[xproto::Rectangle {
                x: 0,
                y: 0,
                width,
                height,
            }],
        )
        .await?;
        dpy.render_fill_rectangles(
            render::PictOp::SRC,
            mask,
            render::Color::default(),
            &[xproto::Rectangle {
                x: 0,
                y: 0,
                width,
                height,
            }],
        )
        .await?;

        // composite the notches onto the picture
        dpy.render_triangles(render::PictOp::OVER, white, mask, 0, 0, 0, notches)
            .await?;

        dpy.render_composite(
            render::PictOp::OVER,
            grey,
            mask,
            dest,
            0,
            0,
            0,
            0,
            0,
            0,
            width,
            height,
        )
        .await?;

        // render the hands now
        dpy.render_fill_rectangles(
            render::PictOp::SRC,
            mask,
            render::Color::default(),
            &[xproto::Rectangle {
                x: 0,
                y: 0,
                width,
                height,
            }],
        )
        .await?;

        // composite the notches onto the picture
        dpy.render_triangles(render::PictOp::OVER, white, mask, 0, 0, 0, &hands)
            .await?;

        dpy.render_composite(
            render::PictOp::OVER,
            cyan,
            mask,
            dest,
            0,
            0,
            0,
            0,
            0,
            0,
            width,
            height,
        )
        .await?;

        dpy.flush().await?;

        Ok(())
    }

    /// Task for running the clock.
    async fn timekeeper_task(changes: mpsc::Sender<ClockHands>) {
        // begin iterating over an interval, rounded from the current time
        // to the nearest second
        let start_time = to_instant(Local::now().round_subsecs(0));
        let mut interval = interval_at(start_time, Duration::from_secs(1));

        loop {
            interval.tick().await;

            // update the clock
            if changes.send(ClockHands::current()).await.is_err() {
                break;
            }
        }
    }

    /// Get a triangle that could be used to represent a hand of a clock.
    fn hand_triangle(
        angle: Angle<f32>,
        length: f32,
        base: f32,
        width: f32,
        height: f32,
    ) -> [render::Triangle; 2] {
        let origin = FlVector::new(width / 2.0, height / 2.0);

        // first point goes out in the direction of the angle
        let (asin, acos) = angle.sin_cos();
        let tip = FlVector::new(acos * length * width / 2.0, asin * length * height / 2.0) + origin;

        // second and third points are perpendicular from the origin
        let perp_angle = angle - Angle::frac_pi_2();
        let (psin, pcos) = perp_angle.sin_cos();
        let dist = FlVector::new(pcos * base * width / 2.0, psin * base * height / 2.0);
        let tip_dist = dist / 3.0;

        let p1 = origin + dist;
        let p2 = origin - dist;
        let p3 = tip + tip_dist;
        let p4 = tip - tip_dist;

        [
            render::Triangle {
                p1: cvt_point(p1),
                p2: cvt_point(p2),
                p3: cvt_point(p3),
            },
            render::Triangle {
                p1: cvt_point(p2),
                p2: cvt_point(p3),
                p3: cvt_point(p4),
            },
        ]
    }

    /// Simple function to convert a `chrono::DateTime<Utc>` to a
    /// `tokio::time::Instant`.
    fn to_instant(dt: DateTime<Local>) -> Instant {
        let std_now = Instant::now();
        let chrono_now = Local::now();
        let duration = (dt - chrono_now).to_std().unwrap_or(Duration::from_secs(0));

        std_now + duration
    }

    /// Given a line and a width, create a trapezoid able to represent
    /// that line in rendering.
    fn cvt_line(p1: FlVector, p2: FlVector, width: f32) -> [render::Triangle; 2] {
        // create a point that is exactly width/2 away from the line at a
        // perpendicular angle
        let dy = p2.y - p1.y;
        let dx = p2.x - p1.x;
        let angle = Angle::radians(dy.atan2(dx));
        let perp_angle = angle - Angle::frac_pi_2();
        let (psin, pcos) = perp_angle.sin_cos();
        let xoffset = pcos * width / 2.0;
        let yoffset = psin * width / 2.0;

        // get points at offset
        let p1l = FlVector::new(p1.x + xoffset, p1.y + yoffset);
        let p1r = FlVector::new(p1.x - xoffset, p1.y - yoffset);
        let p2l = FlVector::new(p2.x + xoffset, p2.y + yoffset);
        let p2r = FlVector::new(p2.x - xoffset, p2.y - yoffset);

        [
            render::Triangle {
                p1: cvt_point(p1l),
                p2: cvt_point(p1r),
                p3: cvt_point(p2l),
            },
            render::Triangle {
                p1: cvt_point(p1r),
                p2: cvt_point(p2l),
                p3: cvt_point(p2r),
            },
        ]
    }

    /// Convert a floating point into a fixed point.
    fn cvt_point(pt: FlVector) -> render::Pointfix {
        render::Pointfix {
            x: f32_to_fixed(pt.x),
            y: f32_to_fixed(pt.y),
        }
    }

    /// Convert a float (`f32`) to an X11 fixed value.
    fn f32_to_fixed(fl: f32) -> render::Fixed {
        const FACTOR: f32 = 0xFFFF as f32;
        (fl * FACTOR) as render::Fixed
    }
}

#[cfg(all(feature = "tokio-support", feature = "render", unix))]
fn main() -> Result<()> {
    tracing_subscriber::fmt::init();
    inner::real_main()
}

#[cfg(not(all(feature = "tokio-support", feature = "render", unix)))]
fn main() {
    println!("`tokio-support` feature needs to be enabled for this example");
}
