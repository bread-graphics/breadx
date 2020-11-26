// MIT/Apache2 License

// MIT/Apache2 License

use super::{
    fit::{
        no_swap, shift_nibbles_left, swap_four_bytes, swap_function_table_index, swap_nibbles,
        swap_three_bytes, swap_two_bytes, HALF_ORDER_TABLE, HALF_WORD_TABLE, SWAP_FUNCTION_TABLE,
    },
    GenericImage, Image,
};
use crate::{
    auto::xproto::{Drawable, Gcontext, ImageFormat, PutImageRequest},
    display::{Connection, Display},
    util::roundup,
};
use alloc::{borrow::Cow, vec, vec::Vec};
use core::{iter, ops::Deref};

/// Prepare a request for an XY image.
fn prepare_xy_image<Conn: Connection, Data: Deref<Target = [u8]>>(
    dpy: &mut Display<Conn>,
    req: &mut PutImageRequest,
    image: &Image<Data>,
    src_x: usize,
    src_y: usize,
) {
    let total_xoffset = image.x_offset() + src_x;
    req.left_pad = total_xoffset & (dpy.setup().bitmap_format_scanline_unit - 1);
    let mut total_xoffset = (total_xoffset - req.left_pad) >> 3;

    if req.left_pad != 0 && req.format == ImageFormat::ZPixmap {
        req.format = ImageFormat::XyPixmap;
    }

    let bytes_per_dest = roundup(
        req.width + req.left_pad,
        dpy.setup().bitmap_format_scanline_pad,
    ) >> 3;
    let bytes_per_dest_plane = bytes_per_dest * req.height;

    let sft_index =
        swap_function_table_index(image.bitmap_unit(), image.bit_order(), image.byte_order());
    let swap_function = SWAP_FUNCTION_TABLE[sft_index][swap_function_table_index(
        dpy.setup().bitmap_format_scanline_unit,
        dpy.setup().bitmap_format_bit_order,
        dpy.setup().image_byte_order,
    )];
    let mut half_order = HALF_ORDER_TABLE[sft_index];
    if let ImageOrder::MsbFirst = half_order {
        half_order = HALF_WORD_TABLE[sft_index];
    }

    src_data = &image.data()[(image.bytes_per_line() * src_y) + total_xoffset];

    // if we don't need to preform any modifications to the data, just use to_vec() and set it
    if swap_function as *const _ == no_swap as *const _
        && image.bytes_per_line() == bytes_per_dest
        && (total_xoffset == 0 && (image.depth() == 1 || image.height() == req.height))
        || (image.depth() == 1 && (src_y + req.height) < image.height())
    {
        req.data = src_data.to_vec();
        return;
    }

    let length = roundup(bytes_per_dest_plane * image.depth(), 4);
    // allocate a vector for it
    let mut buffer: Vec<u8> = iter::cycle(0).take(length).collect();

    if total_xoffset > 0 && image.byte_order() != image.bit_order() {
        unimplemented!();
    }

    let bytes_per_src = (req.width + req.left_pad + 7) >> 3;
    let bytes_per_line = image.bytes_per_line();
    let bytes_per_src_plane = bytes_per_src * image.height();
    total_xoffset &= (image.bitmap_unit() - 1) >> 3;

    buffer
        .chunks_mut(bytes_per_dest_plane)
        .zip(src_data.chunks(bytes_per_src_plane))
        .for_each(|(d, s)| {
            (swap_function)(
                s,
                d,
                bytes_per_src,
                bytes_per_line,
                bytes_per_dest,
                req.height,
                half_order,
            );
        });

    req.data = buffer;
}

/// Prepare a request with a ZImage
#[inline]
fn prepare_z_image<Conn: Connection, Data: Deref<Target = [u8]>>(
    dpy: &mut Display<Conn>,
    req: &mut PutImageRequest,
    image: &Image<Data>,
    src_x: usize,
    src_y: usize,
    dest_bits_per_pixel: usize,
    dest_scanline_pad: usize,
) {
    req.left_pad = 0;
    let bytes_per_src = roundup(req.width * image.bits_per_pixel(), 8) >> 3;
    let bytes_per_dest = roundup(req.width * dest_bits_per_pixel, dest_scanline_pad) >> 3;

    let mut src_data = Cow::Borrowed(
        &image.data()[(src_y * image.bytes_per_line()) + ((src_x * image.bits_per_pixel()) >> 3)..],
    );
    if image.bits_per_pixel() == 4 && src_x & 1 != 0 {
        let mut shifted = iter::cycle(0).take(src_data.len()).collect();
        shift_nibbles_left(
            src_data,
            &mut shifted,
            bytes_per_src,
            image.bytes_per_line(),
            image.bytes_per_line(),
            req.height,
            image.byte_order(),
        );
        src_data = Cow::Owned(shifted);
    }

    // we may be alright with our current set
    if (image.byte_order() == dpy.setup().image_byte_order || image.bits_per_pixel() == 8)
        && image.bytes_per_line() == bytes_per_dest
        && (src_x == 0 || (src_y + req.height) < image.height())
    {
        req.data = match src_data {
            Cow::Borrowed(src_data) => src_data.to_vec(),
            Cow::Owned(src_data) => src_data,
        };
    }

    // determine what kind of shifts we need to do
    let length = roundup(bytes_per_dest * req.height, 4);
    let mut buffer: Vec<u8> = iter::cycle(0).take(length).collect();

    if image.byte_order() == dpy.setup().image_byte_order || image.bits_per_pixel() == 8 {
        no_swap(
            &src_data,
            &mut buffer,
            bytes_per_src,
            image.bytes_per_line(),
            bytes_per_dest,
            req.height,
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
                req.height,
                image.byte_order(),
            ),
            24 => swap_three_bytes(
                &src_data,
                &mut buffer,
                bytes_per_src,
                image.bytes_per_line(),
                bytes_per_dest,
                req.height,
                image.byte_order(),
            ),
            16 => swap_two_bytes(
                &src_data,
                &mut buffer,
                bytes_per_src,
                image.bytes_per_line(),
                bytes_per_dest,
                req.height,
                image.byte_order(),
            ),
            _ => swap_nibles(
                &src_data,
                &mut buffer,
                bytes_per_src,
                image.bytes_per_line(),
                bytes_per_dest,
                req.height,
                image.byte_order(),
            ),
        }
    }

    req.data = buffer;
}

/// Generate a series of requests for a sub-part of an image.
#[inline]
fn put_sub_image_req<Conn: Connection, Data: Deref<Target = [u8]>>(
    dpy: &mut Display<Conn>,
    drawable: Drawable,
    gc: Gcontext,
    image: &Image<Data>,
    src_x: usize,
    src_y: usize,
    dst_x: usize,
    dst_y: usize,
    width: usize,
    height: usize,
    dest_bits_per_pixel: usize,
    dest_scanline_pad: usize,
) -> Vec<PutImageReq> {
    // Base Case: Width and height are zero. We can just return an empty vector.
    if width == 0 || height == 0 {
        return vec![];
    }

    // How many bits are available to use.
    let mut req: PutImageReq = Default::default();
    let available: usize = if dpy.setup().max_request_size > 65535 {
        65536 << 2
    } else {
        dpy.max_request_size << 2
    } - req.size();

    let (left_pad, bytes_per_row) = if image.bits_per_pixel() == 1
        || image.format() != ImageFormat::ZPixmap
    {
        let left_pad = (image.x_offset() + src_x) & (dpy.setup().bitmap_format_scanline_unit - 1);
        (
            left_pad,
            (roundup(width + left_pad, dpy.setup().bitmap_format_scanline_pad) >> 3)
                * image.depth(),
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
pub(crate) fn put_image<Conn: Connection, Data: Deref<Target = [u8]>>(
    dpy: &mut Display<Conn>,
    drawable: Drawable,
    gc: Gcontext,
    image: &Image<Data>,
    src_x: usize,
    src_y: usize,
    dst_x: usize,
    dst_y: usize,
    width: usize,
    height: usize,
) -> Vec<PutImageReq> {
    unimplemented!()
}
