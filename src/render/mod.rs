// MIT/Apache2 License

//! First-class support for the XRender extension.

mod display;
mod picture;
mod tesselate;

pub use display::*;
pub use picture::*;

use crate::auto::render::Fixed;

/// Convert a fixed point value to a 64-bit float.
#[inline]
pub fn fixed_to_double(f: Fixed) -> f64 {
    (f as f64) / 0xFFFF
}

/// Convert a 64-bit float to a fixed point value.
#[inline]
pub fn double_to_fixed(d: f64) -> Fixed {
    (d * 0xFFFF) as Fixed
}
