// MIT/Apache2 License

//! This module is internal-use, and defines a set of functions for rearraging the bytes
//! of an image to conform to whatever standards it needs to conform to.
//! Note that much of this is taken from Xlib's implementation of images.

use crate::{
    auto::xproto::ImageOrder,
    util::{REVERSE_BYTES, REVERSE_NIBS},
};

#[cfg(feature = "parallel")]
use rayon::prelude::*;

/// Do not swap bytes, just copy them over.
pub fn no_swap(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    half_order: ImageOrder,
) {
    if srcinc == destinc {
        // this operation should be vectorized
        dest.copy_from_slice(source)
    } else {
        #[cfg(not(feature = "parallel"))]
        dest.chunks_mut(destinc)
            .zip(source.chunks(srcinc))
            .for_each(|(d, s)| (&mut d[..linelen]).copy_from_slice(&s[..linelen]));
        #[cfg(feature = "parallel")]
        dest.par_chunks_mut(destinc)
            .zip(source.par_chunks(srcinc))
            .for_each(|(d, s)| (&mut d[..linelen]).copy_from_slice(&s[..linelen]));
    }
}

/// Swap every two bytes, if necessary.
pub fn swap_two_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    half_order: ImageOrder,
) {
    #[inline]
    fn swap_operation(source: &[u8], dest: &mut [u8], half_order: ImageOrder) {
        dest.chunks_mut(2).zip(source.chunks(2)).for_each(|(d, s)| {
            d[0] = s[1];
            d[1] = s[0];
        });
    }

    #[cfg(not(feature = "parallel"))]
    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
    #[cfg(feature = "parallel")]
    dest.par_chunks_mut(destinc)
        .zip(source.par_chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
}

/// Swap every three bytes.
pub fn swap_three_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    half_order: ImageOrder,
) {
    #[inline]
    fn swap_operation(source: &[u8], dest: &mut [u8], half_order: ImageOrder) {
        dest.chunks_mut(3).zip(source.chunks(3)).for_each(|(d, s)| {
            d[0] = s[2];
            d[1] = s[1];
            d[2] = s[0];
        });
    }

    #[cfg(not(feature = "parallel"))]
    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
    #[cfg(feature = "parallel")]
    dest.par_chunks_mut(destinc)
        .zip(source.par_chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
}

/// Swap every four bytes.
pub fn swap_four_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    half_order: ImageOrder,
) {
    #[inline]
    fn swap_operation(source: &[u8], dest: &mut [u8], half_order: ImageOrder) {
        dest.chunks_mut(4).zip(source.chunks(4)).for_each(|(d, s)| {
            d[0] = s[3];
            d[1] = s[2];
            d[2] = s[1];
            d[3] = s[0];
        });
    }

    #[cfg(not(feature = "parallel"))]
    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
    #[cfg(feature = "parallel")]
    dest.par_chunks_mut(destinc)
        .zip(source.par_chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
}

/// Swap every word.
pub fn swap_words(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    half_order: ImageOrder,
) {
    #[inline]
    fn swap_operation(source: &[u8], dest: &mut [u8], half_order: ImageOrder) {
        dest.chunks_mut(4).zip(source.chunks(4)).for_each(|(d, s)| {
            d[0] = s[2];
            d[1] = s[3];
            d[2] = s[0];
            d[3] = s[1];
        });
    }

    #[cfg(not(feature = "parallel"))]
    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
    #[cfg(feature = "parallel")]
    dest.par_chunks_mut(destinc)
        .zip(source.par_chunks(srcinc))
        .for_each(|(d, s)| swap_operation(s, d, half_order));
}

/// Swap every nibble.
pub fn swap_nibble(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    half_order: ImageOrder,
) {
    #[cfg(not(feature = "parallel"))]
    dest.iter_mut()
        .zip(source.iter())
        .for_each(|(d, s)| *d = REVERSE_NIBS[*s as usize]);
    #[cfg(feature = "parallel")]
    dest.par_iter_mut()
        .zip(source.par_iter())
        .for_each(|(d, s)| *d = REVERSE_NIBS[*s as usize]);
}

/// Shift the nibbles lest.
pub fn shift_nibbles_left(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    half_order: ImageOrder,
) {
    #[inline]
    fn shift_operation(source: &[u8], dest: &mut [u8], linelen: usize, half_order: ImageOrder) {
        if let ImageOrder::MsbFirst = half_order {
            dest.iter_mut()
                .take(linelen)
                .zip(source.iter().take(linelen))
                .for_each(|(d, s)| *d = ((s & 0x0F) << 4) | ((s & 0xF0) >> 4));
        } else {
        }
    }

    #[cfg(not(feature = "parallel"))]
    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .for_each(|(d, s)| shift_operation(s, d, linelen, half_order));
    #[cfg(feature = "parallel")]
    dest.par_chunks_mut(destinc)
        .zip(source.par_chunks(srcinc))
        .for_each(|(d, s)| shift_operation(s, d, linelen, half_order));
}
