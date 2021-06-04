// MIT/Apache2 License

// MIT/Apache2 License

use super::{
    fit::{
        no_swap, shift_nibbles_left, swap_four_bytes, swap_function_table_index, swap_nibble,
        swap_three_bytes, swap_two_bytes, HALF_ORDER_TABLE, HALF_WORD_TABLE, SWAP_FUNCTION_TABLE,
    },
    GenericImage, Image,
};
use crate::{
    auto::{
        xproto::{Drawable, Gcontext, ImageFormat, ImageOrder, PutImageRequest},
        AsByteSequence,
    },
    display::DisplayBase,
    util::roundup,
};
use alloc::{borrow::Cow, boxed::Box, vec, vec::Vec};
use core::{
    convert::{TryFrom, TryInto},
    iter,
    ops::Deref,
    ptr,
};

/// Prepare a request for an XY image.
fn prepare_xy_image<Dpy: DisplayBase + ?Sized, Data: Deref<Target = [u8]>>(
    dpy: &mut Dpy,
    req: &mut PutImageRequest,
    image: &Image<Data>,
    src_x: usize,
    src_y: usize,
) {
    let total_xoffset = image.x_offset() as usize + src_x;
    req.left_pad = (total_xoffset & (dpy.setup().bitmap_format_scanline_unit as usize - 1)) as u8;
    let total_xoffset = (total_xoffset - req.left_pad as usize) >> 3;

    if req.left_pad != 0 && req.format == ImageFormat::ZPixmap {
        req.format = ImageFormat::XyPixmap;
    }

    let bytes_per_dest = roundup(
        req.width as usize + req.left_pad as usize,
        dpy.setup().bitmap_format_scanline_pad as usize,
    ) >> 3;
    let bytes_per_dest_plane = bytes_per_dest * req.height as usize;

    let sft_index = swap_function_table_index(
        u32::from(image.bitmap_unit()),
        image.bit_order(),
        image.byte_order(),
    );
    let swap_function = SWAP_FUNCTION_TABLE[sft_index][swap_function_table_index(
        u32::from(dpy.setup().bitmap_format_scanline_unit),
        dpy.setup().bitmap_format_bit_order,
        dpy.setup().image_byte_order,
    )];
    let mut half_order = HALF_ORDER_TABLE[sft_index];
    if let ImageOrder::MsbFirst = half_order {
        half_order = HALF_WORD_TABLE[sft_index];
    }

    let src_data = &image.data()[(image.bytes_per_line() * src_y) + total_xoffset..];

    // if we don't need to preform any modifications to the data, just use to_vec() and set it
    if ptr::eq(
        swap_function as *const _ as *const (),
        &no_swap as *const _ as *const (),
    ) && image.bytes_per_line() == bytes_per_dest
        && (total_xoffset == 0 && (image.depth() == 1 || image.height() == req.height as usize))
        || (image.depth() == 1 && (src_y + req.height as usize) < image.height())
    {
        req.data = src_data.to_vec();
        return;
    }

    let length = roundup(bytes_per_dest_plane * image.depth() as usize, 4);
    // allocate a vector for it
    let mut buffer: Vec<u8> = iter::repeat(0).take(length).collect();

    if total_xoffset > 0 && image.byte_order() != image.bit_order() {
        unimplemented!();
    }

    let bytes_per_src = (req.width as usize + req.left_pad as usize + 7) >> 3;
    let bytes_per_line = image.bytes_per_line();
    let bytes_per_src_plane = bytes_per_src * image.height() as usize;

    buffer
        .chunks_mut(bytes_per_dest_plane)
        .zip(src_data.chunks(bytes_per_src_plane))
        .for_each(|(d, s)| {
            (swap_function)(
                s,
                d,
                bytes_per_src as usize,
                bytes_per_line,
                bytes_per_dest,
                req.height as usize,
                half_order,
            );
        });

    req.data = buffer;
}

/// Prepare a request with a `ZImage`
#[inline]
fn prepare_z_image<Dpy: DisplayBase + ?Sized, Data: Deref<Target = [u8]>>(
    dpy: &mut Dpy,
    req: &mut PutImageRequest,
    image: &Image<Data>,
    src_x: usize,
    src_y: usize,
    dest_bits_per_pixel: usize,
    dest_scanline_pad: usize,
) {
    req.left_pad = 0;
    let bytes_per_src = roundup(req.width as usize * image.bits_per_pixel() as usize, 8) >> 3;
    let bytes_per_dest = roundup(
        req.width as usize * dest_bits_per_pixel,
        dest_scanline_pad as usize,
    ) >> 3;

    let mut length = bytes_per_dest * req.height as usize;

    let mut src_data = Cow::Borrowed::<[u8]>(
        &image.data()
            [(src_y * image.bytes_per_line()) + ((src_x * image.bits_per_pixel() as usize) >> 3)..],
    );
    if image.bits_per_pixel() == 4 && src_x & 1 != 0 {
        let mut shifted: Vec<u8> = iter::repeat(0).take(src_data.len()).collect();
        shift_nibbles_left(
            &src_data,
            &mut shifted,
            bytes_per_src,
            image.bytes_per_line(),
            image.bytes_per_line(),
            req.height as usize,
            image.byte_order(),
        );
        src_data = Cow::Owned(shifted);
    }

    // we may be alright with our current set
    if (image.byte_order() == dpy.setup().image_byte_order || image.bits_per_pixel() == 8)
        && image.bytes_per_line() == bytes_per_dest
        && (src_x == 0 || (src_y + req.height as usize) < image.height())
    {
        req.data = match src_data {
            Cow::Borrowed(src_data) => src_data[..length].to_vec(),
            Cow::Owned(mut src_data) => {
                src_data.truncate(length);
                src_data
            }
        };

        return;
    }

    // determine what kind of shifts we need to do
    length = roundup(bytes_per_dest * req.height as usize, 4);
    let mut buffer: Vec<u8> = iter::repeat(0).take(length).collect();
    if image.byte_order() == dpy.setup().image_byte_order || image.bits_per_pixel() == 8 {
        no_swap(
            &src_data,
            &mut buffer,
            bytes_per_src,
            image.bytes_per_line(),
            bytes_per_dest,
            req.height as usize,
            image.byte_order(),
        );
    } else {
        match image.bits_per_pixel() {
            32 => swap_four_bytes(
                &src_data,
                &mut buffer,
                bytes_per_src,
                image.bytes_per_line(),
                bytes_per_dest,
                req.height as usize,
                image.byte_order(),
            ),
            24 => swap_three_bytes(
                &src_data,
                &mut buffer,
                bytes_per_src,
                image.bytes_per_line(),
                bytes_per_dest,
                req.height as usize,
                image.byte_order(),
            ),
            16 => swap_two_bytes(
                &src_data,
                &mut buffer,
                bytes_per_src,
                image.bytes_per_line(),
                bytes_per_dest,
                req.height as usize,
                image.byte_order(),
            ),
            _ => swap_nibble(
                &src_data,
                &mut buffer,
                bytes_per_src,
                image.bytes_per_line(),
                bytes_per_dest,
                req.height as usize,
                image.byte_order(),
            ),
        }
    }

    req.data = buffer;
}

/// Generate a series of requests for a sub-part of an image.
#[allow(clippy::too_many_lines)] // TODO: find good way to split
#[inline]
fn put_sub_image_req<Dpy: DisplayBase + ?Sized, Data: Deref<Target = [u8]>>(
    dpy: &mut Dpy,
    drawable: Drawable,
    gc: Gcontext,
    image: &Image<Data>,
    src_x: usize,
    src_y: usize,
    dst_x: isize,
    dst_y: isize,
    width: usize,
    height: usize,
    dest_bits_per_pixel: usize,
    dest_scanline_pad: usize,
) -> Vec<PutImageRequest> {
    // Base Case: Width and height are zero. We can just return an empty vector.
    if width == 0 || height == 0 {
        return vec![];
    }

    // How many bits are available to use.
    let mut req: PutImageRequest = Default::default();

    #[allow(unused_comparisons, clippy::absurd_extreme_comparisons)]
    let available: usize = if dpy.setup().maximum_request_length > 65535 {
        65536 << 2
    } else {
        (dpy.setup().maximum_request_length as usize) << 2
    } - req.size();

    let (left_pad, bytes_per_row) =
        if image.bits_per_pixel() == 1 || image.format() != ImageFormat::ZPixmap {
            let left_pad =
                (image.x_offset() + src_x) & (dpy.setup().bitmap_format_scanline_unit as usize - 1);
            (
                left_pad,
                (roundup(
                    width + left_pad,
                    dpy.setup().bitmap_format_scanline_pad as usize,
                ) >> 3)
                    * image.depth() as usize,
            )
        } else {
            (
                0,
                roundup(width * dest_bits_per_pixel, dest_scanline_pad) >> 3,
            )
        };

    // If we can fit our image in the available bits, create and return the image request.
    if bytes_per_row * height <= available {
        req.drawable = drawable;
        req.gc = gc;
        req.dst_x = dst_x as _;
        req.dst_y = dst_y as _;
        req.width = width as _;
        req.height = height as _;
        req.depth = image.depth();
        req.format = image.format();

        if image.bits_per_pixel() == 1 || image.format() != ImageFormat::ZPixmap {
            prepare_xy_image(dpy, &mut req, image, src_x, src_y);
        } else {
            prepare_z_image(
                dpy,
                &mut req,
                image,
                src_x,
                src_y,
                dest_bits_per_pixel,
                dest_scanline_pad,
            );
        }

        vec![req]
    } else if height > 1 {
        // figure out how many rows we have
        let mut sub_image_height = available / bytes_per_row;
        if sub_image_height == 0 {
            sub_image_height = 1;
        }

        // divide and conquer into two images based on rows
        let mut reqs = put_sub_image_req(
            dpy,
            drawable,
            gc,
            image,
            src_x,
            src_y,
            dst_x,
            dst_y,
            width,
            sub_image_height,
            dest_bits_per_pixel,
            dest_scanline_pad,
        );
        reqs.extend(put_sub_image_req(
            dpy,
            drawable,
            gc,
            image,
            src_x,
            src_y + sub_image_height,
            dst_x,
            dst_y + sub_image_height as isize,
            width,
            height - sub_image_height,
            dest_bits_per_pixel,
            dest_scanline_pad,
        ));
        reqs
    } else {
        // we've already divided and conquered down to 1 row. divide based on width
        let sub_image_width =
            (((available << 3) / dest_scanline_pad) * dest_scanline_pad) - left_pad;

        let mut reqs = put_sub_image_req(
            dpy,
            drawable,
            gc,
            image,
            src_x,
            src_y,
            dst_x,
            dst_y,
            sub_image_width,
            1,
            dest_bits_per_pixel,
            dest_scanline_pad,
        );
        reqs.extend(put_sub_image_req(
            dpy,
            drawable,
            gc,
            image,
            src_x + sub_image_width,
            src_y,
            dst_x + sub_image_width as isize,
            dst_y,
            width - sub_image_width,
            1,
            dest_bits_per_pixel,
            dest_scanline_pad,
        ));
        reqs
    }
}

/// Begins the recursion for the image requests.
#[inline]
pub(crate) fn put_image_req<Dpy: DisplayBase + ?Sized, Data: Deref<Target = [u8]>>(
    dpy: &mut Dpy,
    drawable: Drawable,
    gc: Gcontext,
    image: &Image<Data>,
    src_x: isize,
    src_y: isize,
    dest_x: isize,
    dest_y: isize,
    mut width: usize,
    mut height: usize,
) -> Vec<PutImageRequest> {
    let src_x: usize = match src_x {
        x_offset if x_offset < 0 => {
            width = width.saturating_sub(usize::try_from(-x_offset).unwrap());
            0
        }
        src_x => src_x.try_into().unwrap(),
    };

    let src_y: usize = match src_y {
        y_offset if y_offset < 0 => {
            height = height.saturating_sub(usize::try_from(-y_offset).unwrap());
            0
        }
        src_y => src_y.try_into().unwrap(),
    };

    if src_x + width > image.width() {
        width = image.width().saturating_sub(src_x);
    }
    if src_y + width > image.height() {
        height = image.height().saturating_sub(src_y);
    }

    if width == 0 || height == 0 {
        log::error!("Width and height of zero, no drawing could be done.");
        return vec![]; // no drawing could be done
    }

    let (dest_bits_per_pixel, dest_scanline_pad) =
        if image.bits_per_pixel() == 1 || image.format() != ImageFormat::ZPixmap {
            (1_usize, dpy.setup().bitmap_format_scanline_pad as usize)
        } else {
            let mut dest_bits_per_pixel = image.bits_per_pixel() as usize;
            let mut dest_scanline_pad = image.bitmap_pad() as usize;

            // scan the display's formats
            dpy.setup().pixmap_formats.iter().for_each(|f| {
                if f.depth == image.depth() {
                    dest_bits_per_pixel = f.bits_per_pixel as usize;
                    dest_scanline_pad = f.scanline_pad as usize;
                }
            });

            if dest_bits_per_pixel != image.bits_per_pixel() as _ {
                let mut new_image = Image {
                    width,
                    height,
                    x_offset: 0,
                    format: ImageFormat::ZPixmap,
                    byte_order: dpy.setup().image_byte_order,
                    bitmap_unit: dpy.setup().bitmap_format_scanline_unit,
                    bit_order: dpy.setup().bitmap_format_bit_order,
                    bitmap_pad: dest_scanline_pad as _,
                    bits_per_pixel: dest_bits_per_pixel as _,
                    depth: image.depth(),
                    red_mask: 0,
                    green_mask: 0,
                    blue_mask: 0,
                    bytes_per_line: roundup(dest_bits_per_pixel * width, dest_scanline_pad) >> 3,
                    data: iter::repeat(0)
                        .take(height * image.bytes_per_line())
                        .collect::<Box<[u8]>>(),
                };

                for j in 0..height {
                    for i in 0..width {
                        new_image.set_pixel(i, j, image.pixel(src_x + i, src_y + j));
                    }
                }

                // put the new image and return
                return put_sub_image_req(
                    dpy,
                    drawable,
                    gc,
                    &new_image,
                    0,
                    0,
                    dest_x,
                    dest_y,
                    width,
                    height,
                    dest_bits_per_pixel,
                    dest_scanline_pad,
                );
            }
            (dest_bits_per_pixel, dest_scanline_pad)
        };

    put_sub_image_req(
        dpy,
        drawable,
        gc,
        image,
        src_x,
        src_y,
        dest_x,
        dest_y,
        width,
        height,
        dest_bits_per_pixel,
        dest_scanline_pad,
    )
}
