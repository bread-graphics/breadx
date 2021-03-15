// MIT/Apache2 License

//! First-class support for the XRender extension.

mod display;
mod picture;
mod tesselate;

pub use display::*;
pub use picture::*;
pub use tesselate::*;

use crate::auto::render::Fixed;

const MULTIPLIER: f64 = 0xFFFF as f64;

/// Convert a fixed point value to a 64-bit float.
#[inline]
pub fn fixed_to_double(f: Fixed) -> f64 {
    (f as f64) / MULTIPLIER
}

/// Convert a 64-bit float to a fixed point value.
#[inline]
pub fn double_to_fixed(d: f64) -> Fixed {
    (d * MULTIPLIER) as Fixed
}
