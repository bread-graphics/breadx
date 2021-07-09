// MIT/Apache2 License

//! First-class support for the XRender extension.

mod display;
mod picture;
mod tesselate;

pub use display::*;
pub use picture::*;
pub use tesselate::*;

pub use crate::auto::render::{
    Color, Fixed, Linefix, PictOp, Pictformat, Picture, Pointfix, Transform, Trapezoid, Triangle,
};

impl Copy for Pointfix {}
impl Copy for Linefix {}
impl Copy for Color {}
impl Copy for Trapezoid {}

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
