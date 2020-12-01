// MIT/Apache2 License

//! [Last time](../event/index.html), we discussed events and how to react to them. Today, we'll talk about
//! color management. It may seem like a trivial subject, but it carries a surprising amount of importance
//! within X11.
//!
//! If you've used frameworks like GTK+ or web browsers, color management might have seemed surprisingly easy.
//! You just pass in your elements representing your red, green, and blue, and the color comes out.
//! Unfortunately, when you get to a level as low as X, it's not always that simple.
//!
//! In an ideal world, all monitors would use standard 24-bit color. Every computer a technician could come
//! across would support R, G and B. In 2020, most of us live in this ideal world. However, those with outdated
//! hardware may be forced to confront a cold reality: monitors that can only hold a limited amount of color.
//! As in, your monitor holds 16 colors. Once you've used all 16, you're out.
//!
//! As an X programmer, you're confronted with a dilemma. Do you take the easy way out, or trudge through
//! on the hard way?
//!
//! # The Easy Way
//!
//! "Bah!" says the programmer, rubbing his unshaven chin and pushing up his glasses. "Why should I support
//! every last piece of outdated, obsolete hardware out there?" He cracks his knuckles. "I'm perfectly
//! fine with taking the easy way out!"
//!
//! You might have noticed that functions like `Display::create_simple_window` take `u32`'s as color values.
//! If you're taking the easy way, then For All Intents and Purposes, this represents a 24-bit color. The
//! lower 8 bits represents blue, the next-lower 8 bits represents green, and the next-lower represents red.
//! In fact, `breadx` provides the [`rgb`](../../colormap/fn.rgb.html) function as a convenience to construct
//! colors in this way.
//!
//! This code would display a window with a red background.
//!
//! ```no_run
//! # use breadx::{BreadError, DisplayConnection};
//! # fn main() -> Result<(), BreadError> {
//! # let mut conn = DisplayConnection::create(None, None)?;
//! use breadx::rgb;
//!
//! let red_color = rgb(std::u8::MAX, 0, 0);
//! let window = conn.create_simple_window(
//!         conn.default_root(),
//!         0,
//!         0,
//!         640,
//!         400,
//!         0,
//!         conn.default_black_pixel(),
//!         // NEW: red_color instead of conn.default_white_pixel()
//!         red_color,
//!     )?;
//! # Ok(())
//! }
//! ```
//!
//! <img src="https://raw.githubusercontent.com/not-a-seagull/breadx/master/images/color_redwindow.png" />
//!
//! As said before, this will *probably* work. This will *probably* produce consistent and reliable results with
//! most X servers. This *probably won't* break any code. This *probably won't* create hard to debug problems
//! for end users on older hardware. This *probably won't* cause said users to report bugs on consumers of the
//! library you wrote, and it *probably won't* make said consumers pull their hair out as they try desperately
//! to trace the problem to code that you wrote.
//!
//! <div class="note">
//! <h2>Note</h2>
//!
//! In all seriousness, the `rgb` function has worked for every computer I've tested it on, including every
//! Linux machine I could get my hands on and an OSX laptop. It's generally a working solution. In addition,
//! it's faster than the "Hard Way" described below, since it's a basic computation instead of sending a
//! request to the server. For the rest of the tutorial, I'll be using the "easy way", since it's generally
//! less verbose than the hard way.
//!
//! To add to that, I'm pretty sure that, as of the time of writing, Rust won't even run on any computer where
//! `rgb` could be expected to fail.
//! </div>
//!
//! # The Hard Way
//!
//! If you've decided to take The Hard Way, you need to be aware of a few extra
//! aspects of X11.
//!
//! The first of these aspects is the Colormap. It can be envisioned as an array containing a series
//! of colors. The [`alloc_color`](../../auto/xproto/struct.Colormap.html#method.alloc_color) function
//! allows you to insert a color into this colormap, and returns the index of the color. The `u32` passed
//! as colors are no longer encoded 24-bit color triplets, but these indexes into the colormap.
//!
//! Consider the following program, which is equivalent to the Easy Way code above.
//!
//! ```no_run
//! # use breadx::{BreadError, DisplayConnection};
//! # fn main() -> Result<(), BreadError> {
//! # let mut conn = DisplayConnection::create(None, None)?;
//! let red_color = conn
//!     .default_colormap()
//!     .alloc_color_immediate(std::u16::MAX, 0, 0)?
//!     .pixel();
//! let window = conn.create_simple_window(
//!         conn.default_root(),
//!         0,
//!         0,
//!         640,
//!         400,
//!         0,
//!         conn.default_black_pixel(),
//!         red_color,
//!     )?;
//! # Ok(())
//! }
//! ```
//!
//! <div class="dissecting">
//! <h2>Dissecting `Colormap::alloc_color`</h2>
//!
//! You may notice two things about the [`alloc_color_immediate`](../../auto/xproto/struct.Colormap.html#method.alloc_color_immediate)
//! function straight away:
//!
//! * It takes `u16`'s instead of `u8`'s, like `rgb` does.
//! * It returns an object of type [`ColorAllocation`](../../colormap/enum.ColorAllocation.html), which has a
//!   method named `pixel` that returns the actual color result.
//!
//! In the first case, X11 actually supports using `u16`'s to create colors instead of `u8`. **TODO: actually
//! figure out if X supports truecolor**
//!
//! In the second case, X11 colormaps will try to return a color already allocated in the colormap if it no
//! longer has any room for color. In case you care about this, `breadx` returns the enum to tell you whether
//! or not the color you put into the function is equal to the color you're gettting. In case you don't, the
//! `pixel` function is an easy way to get the result out.
//! </div>
//!
//! Note that this way is somewhat slower than the Easy Way above. It sends a request to the server which, no
//! matter how you dice it, it always going to be slower than a few bit shifts and some addition. Consider
//! which method best suits you as you program.
//!
//! [`Next time, we'll talk about how to draw on windows.`](../drawing/index.html)
