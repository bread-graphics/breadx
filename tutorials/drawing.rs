// MIT/Apache2 License

//! [`Last time`](../color/index.html), we discussed how to create and use colors in `breadx`. Today, we'll
//! use these colors to draw shapes on our window.
//!
//! # The Graphics Context
//!
//! Drawing is done using the [`Gcontext`](../../auto/xproto/struct.Gcontext.html) object. Creating one is
//! relatively simple:
//!
//! ```no_run
//! # use breadx::{BreadError, DisplayConnection};
//! # fn main() -> Result<(), BreadError> {
//! # let mut conn = DisplayConnection::create(None, None)?;
//! # let window = conn.create_simple_window(conn.default_root(), 0, 0, 640, 400, 0,
//! #                                        conn.default_black_pixel(), conn.default_white_pixel())?;
//! use breadx::{GcParameters, rgb};
//!
//! let props = GcParameters { foreground: Some(rgb(std::u8::MAX, 0, 0)), ..Default::default() };
//! let gc = conn.create_gc(window, props)?;
//! # Ok(())
//! # }
//! ```
//!
//! # The Expose Event
//!
//! **TODO: the rest of this**
