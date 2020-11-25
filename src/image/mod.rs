// MIT/Apache2 License

//! This module defines the Image struct, as well as procedures for manipulating images.

#![allow(clippy::similar_names)]

// Note: There is a lot of code in this file that's copied from Xlib, and it is going to be messy and
// unoptimized. Anyone who knows stuff about image processing is encouraged to PR and fix some of this.

mod fit;

use crate::{
    auto::xproto::{ImageFormat, ImageOrder},
    util::reverse_bytes,
};
use core::ops::{Deref, DerefMut};

/// An image. This acts as a wrapper around data that represents an image.
#[derive(Clone, Debug)]
pub struct Image<Data> {
    /// The width of this image.
    pub width: usize,
    /// The height of this image.    
    pub height: usize,
    /// Number of pixels offset in the X direction.
    pub x_offset: usize,
    /// Format for this image.
    pub format: ImageFormat,
    /// The depth of this image.
    pub depth: u8,
    /// Bit order of this image.
    pub bit_order: ImageOrder,
    /// Byte order of this image.
    pub byte_order: ImageOrder,
    /// The quantity of the scanline (usually 8, 16, or 32)
    pub bitmap_unit: u8,
    pub bitmap_pad: u32,
    pub bytes_per_line: usize,
    pub bits_per_pixel: u8,

    /// The data contained within this image.
    pub data: Data,
}

/// To prevent monomorphization code bloat, this is a type-erased version of the image.
trait GenericImage {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn x_offset(&self) -> usize;
    fn format(&self) -> ImageFormat;
    fn depth(&self) -> u8;
    fn bit_order(&self) -> ImageOrder;
    fn byte_order(&self) -> ImageOrder;
    fn bitmap_unit(&self) -> u8;
    fn bitmap_pad(&self) -> u32;
    fn bytes_per_line(&self) -> usize;
    fn bits_per_pixel(&self) -> u8;
}

impl<Data> GenericImage for Image<Data> {
    #[inline]
    fn width(&self) -> usize {
        self.width
    }
    #[inline]
    fn height(&self) -> usize {
        self.height
    }
    #[inline]
    fn x_offset(&self) -> usize {
        self.x_offset
    }
    #[inline]
    fn format(&self) -> ImageFormat {
        self.format
    }
    #[inline]
    fn depth(&self) -> u8 {
        self.depth
    }
    #[inline]
    fn bit_order(&self) -> ImageOrder {
        self.bit_order
    }
    #[inline]
    fn byte_order(&self) -> ImageOrder {
        self.byte_order
    }
    #[inline]
    fn bitmap_unit(&self) -> u8 {
        self.bitmap_unit
    }
    #[inline]
    fn bitmap_pad(&self) -> u32 {
        self.bitmap_pad
    }
    #[inline]
    fn bytes_per_line(&self) -> usize {
        self.bytes_per_line
    }
    #[inline]
    fn bits_per_pixel(&self) -> u8 {
        self.bits_per_pixel
    }
}

/// Table of lower bits.
const LOW_BITS_TABLE: &[u32] = &[
    0x00000000, 0x00000001, 0x00000003, 0x00000007, 0x0000000f, 0x0000001f, 0x0000003f, 0x0000007f,
    0x000000ff, 0x000001ff, 0x000003ff, 0x000007ff, 0x00000fff, 0x00001fff, 0x00003fff, 0x00007fff,
    0x0000ffff, 0x0001ffff, 0x0003ffff, 0x0007ffff, 0x000fffff, 0x001fffff, 0x003fffff, 0x007fffff,
    0x00ffffff, 0x01ffffff, 0x03ffffff, 0x07ffffff, 0x0fffffff, 0x1fffffff, 0x3fffffff, 0x7fffffff,
    0xffffffff,
];

// The current OS-dependent byte order.
#[cfg(target_endian = "little")]
const OS_BYTE_ORDER: ImageOrder = ImageOrder::LsbFirst;
#[cfg(not(target_endian = "little"))]
const OS_BYTE_ORDER: ImageOrder = ImageOrder::MsbFirst;

impl<Data> Image<Data>
where
    Data: Deref<Target = [u8]>,
{
    /// Assuming that this is a 32 bit ZPixmap, get a pixel.
    #[inline]
    fn pixel32(&self, x: usize, y: usize) -> u32 {
        let addr = (y * self.bytes_per_line) + (x << 2); // the address of the first item
        let byte_slice = &self.data.deref()[addr..addr + 4];

        let mut res = if self.byte_order == OS_BYTE_ORDER {
            // if it matches the OS byte order, just use bytemuck to cast it
            // it should be well-aligned anyways
            let res = bytemuck::cast_slice::<u8, u32>(byte_slice);
            res[0]
        } else {
            match self.byte_order {
                ImageOrder::MsbFirst => {
                    // if our system is little endian and the target is big endian, manually convert it
                    ((byte_slice[0] as u32) << 24)
                        | ((byte_slice[1] as u32) << 16)
                        | ((byte_slice[2] as u32) << 8)
                        | ((byte_slice[3]) as u32)
                }
                ImageOrder::LsbFirst => {
                    ((byte_slice[3] as u32) << 24)
                        | ((byte_slice[2] as u32) << 16)
                        | ((byte_slice[1] as u32) << 8)
                        | (byte_slice[0] as u32)
                }
            }
        };

        if self.depth != 32 {
            res &= LOW_BITS_TABLE[self.depth as usize];
        }

        res
    }

    /// Assuming that this is a 16 bit ZPixmap, get a pixel.
    #[inline]
    fn pixel16(&self, x: usize, y: usize) -> u32 {
        let addr = (y * self.bytes_per_line) + (x << 1);
        let byte_slice = &self.data.deref()[addr..addr + 2];

        let mut res = match self.byte_order {
            ImageOrder::MsbFirst => ((byte_slice[0] as u32) << 8) | byte_slice[1] as u32,
            ImageOrder::LsbFirst => ((byte_slice[1] as u32) << 8) | byte_slice[0] as u32,
        };

        if self.depth != 16 {
            res &= LOW_BITS_TABLE[self.depth as usize];
        }

        res
    }

    /// Assuming that this is an 8 bit ZPixmap, get a pixel.
    #[inline]
    fn pixel8(&self, x: usize, y: usize) -> u32 {
        let addr = (y * self.bytes_per_line) + x;
        let mut res = self.data.deref()[addr] as u32;
        if self.depth != 8 {
            res &= LOW_BITS_TABLE[self.depth as usize];
        }
        res
    }

    /// Assuming this is a 1 bit ZPixmap, get a pixel.
    #[inline]
    fn pixel1(&self, x: usize, y: usize) -> u32 {
        let mut xoff = x + self.x_offset as usize;
        let yoff = (y * self.bytes_per_line) + (xoff >> 3);
        xoff &= 7;

        let bit = match self.bit_order {
            ImageOrder::MsbFirst => 0x80 >> xoff,
            ImageOrder::LsbFirst => 1 << xoff,
        };

        if self.data[yoff] & bit != 0 {
            1
        } else {
            0
        }
    }

    /// Generic getpixel that works for all images.
    #[inline]
    fn pixel_generic(&self, x: usize, y: usize) -> u32 {
        let bits = (x + self.x_offset) as usize % self.bitmap_unit as usize;
        let pixel = if self.bits_per_pixel as u8 | self.depth == 1 {
            let mut pixel: [u8; 4] = [0; 4];
            let addr = xyindex(x, y, self);
            let copy_len = (self.bitmap_unit as usize) >> 3;
            let data_slice = &self.data.deref()[addr..addr + copy_len];
            (&mut pixel[0..copy_len]).copy_from_slice(data_slice);
            xy_normalize_bits(&mut pixel[0..copy_len], self);
            (pixel[bits >> 3] as u32 >> (bits & 7)) & 1
        } else if let ImageFormat::XyPixmap = self.format {
            let mut plane = 0;
            let copy_len = (self.bitmap_unit as usize) >> 3;
            let mut res: u32 = 0;

            for _ in 0..self.depth {
                let mut pixel: [u8; 4] = [0; 4];
                let addr = xyindex(x, y, self) + plane;
                let data_slice = &self.data.deref()[addr..addr + copy_len];
                (&mut pixel[0..copy_len]).copy_from_slice(data_slice);
                xy_normalize_bits(&mut pixel[0..copy_len], self);
                res = (res << 1) | (((pixel[bits >> 3] as u32) >> (bits & 7)) & 1);
                plane += self.bytes_per_line * self.height;
            }

            res
        } else {
            // ZPixmap
            let mut pixel: [u8; 4] = [0; 4];
            let addr = zindex(x, y, self);
            let copy_len = (self.bits_per_pixel as usize + 7) >> 3;
            let data_slice = &self.data.deref()[addr..addr + copy_len];
            (&mut pixel[0..copy_len]).copy_from_slice(data_slice);
            z_normalize_bits(&mut pixel[0..copy_len], self);

            let mut res: u32 = 0;
            for i in (0..4).rev() {
                res = (res << 8) | pixel[i] as u32;
            }

            if self.bits_per_pixel == 4 {
                if res & 1 != 0 {
                    res >>= 4;
                } else {
                    res &= 0x0F;
                }
            }

            res
        };

        if self.bits_per_pixel == self.depth {
            pixel
        } else {
            pixel & LOW_BITS_TABLE[self.depth as usize]
        }
    }

    /// Get the pixel at the specified index.
    #[inline]
    pub fn pixel(&self, x: usize, y: usize) -> u32 {
        match (self.format, self.bits_per_pixel) {
            (ImageFormat::XyPixmap, _) => self.pixel_generic(x, y),
            (_, 32) => self.pixel32(x, y),
            (_, 16) => self.pixel16(x, y),
            (_, 8) => self.pixel8(x, y),
            (_, 1) => self.pixel1(x, y),
            _ => self.pixel_generic(x, y),
        }
    }
}

impl<Data> Image<Data>
where
    Data: Deref<Target = [u8]> + DerefMut,
{
    /// Assuming this is a 32 bit ZPixmap, set a pixel's value.
    #[inline]
    fn set_pixel32(&mut self, x: usize, y: usize, pixel: u32) {
        let addr = (y * self.bytes_per_line) + (x << 2);
        let target = &mut self.data.deref_mut()[addr..addr + 4];
        if OS_BYTE_ORDER == self.byte_order {
            let target = bytemuck::cast_slice_mut::<u8, u32>(target);
            target[0] = pixel;
        } else {
            match self.byte_order {
                ImageOrder::MsbFirst => {
                    target[0] = (pixel >> 24) as u8;
                    target[1] = (pixel >> 16) as u8;
                    target[2] = (pixel >> 8) as u8;
                    target[3] = pixel as u8;
                }
                ImageOrder::LsbFirst => {
                    target[3] = (pixel >> 24) as u8;
                    target[2] = (pixel >> 16) as u8;
                    target[1] = (pixel >> 8) as u8;
                    target[0] = pixel as u8;
                }
            }
        }
    }

    /// Assuming this is a 16 bit ZPixmap, set a pixel's value.
    #[inline]
    fn set_pixel16(&mut self, x: usize, y: usize, pixel: u32) {
        let addr = (y * self.bytes_per_line) + (x << 1);
        let target = &mut self.data.deref_mut()[addr..addr + 2];
        match self.byte_order {
            ImageOrder::MsbFirst => {
                target[0] = (pixel >> 8) as u8;
                target[1] = pixel as u8;
            }
            ImageOrder::LsbFirst => {
                target[1] = (pixel >> 8) as u8;
                target[0] = pixel as u8;
            }
        }
    }

    /// Assuming this is an 8 bit ZPixmap, set a pixel's value.
    #[inline]
    fn set_pixel8(&mut self, x: usize, y: usize, pixel: u32) {
        let addr = (y * self.bytes_per_line) + x;
        self.data.deref_mut()[addr] = pixel as u8;
    }

    /// Assuming this is a 1 bit ZPixmap, set a pixel's value.
    #[inline]
    fn set_pixel1(&mut self, x: usize, y: usize, pixel: u32) {
        let mut xoff = x + self.x_offset;
        let yoff = (y * self.bytes_per_line) + (xoff >> 3);
        xoff &= 7;

        let bit = match self.bit_order {
            ImageOrder::MsbFirst => 0x80 >> xoff,
            ImageOrder::LsbFirst => 1 << xoff,
        };

        if pixel & 1 == 0 {
            self.data.deref_mut()[yoff] &= !bit;
        } else {
            self.data.deref_mut()[yoff] |= bit;
        }
    }

    /// Generic function that works for every image.
    #[inline]
    fn set_pixel_generic(&mut self, x: usize, y: usize, mut pixel: u32) {
        if self.depth == 4 {
            pixel &= 0x0F;
        }

        let mut npixel = pixel;
        let mut px = pixel;
        let pixel_bytes = bytemuck::cast_slice_mut::<u32, u8>(ref_slice::ref_slice_mut(&mut pixel));
        for i in 0..4 {
            pixel_bytes[i] = px as u8;
            px >>= 8;
        }

        let nbytes = self.bitmap_unit as usize >> 3;

        if (self.bits_per_pixel | self.depth) == 1 {
            let mut buffer: [u8; 4] = [0; 4];
            let addr = xyindex(x, y, self);
            (&mut buffer[0..nbytes]).copy_from_slice(&self.data.deref()[addr..addr + nbytes]);
            xy_normalize_bits(&mut buffer[0..nbytes], self);
            let index = (x + self.x_offset) % self.bitmap_unit as usize;
            let incoming_data = bytemuck::cast_slice::<u32, u8>(ref_slice::ref_slice(&pixel));
            put_bits(incoming_data, index, 1, &mut buffer[0..nbytes]);
            xy_normalize_bits(&mut buffer[0..nbytes], self);
            (&mut self.data.deref_mut()[addr..addr + nbytes]).copy_from_slice(&buffer[0..nbytes]);
        } else if let ImageFormat::XyPixmap = self.format {
            let mut plane = (self.bytes_per_line * self.height) * (self.depth as usize - 1);
            let index = (x + self.x_offset) % self.bitmap_unit as usize;
            for _ in 0..self.depth {
                let mut buffer: [u8; 4] = [0; 4];
                let addr = xyindex(x, y, self) + plane;
                (&mut buffer[0..nbytes]).copy_from_slice(&self.data.deref()[addr..addr + nbytes]);
                xy_normalize_bits(&mut buffer[0..nbytes], self);
                let incoming_data = bytemuck::cast_slice::<u32, u8>(ref_slice::ref_slice(&pixel));
                put_bits(incoming_data, index, 1, &mut buffer[0..nbytes]);
                xy_normalize_bits(&mut buffer[0..nbytes], self);
                (&mut self.data.deref_mut()[addr..addr + nbytes])
                    .copy_from_slice(&buffer[0..nbytes]);

                npixel >>= 1;
                let outgoing_data =
                    bytemuck::cast_slice_mut::<u32, u8>(ref_slice::ref_slice_mut(&mut pixel));
                px = npixel;
                for i in 0..4 {
                    outgoing_data[i] = px as u8;
                    px >>= 8;
                }

                plane -= self.bytes_per_line * self.height;
            }
        } else {
            // is a ZImage
            let addr = zindex(x, y, self);
            let mut buffer: [u8; 4] = [0; 4];
            let nbytes = (self.bits_per_pixel as usize + 7) >> 3;
            (&mut buffer[0..nbytes]).copy_from_slice(&self.data.deref()[addr..addr + nbytes]);
            z_normalize_bits(&mut buffer[0..nbytes], self);
            put_bits(
                bytemuck::cast_slice::<u32, u8>(ref_slice::ref_slice(&pixel)),
                (x * self.bits_per_pixel as usize) & 7,
                self.bits_per_pixel as usize,
                &mut buffer[0..nbytes],
            );
            z_normalize_bits(&mut buffer[0..nbytes], self);
            (&mut self.data.deref_mut()[addr..addr + nbytes]).copy_from_slice(&buffer[0..nbytes]);
        }
    }

    /// Set a pixel at the specified coordinates to the specified value.
    #[inline]
    pub fn set_pixel(&mut self, x: usize, y: usize, pixel: u32) {
        match (self.format, self.bits_per_pixel) {
            (ImageFormat::XyPixmap, _) => self.set_pixel_generic(x, y, pixel),
            (_, 32) => self.set_pixel32(x, y, pixel),
            (_, 16) => self.set_pixel16(x, y, pixel),
            (_, 8) => self.set_pixel8(x, y, pixel),
            (_, 1) => self.set_pixel1(x, y, pixel),
            _ => self.set_pixel_generic(x, y, pixel),
        }
    }
}

// helper functions
#[inline]
fn xyindex(x: usize, y: usize, image: &dyn GenericImage) -> usize {
    (y * image.bytes_per_line())
        + ((x + image.x_offset()) / image.bitmap_unit() as usize)
            * ((image.bitmap_unit() as usize) >> 3)
}

#[inline]
fn zindex(x: usize, y: usize, image: &dyn GenericImage) -> usize {
    (y * image.bytes_per_line()) + ((x * image.bits_per_pixel() as usize) >> 3)
}

#[inline]
fn xy_normalize_bits(bits: &mut [u8], image: &dyn GenericImage) {
    if image.byte_order() == ImageOrder::MsbFirst || image.bit_order() == ImageOrder::MsbFirst {
        if image.byte_order() != image.bit_order() {
            match image.bitmap_unit() {
                16 => bits.swap(0, 1),
                32 => {
                    bits.swap(0, 3);
                    bits.swap(1, 2);
                }
                _ => (),
            }
        }

        reverse_bytes(bits);
    }
}

#[inline]
fn z_normalize_bits(bits: &mut [u8], image: &dyn GenericImage) {
    if image.byte_order() == ImageOrder::MsbFirst || image.bit_order() == ImageOrder::MsbFirst {
        match image.bits_per_pixel() {
            4 => {
                bits[0] = ((bits[0] >> 4) & 0x0F) | ((bits[0] << 4) & !0x0F);
            }
            16 => {
                bits.swap(0, 1);
            }
            24 => {
                bits.swap(0, 2);
            }
            32 => {
                bits.swap(0, 3);
                bits.swap(1, 2);
            }
            bpp => log::error!("Invalid bits per pixel: {}", bpp),
        }
    }
}

const LOMASK: [u8; 0x09] = [0x00, 0x01, 0x03, 0x07, 0x0F, 0x1F, 0x3F, 0x7F, 0xFF];
const HIMASK: [u8; 0x09] = [0xFF, 0xFE, 0xFC, 0xF8, 0xF0, 0xE0, 0xC0, 0x80, 0x00];

#[inline]
fn put_bits(source: &[u8], mut dstoffset: usize, mut numbits: usize, dest: &mut [u8]) {
    let mut src_index = 0;
    let mut dst_index = dstoffset >> 3;

    dstoffset &= 7;
    let hibits = 8 - dstoffset;
    let mut chlo = dest[dst_index] & LOMASK[dstoffset];

    loop {
        let mut chhi = (source[src_index] << dstoffset) & HIMASK[dstoffset];
        if numbits <= hibits {
            chhi &= LOMASK[dstoffset + numbits];
            dest[dst_index] = (dest[dst_index] & HIMASK[dstoffset + numbits]) | chlo | chhi;
            break;
        }

        dest[dst_index] = chlo | chhi;
        dst_index += 1;
        numbits -= hibits;

        chlo = (source[src_index] & HIMASK[hibits]) >> hibits;
        src_index += 1;

        if numbits <= dstoffset {
            chlo &= LOMASK[numbits];
            dest[dst_index] = (dest[dst_index] & HIMASK[numbits]) | chlo;
            break;
        }

        numbits -= dstoffset;
    }
}
