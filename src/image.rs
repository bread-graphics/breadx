// MIT/Apache2 License

//! This module defines the Image struct, as well as procedures for manipulating images.

use crate::auto::xproto::{ImageFormat, ImageOrder};
use alloc::{boxed::Box, vec::Vec};

/// An image. This acts as a wrapper around data that represents an image.
#[repr(C)]
pub struct Image<Data: ?Sized> {
    /// The width of this image.
    pub width: u32,
    /// The height of this image.    
    pub height: u32,
    /// The depth of this image.
    pub depth: u8,
    /// Byte order of this image.
    pub bit_order: ImageOrder,
    /// Format for this image.
    pub format: ImageFormat,
    /// The quantity of the scanline (usually 8, 16, or 32)
    pub bitmap_unit: u32,
    pub bitmap_pad: u32,
    pub bytes_per_line: u32,
    pub bits_per_pixel: u32,

    /// The data contained within this image.
    pub data: Data,
}
