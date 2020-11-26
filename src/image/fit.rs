// MIT/Apache2 License

//! This module is internal-use, and defines a set of functions for rearraging the bytes
//! of an image to conform to whatever standards it needs to conform to.
//! Note that much of this is taken from Xlib's implementation of images.

use crate::{
    auto::xproto::ImageOrder,
    util::{roundup, REVERSE_BYTES, REVERSE_NIBS},
};

/// Do not swap bytes, just copy them over.
pub(crate) fn no_swap(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    _height: usize,
    _half_order: ImageOrder,
) {
    if srcinc == destinc {
        // this operation should be vectorized
        dest.copy_from_slice(source)
    } else {
        dest.chunks_mut(destinc)
            .zip(source.chunks(srcinc))
            .for_each(|(d, s)| (&mut d[..linelen]).copy_from_slice(&s[..linelen]));
    }
}

/// Swap every two bytes, if necessary.
pub(crate) fn swap_two_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    height: usize,
    half_order: ImageOrder,
) {
    let mut length = roundup(linelen, 2);
    let mut swap_operation = move |index: usize, source: &[u8], dest: &mut [u8]| {
        if index == height - 1 && linelen != length {
            length -= 2;
            if let ImageOrder::MsbFirst = half_order {
                dest[length] = source[length + 1];
            } else {
                dest[length + 1] = source[length];
            }
        }

        dest.chunks_mut(2)
            .take(length / 2)
            .zip(source.chunks(2).take(length / 2))
            .for_each(|(d, s)| {
                d[0] = s[1];
                d[1] = s[0];
            });
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .enumerate()
        .for_each(|(i, (d, s))| swap_operation(i, s, d));
}

/// Swap every three bytes.
pub(crate) fn swap_three_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    height: usize,
    half_order: ImageOrder,
) {
    let mut length = ((linelen + 2) / 3) * 3; // round up to 3

    let mut swap_operation = move |index: usize, source: &[u8], dest: &mut [u8]| {
        if index == height - 1 && linelen != length {
            length -= 3;
            if linelen - length == 2 {
                dest[length + 1] = source[length + 1];
            } else if let ImageOrder::MsbFirst = half_order {
                dest[length] = source[length + 2];
            } else {
                dest[length + 2] = source[length];
            }
        }

        dest.chunks_mut(3)
            .take(length / 3)
            .zip(source.chunks(3).take(length / 3))
            .for_each(|(d, s)| {
                d[0] = s[2];
                d[1] = s[1];
                d[2] = s[0];
            });
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .enumerate()
        .for_each(|(i, (d, s))| swap_operation(i, s, d));
}

/// Swap every four bytes.
pub(crate) fn swap_four_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    height: usize,
    half_order: ImageOrder,
) {
    let mut length = roundup(linelen, 4);

    let mut swap_operation = move |index: usize, source: &[u8], dest: &mut [u8]| {
        if index == height - 1 && linelen != length {
            length -= 4;
            if let ImageOrder::MsbFirst = half_order {
                dest[length] = source[length + 3];
            }
            if (half_order == ImageOrder::LsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::MsbFirst && linelen & 2 != 0)
            {
                dest[length + 1] = source[length + 2];
            }
            if (half_order == ImageOrder::MsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::LsbFirst && linelen & 2 != 0)
            {
                dest[length + 2] = source[length + 1]
            }
            if let ImageOrder::LsbFirst = half_order {
                dest[length + 3] = source[length];
            }
        }

        dest.chunks_mut(4)
            .take(length / 4)
            .zip(source.chunks(4).take(length / 4))
            .for_each(|(d, s)| {
                d[0] = s[3];
                d[1] = s[2];
                d[2] = s[1];
                d[3] = s[0];
            });
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .enumerate()
        .for_each(|(i, (d, s))| swap_operation(i, s, d));
}

/// Swap every word.
pub(crate) fn swap_words(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    height: usize,
    half_order: ImageOrder,
) {
    let mut length = roundup(linelen, 4);

    let mut swap_operation = |index: usize, source: &[u8], dest: &mut [u8]| {
        if index == height - 1 && linelen != length {
            length -= 4;

            if let ImageOrder::MsbFirst = half_order {
                dest[length + 1] = source[length + 3];
            }
            if (half_order == ImageOrder::LsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::MsbFirst && linelen & 2 != 0)
            {
                dest[length] = source[length + 2];
            }
            if (half_order == ImageOrder::MsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::LsbFirst && linelen & 2 != 0)
            {
                dest[length + 3] = source[length + 1]
            }
            if let ImageOrder::LsbFirst = half_order {
                dest[length + 2] = source[length];
            }
        }

        dest.chunks_mut(4)
            .take(length / 4)
            .zip(source.chunks(4).take(length / 4))
            .for_each(|(d, s)| {
                d[0] = s[2];
                d[1] = s[3];
                d[2] = s[0];
                d[3] = s[1];
            });
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .enumerate()
        .for_each(|(i, (d, s))| swap_operation(i, s, d));
}

/// Swap every nibble.
pub(crate) fn swap_nibble(
    source: &[u8],
    dest: &mut [u8],
    _linelen: usize,
    _srcinc: usize,
    _destinc: usize,
    _height: usize,
    _half_order: ImageOrder,
) {
    dest.iter_mut()
        .zip(source.iter())
        .for_each(|(d, s)| *d = REVERSE_NIBS[*s as usize]);
}

/// Shift the nibbles lest.
pub(crate) fn shift_nibbles_left(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    _height: usize,
    half_order: ImageOrder,
) {
    let shift_operation = move |source: &[u8], dest: &mut [u8]| {
        if let ImageOrder::MsbFirst = half_order {
            dest.iter_mut()
                .take(linelen)
                .zip(source.windows(2).take(linelen))
                .for_each(|(d, s)| *d = ((s[0] & 0x0F) << 4) | ((s[1] & 0xF0) >> 4));
        } else {
            dest.iter_mut()
                .take(linelen)
                .zip(source.windows(2).take(linelen))
                .for_each(|(d, s)| {
                    *d = ((s[1] & 0x0F) << 4) | ((s[0] & 0xF0) >> 4);
                })
        }
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .for_each(|(d, s)| shift_operation(s, d));
}

/// Swap bits.
pub(crate) fn swap_bits(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    _height: usize,
    _half_order: ImageOrder,
) {
    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .for_each(|(d, s)| {
            d.iter_mut()
                .take(linelen)
                .zip(s.iter().take(linelen))
                .for_each(|(d, s)| *d = REVERSE_BYTES[*s as usize])
        });
}

/// Swap bits and every two bytes.
pub(crate) fn swap_bits_and_two_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    height: usize,
    half_order: ImageOrder,
) {
    let mut length = roundup(linelen, 2);

    let mut shift_operation = move |index: usize, source: &[u8], dest: &mut [u8]| {
        if height - 1 == index && linelen != length {
            length -= 2;
            if let ImageOrder::MsbFirst = half_order {
                dest[length] = REVERSE_BYTES[source[length + 1] as usize];
            } else {
                dest[length + 1] = REVERSE_BYTES[source[length] as usize];
            }
        }

        dest.chunks_mut(2)
            .take(length / 2)
            .zip(source.chunks(2).take(length / 2))
            .for_each(|(d, s)| {
                d[0] = REVERSE_BYTES[s[1] as usize];
                d[1] = REVERSE_BYTES[s[0] as usize];
            });
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .enumerate()
        .for_each(|(i, (d, s))| shift_operation(i, s, d));
}

/// Swap bits and every four bytes.
pub(crate) fn swap_bits_and_four_bytes(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    height: usize,
    half_order: ImageOrder,
) {
    let mut length = roundup(linelen, 4);

    let mut swap_operation = |index: usize, source: &[u8], dest: &mut [u8]| {
        if index == height - 1 && linelen != length {
            length -= 4;
            if let ImageOrder::MsbFirst = half_order {
                dest[length] = REVERSE_BYTES[source[length + 3] as usize];
            }
            if (half_order == ImageOrder::LsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::MsbFirst && linelen & 2 != 0)
            {
                dest[length + 1] = REVERSE_BYTES[source[length + 2] as usize];
            }
            if (half_order == ImageOrder::MsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::LsbFirst && linelen & 2 != 0)
            {
                dest[length + 2] = REVERSE_BYTES[source[length + 1] as usize];
            }
            if let ImageOrder::LsbFirst = half_order {
                dest[length + 3] = REVERSE_BYTES[source[length] as usize];
            }
        }

        dest.chunks_mut(4)
            .take(length / 4)
            .zip(source.chunks(4).take(length / 4))
            .for_each(|(d, s)| {
                d[0] = REVERSE_BYTES[s[3] as usize];
                d[1] = REVERSE_BYTES[s[2] as usize];
                d[2] = REVERSE_BYTES[s[1] as usize];
                d[3] = REVERSE_BYTES[s[0] as usize];
            });
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .enumerate()
        .for_each(|(i, (d, s))| swap_operation(i, s, d));
}

/// Swap bits and words.
pub(crate) fn swap_bits_and_words(
    source: &[u8],
    dest: &mut [u8],
    linelen: usize,
    srcinc: usize,
    destinc: usize,
    height: usize,
    half_order: ImageOrder,
) {
    let mut length = roundup(linelen, 4);

    let mut swap_operation = |index: usize, source: &[u8], dest: &mut [u8]| {
        if index == height - 1 && linelen != length {
            length -= 4;

            if let ImageOrder::MsbFirst = half_order {
                dest[length + 1] = REVERSE_BYTES[source[length + 3] as usize];
            }
            if (half_order == ImageOrder::LsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::MsbFirst && linelen & 2 != 0)
            {
                dest[length] = REVERSE_BYTES[source[length + 2] as usize];
            }
            if (half_order == ImageOrder::MsbFirst && linelen - length == 3)
                || (half_order == ImageOrder::LsbFirst && linelen & 2 != 0)
            {
                dest[length + 3] = REVERSE_BYTES[source[length + 1] as usize];
            }
            if let ImageOrder::LsbFirst = half_order {
                dest[length + 2] = REVERSE_BYTES[source[length] as usize];
            }
        }

        dest.chunks_mut(4)
            .take(length / 4)
            .zip(source.chunks(4).take(length / 4))
            .for_each(|(d, s)| {
                d[0] = REVERSE_BYTES[s[2] as usize];
                d[1] = REVERSE_BYTES[s[3] as usize];
                d[2] = REVERSE_BYTES[s[0] as usize];
                d[3] = REVERSE_BYTES[s[1] as usize];
            });
    };

    dest.chunks_mut(destinc)
        .zip(source.chunks(srcinc))
        .enumerate()
        .for_each(|(i, (d, s))| swap_operation(i, s, d));
}

/// A function to iterate over a set of bytes and apply some kind of swapping operation to them.
pub(crate) type SwapFunction =
    &'static dyn Fn(&[u8], &mut [u8], usize, usize, usize, usize, ImageOrder);

/// Module so it doesn't pollute the namespace.
mod swap_function {
    #![allow(non_upper_case_globals)]

    use super::SwapFunction;

    const n: SwapFunction = &super::no_swap;
    const s: SwapFunction = &super::swap_two_bytes;
    const l: SwapFunction = &super::swap_four_bytes;
    const w: SwapFunction = &super::swap_words;
    const R: SwapFunction = &super::swap_bits;
    const S: SwapFunction = &super::swap_bits_and_two_bytes;
    const L: SwapFunction = &super::swap_bits_and_four_bytes;
    const W: SwapFunction = &super::swap_bits_and_words;

    // Copied verbatim from Xlib:
    /*
    The following table gives the bit ordering within bytes (when accessed
    sequentially) for a scanline containing 32 bits, with bits numbered 0 to
    31, where bit 0 should be leftmost on the display.  For a given byte
    labelled A-B, A is for the most significant bit of the byte, and B is
    for the least significant bit.
    legend:
        1   scanline-unit = 8
        2   scanline-unit = 16
        4   scanline-unit = 32
        M   byte-order = MostSignificant
        L,  byte-order = LeastSignificant
        m   bit-order = MostSignificant
        l,  bit-order = LeastSignificant
    format	ordering
    1Mm	00-07 08-15 16-23 24-31
    2Mm	00-07 08-15 16-23 24-31
    4Mm	00-07 08-15 16-23 24-31
    1Ml	07-00 15-08 23-16 31-24
    2Ml	15-08 07-00 31-24 23-16
    4Ml	31-24 23-16 15-08 07-00
    1Lm	00-07 08-15 16-23 24-31
    2Lm	08-15 00-07 24-31 16-23
    4Lm	24-31 16-23 08-15 00-07
    1Ll	07-00 15-08 23-16 31-24
    2Ll	07-00 15-08 23-16 31-24
    4Ll	07-00 15-08 23-16 31-24
    The following table gives the required conversion between any two
    formats.  It is based strictly on the table above.  If you believe one,
    you should believe the other.
    legend:
        n,  no changes
        s,  reverse 8-bit units within 16-bit units
        l,  reverse 8-bit units within 32-bit units
        w,  reverse 16-bit units within 32-bit units
        R,  reverse bits within 8-bit units
        S,  s+R
        L,  l+R
        W,  w+R
    */

    #[rustfmt::skip]
    pub(crate) const SWAP_FUNCTION_TABLE: [[SwapFunction; 12]; 12] = [
        /*         1Mm 2Mm 4Mm 1Ml 2Ml 4Ml 1Lm 2Lm 4Lm 1Ll 2Ll 4Ll,  */
        /* 1Mm */ [n, n, n, R, S, L, n, s, l, R, R, R],
        /* 2Mm */ [n, n, n, R, S, L, n, s, l, R, R, R],
        /* 4Mm */ [n, n, n, R, S, L, n, s, l, R, R, R],
        /* 1Ml */ [R, R, R, n, s, l, R, S, L, n, n, n],
        /* 2Ml */ [S, S, S, s, n, w, S, R, W, s, s, s],
        /* 4Ml */ [L, L, L, l, w, n, L, W, R, l, l, l],
        /* 1Lm */ [n, n, n, R, S, L, n, s, l, R, R, R],
        /* 2Lm */ [s, s, s, S, R, W, s, n, w, S, S, S],
        /* 4Lm */ [l, l, l, L, W, R, l, w, n, L, L, L],
        /* 1Ll */ [R, R, R, n, s, l, R, S, L, n, n, n],
        /* 2Ll */ [R, R, R, n, s, l, R, S, L, n, n, n],
        /* 4Ll */ [R, R, R, n, s, l, R, S, L, n, n, n],
    ];

    use crate::auto::xproto::ImageOrder::{self, LsbFirst, MsbFirst};

    // We also need to factor in the order of the source data.
    pub(crate) const HALF_ORDER_TABLE: [ImageOrder; 12] = [
        LsbFirst, LsbFirst, LsbFirst, LsbFirst, MsbFirst, MsbFirst, LsbFirst, MsbFirst, MsbFirst,
        LsbFirst, LsbFirst, LsbFirst,
    ];

    pub(crate) const HALF_WORD_TABLE: [ImageOrder; 12] = [
        MsbFirst, MsbFirst, MsbFirst, MsbFirst, MsbFirst, LsbFirst, MsbFirst, MsbFirst, LsbFirst,
        MsbFirst, MsbFirst, MsbFirst,
    ];
}

pub(crate) use swap_function::{HALF_ORDER_TABLE, HALF_WORD_TABLE, SWAP_FUNCTION_TABLE};

/// Given a set of properties, create a value that can index into the swap function table
/// above.
#[const_fn::const_fn("1.47")]
#[inline]
pub(crate) const fn swap_function_table_index(
    quantum: u32,
    bit_order: ImageOrder,
    byte_order: ImageOrder,
) -> usize {
    (match quantum {
        32 => 2,
        16 => 1,
        _ => 0,
    }) + (match bit_order {
        ImageOrder::MsbFirst => 0,
        ImageOrder::LsbFirst => 3,
    }) + (match byte_order {
        ImageOrder::MsbFirst => 0,
        ImageOrder::LsbFirst => 6,
    })
}
