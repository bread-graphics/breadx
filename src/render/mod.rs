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
use core::{
    cmp::Ordering,
    hash::{Hash, Hasher},
};

impl Copy for Pointfix {}
impl Copy for Linefix {}
impl Copy for Color {}
impl Copy for Trapezoid {}

impl Eq for Color {}

impl Ord for Color {
    #[inline]
    fn cmp(&self, other: &Color) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Hash for Color {
    #[inline]
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u16(self.red);
        state.write_u16(self.green);
        state.write_u16(self.blue);
        state.write_u16(self.alpha);
    }
}

const MULTIPLIER: f64 = 65536.0;

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
