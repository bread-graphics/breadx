// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

#[derive(Clone, Debug, Default)]
pub struct Char2b {
    pub byte1: Card8,
    pub byte2: Card8,
}
impl Char2b {}
impl AsByteSequence for Char2b {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.byte1.as_bytes(&mut bytes[index..]);
        index += self.byte2.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Char2b from byte buffer");
        let (byte1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (byte2, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Char2b {
                byte1: byte1,
                byte2: byte2,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.byte1.size() + self.byte2.size()
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Window {
    pub xid: XID,
}
impl Window {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Window {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pixmap {
    pub xid: XID,
}
impl Pixmap {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Pixmap {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cursor {
    pub xid: XID,
}
impl Cursor {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Cursor {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Font {
    pub xid: XID,
}
impl Font {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Font {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gcontext {
    pub xid: XID,
}
impl Gcontext {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Gcontext {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Colormap {
    pub xid: XID,
}
impl Colormap {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Colormap {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Atom {
    pub xid: XID,
}
impl Atom {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Atom {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Drawable {
    pub xid: XID,
}
impl Drawable {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Drawable {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl From<Window> for Drawable {
    #[inline]
    fn from(base: Window) -> Self {
        <Drawable>::const_from_xid(base.xid)
    }
}
impl From<Pixmap> for Drawable {
    #[inline]
    fn from(base: Pixmap) -> Self {
        <Drawable>::const_from_xid(base.xid)
    }
}
#[repr(transparent)]
#[derive(Default, Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Fontable {
    pub xid: XID,
}
impl Fontable {
    #[inline]
    pub const fn const_from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl XidType for Fontable {
    #[inline]
    fn xid(&self) -> XID {
        self.xid
    }
    #[inline]
    fn from_xid(xid: XID) -> Self {
        Self { xid: xid }
    }
}
impl From<Font> for Fontable {
    #[inline]
    fn from(base: Font) -> Self {
        <Fontable>::const_from_xid(base.xid)
    }
}
impl From<Gcontext> for Fontable {
    #[inline]
    fn from(base: Gcontext) -> Self {
        <Fontable>::const_from_xid(base.xid)
    }
}
pub type Bool32 = Card32;
pub type Visualid = Card32;
pub type Timestamp = Card32;
pub type Keysym = Card32;
pub type Keycode = Card8;
pub type Keycode32 = Card32;
pub type Button = Card8;
#[derive(Clone, Debug, Default)]
pub struct Point {
    pub x: Int16,
    pub y: Int16,
}
impl Point {}
impl AsByteSequence for Point {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Point from byte buffer");
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((Point { x: x, y: y }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.x.size() + self.y.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Rectangle {
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
}
impl Rectangle {}
impl AsByteSequence for Rectangle {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Rectangle from byte buffer");
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Rectangle {
                x: x,
                y: y,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.x.size() + self.y.size() + self.width.size() + self.height.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Arc {
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub angle1: Int16,
    pub angle2: Int16,
}
impl Arc {}
impl AsByteSequence for Arc {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.angle1.as_bytes(&mut bytes[index..]);
        index += self.angle2.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Arc from byte buffer");
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (angle1, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (angle2, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Arc {
                x: x,
                y: y,
                width: width,
                height: height,
                angle1: angle1,
                angle2: angle2,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.angle1.size()
            + self.angle2.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Format {
    pub depth: Card8,
    pub bits_per_pixel: Card8,
    pub scanline_pad: Card8,
}
impl Format {}
impl AsByteSequence for Format {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.bits_per_pixel.as_bytes(&mut bytes[index..]);
        index += self.scanline_pad.as_bytes(&mut bytes[index..]);
        index += 5;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Format from byte buffer");
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bits_per_pixel, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (scanline_pad, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 5;
        Some((
            Format {
                depth: depth,
                bits_per_pixel: bits_per_pixel,
                scanline_pad: scanline_pad,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.depth.size() + self.bits_per_pixel.size() + self.scanline_pad.size() + 5
    }
}
#[derive(Clone, Debug, Default)]
pub struct Visualtype {
    pub visual_id: Visualid,
    pub class: VisualClass,
    pub bits_per_rgb_value: Card8,
    pub colormap_entries: Card16,
    pub red_mask: Card32,
    pub green_mask: Card32,
    pub blue_mask: Card32,
}
impl Visualtype {}
impl AsByteSequence for Visualtype {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.visual_id.as_bytes(&mut bytes[index..]);
        index += self.class.as_bytes(&mut bytes[index..]);
        index += self.bits_per_rgb_value.as_bytes(&mut bytes[index..]);
        index += self.colormap_entries.as_bytes(&mut bytes[index..]);
        index += self.red_mask.as_bytes(&mut bytes[index..]);
        index += self.green_mask.as_bytes(&mut bytes[index..]);
        index += self.blue_mask.as_bytes(&mut bytes[index..]);
        index += 4;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Visualtype from byte buffer");
        let (visual_id, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (class, sz): (VisualClass, usize) = <VisualClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bits_per_rgb_value, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (colormap_entries, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        Some((
            Visualtype {
                visual_id: visual_id,
                class: class,
                bits_per_rgb_value: bits_per_rgb_value,
                colormap_entries: colormap_entries,
                red_mask: red_mask,
                green_mask: green_mask,
                blue_mask: blue_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.visual_id.size()
            + self.class.size()
            + self.bits_per_rgb_value.size()
            + self.colormap_entries.size()
            + self.red_mask.size()
            + self.green_mask.size()
            + self.blue_mask.size()
            + 4
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum VisualClass {
    StaticGray = 0,
    GrayScale = 1,
    StaticColor = 2,
    PseudoColor = 3,
    TrueColor = 4,
    DirectColor = 5,
}
impl AsByteSequence for VisualClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::StaticGray, sz)),
            1 => Some((Self::GrayScale, sz)),
            2 => Some((Self::StaticColor, sz)),
            3 => Some((Self::PseudoColor, sz)),
            4 => Some((Self::TrueColor, sz)),
            5 => Some((Self::DirectColor, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for VisualClass {
    #[inline]
    fn default() -> VisualClass {
        VisualClass::StaticGray
    }
}
#[derive(Clone, Debug, Default)]
pub struct Depth {
    pub depth: Card8,
    pub visuals: Vec<Visualtype>,
}
impl Depth {}
impl AsByteSequence for Depth {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 1;
        index += (self.visuals.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.visuals, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Visualtype>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Depth from byte buffer");
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (visuals, block_len): (Vec<Visualtype>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Visualtype>());
        Some((
            Depth {
                depth: depth,
                visuals: visuals,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.depth.size() + 1 + ::core::mem::size_of::<Card16>() + 4 + {
            let block_len: usize = self.visuals.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Visualtype>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Screen {
    pub root: Window,
    pub default_colormap: Colormap,
    pub white_pixel: Card32,
    pub black_pixel: Card32,
    pub current_input_masks: EventMask,
    pub width_in_pixels: Card16,
    pub height_in_pixels: Card16,
    pub width_in_millimeters: Card16,
    pub height_in_millimeters: Card16,
    pub min_installed_maps: Card16,
    pub max_installed_maps: Card16,
    pub root_visual: Visualid,
    pub backing_stores: BackingStore,
    pub save_unders: bool,
    pub root_depth: Card8,
    pub allowed_depths: Vec<Depth>,
}
impl Screen {}
impl AsByteSequence for Screen {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.default_colormap.as_bytes(&mut bytes[index..]);
        index += self.white_pixel.as_bytes(&mut bytes[index..]);
        index += self.black_pixel.as_bytes(&mut bytes[index..]);
        index += self.current_input_masks.as_bytes(&mut bytes[index..]);
        index += self.width_in_pixels.as_bytes(&mut bytes[index..]);
        index += self.height_in_pixels.as_bytes(&mut bytes[index..]);
        index += self.width_in_millimeters.as_bytes(&mut bytes[index..]);
        index += self.height_in_millimeters.as_bytes(&mut bytes[index..]);
        index += self.min_installed_maps.as_bytes(&mut bytes[index..]);
        index += self.max_installed_maps.as_bytes(&mut bytes[index..]);
        index += self.root_visual.as_bytes(&mut bytes[index..]);
        index += self.backing_stores.as_bytes(&mut bytes[index..]);
        index += self.save_unders.as_bytes(&mut bytes[index..]);
        index += self.root_depth.as_bytes(&mut bytes[index..]);
        index += (self.allowed_depths.len() as Card8).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.allowed_depths, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Depth>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Screen from byte buffer");
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (default_colormap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (white_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (black_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (current_input_masks, sz): (EventMask, usize) =
            <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width_in_pixels, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height_in_pixels, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width_in_millimeters, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height_in_millimeters, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_installed_maps, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_installed_maps, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (backing_stores, sz): (BackingStore, usize) =
            <BackingStore>::from_bytes(&bytes[index..])?;
        index += sz;
        let (save_unders, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (allowed_depths, block_len): (Vec<Depth>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Depth>());
        Some((
            Screen {
                root: root,
                default_colormap: default_colormap,
                white_pixel: white_pixel,
                black_pixel: black_pixel,
                current_input_masks: current_input_masks,
                width_in_pixels: width_in_pixels,
                height_in_pixels: height_in_pixels,
                width_in_millimeters: width_in_millimeters,
                height_in_millimeters: height_in_millimeters,
                min_installed_maps: min_installed_maps,
                max_installed_maps: max_installed_maps,
                root_visual: root_visual,
                backing_stores: backing_stores,
                save_unders: save_unders,
                root_depth: root_depth,
                allowed_depths: allowed_depths,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.root.size()
            + self.default_colormap.size()
            + self.white_pixel.size()
            + self.black_pixel.size()
            + self.current_input_masks.size()
            + self.width_in_pixels.size()
            + self.height_in_pixels.size()
            + self.width_in_millimeters.size()
            + self.height_in_millimeters.size()
            + self.min_installed_maps.size()
            + self.max_installed_maps.size()
            + self.root_visual.size()
            + self.backing_stores.size()
            + self.save_unders.size()
            + self.root_depth.size()
            + ::core::mem::size_of::<Card8>()
            + {
                let block_len: usize = self.allowed_depths.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Depth>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventMask {
    pub inner: u32,
}
impl EventMask {
    #[inline]
    pub fn key_press(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_key_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn key_release(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_key_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn button_press(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_button_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn button_release(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_button_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn enter_window(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_enter_window(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn leave_window(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_leave_window(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn pointer_motion(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_pointer_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn pointer_motion_hint(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_pointer_motion_hint(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn button1_motion(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_button1_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn button2_motion(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_button2_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn button3_motion(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_button3_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn button4_motion(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_button4_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn button5_motion(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_button5_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn button_motion(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_button_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn keymap_state(&self) -> bool {
        self.inner & (1 << 14) != 0
    }
    #[inline]
    pub fn set_keymap_state(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 14;
        } else {
            self.inner &= !(1 << 14);
        }
        self
    }
    #[inline]
    pub fn exposure(&self) -> bool {
        self.inner & (1 << 15) != 0
    }
    #[inline]
    pub fn set_exposure(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 15;
        } else {
            self.inner &= !(1 << 15);
        }
        self
    }
    #[inline]
    pub fn visibility_change(&self) -> bool {
        self.inner & (1 << 16) != 0
    }
    #[inline]
    pub fn set_visibility_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 16;
        } else {
            self.inner &= !(1 << 16);
        }
        self
    }
    #[inline]
    pub fn structure_notify(&self) -> bool {
        self.inner & (1 << 17) != 0
    }
    #[inline]
    pub fn set_structure_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 17;
        } else {
            self.inner &= !(1 << 17);
        }
        self
    }
    #[inline]
    pub fn resize_redirect(&self) -> bool {
        self.inner & (1 << 18) != 0
    }
    #[inline]
    pub fn set_resize_redirect(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 18;
        } else {
            self.inner &= !(1 << 18);
        }
        self
    }
    #[inline]
    pub fn substructure_notify(&self) -> bool {
        self.inner & (1 << 19) != 0
    }
    #[inline]
    pub fn set_substructure_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 19;
        } else {
            self.inner &= !(1 << 19);
        }
        self
    }
    #[inline]
    pub fn substructure_redirect(&self) -> bool {
        self.inner & (1 << 20) != 0
    }
    #[inline]
    pub fn set_substructure_redirect(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 20;
        } else {
            self.inner &= !(1 << 20);
        }
        self
    }
    #[inline]
    pub fn focus_change(&self) -> bool {
        self.inner & (1 << 21) != 0
    }
    #[inline]
    pub fn set_focus_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 21;
        } else {
            self.inner &= !(1 << 21);
        }
        self
    }
    #[inline]
    pub fn property_change(&self) -> bool {
        self.inner & (1 << 22) != 0
    }
    #[inline]
    pub fn set_property_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 22;
        } else {
            self.inner &= !(1 << 22);
        }
        self
    }
    #[inline]
    pub fn color_map_change(&self) -> bool {
        self.inner & (1 << 23) != 0
    }
    #[inline]
    pub fn set_color_map_change(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 23;
        } else {
            self.inner &= !(1 << 23);
        }
        self
    }
    #[inline]
    pub fn owner_grab_button(&self) -> bool {
        self.inner & (1 << 24) != 0
    }
    #[inline]
    pub fn set_owner_grab_button(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 24;
        } else {
            self.inner &= !(1 << 24);
        }
        self
    }
    #[inline]
    pub fn new(
        key_press: bool,
        key_release: bool,
        button_press: bool,
        button_release: bool,
        enter_window: bool,
        leave_window: bool,
        pointer_motion: bool,
        pointer_motion_hint: bool,
        button1_motion: bool,
        button2_motion: bool,
        button3_motion: bool,
        button4_motion: bool,
        button5_motion: bool,
        button_motion: bool,
        keymap_state: bool,
        exposure: bool,
        visibility_change: bool,
        structure_notify: bool,
        resize_redirect: bool,
        substructure_notify: bool,
        substructure_redirect: bool,
        focus_change: bool,
        property_change: bool,
        color_map_change: bool,
        owner_grab_button: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if key_press {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if key_release {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if button_press {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if button_release {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        if enter_window {
            inner |= 1 << 4;
        } else {
            inner &= !(1 << 4);
        }
        if leave_window {
            inner |= 1 << 5;
        } else {
            inner &= !(1 << 5);
        }
        if pointer_motion {
            inner |= 1 << 6;
        } else {
            inner &= !(1 << 6);
        }
        if pointer_motion_hint {
            inner |= 1 << 7;
        } else {
            inner &= !(1 << 7);
        }
        if button1_motion {
            inner |= 1 << 8;
        } else {
            inner &= !(1 << 8);
        }
        if button2_motion {
            inner |= 1 << 9;
        } else {
            inner &= !(1 << 9);
        }
        if button3_motion {
            inner |= 1 << 10;
        } else {
            inner &= !(1 << 10);
        }
        if button4_motion {
            inner |= 1 << 11;
        } else {
            inner &= !(1 << 11);
        }
        if button5_motion {
            inner |= 1 << 12;
        } else {
            inner &= !(1 << 12);
        }
        if button_motion {
            inner |= 1 << 13;
        } else {
            inner &= !(1 << 13);
        }
        if keymap_state {
            inner |= 1 << 14;
        } else {
            inner &= !(1 << 14);
        }
        if exposure {
            inner |= 1 << 15;
        } else {
            inner &= !(1 << 15);
        }
        if visibility_change {
            inner |= 1 << 16;
        } else {
            inner &= !(1 << 16);
        }
        if structure_notify {
            inner |= 1 << 17;
        } else {
            inner &= !(1 << 17);
        }
        if resize_redirect {
            inner |= 1 << 18;
        } else {
            inner &= !(1 << 18);
        }
        if substructure_notify {
            inner |= 1 << 19;
        } else {
            inner &= !(1 << 19);
        }
        if substructure_redirect {
            inner |= 1 << 20;
        } else {
            inner &= !(1 << 20);
        }
        if focus_change {
            inner |= 1 << 21;
        } else {
            inner &= !(1 << 21);
        }
        if property_change {
            inner |= 1 << 22;
        } else {
            inner &= !(1 << 22);
        }
        if color_map_change {
            inner |= 1 << 23;
        } else {
            inner &= !(1 << 23);
        }
        if owner_grab_button {
            inner |= 1 << 24;
        } else {
            inner &= !(1 << 24);
        }
        EventMask { inner: inner }
    }
}
impl AsByteSequence for EventMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((EventMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackingStore {
    NotUseful = 0,
    WhenMapped = 1,
    Always = 2,
}
impl AsByteSequence for BackingStore {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NotUseful, sz)),
            1 => Some((Self::WhenMapped, sz)),
            2 => Some((Self::Always, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for BackingStore {
    #[inline]
    fn default() -> BackingStore {
        BackingStore::NotUseful
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetupRequest {
    pub byte_order: Card8,
    pub protocol_major_version: Card16,
    pub protocol_minor_version: Card16,
    pub authorization_protocol_name: String,
    pub authorization_protocol_data: Vec<Card8>,
}
impl SetupRequest {}
impl AsByteSequence for SetupRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.byte_order.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.protocol_major_version.as_bytes(&mut bytes[index..]);
        index += self.protocol_minor_version.as_bytes(&mut bytes[index..]);
        index += (self.authorization_protocol_name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.authorization_protocol_data.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize =
            string_as_bytes(&self.authorization_protocol_name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize =
            vector_as_bytes(&self.authorization_protocol_data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetupRequest from byte buffer");
        let (byte_order, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (protocol_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (protocol_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (authorization_protocol_name, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (authorization_protocol_data, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            SetupRequest {
                byte_order: byte_order,
                protocol_major_version: protocol_major_version,
                protocol_minor_version: protocol_minor_version,
                authorization_protocol_name: authorization_protocol_name,
                authorization_protocol_data: authorization_protocol_data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.byte_order.size()
            + 1
            + self.protocol_major_version.size()
            + self.protocol_minor_version.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.authorization_protocol_name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self
                    .authorization_protocol_data
                    .iter()
                    .map(|i| i.size())
                    .sum();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetupFailed {
    pub status: Card8,
    pub protocol_major_version: Card16,
    pub protocol_minor_version: Card16,
    pub length: Card16,
    pub reason: String,
}
impl SetupFailed {}
impl AsByteSequence for SetupFailed {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.status.as_bytes(&mut bytes[index..]);
        index += (self.reason.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.protocol_major_version.as_bytes(&mut bytes[index..]);
        index += self.protocol_minor_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.reason, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetupFailed from byte buffer");
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (protocol_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (protocol_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reason, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetupFailed {
                status: status,
                protocol_major_version: protocol_major_version,
                protocol_minor_version: protocol_minor_version,
                length: length,
                reason: reason,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.status.size()
            + ::core::mem::size_of::<Card8>()
            + self.protocol_major_version.size()
            + self.protocol_minor_version.size()
            + self.length.size()
            + {
                let block_len: usize = self.reason.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetupAuthenticate {
    pub status: Card8,
    pub length: Card16,
    pub reason: String,
}
impl SetupAuthenticate {}
impl AsByteSequence for SetupAuthenticate {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 5;
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.reason, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetupAuthenticate from byte buffer");
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 5;
        let (length, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reason, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            SetupAuthenticate {
                status: status,
                length: length,
                reason: reason,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.status.size() + 5 + self.length.size() + {
            let block_len: usize = self.reason.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Setup {
    pub status: Card8,
    pub protocol_major_version: Card16,
    pub protocol_minor_version: Card16,
    pub length: Card16,
    pub release_number: Card32,
    pub resource_id_base: Card32,
    pub resource_id_mask: Card32,
    pub motion_buffer_size: Card32,
    pub maximum_request_length: Card16,
    pub image_byte_order: ImageOrder,
    pub bitmap_format_bit_order: ImageOrder,
    pub bitmap_format_scanline_unit: Card8,
    pub bitmap_format_scanline_pad: Card8,
    pub min_keycode: Keycode,
    pub max_keycode: Keycode,
    pub vendor: String,
    pub pixmap_formats: Vec<Format>,
    pub roots: Vec<Screen>,
}
impl Setup {}
impl AsByteSequence for Setup {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.protocol_major_version.as_bytes(&mut bytes[index..]);
        index += self.protocol_minor_version.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.release_number.as_bytes(&mut bytes[index..]);
        index += self.resource_id_base.as_bytes(&mut bytes[index..]);
        index += self.resource_id_mask.as_bytes(&mut bytes[index..]);
        index += self.motion_buffer_size.as_bytes(&mut bytes[index..]);
        index += (self.vendor.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.maximum_request_length.as_bytes(&mut bytes[index..]);
        index += (self.roots.len() as Card8).as_bytes(&mut bytes[index..]);
        index += (self.pixmap_formats.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.image_byte_order.as_bytes(&mut bytes[index..]);
        index += self.bitmap_format_bit_order.as_bytes(&mut bytes[index..]);
        index += self
            .bitmap_format_scanline_unit
            .as_bytes(&mut bytes[index..]);
        index += self
            .bitmap_format_scanline_pad
            .as_bytes(&mut bytes[index..]);
        index += self.min_keycode.as_bytes(&mut bytes[index..]);
        index += self.max_keycode.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = string_as_bytes(&self.vendor, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.pixmap_formats, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Format>());
        let block_len: usize = vector_as_bytes(&self.roots, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Screen>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Setup from byte buffer");
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (protocol_major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (protocol_minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (release_number, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resource_id_base, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resource_id_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (motion_buffer_size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (maximum_request_length, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (image_byte_order, sz): (ImageOrder, usize) =
            <ImageOrder>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bitmap_format_bit_order, sz): (ImageOrder, usize) =
            <ImageOrder>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bitmap_format_scanline_unit, sz): (Card8, usize) =
            <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bitmap_format_scanline_pad, sz): (Card8, usize) =
            <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (vendor, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (pixmap_formats, block_len): (Vec<Format>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Format>());
        let (roots, block_len): (Vec<Screen>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Screen>());
        Some((
            Setup {
                status: status,
                protocol_major_version: protocol_major_version,
                protocol_minor_version: protocol_minor_version,
                length: length,
                release_number: release_number,
                resource_id_base: resource_id_base,
                resource_id_mask: resource_id_mask,
                motion_buffer_size: motion_buffer_size,
                maximum_request_length: maximum_request_length,
                image_byte_order: image_byte_order,
                bitmap_format_bit_order: bitmap_format_bit_order,
                bitmap_format_scanline_unit: bitmap_format_scanline_unit,
                bitmap_format_scanline_pad: bitmap_format_scanline_pad,
                min_keycode: min_keycode,
                max_keycode: max_keycode,
                vendor: vendor,
                pixmap_formats: pixmap_formats,
                roots: roots,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.status.size()
            + 1
            + self.protocol_major_version.size()
            + self.protocol_minor_version.size()
            + self.length.size()
            + self.release_number.size()
            + self.resource_id_base.size()
            + self.resource_id_mask.size()
            + self.motion_buffer_size.size()
            + ::core::mem::size_of::<Card16>()
            + self.maximum_request_length.size()
            + ::core::mem::size_of::<Card8>()
            + ::core::mem::size_of::<Card8>()
            + self.image_byte_order.size()
            + self.bitmap_format_bit_order.size()
            + self.bitmap_format_scanline_unit.size()
            + self.bitmap_format_scanline_pad.size()
            + self.min_keycode.size()
            + self.max_keycode.size()
            + 4
            + {
                let block_len: usize = self.vendor.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.pixmap_formats.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Format>());
                block_len + pad
            }
            + {
                let block_len: usize = self.roots.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Screen>());
                block_len + pad
            }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageOrder {
    LsbFirst = 0,
    MsbFirst = 1,
}
impl AsByteSequence for ImageOrder {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::LsbFirst, sz)),
            1 => Some((Self::MsbFirst, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ImageOrder {
    #[inline]
    fn default() -> ImageOrder {
        ImageOrder::LsbFirst
    }
}
pub const WINDOW_NONE: Window = <Window>::const_from_xid(0);
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct KeyButMask {
    pub inner: u16,
}
impl KeyButMask {
    #[inline]
    pub fn shift(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_shift(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn lock(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn control(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_control(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn mod1(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_mod1(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn mod2(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_mod2(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn mod3(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_mod3(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn mod4(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_mod4(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn mod5(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_mod5(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn button1(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_button1(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn button2(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_button2(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn button3(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_button3(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn button4(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_button4(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn button5(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_button5(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn new(
        shift: bool,
        lock: bool,
        control: bool,
        mod1: bool,
        mod2: bool,
        mod3: bool,
        mod4: bool,
        mod5: bool,
        button1: bool,
        button2: bool,
        button3: bool,
        button4: bool,
        button5: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if shift {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if lock {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if control {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if mod1 {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        if mod2 {
            inner |= 1 << 4;
        } else {
            inner &= !(1 << 4);
        }
        if mod3 {
            inner |= 1 << 5;
        } else {
            inner &= !(1 << 5);
        }
        if mod4 {
            inner |= 1 << 6;
        } else {
            inner &= !(1 << 6);
        }
        if mod5 {
            inner |= 1 << 7;
        } else {
            inner &= !(1 << 7);
        }
        if button1 {
            inner |= 1 << 8;
        } else {
            inner &= !(1 << 8);
        }
        if button2 {
            inner |= 1 << 9;
        } else {
            inner &= !(1 << 9);
        }
        if button3 {
            inner |= 1 << 10;
        } else {
            inner &= !(1 << 10);
        }
        if button4 {
            inner |= 1 << 11;
        } else {
            inner &= !(1 << 11);
        }
        if button5 {
            inner |= 1 << 12;
        } else {
            inner &= !(1 << 12);
        }
        KeyButMask { inner: inner }
    }
}
impl AsByteSequence for KeyButMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((KeyButMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Motion {
    Normal = 0,
    Hint = 1,
}
impl AsByteSequence for Motion {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Normal, sz)),
            1 => Some((Self::Hint, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Motion {
    #[inline]
    fn default() -> Motion {
        Motion::Normal
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotifyDetail {
    Ancestor = 0,
    Virtual = 1,
    Inferior = 2,
    Nonlinear = 3,
    NonlinearVirtual = 4,
    Pointer = 5,
    PointerRoot = 6,
    None = 7,
}
impl AsByteSequence for NotifyDetail {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Ancestor, sz)),
            1 => Some((Self::Virtual, sz)),
            2 => Some((Self::Inferior, sz)),
            3 => Some((Self::Nonlinear, sz)),
            4 => Some((Self::NonlinearVirtual, sz)),
            5 => Some((Self::Pointer, sz)),
            6 => Some((Self::PointerRoot, sz)),
            7 => Some((Self::None, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for NotifyDetail {
    #[inline]
    fn default() -> NotifyDetail {
        NotifyDetail::Ancestor
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum NotifyMode {
    Normal = 0,
    Grab = 1,
    Ungrab = 2,
    WhileGrabbed = 3,
}
impl AsByteSequence for NotifyMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Normal, sz)),
            1 => Some((Self::Grab, sz)),
            2 => Some((Self::Ungrab, sz)),
            3 => Some((Self::WhileGrabbed, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for NotifyMode {
    #[inline]
    fn default() -> NotifyMode {
        NotifyMode::Normal
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
    Unobscured = 0,
    PartiallyObscured = 1,
    FullyObscured = 2,
}
impl AsByteSequence for Visibility {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Unobscured, sz)),
            1 => Some((Self::PartiallyObscured, sz)),
            2 => Some((Self::FullyObscured, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Visibility {
    #[inline]
    fn default() -> Visibility {
        Visibility::Unobscured
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum StackMode {
    Above = 0,
    Below = 1,
    TopIf = 2,
    BottomIf = 3,
    Opposite = 4,
}
impl AsByteSequence for StackMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Above, sz)),
            1 => Some((Self::Below, sz)),
            2 => Some((Self::TopIf, sz)),
            3 => Some((Self::BottomIf, sz)),
            4 => Some((Self::Opposite, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for StackMode {
    #[inline]
    fn default() -> StackMode {
        StackMode::Above
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ConfigWindow {
    pub inner: u16,
}
impl ConfigWindow {
    #[inline]
    pub fn x(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_x(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn y(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_y(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn width(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_width(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn height(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_height(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn border_width(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_border_width(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn sibling(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_sibling(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn stack_mode(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_stack_mode(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn new(
        x: bool,
        y: bool,
        width: bool,
        height: bool,
        border_width: bool,
        sibling: bool,
        stack_mode: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if x {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if y {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if width {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if height {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        if border_width {
            inner |= 1 << 4;
        } else {
            inner &= !(1 << 4);
        }
        if sibling {
            inner |= 1 << 5;
        } else {
            inner &= !(1 << 5);
        }
        if stack_mode {
            inner |= 1 << 6;
        } else {
            inner &= !(1 << 6);
        }
        ConfigWindow { inner: inner }
    }
}
impl AsByteSequence for ConfigWindow {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((ConfigWindow { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Place {
    OnTop = 0,
    OnBottom = 1,
}
impl AsByteSequence for Place {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::OnTop, sz)),
            1 => Some((Self::OnBottom, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Place {
    #[inline]
    fn default() -> Place {
        Place::OnTop
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Property {
    NewValue = 0,
    Delete = 1,
}
impl AsByteSequence for Property {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NewValue, sz)),
            1 => Some((Self::Delete, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Property {
    #[inline]
    fn default() -> Property {
        Property::NewValue
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Time {
    CurrentTime = 0,
}
impl AsByteSequence for Time {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::CurrentTime, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for Time {
    #[inline]
    fn default() -> Time {
        Time::CurrentTime
    }
}
pub const ATOM_NONE: Atom = <Atom>::const_from_xid(0);
pub const ATOM_ANY: Atom = <Atom>::const_from_xid(0);
pub const ATOM_PRIMARY: Atom = <Atom>::const_from_xid(1);
pub const ATOM_SECONDARY: Atom = <Atom>::const_from_xid(2);
pub const ATOM_ARC: Atom = <Atom>::const_from_xid(3);
pub const ATOM_ATOM: Atom = <Atom>::const_from_xid(4);
pub const ATOM_BITMAP: Atom = <Atom>::const_from_xid(5);
pub const ATOM_CARDINAL: Atom = <Atom>::const_from_xid(6);
pub const ATOM_COLORMAP: Atom = <Atom>::const_from_xid(7);
pub const ATOM_CURSOR: Atom = <Atom>::const_from_xid(8);
pub const ATOM_CUT_BUFFER0: Atom = <Atom>::const_from_xid(9);
pub const ATOM_CUT_BUFFER1: Atom = <Atom>::const_from_xid(10);
pub const ATOM_CUT_BUFFER2: Atom = <Atom>::const_from_xid(11);
pub const ATOM_CUT_BUFFER3: Atom = <Atom>::const_from_xid(12);
pub const ATOM_CUT_BUFFER4: Atom = <Atom>::const_from_xid(13);
pub const ATOM_CUT_BUFFER5: Atom = <Atom>::const_from_xid(14);
pub const ATOM_CUT_BUFFER6: Atom = <Atom>::const_from_xid(15);
pub const ATOM_CUT_BUFFER7: Atom = <Atom>::const_from_xid(16);
pub const ATOM_DRAWABLE: Atom = <Atom>::const_from_xid(17);
pub const ATOM_FONT: Atom = <Atom>::const_from_xid(18);
pub const ATOM_INTEGER: Atom = <Atom>::const_from_xid(19);
pub const ATOM_PIXMAP: Atom = <Atom>::const_from_xid(20);
pub const ATOM_POINT: Atom = <Atom>::const_from_xid(21);
pub const ATOM_RECTANGLE: Atom = <Atom>::const_from_xid(22);
pub const ATOM_RESOURCE_MANAGER: Atom = <Atom>::const_from_xid(23);
pub const ATOM_RGB_COLOR_MAP: Atom = <Atom>::const_from_xid(24);
pub const ATOM_RGB_BEST_MAP: Atom = <Atom>::const_from_xid(25);
pub const ATOM_RGB_BLUE_MAP: Atom = <Atom>::const_from_xid(26);
pub const ATOM_RGB_DEFAULT_MAP: Atom = <Atom>::const_from_xid(27);
pub const ATOM_RGB_GRAY_MAP: Atom = <Atom>::const_from_xid(28);
pub const ATOM_RGB_GREEN_MAP: Atom = <Atom>::const_from_xid(29);
pub const ATOM_RGB_RED_MAP: Atom = <Atom>::const_from_xid(30);
pub const ATOM_STRING: Atom = <Atom>::const_from_xid(31);
pub const ATOM_VISUALID: Atom = <Atom>::const_from_xid(32);
pub const ATOM_WINDOW: Atom = <Atom>::const_from_xid(33);
pub const ATOM_WM_COMMAND: Atom = <Atom>::const_from_xid(34);
pub const ATOM_WM_HINTS: Atom = <Atom>::const_from_xid(35);
pub const ATOM_WM_CLIENT_MACHINE: Atom = <Atom>::const_from_xid(36);
pub const ATOM_WM_ICON_NAME: Atom = <Atom>::const_from_xid(37);
pub const ATOM_WM_ICON_SIZE: Atom = <Atom>::const_from_xid(38);
pub const ATOM_WM_NAME: Atom = <Atom>::const_from_xid(39);
pub const ATOM_WM_NORMAL_HINTS: Atom = <Atom>::const_from_xid(40);
pub const ATOM_WM_SIZE_HINTS: Atom = <Atom>::const_from_xid(41);
pub const ATOM_WM_ZOOM_HINTS: Atom = <Atom>::const_from_xid(42);
pub const ATOM_MIN_SPACE: Atom = <Atom>::const_from_xid(43);
pub const ATOM_NORM_SPACE: Atom = <Atom>::const_from_xid(44);
pub const ATOM_MAX_SPACE: Atom = <Atom>::const_from_xid(45);
pub const ATOM_END_SPACE: Atom = <Atom>::const_from_xid(46);
pub const ATOM_SUPERSCRIPT_X: Atom = <Atom>::const_from_xid(47);
pub const ATOM_SUPERSCRIPT_Y: Atom = <Atom>::const_from_xid(48);
pub const ATOM_SUBSCRIPT_X: Atom = <Atom>::const_from_xid(49);
pub const ATOM_SUBSCRIPT_Y: Atom = <Atom>::const_from_xid(50);
pub const ATOM_UNDERLINE_POSITION: Atom = <Atom>::const_from_xid(51);
pub const ATOM_UNDERLINE_THICKNESS: Atom = <Atom>::const_from_xid(52);
pub const ATOM_STRIKEOUT_ASCENT: Atom = <Atom>::const_from_xid(53);
pub const ATOM_STRIKEOUT_DESCENT: Atom = <Atom>::const_from_xid(54);
pub const ATOM_ITALIC_ANGLE: Atom = <Atom>::const_from_xid(55);
pub const ATOM_X_HEIGHT: Atom = <Atom>::const_from_xid(56);
pub const ATOM_QUAD_WIDTH: Atom = <Atom>::const_from_xid(57);
pub const ATOM_WEIGHT: Atom = <Atom>::const_from_xid(58);
pub const ATOM_POINT_SIZE: Atom = <Atom>::const_from_xid(59);
pub const ATOM_RESOLUTION: Atom = <Atom>::const_from_xid(60);
pub const ATOM_COPYRIGHT: Atom = <Atom>::const_from_xid(61);
pub const ATOM_NOTICE: Atom = <Atom>::const_from_xid(62);
pub const ATOM_FONT_NAME: Atom = <Atom>::const_from_xid(63);
pub const ATOM_FAMILY_NAME: Atom = <Atom>::const_from_xid(64);
pub const ATOM_FULL_NAME: Atom = <Atom>::const_from_xid(65);
pub const ATOM_CAP_HEIGHT: Atom = <Atom>::const_from_xid(66);
pub const ATOM_WM_CLASS: Atom = <Atom>::const_from_xid(67);
pub const ATOM_WM_TRANSIENT_FOR: Atom = <Atom>::const_from_xid(68);
pub const COLORMAP_NONE: Colormap = <Colormap>::const_from_xid(0);
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColormapState {
    Uninstalled = 0,
    Installed = 1,
}
impl AsByteSequence for ColormapState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Uninstalled, sz)),
            1 => Some((Self::Installed, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ColormapState {
    #[inline]
    fn default() -> ColormapState {
        ColormapState::Uninstalled
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Mapping {
    Modifier = 0,
    Keyboard = 1,
    Pointer = 2,
}
impl AsByteSequence for Mapping {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Modifier, sz)),
            1 => Some((Self::Keyboard, sz)),
            2 => Some((Self::Pointer, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Mapping {
    #[inline]
    fn default() -> Mapping {
        Mapping::Modifier
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreateWindowRequest {
    pub req_type: u8,
    pub depth: Card8,
    pub length: u16,
    pub wid: Window,
    pub parent: Window,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub border_width: Card16,
    pub class: WindowClass,
    pub visual: Visualid,
    pub value_mask: Cw,
    pub background_pixmap: Pixmap,
    pub background_pixel: Card32,
    pub border_pixmap: Pixmap,
    pub border_pixel: Card32,
    pub bit_gravity: Gravity,
    pub win_gravity: Gravity,
    pub backing_store: BackingStore,
    pub backing_planes: Card32,
    pub backing_pixel: Card32,
    pub override_redirect: Bool32,
    pub save_under: Bool32,
    pub event_mask: EventMask,
    pub do_not_propogate_mask: EventMask,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
impl CreateWindowRequest {}
impl AsByteSequence for CreateWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.wid.as_bytes(&mut bytes[index..]);
        index += self.parent.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.border_width.as_bytes(&mut bytes[index..]);
        index += self.class.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.back_pixmap() {
            index += self.background_pixmap.as_bytes(&mut bytes[index..]);
        }
        if cond0.back_pixel() {
            index += self.background_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.border_pixmap() {
            index += self.border_pixmap.as_bytes(&mut bytes[index..]);
        }
        if cond0.border_pixel() {
            index += self.border_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.bit_gravity() {
            index += self.bit_gravity.as_bytes(&mut bytes[index..]);
        }
        if cond0.win_gravity() {
            index += self.win_gravity.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_store() {
            index += self.backing_store.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_planes() {
            index += self.backing_planes.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_pixel() {
            index += self.backing_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.override_redirect() {
            index += self.override_redirect.as_bytes(&mut bytes[index..]);
        }
        if cond0.save_under() {
            index += self.save_under.as_bytes(&mut bytes[index..]);
        }
        if cond0.event_mask() {
            index += self.event_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.dont_propagate() {
            index += self.do_not_propogate_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.colormap() {
            index += self.colormap.as_bytes(&mut bytes[index..]);
        }
        if cond0.cursor() {
            index += self.cursor.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wid, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parent, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (class, sz): (WindowClass, usize) = <WindowClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Cw, usize) = <Cw>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let background_pixmap: Pixmap = if cond0.back_pixmap() {
            let (background_pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            background_pixmap
        } else {
            Default::default()
        };
        let background_pixel: Card32 = if cond0.back_pixel() {
            let (background_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            background_pixel
        } else {
            Default::default()
        };
        let border_pixmap: Pixmap = if cond0.border_pixmap() {
            let (border_pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            border_pixmap
        } else {
            Default::default()
        };
        let border_pixel: Card32 = if cond0.border_pixel() {
            let (border_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            border_pixel
        } else {
            Default::default()
        };
        let bit_gravity: Gravity = if cond0.bit_gravity() {
            let (bit_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
            index += sz;
            bit_gravity
        } else {
            Default::default()
        };
        let win_gravity: Gravity = if cond0.win_gravity() {
            let (win_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
            index += sz;
            win_gravity
        } else {
            Default::default()
        };
        let backing_store: BackingStore = if cond0.backing_store() {
            let (backing_store, sz): (BackingStore, usize) =
                <BackingStore>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_store
        } else {
            Default::default()
        };
        let backing_planes: Card32 = if cond0.backing_planes() {
            let (backing_planes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_planes
        } else {
            Default::default()
        };
        let backing_pixel: Card32 = if cond0.backing_pixel() {
            let (backing_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_pixel
        } else {
            Default::default()
        };
        let override_redirect: Bool32 = if cond0.override_redirect() {
            let (override_redirect, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            override_redirect
        } else {
            Default::default()
        };
        let save_under: Bool32 = if cond0.save_under() {
            let (save_under, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            save_under
        } else {
            Default::default()
        };
        let event_mask: EventMask = if cond0.event_mask() {
            let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
            index += sz;
            event_mask
        } else {
            Default::default()
        };
        let do_not_propogate_mask: EventMask = if cond0.dont_propagate() {
            let (do_not_propogate_mask, sz): (EventMask, usize) =
                <EventMask>::from_bytes(&bytes[index..])?;
            index += sz;
            do_not_propogate_mask
        } else {
            Default::default()
        };
        let colormap: Colormap = if cond0.colormap() {
            let (colormap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
            index += sz;
            colormap
        } else {
            Default::default()
        };
        let cursor: Cursor = if cond0.cursor() {
            let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
            index += sz;
            cursor
        } else {
            Default::default()
        };
        Some((
            CreateWindowRequest {
                req_type: req_type,
                depth: depth,
                length: length,
                wid: wid,
                parent: parent,
                x: x,
                y: y,
                width: width,
                height: height,
                border_width: border_width,
                class: class,
                visual: visual,
                value_mask: value_mask,
                background_pixmap: background_pixmap,
                background_pixel: background_pixel,
                border_pixmap: border_pixmap,
                border_pixel: border_pixel,
                bit_gravity: bit_gravity,
                win_gravity: win_gravity,
                backing_store: backing_store,
                backing_planes: backing_planes,
                backing_pixel: backing_pixel,
                override_redirect: override_redirect,
                save_under: save_under,
                event_mask: event_mask,
                do_not_propogate_mask: do_not_propogate_mask,
                colormap: colormap,
                cursor: cursor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.depth.size()
            + self.length.size()
            + self.wid.size()
            + self.parent.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.border_width.size()
            + self.class.size()
            + self.visual.size()
            + self.value_mask.size()
            + self.background_pixmap.size()
            + self.background_pixel.size()
            + self.border_pixmap.size()
            + self.border_pixel.size()
            + self.bit_gravity.size()
            + self.win_gravity.size()
            + self.backing_store.size()
            + self.backing_planes.size()
            + self.backing_pixel.size()
            + self.override_redirect.size()
            + self.save_under.size()
            + self.event_mask.size()
            + self.do_not_propogate_mask.size()
            + self.colormap.size()
            + self.cursor.size()
    }
}
impl Request for CreateWindowRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum WindowClass {
    CopyFromParent = 0,
    InputOutput = 1,
    InputOnly = 2,
}
impl AsByteSequence for WindowClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::CopyFromParent, sz)),
            1 => Some((Self::InputOutput, sz)),
            2 => Some((Self::InputOnly, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for WindowClass {
    #[inline]
    fn default() -> WindowClass {
        WindowClass::CopyFromParent
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cw {
    pub inner: u32,
}
impl Cw {
    #[inline]
    pub fn back_pixmap(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_back_pixmap(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn back_pixel(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_back_pixel(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn border_pixmap(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_border_pixmap(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn border_pixel(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_border_pixel(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn bit_gravity(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_bit_gravity(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn win_gravity(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_win_gravity(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn backing_store(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_backing_store(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn backing_planes(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_backing_planes(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn backing_pixel(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_backing_pixel(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn override_redirect(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_override_redirect(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn save_under(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_save_under(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn event_mask(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_event_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn dont_propagate(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_dont_propagate(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn colormap(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_colormap(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn cursor(&self) -> bool {
        self.inner & (1 << 14) != 0
    }
    #[inline]
    pub fn set_cursor(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 14;
        } else {
            self.inner &= !(1 << 14);
        }
        self
    }
    #[inline]
    pub fn new(
        back_pixmap: bool,
        back_pixel: bool,
        border_pixmap: bool,
        border_pixel: bool,
        bit_gravity: bool,
        win_gravity: bool,
        backing_store: bool,
        backing_planes: bool,
        backing_pixel: bool,
        override_redirect: bool,
        save_under: bool,
        event_mask: bool,
        dont_propagate: bool,
        colormap: bool,
        cursor: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if back_pixmap {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if back_pixel {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if border_pixmap {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if border_pixel {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        if bit_gravity {
            inner |= 1 << 4;
        } else {
            inner &= !(1 << 4);
        }
        if win_gravity {
            inner |= 1 << 5;
        } else {
            inner &= !(1 << 5);
        }
        if backing_store {
            inner |= 1 << 6;
        } else {
            inner &= !(1 << 6);
        }
        if backing_planes {
            inner |= 1 << 7;
        } else {
            inner &= !(1 << 7);
        }
        if backing_pixel {
            inner |= 1 << 8;
        } else {
            inner &= !(1 << 8);
        }
        if override_redirect {
            inner |= 1 << 9;
        } else {
            inner &= !(1 << 9);
        }
        if save_under {
            inner |= 1 << 10;
        } else {
            inner &= !(1 << 10);
        }
        if event_mask {
            inner |= 1 << 11;
        } else {
            inner &= !(1 << 11);
        }
        if dont_propagate {
            inner |= 1 << 12;
        } else {
            inner &= !(1 << 12);
        }
        if colormap {
            inner |= 1 << 13;
        } else {
            inner &= !(1 << 13);
        }
        if cursor {
            inner |= 1 << 14;
        } else {
            inner &= !(1 << 14);
        }
        Cw { inner: inner }
    }
}
impl AsByteSequence for Cw {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Cw { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangeWindowAttributesRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub value_mask: Cw,
    pub background_pixmap: Pixmap,
    pub background_pixel: Card32,
    pub border_pixmap: Pixmap,
    pub border_pixel: Card32,
    pub bit_gravity: Gravity,
    pub win_gravity: Gravity,
    pub backing_store: BackingStore,
    pub backing_planes: Card32,
    pub backing_pixel: Card32,
    pub override_redirect: Bool32,
    pub save_under: Bool32,
    pub event_mask: EventMask,
    pub do_not_propogate_mask: EventMask,
    pub colormap: Colormap,
    pub cursor: Cursor,
}
impl ChangeWindowAttributesRequest {}
impl AsByteSequence for ChangeWindowAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.back_pixmap() {
            index += self.background_pixmap.as_bytes(&mut bytes[index..]);
        }
        if cond0.back_pixel() {
            index += self.background_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.border_pixmap() {
            index += self.border_pixmap.as_bytes(&mut bytes[index..]);
        }
        if cond0.border_pixel() {
            index += self.border_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.bit_gravity() {
            index += self.bit_gravity.as_bytes(&mut bytes[index..]);
        }
        if cond0.win_gravity() {
            index += self.win_gravity.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_store() {
            index += self.backing_store.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_planes() {
            index += self.backing_planes.as_bytes(&mut bytes[index..]);
        }
        if cond0.backing_pixel() {
            index += self.backing_pixel.as_bytes(&mut bytes[index..]);
        }
        if cond0.override_redirect() {
            index += self.override_redirect.as_bytes(&mut bytes[index..]);
        }
        if cond0.save_under() {
            index += self.save_under.as_bytes(&mut bytes[index..]);
        }
        if cond0.event_mask() {
            index += self.event_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.dont_propagate() {
            index += self.do_not_propogate_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.colormap() {
            index += self.colormap.as_bytes(&mut bytes[index..]);
        }
        if cond0.cursor() {
            index += self.cursor.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeWindowAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Cw, usize) = <Cw>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let background_pixmap: Pixmap = if cond0.back_pixmap() {
            let (background_pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            background_pixmap
        } else {
            Default::default()
        };
        let background_pixel: Card32 = if cond0.back_pixel() {
            let (background_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            background_pixel
        } else {
            Default::default()
        };
        let border_pixmap: Pixmap = if cond0.border_pixmap() {
            let (border_pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            border_pixmap
        } else {
            Default::default()
        };
        let border_pixel: Card32 = if cond0.border_pixel() {
            let (border_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            border_pixel
        } else {
            Default::default()
        };
        let bit_gravity: Gravity = if cond0.bit_gravity() {
            let (bit_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
            index += sz;
            bit_gravity
        } else {
            Default::default()
        };
        let win_gravity: Gravity = if cond0.win_gravity() {
            let (win_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
            index += sz;
            win_gravity
        } else {
            Default::default()
        };
        let backing_store: BackingStore = if cond0.backing_store() {
            let (backing_store, sz): (BackingStore, usize) =
                <BackingStore>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_store
        } else {
            Default::default()
        };
        let backing_planes: Card32 = if cond0.backing_planes() {
            let (backing_planes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_planes
        } else {
            Default::default()
        };
        let backing_pixel: Card32 = if cond0.backing_pixel() {
            let (backing_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            backing_pixel
        } else {
            Default::default()
        };
        let override_redirect: Bool32 = if cond0.override_redirect() {
            let (override_redirect, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            override_redirect
        } else {
            Default::default()
        };
        let save_under: Bool32 = if cond0.save_under() {
            let (save_under, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            save_under
        } else {
            Default::default()
        };
        let event_mask: EventMask = if cond0.event_mask() {
            let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
            index += sz;
            event_mask
        } else {
            Default::default()
        };
        let do_not_propogate_mask: EventMask = if cond0.dont_propagate() {
            let (do_not_propogate_mask, sz): (EventMask, usize) =
                <EventMask>::from_bytes(&bytes[index..])?;
            index += sz;
            do_not_propogate_mask
        } else {
            Default::default()
        };
        let colormap: Colormap = if cond0.colormap() {
            let (colormap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
            index += sz;
            colormap
        } else {
            Default::default()
        };
        let cursor: Cursor = if cond0.cursor() {
            let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
            index += sz;
            cursor
        } else {
            Default::default()
        };
        Some((
            ChangeWindowAttributesRequest {
                req_type: req_type,
                length: length,
                window: window,
                value_mask: value_mask,
                background_pixmap: background_pixmap,
                background_pixel: background_pixel,
                border_pixmap: border_pixmap,
                border_pixel: border_pixel,
                bit_gravity: bit_gravity,
                win_gravity: win_gravity,
                backing_store: backing_store,
                backing_planes: backing_planes,
                backing_pixel: backing_pixel,
                override_redirect: override_redirect,
                save_under: save_under,
                event_mask: event_mask,
                do_not_propogate_mask: do_not_propogate_mask,
                colormap: colormap,
                cursor: cursor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.window.size()
            + self.value_mask.size()
            + self.background_pixmap.size()
            + self.background_pixel.size()
            + self.border_pixmap.size()
            + self.border_pixel.size()
            + self.bit_gravity.size()
            + self.win_gravity.size()
            + self.backing_store.size()
            + self.backing_planes.size()
            + self.backing_pixel.size()
            + self.override_redirect.size()
            + self.save_under.size()
            + self.event_mask.size()
            + self.do_not_propogate_mask.size()
            + self.colormap.size()
            + self.cursor.size()
    }
}
impl Request for ChangeWindowAttributesRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetWindowAttributesRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetWindowAttributesRequest {}
impl AsByteSequence for GetWindowAttributesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetWindowAttributesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetWindowAttributesRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for GetWindowAttributesRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetWindowAttributesReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetWindowAttributesReply {
    pub reply_type: u8,
    pub backing_store: BackingStore,
    pub sequence: u16,
    pub length: u32,
    pub visual: Visualid,
    pub class: WindowClass,
    pub bit_gravity: Gravity,
    pub win_gravity: Gravity,
    pub backing_planes: Card32,
    pub backing_pixel: Card32,
    pub save_under: bool,
    pub map_is_installed: bool,
    pub map_state: MapState,
    pub override_redirect: bool,
    pub colormap: Colormap,
    pub all_event_masks: EventMask,
    pub your_event_mask: EventMask,
    pub do_not_propagate_mask: EventMask,
}
impl GetWindowAttributesReply {}
impl AsByteSequence for GetWindowAttributesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.backing_store.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += self.class.as_bytes(&mut bytes[index..]);
        index += self.bit_gravity.as_bytes(&mut bytes[index..]);
        index += self.win_gravity.as_bytes(&mut bytes[index..]);
        index += self.backing_planes.as_bytes(&mut bytes[index..]);
        index += self.backing_pixel.as_bytes(&mut bytes[index..]);
        index += self.save_under.as_bytes(&mut bytes[index..]);
        index += self.map_is_installed.as_bytes(&mut bytes[index..]);
        index += self.map_state.as_bytes(&mut bytes[index..]);
        index += self.override_redirect.as_bytes(&mut bytes[index..]);
        index += self.colormap.as_bytes(&mut bytes[index..]);
        index += self.all_event_masks.as_bytes(&mut bytes[index..]);
        index += self.your_event_mask.as_bytes(&mut bytes[index..]);
        index += self.do_not_propagate_mask.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetWindowAttributesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (backing_store, sz): (BackingStore, usize) =
            <BackingStore>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        let (class, sz): (WindowClass, usize) = <WindowClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bit_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
        index += sz;
        let (win_gravity, sz): (Gravity, usize) = <Gravity>::from_bytes(&bytes[index..])?;
        index += sz;
        let (backing_planes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (backing_pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (save_under, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_is_installed, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_state, sz): (MapState, usize) = <MapState>::from_bytes(&bytes[index..])?;
        index += sz;
        let (override_redirect, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (colormap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (all_event_masks, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (your_event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (do_not_propagate_mask, sz): (EventMask, usize) =
            <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetWindowAttributesReply {
                reply_type: reply_type,
                backing_store: backing_store,
                sequence: sequence,
                length: length,
                visual: visual,
                class: class,
                bit_gravity: bit_gravity,
                win_gravity: win_gravity,
                backing_planes: backing_planes,
                backing_pixel: backing_pixel,
                save_under: save_under,
                map_is_installed: map_is_installed,
                map_state: map_state,
                override_redirect: override_redirect,
                colormap: colormap,
                all_event_masks: all_event_masks,
                your_event_mask: your_event_mask,
                do_not_propagate_mask: do_not_propagate_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.backing_store.size()
            + self.sequence.size()
            + self.length.size()
            + self.visual.size()
            + self.class.size()
            + self.bit_gravity.size()
            + self.win_gravity.size()
            + self.backing_planes.size()
            + self.backing_pixel.size()
            + self.save_under.size()
            + self.map_is_installed.size()
            + self.map_state.size()
            + self.override_redirect.size()
            + self.colormap.size()
            + self.all_event_masks.size()
            + self.your_event_mask.size()
            + self.do_not_propagate_mask.size()
            + 2
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gravity {
    BitForget = 0,
    NorthWest = 1,
    North = 2,
    NorthEast = 3,
    West = 4,
    Center = 5,
    East = 6,
    SouthWest = 7,
    South = 8,
    SouthEast = 9,
    Static = 10,
}
impl AsByteSequence for Gravity {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::BitForget, sz)),
            1 => Some((Self::NorthWest, sz)),
            2 => Some((Self::North, sz)),
            3 => Some((Self::NorthEast, sz)),
            4 => Some((Self::West, sz)),
            5 => Some((Self::Center, sz)),
            6 => Some((Self::East, sz)),
            7 => Some((Self::SouthWest, sz)),
            8 => Some((Self::South, sz)),
            9 => Some((Self::SouthEast, sz)),
            10 => Some((Self::Static, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Gravity {
    #[inline]
    fn default() -> Gravity {
        Gravity::BitForget
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MapState {
    Unmapped = 0,
    Unviewable = 1,
    Viewable = 2,
}
impl AsByteSequence for MapState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Unmapped, sz)),
            1 => Some((Self::Unviewable, sz)),
            2 => Some((Self::Viewable, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for MapState {
    #[inline]
    fn default() -> MapState {
        MapState::Unmapped
    }
}
#[derive(Clone, Debug, Default)]
pub struct DestroyWindowRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl DestroyWindowRequest {}
impl AsByteSequence for DestroyWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyWindowRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for DestroyWindowRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct DestroySubwindowsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl DestroySubwindowsRequest {}
impl AsByteSequence for DestroySubwindowsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroySubwindowsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroySubwindowsRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for DestroySubwindowsRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ChangeSaveSetRequest {
    pub req_type: u8,
    pub mode: SetMode,
    pub length: u16,
    pub window: Window,
}
impl ChangeSaveSetRequest {}
impl AsByteSequence for ChangeSaveSetRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeSaveSetRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (SetMode, usize) = <SetMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ChangeSaveSetRequest {
                req_type: req_type,
                mode: mode,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.mode.size() + self.length.size() + self.window.size()
    }
}
impl Request for ChangeSaveSetRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SetMode {
    Insert = 0,
    Delete = 1,
}
impl AsByteSequence for SetMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Insert, sz)),
            1 => Some((Self::Delete, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SetMode {
    #[inline]
    fn default() -> SetMode {
        SetMode::Insert
    }
}
#[derive(Clone, Debug, Default)]
pub struct ReparentWindowRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub parent: Window,
    pub x: Int16,
    pub y: Int16,
}
impl ReparentWindowRequest {}
impl AsByteSequence for ReparentWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.parent.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ReparentWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parent, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ReparentWindowRequest {
                req_type: req_type,
                length: length,
                window: window,
                parent: parent,
                x: x,
                y: y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.window.size()
            + self.parent.size()
            + self.x.size()
            + self.y.size()
    }
}
impl Request for ReparentWindowRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct MapWindowRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl MapWindowRequest {}
impl AsByteSequence for MapWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MapWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            MapWindowRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for MapWindowRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct MapSubwindowsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl MapSubwindowsRequest {}
impl AsByteSequence for MapSubwindowsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MapSubwindowsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            MapSubwindowsRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for MapSubwindowsRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UnmapWindowRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl UnmapWindowRequest {}
impl AsByteSequence for UnmapWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UnmapWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UnmapWindowRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for UnmapWindowRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UnmapSubwindowsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl UnmapSubwindowsRequest {}
impl AsByteSequence for UnmapSubwindowsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UnmapSubwindowsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UnmapSubwindowsRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for UnmapSubwindowsRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ConfigureWindowRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub value_mask: ConfigWindow,
    pub x: Int32,
    pub y: Int32,
    pub width: Card32,
    pub height: Card32,
    pub border_width: Card32,
    pub sibling: Window,
    pub stack_mode: StackMode,
}
impl ConfigureWindowRequest {}
impl AsByteSequence for ConfigureWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        index += 2;
        let cond0 = (self.value_mask);
        if cond0.x() {
            index += self.x.as_bytes(&mut bytes[index..]);
        }
        if cond0.y() {
            index += self.y.as_bytes(&mut bytes[index..]);
        }
        if cond0.width() {
            index += self.width.as_bytes(&mut bytes[index..]);
        }
        if cond0.height() {
            index += self.height.as_bytes(&mut bytes[index..]);
        }
        if cond0.border_width() {
            index += self.border_width.as_bytes(&mut bytes[index..]);
        }
        if cond0.sibling() {
            index += self.sibling.as_bytes(&mut bytes[index..]);
        }
        if cond0.stack_mode() {
            index += self.stack_mode.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConfigureWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (ConfigWindow, usize) = <ConfigWindow>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let cond0 = (value_mask);
        let x: Int32 = if cond0.x() {
            let (x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            x
        } else {
            Default::default()
        };
        let y: Int32 = if cond0.y() {
            let (y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            y
        } else {
            Default::default()
        };
        let width: Card32 = if cond0.width() {
            let (width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            width
        } else {
            Default::default()
        };
        let height: Card32 = if cond0.height() {
            let (height, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            height
        } else {
            Default::default()
        };
        let border_width: Card32 = if cond0.border_width() {
            let (border_width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            border_width
        } else {
            Default::default()
        };
        let sibling: Window = if cond0.sibling() {
            let (sibling, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
            index += sz;
            sibling
        } else {
            Default::default()
        };
        let stack_mode: StackMode = if cond0.stack_mode() {
            let (stack_mode, sz): (StackMode, usize) = <StackMode>::from_bytes(&bytes[index..])?;
            index += sz;
            stack_mode
        } else {
            Default::default()
        };
        Some((
            ConfigureWindowRequest {
                req_type: req_type,
                length: length,
                window: window,
                value_mask: value_mask,
                x: x,
                y: y,
                width: width,
                height: height,
                border_width: border_width,
                sibling: sibling,
                stack_mode: stack_mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.window.size()
            + self.value_mask.size()
            + 2
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.border_width.size()
            + self.sibling.size()
            + self.stack_mode.size()
    }
}
impl Request for ConfigureWindowRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CirculateWindowRequest {
    pub req_type: u8,
    pub direction: Circulate,
    pub length: u16,
    pub window: Window,
}
impl CirculateWindowRequest {}
impl AsByteSequence for CirculateWindowRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.direction.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CirculateWindowRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (direction, sz): (Circulate, usize) = <Circulate>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CirculateWindowRequest {
                req_type: req_type,
                direction: direction,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.direction.size() + self.length.size() + self.window.size()
    }
}
impl Request for CirculateWindowRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Circulate {
    RaiseLowest = 0,
    LowerHighest = 1,
}
impl AsByteSequence for Circulate {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::RaiseLowest, sz)),
            1 => Some((Self::LowerHighest, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Circulate {
    #[inline]
    fn default() -> Circulate {
        Circulate::RaiseLowest
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetGeometryRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
}
impl GetGeometryRequest {}
impl AsByteSequence for GetGeometryRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetGeometryRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetGeometryRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size()
    }
}
impl Request for GetGeometryRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetGeometryReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetGeometryReply {
    pub reply_type: u8,
    pub depth: Card8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub border_width: Card16,
}
impl GetGeometryReply {}
impl AsByteSequence for GetGeometryReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.border_width.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetGeometryReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetGeometryReply {
                reply_type: reply_type,
                depth: depth,
                sequence: sequence,
                length: length,
                root: root,
                x: x,
                y: y,
                width: width,
                height: height,
                border_width: border_width,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.depth.size()
            + self.sequence.size()
            + self.length.size()
            + self.root.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.border_width.size()
            + 2
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryTreeRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl QueryTreeRequest {}
impl AsByteSequence for QueryTreeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryTreeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryTreeRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for QueryTreeRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryTreeReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryTreeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub parent: Window,
    pub children: Vec<Window>,
}
impl QueryTreeReply {}
impl AsByteSequence for QueryTreeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.parent.as_bytes(&mut bytes[index..]);
        index += (self.children.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 14;
        let block_len: usize = vector_as_bytes(&self.children, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Window>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryTreeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parent, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 14;
        let (children, block_len): (Vec<Window>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Window>());
        Some((
            QueryTreeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                root: root,
                parent: parent,
                children: children,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.root.size()
            + self.parent.size()
            + ::core::mem::size_of::<Card16>()
            + 14
            + {
                let block_len: usize = self.children.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Window>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct InternAtomRequest {
    pub req_type: u8,
    pub only_if_exists: bool,
    pub length: u16,
    pub name: String,
}
impl InternAtomRequest {}
impl AsByteSequence for InternAtomRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.only_if_exists.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InternAtomRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (only_if_exists, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            InternAtomRequest {
                req_type: req_type,
                only_if_exists: only_if_exists,
                length: length,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.only_if_exists.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for InternAtomRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = InternAtomReply;
}
#[derive(Clone, Debug, Default)]
pub struct InternAtomReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atom: Atom,
}
impl InternAtomReply {}
impl AsByteSequence for InternAtomReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.atom.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InternAtomReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (atom, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InternAtomReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                atom: atom,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.atom.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetAtomNameRequest {
    pub req_type: u8,
    pub length: u16,
    pub atom: Atom,
}
impl GetAtomNameRequest {}
impl AsByteSequence for GetAtomNameRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.atom.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetAtomNameRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (atom, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetAtomNameRequest {
                req_type: req_type,
                length: length,
                atom: atom,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.atom.size()
    }
}
impl Request for GetAtomNameRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetAtomNameReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetAtomNameReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub name: String,
}
impl GetAtomNameReply {}
impl AsByteSequence for GetAtomNameReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetAtomNameReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetAtomNameReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangePropertyRequest {
    pub req_type: u8,
    pub mode: PropMode,
    pub length: u16,
    pub window: Window,
    pub property: Atom,
    pub ty: Atom,
    pub format: Card8,
    pub data_len: Card32,
    pub data: Vec<Void>,
}
impl ChangePropertyRequest {}
impl AsByteSequence for ChangePropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.data_len.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangePropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (PropMode, usize) = <PropMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (data_len, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, block_len): (Vec<Void>, usize) = vector_from_bytes(
            &bytes[index..],
            (((data_len as usize) * (format as usize)) / (8)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        Some((
            ChangePropertyRequest {
                req_type: req_type,
                mode: mode,
                length: length,
                window: window,
                property: property,
                ty: ty,
                format: format,
                data_len: data_len,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.mode.size()
            + self.length.size()
            + self.window.size()
            + self.property.size()
            + self.ty.size()
            + self.format.size()
            + 3
            + self.data_len.size()
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
    }
}
impl Request for ChangePropertyRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PropMode {
    Replace = 0,
    Prepend = 1,
    Append = 2,
}
impl AsByteSequence for PropMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Replace, sz)),
            1 => Some((Self::Prepend, sz)),
            2 => Some((Self::Append, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for PropMode {
    #[inline]
    fn default() -> PropMode {
        PropMode::Replace
    }
}
#[derive(Clone, Debug, Default)]
pub struct DeletePropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub property: Atom,
}
impl DeletePropertyRequest {}
impl AsByteSequence for DeletePropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeletePropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeletePropertyRequest {
                req_type: req_type,
                length: length,
                window: window,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size() + self.property.size()
    }
}
impl Request for DeletePropertyRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetPropertyRequest {
    pub req_type: u8,
    pub delete: bool,
    pub length: u16,
    pub window: Window,
    pub property: Atom,
    pub ty: Atom,
    pub long_offset: Card32,
    pub long_length: Card32,
}
impl GetPropertyRequest {}
impl AsByteSequence for GetPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.delete.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.long_offset.as_bytes(&mut bytes[index..]);
        index += self.long_length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delete, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (long_offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (long_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPropertyRequest {
                req_type: req_type,
                delete: delete,
                length: length,
                window: window,
                property: property,
                ty: ty,
                long_offset: long_offset,
                long_length: long_length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.delete.size()
            + self.length.size()
            + self.window.size()
            + self.property.size()
            + self.ty.size()
            + self.long_offset.size()
            + self.long_length.size()
    }
}
impl Request for GetPropertyRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPropertyReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPropertyReply {
    pub reply_type: u8,
    pub format: Card8,
    pub sequence: u16,
    pub length: u32,
    pub ty: Atom,
    pub bytes_after: Card32,
    pub value_len: Card32,
    pub value: Vec<Void>,
}
impl GetPropertyReply {}
impl AsByteSequence for GetPropertyReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.bytes_after.as_bytes(&mut bytes[index..]);
        index += self.value_len.as_bytes(&mut bytes[index..]);
        index += 12;
        let block_len: usize = vector_as_bytes(&self.value, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPropertyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_after, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_len, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        let (value, block_len): (Vec<Void>, usize) = vector_from_bytes(
            &bytes[index..],
            ((value_len as usize) * ((format as usize) / (8))) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        Some((
            GetPropertyReply {
                reply_type: reply_type,
                format: format,
                sequence: sequence,
                length: length,
                ty: ty,
                bytes_after: bytes_after,
                value_len: value_len,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.format.size()
            + self.sequence.size()
            + self.length.size()
            + self.ty.size()
            + self.bytes_after.size()
            + self.value_len.size()
            + 12
            + {
                let block_len: usize = self.value.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GetPropertyType {
    Any = 0,
}
impl AsByteSequence for GetPropertyType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Any, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for GetPropertyType {
    #[inline]
    fn default() -> GetPropertyType {
        GetPropertyType::Any
    }
}
#[derive(Clone, Debug, Default)]
pub struct ListPropertiesRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl ListPropertiesRequest {}
impl AsByteSequence for ListPropertiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListPropertiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListPropertiesRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for ListPropertiesRequest {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListPropertiesReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListPropertiesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Vec<Atom>,
}
impl ListPropertiesReply {}
impl AsByteSequence for ListPropertiesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.atoms.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.atoms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListPropertiesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (atoms, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            ListPropertiesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                atoms: atoms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.atoms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetSelectionOwnerRequest {
    pub req_type: u8,
    pub length: u16,
    pub owner: Window,
    pub selection: Atom,
    pub time: Timestamp,
}
impl SetSelectionOwnerRequest {}
impl AsByteSequence for SetSelectionOwnerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.owner.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetSelectionOwnerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetSelectionOwnerRequest {
                req_type: req_type,
                length: length,
                owner: owner,
                selection: selection,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.owner.size()
            + self.selection.size()
            + self.time.size()
    }
}
impl Request for SetSelectionOwnerRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetSelectionOwnerRequest {
    pub req_type: u8,
    pub length: u16,
    pub selection: Atom,
}
impl GetSelectionOwnerRequest {}
impl AsByteSequence for GetSelectionOwnerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionOwnerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSelectionOwnerRequest {
                req_type: req_type,
                length: length,
                selection: selection,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.selection.size()
    }
}
impl Request for GetSelectionOwnerRequest {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSelectionOwnerReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetSelectionOwnerReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub owner: Window,
}
impl GetSelectionOwnerReply {}
impl AsByteSequence for GetSelectionOwnerReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.owner.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectionOwnerReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSelectionOwnerReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                owner: owner,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.owner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct ConvertSelectionRequest {
    pub req_type: u8,
    pub length: u16,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
    pub time: Timestamp,
}
impl ConvertSelectionRequest {}
impl AsByteSequence for ConvertSelectionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.requestor.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConvertSelectionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (requestor, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ConvertSelectionRequest {
                req_type: req_type,
                length: length,
                requestor: requestor,
                selection: selection,
                target: target,
                property: property,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.requestor.size()
            + self.selection.size()
            + self.target.size()
            + self.property.size()
            + self.time.size()
    }
}
impl Request for ConvertSelectionRequest {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SendEventRequest {
    pub req_type: u8,
    pub propagate: bool,
    pub length: u16,
    pub destination: Window,
    pub event_mask: EventMask,
    pub event: [c_char; 32],
}
impl SendEventRequest {}
impl AsByteSequence for SendEventRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.propagate.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SendEventRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (propagate, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): ([c_char; 32], usize) = <[c_char; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SendEventRequest {
                req_type: req_type,
                propagate: propagate,
                length: length,
                destination: destination,
                event_mask: event_mask,
                event: event,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.propagate.size()
            + self.length.size()
            + self.destination.size()
            + self.event_mask.size()
            + self.event.size()
    }
}
impl Request for SendEventRequest {
    const OPCODE: u8 = 25;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SendEventDest {
    PointerWindow = 0,
    ItemFocus = 1,
}
impl AsByteSequence for SendEventDest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::PointerWindow, sz)),
            1 => Some((Self::ItemFocus, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for SendEventDest {
    #[inline]
    fn default() -> SendEventDest {
        SendEventDest::PointerWindow
    }
}
#[derive(Clone, Debug, Default)]
pub struct GrabPointerRequest {
    pub req_type: u8,
    pub owner_events: bool,
    pub length: u16,
    pub grab_window: Window,
    pub event_mask: EventMask,
    pub pointer_mode: GrabMode,
    pub keyboard_mode: GrabMode,
    pub confine_to: Window,
    pub cursor: Cursor,
    pub time: Timestamp,
}
impl GrabPointerRequest {}
impl AsByteSequence for GrabPointerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index += self.pointer_mode.as_bytes(&mut bytes[index..]);
        index += self.keyboard_mode.as_bytes(&mut bytes[index..]);
        index += self.confine_to.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pointer_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keyboard_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (confine_to, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GrabPointerRequest {
                req_type: req_type,
                owner_events: owner_events,
                length: length,
                grab_window: grab_window,
                event_mask: event_mask,
                pointer_mode: pointer_mode,
                keyboard_mode: keyboard_mode,
                confine_to: confine_to,
                cursor: cursor,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.owner_events.size()
            + self.length.size()
            + self.grab_window.size()
            + self.event_mask.size()
            + self.pointer_mode.size()
            + self.keyboard_mode.size()
            + self.confine_to.size()
            + self.cursor.size()
            + self.time.size()
    }
}
impl Request for GrabPointerRequest {
    const OPCODE: u8 = 26;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GrabPointerReply;
}
#[derive(Clone, Debug, Default)]
pub struct GrabPointerReply {
    pub reply_type: u8,
    pub status: GrabStatus,
    pub sequence: u16,
    pub length: u32,
}
impl GrabPointerReply {}
impl AsByteSequence for GrabPointerReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabPointerReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GrabPointerReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.status.size() + self.sequence.size() + self.length.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrabMode {
    Sync = 0,
    Async = 1,
}
impl AsByteSequence for GrabMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Sync, sz)),
            1 => Some((Self::Async, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for GrabMode {
    #[inline]
    fn default() -> GrabMode {
        GrabMode::Sync
    }
}
pub const CURSOR_NONE: Cursor = <Cursor>::const_from_xid(0);
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum GrabStatus {
    Success = 0,
    AlreadyGrabbed = 1,
    InvalidTime = 2,
    NotViewable = 3,
    Frozen = 4,
}
impl AsByteSequence for GrabStatus {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Success, sz)),
            1 => Some((Self::AlreadyGrabbed, sz)),
            2 => Some((Self::InvalidTime, sz)),
            3 => Some((Self::NotViewable, sz)),
            4 => Some((Self::Frozen, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for GrabStatus {
    #[inline]
    fn default() -> GrabStatus {
        GrabStatus::Success
    }
}
#[derive(Clone, Debug, Default)]
pub struct UngrabPointerRequest {
    pub req_type: u8,
    pub length: u16,
    pub time: Timestamp,
}
impl UngrabPointerRequest {}
impl AsByteSequence for UngrabPointerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UngrabPointerRequest {
                req_type: req_type,
                length: length,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.time.size()
    }
}
impl Request for UngrabPointerRequest {
    const OPCODE: u8 = 27;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GrabButtonRequest {
    pub req_type: u8,
    pub owner_events: bool,
    pub length: u16,
    pub grab_window: Window,
    pub event_mask: EventMask,
    pub pointer_mode: GrabMode,
    pub keyboard_mode: GrabMode,
    pub confine_to: Window,
    pub cursor: Cursor,
    pub button: ButtonIndex,
    pub modifiers: ModMask,
}
impl GrabButtonRequest {}
impl AsByteSequence for GrabButtonRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index += self.pointer_mode.as_bytes(&mut bytes[index..]);
        index += self.keyboard_mode.as_bytes(&mut bytes[index..]);
        index += self.confine_to.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.button.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabButtonRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pointer_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keyboard_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (confine_to, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button, sz): (ButtonIndex, usize) = <ButtonIndex>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GrabButtonRequest {
                req_type: req_type,
                owner_events: owner_events,
                length: length,
                grab_window: grab_window,
                event_mask: event_mask,
                pointer_mode: pointer_mode,
                keyboard_mode: keyboard_mode,
                confine_to: confine_to,
                cursor: cursor,
                button: button,
                modifiers: modifiers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.owner_events.size()
            + self.length.size()
            + self.grab_window.size()
            + self.event_mask.size()
            + self.pointer_mode.size()
            + self.keyboard_mode.size()
            + self.confine_to.size()
            + self.cursor.size()
            + self.button.size()
            + 1
            + self.modifiers.size()
    }
}
impl Request for GrabButtonRequest {
    const OPCODE: u8 = 28;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ButtonIndex {
    Any = 0,
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
}
impl AsByteSequence for ButtonIndex {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Any, sz)),
            1 => Some((Self::One, sz)),
            2 => Some((Self::Two, sz)),
            3 => Some((Self::Three, sz)),
            4 => Some((Self::Four, sz)),
            5 => Some((Self::Five, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ButtonIndex {
    #[inline]
    fn default() -> ButtonIndex {
        ButtonIndex::Any
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ModMask {
    pub inner: u16,
}
impl ModMask {
    #[inline]
    pub fn shift(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_shift(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn lock(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn control(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_control(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn One(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_One(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn Two(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_Two(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn Three(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_Three(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn Four(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_Four(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn Five(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_Five(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn any(&self) -> bool {
        self.inner & (1 << 15) != 0
    }
    #[inline]
    pub fn set_any(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 15;
        } else {
            self.inner &= !(1 << 15);
        }
        self
    }
    #[inline]
    pub fn new(
        shift: bool,
        lock: bool,
        control: bool,
        One: bool,
        Two: bool,
        Three: bool,
        Four: bool,
        Five: bool,
        any: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if shift {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if lock {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if control {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if One {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        if Two {
            inner |= 1 << 4;
        } else {
            inner &= !(1 << 4);
        }
        if Three {
            inner |= 1 << 5;
        } else {
            inner &= !(1 << 5);
        }
        if Four {
            inner |= 1 << 6;
        } else {
            inner &= !(1 << 6);
        }
        if Five {
            inner |= 1 << 7;
        } else {
            inner &= !(1 << 7);
        }
        if any {
            inner |= 1 << 15;
        } else {
            inner &= !(1 << 15);
        }
        ModMask { inner: inner }
    }
}
impl AsByteSequence for ModMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((ModMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct UngrabButtonRequest {
    pub req_type: u8,
    pub button: ButtonIndex,
    pub length: u16,
    pub grab_window: Window,
    pub modifiers: ModMask,
}
impl UngrabButtonRequest {}
impl AsByteSequence for UngrabButtonRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.button.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabButtonRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button, sz): (ButtonIndex, usize) = <ButtonIndex>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            UngrabButtonRequest {
                req_type: req_type,
                button: button,
                length: length,
                grab_window: grab_window,
                modifiers: modifiers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.button.size()
            + self.length.size()
            + self.grab_window.size()
            + self.modifiers.size()
            + 2
    }
}
impl Request for UngrabButtonRequest {
    const OPCODE: u8 = 29;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ChangeActivePointerGrabRequest {
    pub req_type: u8,
    pub length: u16,
    pub cursor: Cursor,
    pub time: Timestamp,
    pub event_mask: EventMask,
}
impl ChangeActivePointerGrabRequest {}
impl AsByteSequence for ChangeActivePointerGrabRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.event_mask.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeActivePointerGrabRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mask, sz): (EventMask, usize) = <EventMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            ChangeActivePointerGrabRequest {
                req_type: req_type,
                length: length,
                cursor: cursor,
                time: time,
                event_mask: event_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cursor.size()
            + self.time.size()
            + self.event_mask.size()
            + 2
    }
}
impl Request for ChangeActivePointerGrabRequest {
    const OPCODE: u8 = 30;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GrabKeyboardRequest {
    pub req_type: u8,
    pub owner_events: bool,
    pub length: u16,
    pub grab_window: Window,
    pub time: Timestamp,
    pub pointer_mode: GrabMode,
    pub keyboard_mode: GrabMode,
}
impl GrabKeyboardRequest {}
impl AsByteSequence for GrabKeyboardRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.pointer_mode.as_bytes(&mut bytes[index..]);
        index += self.keyboard_mode.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabKeyboardRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pointer_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keyboard_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GrabKeyboardRequest {
                req_type: req_type,
                owner_events: owner_events,
                length: length,
                grab_window: grab_window,
                time: time,
                pointer_mode: pointer_mode,
                keyboard_mode: keyboard_mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.owner_events.size()
            + self.length.size()
            + self.grab_window.size()
            + self.time.size()
            + self.pointer_mode.size()
            + self.keyboard_mode.size()
            + 2
    }
}
impl Request for GrabKeyboardRequest {
    const OPCODE: u8 = 31;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GrabKeyboardReply;
}
#[derive(Clone, Debug, Default)]
pub struct GrabKeyboardReply {
    pub reply_type: u8,
    pub status: GrabStatus,
    pub sequence: u16,
    pub length: u32,
}
impl GrabKeyboardReply {}
impl AsByteSequence for GrabKeyboardReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabKeyboardReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GrabKeyboardReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.status.size() + self.sequence.size() + self.length.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct UngrabKeyboardRequest {
    pub req_type: u8,
    pub length: u16,
    pub time: Timestamp,
}
impl UngrabKeyboardRequest {}
impl AsByteSequence for UngrabKeyboardRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabKeyboardRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UngrabKeyboardRequest {
                req_type: req_type,
                length: length,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.time.size()
    }
}
impl Request for UngrabKeyboardRequest {
    const OPCODE: u8 = 32;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GrabKeyRequest {
    pub req_type: u8,
    pub owner_events: bool,
    pub length: u16,
    pub grab_window: Window,
    pub modifiers: ModMask,
    pub key: Keycode,
    pub pointer_mode: GrabMode,
    pub keyboard_mode: GrabMode,
}
impl GrabKeyRequest {}
impl AsByteSequence for GrabKeyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += self.key.as_bytes(&mut bytes[index..]);
        index += self.pointer_mode.as_bytes(&mut bytes[index..]);
        index += self.keyboard_mode.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabKeyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pointer_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keyboard_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GrabKeyRequest {
                req_type: req_type,
                owner_events: owner_events,
                length: length,
                grab_window: grab_window,
                modifiers: modifiers,
                key: key,
                pointer_mode: pointer_mode,
                keyboard_mode: keyboard_mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.owner_events.size()
            + self.length.size()
            + self.grab_window.size()
            + self.modifiers.size()
            + self.key.size()
            + self.pointer_mode.size()
            + self.keyboard_mode.size()
            + 3
    }
}
impl Request for GrabKeyRequest {
    const OPCODE: u8 = 33;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Grab {
    Any = 0,
}
impl AsByteSequence for Grab {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Any, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Grab {
    #[inline]
    fn default() -> Grab {
        Grab::Any
    }
}
#[derive(Clone, Debug, Default)]
pub struct UngrabKeyRequest {
    pub req_type: u8,
    pub key: Keycode,
    pub length: u16,
    pub grab_window: Window,
    pub modifiers: ModMask,
}
impl UngrabKeyRequest {}
impl AsByteSequence for UngrabKeyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.key.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabKeyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            UngrabKeyRequest {
                req_type: req_type,
                key: key,
                length: length,
                grab_window: grab_window,
                modifiers: modifiers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.key.size()
            + self.length.size()
            + self.grab_window.size()
            + self.modifiers.size()
            + 2
    }
}
impl Request for UngrabKeyRequest {
    const OPCODE: u8 = 34;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct AllowEventsRequest {
    pub req_type: u8,
    pub mode: Allow,
    pub length: u16,
    pub time: Timestamp,
}
impl AllowEventsRequest {}
impl AsByteSequence for AllowEventsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllowEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (Allow, usize) = <Allow>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AllowEventsRequest {
                req_type: req_type,
                mode: mode,
                length: length,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.mode.size() + self.length.size() + self.time.size()
    }
}
impl Request for AllowEventsRequest {
    const OPCODE: u8 = 35;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Allow {
    AsyncPointer = 0,
    SyncPointer = 1,
    ReplayPointer = 2,
    AsyncKeyboard = 3,
    SyncKeyboard = 4,
    ReplayKeyboard = 5,
    AsyncBoth = 6,
    SyncBoth = 7,
}
impl AsByteSequence for Allow {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::AsyncPointer, sz)),
            1 => Some((Self::SyncPointer, sz)),
            2 => Some((Self::ReplayPointer, sz)),
            3 => Some((Self::AsyncKeyboard, sz)),
            4 => Some((Self::SyncKeyboard, sz)),
            5 => Some((Self::ReplayKeyboard, sz)),
            6 => Some((Self::AsyncBoth, sz)),
            7 => Some((Self::SyncBoth, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Allow {
    #[inline]
    fn default() -> Allow {
        Allow::AsyncPointer
    }
}
#[derive(Clone, Debug, Default)]
pub struct GrabServerRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GrabServerRequest {}
impl AsByteSequence for GrabServerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabServerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GrabServerRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GrabServerRequest {
    const OPCODE: u8 = 36;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UngrabServerRequest {
    pub req_type: u8,
    pub length: u16,
}
impl UngrabServerRequest {}
impl AsByteSequence for UngrabServerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabServerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UngrabServerRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for UngrabServerRequest {
    const OPCODE: u8 = 37;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct QueryPointerRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl QueryPointerRequest {}
impl AsByteSequence for QueryPointerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryPointerRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for QueryPointerRequest {
    const OPCODE: u8 = 38;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryPointerReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryPointerReply {
    pub reply_type: u8,
    pub same_screen: bool,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub win_x: Int16,
    pub win_y: Int16,
    pub mask: KeyButMask,
}
impl QueryPointerReply {}
impl AsByteSequence for QueryPointerReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.win_x.as_bytes(&mut bytes[index..]);
        index += self.win_y.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryPointerReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (win_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (win_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            QueryPointerReply {
                reply_type: reply_type,
                same_screen: same_screen,
                sequence: sequence,
                length: length,
                root: root,
                child: child,
                root_x: root_x,
                root_y: root_y,
                win_x: win_x,
                win_y: win_y,
                mask: mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.same_screen.size()
            + self.sequence.size()
            + self.length.size()
            + self.root.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.win_x.size()
            + self.win_y.size()
            + self.mask.size()
            + 2
    }
}
#[derive(Clone, Debug, Default)]
pub struct Timecoord {
    pub time: Timestamp,
    pub x: Int16,
    pub y: Int16,
}
impl Timecoord {}
impl AsByteSequence for Timecoord {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Timecoord from byte buffer");
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Timecoord {
                time: time,
                x: x,
                y: y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.time.size() + self.x.size() + self.y.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMotionEventsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub start: Timestamp,
    pub stop: Timestamp,
}
impl GetMotionEventsRequest {}
impl AsByteSequence for GetMotionEventsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.start.as_bytes(&mut bytes[index..]);
        index += self.stop.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMotionEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (start, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stop, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetMotionEventsRequest {
                req_type: req_type,
                length: length,
                window: window,
                start: start,
                stop: stop,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.window.size()
            + self.start.size()
            + self.stop.size()
    }
}
impl Request for GetMotionEventsRequest {
    const OPCODE: u8 = 39;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMotionEventsReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMotionEventsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub events: Vec<Timecoord>,
}
impl GetMotionEventsReply {}
impl AsByteSequence for GetMotionEventsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.events.len() as Card32).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.events, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Timecoord>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMotionEventsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (events, block_len): (Vec<Timecoord>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Timecoord>());
        Some((
            GetMotionEventsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                events: events,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card32>()
            + 20
            + {
                let block_len: usize = self.events.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Timecoord>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct TranslateCoordinatesRequest {
    pub req_type: u8,
    pub length: u16,
    pub src_window: Window,
    pub dst_window: Window,
    pub src_x: Int16,
    pub src_y: Int16,
}
impl TranslateCoordinatesRequest {}
impl AsByteSequence for TranslateCoordinatesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.src_window.as_bytes(&mut bytes[index..]);
        index += self.dst_window.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TranslateCoordinatesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            TranslateCoordinatesRequest {
                req_type: req_type,
                length: length,
                src_window: src_window,
                dst_window: dst_window,
                src_x: src_x,
                src_y: src_y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.src_window.size()
            + self.dst_window.size()
            + self.src_x.size()
            + self.src_y.size()
    }
}
impl Request for TranslateCoordinatesRequest {
    const OPCODE: u8 = 40;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = TranslateCoordinatesReply;
}
#[derive(Clone, Debug, Default)]
pub struct TranslateCoordinatesReply {
    pub reply_type: u8,
    pub same_screen: bool,
    pub sequence: u16,
    pub length: u32,
    pub child: Window,
    pub dst_x: Int16,
    pub dst_y: Int16,
}
impl TranslateCoordinatesReply {}
impl AsByteSequence for TranslateCoordinatesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TranslateCoordinatesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            TranslateCoordinatesReply {
                reply_type: reply_type,
                same_screen: same_screen,
                sequence: sequence,
                length: length,
                child: child,
                dst_x: dst_x,
                dst_y: dst_y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.same_screen.size()
            + self.sequence.size()
            + self.length.size()
            + self.child.size()
            + self.dst_x.size()
            + self.dst_y.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct WarpPointerRequest {
    pub req_type: u8,
    pub length: u16,
    pub src_window: Window,
    pub dst_window: Window,
    pub src_x: Int16,
    pub src_y: Int16,
    pub src_width: Card16,
    pub src_height: Card16,
    pub dst_x: Int16,
    pub dst_y: Int16,
}
impl WarpPointerRequest {}
impl AsByteSequence for WarpPointerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.src_window.as_bytes(&mut bytes[index..]);
        index += self.dst_window.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.src_width.as_bytes(&mut bytes[index..]);
        index += self.src_height.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WarpPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            WarpPointerRequest {
                req_type: req_type,
                length: length,
                src_window: src_window,
                dst_window: dst_window,
                src_x: src_x,
                src_y: src_y,
                src_width: src_width,
                src_height: src_height,
                dst_x: dst_x,
                dst_y: dst_y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.src_window.size()
            + self.dst_window.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.src_width.size()
            + self.src_height.size()
            + self.dst_x.size()
            + self.dst_y.size()
    }
}
impl Request for WarpPointerRequest {
    const OPCODE: u8 = 41;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetInputFocusRequest {
    pub req_type: u8,
    pub revert_to: InputFocus,
    pub length: u16,
    pub focus: Window,
    pub time: Timestamp,
}
impl SetInputFocusRequest {}
impl AsByteSequence for SetInputFocusRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.revert_to.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetInputFocusRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (revert_to, sz): (InputFocus, usize) = <InputFocus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetInputFocusRequest {
                req_type: req_type,
                revert_to: revert_to,
                length: length,
                focus: focus,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.revert_to.size()
            + self.length.size()
            + self.focus.size()
            + self.time.size()
    }
}
impl Request for SetInputFocusRequest {
    const OPCODE: u8 = 42;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum InputFocus {
    None = 0,
    PointerRoot = 1,
    Parent = 2,
    FollowKeyboard = 3,
}
impl AsByteSequence for InputFocus {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::None, sz)),
            1 => Some((Self::PointerRoot, sz)),
            2 => Some((Self::Parent, sz)),
            3 => Some((Self::FollowKeyboard, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for InputFocus {
    #[inline]
    fn default() -> InputFocus {
        InputFocus::None
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetInputFocusRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetInputFocusRequest {}
impl AsByteSequence for GetInputFocusRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetInputFocusRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetInputFocusRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GetInputFocusRequest {
    const OPCODE: u8 = 43;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetInputFocusReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetInputFocusReply {
    pub reply_type: u8,
    pub revert_to: InputFocus,
    pub sequence: u16,
    pub length: u32,
    pub focus: Window,
}
impl GetInputFocusReply {}
impl AsByteSequence for GetInputFocusReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.revert_to.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetInputFocusReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (revert_to, sz): (InputFocus, usize) = <InputFocus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetInputFocusReply {
                reply_type: reply_type,
                revert_to: revert_to,
                sequence: sequence,
                length: length,
                focus: focus,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.revert_to.size()
            + self.sequence.size()
            + self.length.size()
            + self.focus.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryKeymapRequest {
    pub req_type: u8,
    pub length: u16,
}
impl QueryKeymapRequest {}
impl AsByteSequence for QueryKeymapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryKeymapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryKeymapRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for QueryKeymapRequest {
    const OPCODE: u8 = 44;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryKeymapReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryKeymapReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub keys: [Card8; 32],
}
impl QueryKeymapReply {}
impl AsByteSequence for QueryKeymapReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.keys.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryKeymapReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keys, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryKeymapReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                keys: keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + 1 + self.sequence.size() + self.length.size() + self.keys.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct OpenFontRequest {
    pub req_type: u8,
    pub length: u16,
    pub fid: Font,
    pub name: String,
}
impl OpenFontRequest {}
impl AsByteSequence for OpenFontRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.fid.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OpenFontRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fid, sz): (Font, usize) = <Font>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            OpenFontRequest {
                req_type: req_type,
                length: length,
                fid: fid,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.fid.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for OpenFontRequest {
    const OPCODE: u8 = 45;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CloseFontRequest {
    pub req_type: u8,
    pub length: u16,
    pub font: Font,
}
impl CloseFontRequest {}
impl AsByteSequence for CloseFontRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.font.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CloseFontRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font, sz): (Font, usize) = <Font>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CloseFontRequest {
                req_type: req_type,
                length: length,
                font: font,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.font.size()
    }
}
impl Request for CloseFontRequest {
    const OPCODE: u8 = 46;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct Fontprop {
    pub name: Atom,
    pub value: Card32,
}
impl Fontprop {}
impl AsByteSequence for Fontprop {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.name.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Fontprop from byte buffer");
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Fontprop {
                name: name,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.name.size() + self.value.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Charinfo {
    pub left_side_bearing: Int16,
    pub right_side_bearing: Int16,
    pub character_width: Int16,
    pub ascent: Int16,
    pub descent: Int16,
    pub attributes: Card16,
}
impl Charinfo {}
impl AsByteSequence for Charinfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.left_side_bearing.as_bytes(&mut bytes[index..]);
        index += self.right_side_bearing.as_bytes(&mut bytes[index..]);
        index += self.character_width.as_bytes(&mut bytes[index..]);
        index += self.ascent.as_bytes(&mut bytes[index..]);
        index += self.descent.as_bytes(&mut bytes[index..]);
        index += self.attributes.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Charinfo from byte buffer");
        let (left_side_bearing, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (right_side_bearing, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (character_width, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ascent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (descent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attributes, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Charinfo {
                left_side_bearing: left_side_bearing,
                right_side_bearing: right_side_bearing,
                character_width: character_width,
                ascent: ascent,
                descent: descent,
                attributes: attributes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.left_side_bearing.size()
            + self.right_side_bearing.size()
            + self.character_width.size()
            + self.ascent.size()
            + self.descent.size()
            + self.attributes.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryFontRequest {
    pub req_type: u8,
    pub length: u16,
    pub font: Fontable,
}
impl QueryFontRequest {}
impl AsByteSequence for QueryFontRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.font.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryFontRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font, sz): (Fontable, usize) = <Fontable>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryFontRequest {
                req_type: req_type,
                length: length,
                font: font,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.font.size()
    }
}
impl Request for QueryFontRequest {
    const OPCODE: u8 = 47;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryFontReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryFontReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: Charinfo,
    pub max_bounds: Charinfo,
    pub min_char_or_byte2: Card16,
    pub max_char_or_byte2: Card16,
    pub default_char: Card16,
    pub draw_direction: FontDraw,
    pub min_byte1: Card8,
    pub max_byte1: Card8,
    pub all_chars_exist: bool,
    pub font_ascent: Int16,
    pub font_descent: Int16,
    pub properties: Vec<Fontprop>,
    pub char_infos: Vec<Charinfo>,
}
impl QueryFontReply {}
impl AsByteSequence for QueryFontReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.min_bounds.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.max_bounds.as_bytes(&mut bytes[index..]);
        index += self.min_char_or_byte2.as_bytes(&mut bytes[index..]);
        index += self.max_char_or_byte2.as_bytes(&mut bytes[index..]);
        index += self.default_char.as_bytes(&mut bytes[index..]);
        index += (self.properties.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.draw_direction.as_bytes(&mut bytes[index..]);
        index += self.min_byte1.as_bytes(&mut bytes[index..]);
        index += self.max_byte1.as_bytes(&mut bytes[index..]);
        index += self.all_chars_exist.as_bytes(&mut bytes[index..]);
        index += self.font_ascent.as_bytes(&mut bytes[index..]);
        index += self.font_descent.as_bytes(&mut bytes[index..]);
        index += (self.char_infos.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.properties, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fontprop>());
        let block_len: usize = vector_as_bytes(&self.char_infos, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Charinfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryFontReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_bounds, sz): (Charinfo, usize) = <Charinfo>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (max_bounds, sz): (Charinfo, usize) = <Charinfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_char_or_byte2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_char_or_byte2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (default_char, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (draw_direction, sz): (FontDraw, usize) = <FontDraw>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_byte1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_byte1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (all_chars_exist, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font_ascent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font_descent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (properties, block_len): (Vec<Fontprop>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fontprop>());
        let (char_infos, block_len): (Vec<Charinfo>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Charinfo>());
        Some((
            QueryFontReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                min_bounds: min_bounds,
                max_bounds: max_bounds,
                min_char_or_byte2: min_char_or_byte2,
                max_char_or_byte2: max_char_or_byte2,
                default_char: default_char,
                draw_direction: draw_direction,
                min_byte1: min_byte1,
                max_byte1: max_byte1,
                all_chars_exist: all_chars_exist,
                font_ascent: font_ascent,
                font_descent: font_descent,
                properties: properties,
                char_infos: char_infos,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.min_bounds.size()
            + 4
            + self.max_bounds.size()
            + self.min_char_or_byte2.size()
            + self.max_char_or_byte2.size()
            + self.default_char.size()
            + ::core::mem::size_of::<Card16>()
            + self.draw_direction.size()
            + self.min_byte1.size()
            + self.max_byte1.size()
            + self.all_chars_exist.size()
            + self.font_ascent.size()
            + self.font_descent.size()
            + ::core::mem::size_of::<Card32>()
            + {
                let block_len: usize = self.properties.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fontprop>());
                block_len + pad
            }
            + {
                let block_len: usize = self.char_infos.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Charinfo>());
                block_len + pad
            }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FontDraw {
    LeftToRight = 0,
    RightToLeft = 1,
}
impl AsByteSequence for FontDraw {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::LeftToRight, sz)),
            1 => Some((Self::RightToLeft, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for FontDraw {
    #[inline]
    fn default() -> FontDraw {
        FontDraw::LeftToRight
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryTextExtentsRequest {
    pub req_type: u8,
    pub font: Fontable,
    pub length: u16,
    pub string: Vec<Char2b>,
}
impl QueryTextExtentsRequest {}
impl AsByteSequence for QueryTextExtentsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.font.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Char2b>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryTextExtentsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font, sz): (Fontable, usize) = <Fontable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string, block_len): (Vec<Char2b>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Char2b>());
        Some((
            QueryTextExtentsRequest {
                req_type: req_type,
                font: font,
                length: length,
                string: string,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.font.size() + self.length.size() + {
            let block_len: usize = self.string.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Char2b>());
            block_len + pad
        }
    }
}
impl Request for QueryTextExtentsRequest {
    const OPCODE: u8 = 48;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryTextExtentsReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryTextExtentsReply {
    pub reply_type: u8,
    pub draw_direction: FontDraw,
    pub sequence: u16,
    pub length: u32,
    pub font_ascent: Int16,
    pub font_descent: Int16,
    pub overall_ascent: Int16,
    pub overall_descent: Int16,
    pub overall_width: Int32,
    pub overall_left: Int32,
    pub overall_right: Int32,
}
impl QueryTextExtentsReply {}
impl AsByteSequence for QueryTextExtentsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.draw_direction.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.font_ascent.as_bytes(&mut bytes[index..]);
        index += self.font_descent.as_bytes(&mut bytes[index..]);
        index += self.overall_ascent.as_bytes(&mut bytes[index..]);
        index += self.overall_descent.as_bytes(&mut bytes[index..]);
        index += self.overall_width.as_bytes(&mut bytes[index..]);
        index += self.overall_left.as_bytes(&mut bytes[index..]);
        index += self.overall_right.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryTextExtentsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (draw_direction, sz): (FontDraw, usize) = <FontDraw>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font_ascent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font_descent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (overall_ascent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (overall_descent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (overall_width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (overall_left, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (overall_right, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryTextExtentsReply {
                reply_type: reply_type,
                draw_direction: draw_direction,
                sequence: sequence,
                length: length,
                font_ascent: font_ascent,
                font_descent: font_descent,
                overall_ascent: overall_ascent,
                overall_descent: overall_descent,
                overall_width: overall_width,
                overall_left: overall_left,
                overall_right: overall_right,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.draw_direction.size()
            + self.sequence.size()
            + self.length.size()
            + self.font_ascent.size()
            + self.font_descent.size()
            + self.overall_ascent.size()
            + self.overall_descent.size()
            + self.overall_width.size()
            + self.overall_left.size()
            + self.overall_right.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct Str {
    pub name: String,
}
impl Str {}
impl AsByteSequence for Str {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += (self.name.len() as Card8).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Str from byte buffer");
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((Str { name: name }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<Card8>() + {
            let block_len: usize = self.name.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ListFontsRequest {
    pub req_type: u8,
    pub length: u16,
    pub max_names: Card16,
    pub pattern: String,
}
impl ListFontsRequest {}
impl AsByteSequence for ListFontsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.max_names.as_bytes(&mut bytes[index..]);
        index += (self.pattern.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.pattern, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListFontsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_names, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pattern, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            ListFontsRequest {
                req_type: req_type,
                length: length,
                max_names: max_names,
                pattern: pattern,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.max_names.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.pattern.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for ListFontsRequest {
    const OPCODE: u8 = 49;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListFontsReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListFontsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub names: Vec<Str>,
}
impl ListFontsReply {}
impl AsByteSequence for ListFontsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.names.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListFontsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (names, block_len): (Vec<Str>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        Some((
            ListFontsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                names: names,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Str>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ListFontsWithInfoRequest {
    pub req_type: u8,
    pub length: u16,
    pub max_names: Card16,
    pub pattern: String,
}
impl ListFontsWithInfoRequest {}
impl AsByteSequence for ListFontsWithInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.max_names.as_bytes(&mut bytes[index..]);
        index += (self.pattern.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.pattern, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListFontsWithInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_names, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pattern, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            ListFontsWithInfoRequest {
                req_type: req_type,
                length: length,
                max_names: max_names,
                pattern: pattern,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.max_names.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.pattern.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for ListFontsWithInfoRequest {
    const OPCODE: u8 = 50;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListFontsWithInfoReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListFontsWithInfoReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub min_bounds: Charinfo,
    pub max_bounds: Charinfo,
    pub min_char_or_byte2: Card16,
    pub max_char_or_byte2: Card16,
    pub default_char: Card16,
    pub draw_direction: FontDraw,
    pub min_byte1: Card8,
    pub max_byte1: Card8,
    pub all_chars_exist: bool,
    pub font_ascent: Int16,
    pub font_descent: Int16,
    pub replies_hint: Card32,
    pub properties: Vec<Fontprop>,
    pub name: String,
}
impl ListFontsWithInfoReply {}
impl AsByteSequence for ListFontsWithInfoReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.min_bounds.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.max_bounds.as_bytes(&mut bytes[index..]);
        index += self.min_char_or_byte2.as_bytes(&mut bytes[index..]);
        index += self.max_char_or_byte2.as_bytes(&mut bytes[index..]);
        index += self.default_char.as_bytes(&mut bytes[index..]);
        index += (self.properties.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.draw_direction.as_bytes(&mut bytes[index..]);
        index += self.min_byte1.as_bytes(&mut bytes[index..]);
        index += self.max_byte1.as_bytes(&mut bytes[index..]);
        index += self.all_chars_exist.as_bytes(&mut bytes[index..]);
        index += self.font_ascent.as_bytes(&mut bytes[index..]);
        index += self.font_descent.as_bytes(&mut bytes[index..]);
        index += self.replies_hint.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.properties, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fontprop>());
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListFontsWithInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_bounds, sz): (Charinfo, usize) = <Charinfo>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (max_bounds, sz): (Charinfo, usize) = <Charinfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_char_or_byte2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_char_or_byte2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (default_char, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (draw_direction, sz): (FontDraw, usize) = <FontDraw>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_byte1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_byte1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (all_chars_exist, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font_ascent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (font_descent, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (replies_hint, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (properties, block_len): (Vec<Fontprop>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fontprop>());
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            ListFontsWithInfoReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                min_bounds: min_bounds,
                max_bounds: max_bounds,
                min_char_or_byte2: min_char_or_byte2,
                max_char_or_byte2: max_char_or_byte2,
                default_char: default_char,
                draw_direction: draw_direction,
                min_byte1: min_byte1,
                max_byte1: max_byte1,
                all_chars_exist: all_chars_exist,
                font_ascent: font_ascent,
                font_descent: font_descent,
                replies_hint: replies_hint,
                properties: properties,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + ::core::mem::size_of::<Card8>()
            + self.sequence.size()
            + self.length.size()
            + self.min_bounds.size()
            + 4
            + self.max_bounds.size()
            + self.min_char_or_byte2.size()
            + self.max_char_or_byte2.size()
            + self.default_char.size()
            + ::core::mem::size_of::<Card16>()
            + self.draw_direction.size()
            + self.min_byte1.size()
            + self.max_byte1.size()
            + self.all_chars_exist.size()
            + self.font_ascent.size()
            + self.font_descent.size()
            + self.replies_hint.size()
            + {
                let block_len: usize = self.properties.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fontprop>());
                block_len + pad
            }
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetFontPathRequest {
    pub req_type: u8,
    pub length: u16,
    pub font: Vec<Str>,
}
impl SetFontPathRequest {}
impl AsByteSequence for SetFontPathRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.font.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.font, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetFontPathRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (font, block_len): (Vec<Str>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        Some((
            SetFontPathRequest {
                req_type: req_type,
                length: length,
                font: font,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card16>() + 2 + {
            let block_len: usize = self.font.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Str>());
            block_len + pad
        }
    }
}
impl Request for SetFontPathRequest {
    const OPCODE: u8 = 51;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetFontPathRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetFontPathRequest {}
impl AsByteSequence for GetFontPathRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFontPathRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetFontPathRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GetFontPathRequest {
    const OPCODE: u8 = 52;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetFontPathReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetFontPathReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub path: Vec<Str>,
}
impl GetFontPathReply {}
impl AsByteSequence for GetFontPathReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.path.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.path, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFontPathReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (path, block_len): (Vec<Str>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        Some((
            GetFontPathReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                path: path,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.path.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Str>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreatePixmapRequest {
    pub req_type: u8,
    pub depth: Card8,
    pub length: u16,
    pub pid: Pixmap,
    pub drawable: Drawable,
    pub width: Card16,
    pub height: Card16,
}
impl CreatePixmapRequest {}
impl AsByteSequence for CreatePixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pid.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreatePixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pid, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreatePixmapRequest {
                req_type: req_type,
                depth: depth,
                length: length,
                pid: pid,
                drawable: drawable,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.depth.size()
            + self.length.size()
            + self.pid.size()
            + self.drawable.size()
            + self.width.size()
            + self.height.size()
    }
}
impl Request for CreatePixmapRequest {
    const OPCODE: u8 = 53;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FreePixmapRequest {
    pub req_type: u8,
    pub length: u16,
    pub pixmap: Pixmap,
}
impl FreePixmapRequest {}
impl AsByteSequence for FreePixmapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pixmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreePixmapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixmap, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreePixmapRequest {
                req_type: req_type,
                length: length,
                pixmap: pixmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.pixmap.size()
    }
}
impl Request for FreePixmapRequest {
    const OPCODE: u8 = 54;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateGcRequest {
    pub req_type: u8,
    pub length: u16,
    pub cid: Gcontext,
    pub drawable: Drawable,
    pub value_mask: Gc,
    pub function: Gx,
    pub plane_mask: Card32,
    pub foreground: Card32,
    pub background: Card32,
    pub line_width: Card32,
    pub line_style: LineStyle,
    pub cap_style: CapStyle,
    pub join_style: JoinStyle,
    pub fill_style: FillStyle,
    pub fill_rule: FillRule,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub tile_stipple_x_origin: Int32,
    pub tile_stipple_y_origin: Int32,
    pub font: Font,
    pub subwindow_mode: SubwindowMode,
    pub graphics_exposures: Bool32,
    pub clip_x_origin: Int32,
    pub clip_y_origin: Int32,
    pub clip_mask: Pixmap,
    pub dash_offset: Card32,
    pub dashes: Card32,
    pub arc_mode: ArcMode,
}
impl CreateGcRequest {}
impl AsByteSequence for CreateGcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cid.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.function() {
            index += self.function.as_bytes(&mut bytes[index..]);
        }
        if cond0.plane_mask() {
            index += self.plane_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.foreground() {
            index += self.foreground.as_bytes(&mut bytes[index..]);
        }
        if cond0.background() {
            index += self.background.as_bytes(&mut bytes[index..]);
        }
        if cond0.line_width() {
            index += self.line_width.as_bytes(&mut bytes[index..]);
        }
        if cond0.line_style() {
            index += self.line_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.cap_style() {
            index += self.cap_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.join_style() {
            index += self.join_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.fill_style() {
            index += self.fill_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.fill_rule() {
            index += self.fill_rule.as_bytes(&mut bytes[index..]);
        }
        if cond0.tile() {
            index += self.tile.as_bytes(&mut bytes[index..]);
        }
        if cond0.stipple() {
            index += self.stipple.as_bytes(&mut bytes[index..]);
        }
        if cond0.tile_stipple_origin_x() {
            index += self.tile_stipple_x_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.tile_stipple_origin_y() {
            index += self.tile_stipple_y_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.font() {
            index += self.font.as_bytes(&mut bytes[index..]);
        }
        if cond0.subwindow_mode() {
            index += self.subwindow_mode.as_bytes(&mut bytes[index..]);
        }
        if cond0.graphics_exposures() {
            index += self.graphics_exposures.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_origin_x() {
            index += self.clip_x_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_origin_y() {
            index += self.clip_y_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_mask() {
            index += self.clip_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.dash_offset() {
            index += self.dash_offset.as_bytes(&mut bytes[index..]);
        }
        if cond0.dash_list() {
            index += self.dashes.as_bytes(&mut bytes[index..]);
        }
        if cond0.arc_mode() {
            index += self.arc_mode.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateGcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cid, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Gc, usize) = <Gc>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let function: Gx = if cond0.function() {
            let (function, sz): (Gx, usize) = <Gx>::from_bytes(&bytes[index..])?;
            index += sz;
            function
        } else {
            Default::default()
        };
        let plane_mask: Card32 = if cond0.plane_mask() {
            let (plane_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            plane_mask
        } else {
            Default::default()
        };
        let foreground: Card32 = if cond0.foreground() {
            let (foreground, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            foreground
        } else {
            Default::default()
        };
        let background: Card32 = if cond0.background() {
            let (background, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            background
        } else {
            Default::default()
        };
        let line_width: Card32 = if cond0.line_width() {
            let (line_width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            line_width
        } else {
            Default::default()
        };
        let line_style: LineStyle = if cond0.line_style() {
            let (line_style, sz): (LineStyle, usize) = <LineStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            line_style
        } else {
            Default::default()
        };
        let cap_style: CapStyle = if cond0.cap_style() {
            let (cap_style, sz): (CapStyle, usize) = <CapStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            cap_style
        } else {
            Default::default()
        };
        let join_style: JoinStyle = if cond0.join_style() {
            let (join_style, sz): (JoinStyle, usize) = <JoinStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            join_style
        } else {
            Default::default()
        };
        let fill_style: FillStyle = if cond0.fill_style() {
            let (fill_style, sz): (FillStyle, usize) = <FillStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            fill_style
        } else {
            Default::default()
        };
        let fill_rule: FillRule = if cond0.fill_rule() {
            let (fill_rule, sz): (FillRule, usize) = <FillRule>::from_bytes(&bytes[index..])?;
            index += sz;
            fill_rule
        } else {
            Default::default()
        };
        let tile: Pixmap = if cond0.tile() {
            let (tile, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            tile
        } else {
            Default::default()
        };
        let stipple: Pixmap = if cond0.stipple() {
            let (stipple, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            stipple
        } else {
            Default::default()
        };
        let tile_stipple_x_origin: Int32 = if cond0.tile_stipple_origin_x() {
            let (tile_stipple_x_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            tile_stipple_x_origin
        } else {
            Default::default()
        };
        let tile_stipple_y_origin: Int32 = if cond0.tile_stipple_origin_y() {
            let (tile_stipple_y_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            tile_stipple_y_origin
        } else {
            Default::default()
        };
        let font: Font = if cond0.font() {
            let (font, sz): (Font, usize) = <Font>::from_bytes(&bytes[index..])?;
            index += sz;
            font
        } else {
            Default::default()
        };
        let subwindow_mode: SubwindowMode = if cond0.subwindow_mode() {
            let (subwindow_mode, sz): (SubwindowMode, usize) =
                <SubwindowMode>::from_bytes(&bytes[index..])?;
            index += sz;
            subwindow_mode
        } else {
            Default::default()
        };
        let graphics_exposures: Bool32 = if cond0.graphics_exposures() {
            let (graphics_exposures, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            graphics_exposures
        } else {
            Default::default()
        };
        let clip_x_origin: Int32 = if cond0.clip_origin_x() {
            let (clip_x_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clip_x_origin
        } else {
            Default::default()
        };
        let clip_y_origin: Int32 = if cond0.clip_origin_y() {
            let (clip_y_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clip_y_origin
        } else {
            Default::default()
        };
        let clip_mask: Pixmap = if cond0.clip_mask() {
            let (clip_mask, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            clip_mask
        } else {
            Default::default()
        };
        let dash_offset: Card32 = if cond0.dash_offset() {
            let (dash_offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            dash_offset
        } else {
            Default::default()
        };
        let dashes: Card32 = if cond0.dash_list() {
            let (dashes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            dashes
        } else {
            Default::default()
        };
        let arc_mode: ArcMode = if cond0.arc_mode() {
            let (arc_mode, sz): (ArcMode, usize) = <ArcMode>::from_bytes(&bytes[index..])?;
            index += sz;
            arc_mode
        } else {
            Default::default()
        };
        Some((
            CreateGcRequest {
                req_type: req_type,
                length: length,
                cid: cid,
                drawable: drawable,
                value_mask: value_mask,
                function: function,
                plane_mask: plane_mask,
                foreground: foreground,
                background: background,
                line_width: line_width,
                line_style: line_style,
                cap_style: cap_style,
                join_style: join_style,
                fill_style: fill_style,
                fill_rule: fill_rule,
                tile: tile,
                stipple: stipple,
                tile_stipple_x_origin: tile_stipple_x_origin,
                tile_stipple_y_origin: tile_stipple_y_origin,
                font: font,
                subwindow_mode: subwindow_mode,
                graphics_exposures: graphics_exposures,
                clip_x_origin: clip_x_origin,
                clip_y_origin: clip_y_origin,
                clip_mask: clip_mask,
                dash_offset: dash_offset,
                dashes: dashes,
                arc_mode: arc_mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cid.size()
            + self.drawable.size()
            + self.value_mask.size()
            + self.function.size()
            + self.plane_mask.size()
            + self.foreground.size()
            + self.background.size()
            + self.line_width.size()
            + self.line_style.size()
            + self.cap_style.size()
            + self.join_style.size()
            + self.fill_style.size()
            + self.fill_rule.size()
            + self.tile.size()
            + self.stipple.size()
            + self.tile_stipple_x_origin.size()
            + self.tile_stipple_y_origin.size()
            + self.font.size()
            + self.subwindow_mode.size()
            + self.graphics_exposures.size()
            + self.clip_x_origin.size()
            + self.clip_y_origin.size()
            + self.clip_mask.size()
            + self.dash_offset.size()
            + self.dashes.size()
            + self.arc_mode.size()
    }
}
impl Request for CreateGcRequest {
    const OPCODE: u8 = 55;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Gc {
    pub inner: u32,
}
impl Gc {
    #[inline]
    pub fn function(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_function(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn plane_mask(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_plane_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn foreground(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_foreground(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn background(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_background(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn line_width(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_line_width(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn line_style(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_line_style(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn cap_style(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_cap_style(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn join_style(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_join_style(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn fill_style(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_fill_style(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn fill_rule(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_fill_rule(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn tile(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_tile(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn stipple(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_stipple(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn tile_stipple_origin_x(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_tile_stipple_origin_x(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn tile_stipple_origin_y(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_tile_stipple_origin_y(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn font(&self) -> bool {
        self.inner & (1 << 14) != 0
    }
    #[inline]
    pub fn set_font(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 14;
        } else {
            self.inner &= !(1 << 14);
        }
        self
    }
    #[inline]
    pub fn subwindow_mode(&self) -> bool {
        self.inner & (1 << 15) != 0
    }
    #[inline]
    pub fn set_subwindow_mode(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 15;
        } else {
            self.inner &= !(1 << 15);
        }
        self
    }
    #[inline]
    pub fn graphics_exposures(&self) -> bool {
        self.inner & (1 << 16) != 0
    }
    #[inline]
    pub fn set_graphics_exposures(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 16;
        } else {
            self.inner &= !(1 << 16);
        }
        self
    }
    #[inline]
    pub fn clip_origin_x(&self) -> bool {
        self.inner & (1 << 17) != 0
    }
    #[inline]
    pub fn set_clip_origin_x(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 17;
        } else {
            self.inner &= !(1 << 17);
        }
        self
    }
    #[inline]
    pub fn clip_origin_y(&self) -> bool {
        self.inner & (1 << 18) != 0
    }
    #[inline]
    pub fn set_clip_origin_y(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 18;
        } else {
            self.inner &= !(1 << 18);
        }
        self
    }
    #[inline]
    pub fn clip_mask(&self) -> bool {
        self.inner & (1 << 19) != 0
    }
    #[inline]
    pub fn set_clip_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 19;
        } else {
            self.inner &= !(1 << 19);
        }
        self
    }
    #[inline]
    pub fn dash_offset(&self) -> bool {
        self.inner & (1 << 20) != 0
    }
    #[inline]
    pub fn set_dash_offset(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 20;
        } else {
            self.inner &= !(1 << 20);
        }
        self
    }
    #[inline]
    pub fn dash_list(&self) -> bool {
        self.inner & (1 << 21) != 0
    }
    #[inline]
    pub fn set_dash_list(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 21;
        } else {
            self.inner &= !(1 << 21);
        }
        self
    }
    #[inline]
    pub fn arc_mode(&self) -> bool {
        self.inner & (1 << 22) != 0
    }
    #[inline]
    pub fn set_arc_mode(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 22;
        } else {
            self.inner &= !(1 << 22);
        }
        self
    }
    #[inline]
    pub fn new(
        function: bool,
        plane_mask: bool,
        foreground: bool,
        background: bool,
        line_width: bool,
        line_style: bool,
        cap_style: bool,
        join_style: bool,
        fill_style: bool,
        fill_rule: bool,
        tile: bool,
        stipple: bool,
        tile_stipple_origin_x: bool,
        tile_stipple_origin_y: bool,
        font: bool,
        subwindow_mode: bool,
        graphics_exposures: bool,
        clip_origin_x: bool,
        clip_origin_y: bool,
        clip_mask: bool,
        dash_offset: bool,
        dash_list: bool,
        arc_mode: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if function {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if plane_mask {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if foreground {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if background {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        if line_width {
            inner |= 1 << 4;
        } else {
            inner &= !(1 << 4);
        }
        if line_style {
            inner |= 1 << 5;
        } else {
            inner &= !(1 << 5);
        }
        if cap_style {
            inner |= 1 << 6;
        } else {
            inner &= !(1 << 6);
        }
        if join_style {
            inner |= 1 << 7;
        } else {
            inner &= !(1 << 7);
        }
        if fill_style {
            inner |= 1 << 8;
        } else {
            inner &= !(1 << 8);
        }
        if fill_rule {
            inner |= 1 << 9;
        } else {
            inner &= !(1 << 9);
        }
        if tile {
            inner |= 1 << 10;
        } else {
            inner &= !(1 << 10);
        }
        if stipple {
            inner |= 1 << 11;
        } else {
            inner &= !(1 << 11);
        }
        if tile_stipple_origin_x {
            inner |= 1 << 12;
        } else {
            inner &= !(1 << 12);
        }
        if tile_stipple_origin_y {
            inner |= 1 << 13;
        } else {
            inner &= !(1 << 13);
        }
        if font {
            inner |= 1 << 14;
        } else {
            inner &= !(1 << 14);
        }
        if subwindow_mode {
            inner |= 1 << 15;
        } else {
            inner &= !(1 << 15);
        }
        if graphics_exposures {
            inner |= 1 << 16;
        } else {
            inner &= !(1 << 16);
        }
        if clip_origin_x {
            inner |= 1 << 17;
        } else {
            inner &= !(1 << 17);
        }
        if clip_origin_y {
            inner |= 1 << 18;
        } else {
            inner &= !(1 << 18);
        }
        if clip_mask {
            inner |= 1 << 19;
        } else {
            inner &= !(1 << 19);
        }
        if dash_offset {
            inner |= 1 << 20;
        } else {
            inner &= !(1 << 20);
        }
        if dash_list {
            inner |= 1 << 21;
        } else {
            inner &= !(1 << 21);
        }
        if arc_mode {
            inner |= 1 << 22;
        } else {
            inner &= !(1 << 22);
        }
        Gc { inner: inner }
    }
}
impl AsByteSequence for Gc {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Gc { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangeGcRequest {
    pub req_type: u8,
    pub length: u16,
    pub gc: Gcontext,
    pub value_mask: Gc,
    pub function: Gx,
    pub plane_mask: Card32,
    pub foreground: Card32,
    pub background: Card32,
    pub line_width: Card32,
    pub line_style: LineStyle,
    pub cap_style: CapStyle,
    pub join_style: JoinStyle,
    pub fill_style: FillStyle,
    pub fill_rule: FillRule,
    pub tile: Pixmap,
    pub stipple: Pixmap,
    pub tile_stipple_x_origin: Int32,
    pub tile_stipple_y_origin: Int32,
    pub font: Font,
    pub subwindow_mode: SubwindowMode,
    pub graphics_exposures: Bool32,
    pub clip_x_origin: Int32,
    pub clip_y_origin: Int32,
    pub clip_mask: Pixmap,
    pub dash_offset: Card32,
    pub dashes: Card32,
    pub arc_mode: ArcMode,
}
impl ChangeGcRequest {}
impl AsByteSequence for ChangeGcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.function() {
            index += self.function.as_bytes(&mut bytes[index..]);
        }
        if cond0.plane_mask() {
            index += self.plane_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.foreground() {
            index += self.foreground.as_bytes(&mut bytes[index..]);
        }
        if cond0.background() {
            index += self.background.as_bytes(&mut bytes[index..]);
        }
        if cond0.line_width() {
            index += self.line_width.as_bytes(&mut bytes[index..]);
        }
        if cond0.line_style() {
            index += self.line_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.cap_style() {
            index += self.cap_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.join_style() {
            index += self.join_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.fill_style() {
            index += self.fill_style.as_bytes(&mut bytes[index..]);
        }
        if cond0.fill_rule() {
            index += self.fill_rule.as_bytes(&mut bytes[index..]);
        }
        if cond0.tile() {
            index += self.tile.as_bytes(&mut bytes[index..]);
        }
        if cond0.stipple() {
            index += self.stipple.as_bytes(&mut bytes[index..]);
        }
        if cond0.tile_stipple_origin_x() {
            index += self.tile_stipple_x_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.tile_stipple_origin_y() {
            index += self.tile_stipple_y_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.font() {
            index += self.font.as_bytes(&mut bytes[index..]);
        }
        if cond0.subwindow_mode() {
            index += self.subwindow_mode.as_bytes(&mut bytes[index..]);
        }
        if cond0.graphics_exposures() {
            index += self.graphics_exposures.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_origin_x() {
            index += self.clip_x_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_origin_y() {
            index += self.clip_y_origin.as_bytes(&mut bytes[index..]);
        }
        if cond0.clip_mask() {
            index += self.clip_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0.dash_offset() {
            index += self.dash_offset.as_bytes(&mut bytes[index..]);
        }
        if cond0.dash_list() {
            index += self.dashes.as_bytes(&mut bytes[index..]);
        }
        if cond0.arc_mode() {
            index += self.arc_mode.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeGcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Gc, usize) = <Gc>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let function: Gx = if cond0.function() {
            let (function, sz): (Gx, usize) = <Gx>::from_bytes(&bytes[index..])?;
            index += sz;
            function
        } else {
            Default::default()
        };
        let plane_mask: Card32 = if cond0.plane_mask() {
            let (plane_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            plane_mask
        } else {
            Default::default()
        };
        let foreground: Card32 = if cond0.foreground() {
            let (foreground, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            foreground
        } else {
            Default::default()
        };
        let background: Card32 = if cond0.background() {
            let (background, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            background
        } else {
            Default::default()
        };
        let line_width: Card32 = if cond0.line_width() {
            let (line_width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            line_width
        } else {
            Default::default()
        };
        let line_style: LineStyle = if cond0.line_style() {
            let (line_style, sz): (LineStyle, usize) = <LineStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            line_style
        } else {
            Default::default()
        };
        let cap_style: CapStyle = if cond0.cap_style() {
            let (cap_style, sz): (CapStyle, usize) = <CapStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            cap_style
        } else {
            Default::default()
        };
        let join_style: JoinStyle = if cond0.join_style() {
            let (join_style, sz): (JoinStyle, usize) = <JoinStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            join_style
        } else {
            Default::default()
        };
        let fill_style: FillStyle = if cond0.fill_style() {
            let (fill_style, sz): (FillStyle, usize) = <FillStyle>::from_bytes(&bytes[index..])?;
            index += sz;
            fill_style
        } else {
            Default::default()
        };
        let fill_rule: FillRule = if cond0.fill_rule() {
            let (fill_rule, sz): (FillRule, usize) = <FillRule>::from_bytes(&bytes[index..])?;
            index += sz;
            fill_rule
        } else {
            Default::default()
        };
        let tile: Pixmap = if cond0.tile() {
            let (tile, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            tile
        } else {
            Default::default()
        };
        let stipple: Pixmap = if cond0.stipple() {
            let (stipple, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            stipple
        } else {
            Default::default()
        };
        let tile_stipple_x_origin: Int32 = if cond0.tile_stipple_origin_x() {
            let (tile_stipple_x_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            tile_stipple_x_origin
        } else {
            Default::default()
        };
        let tile_stipple_y_origin: Int32 = if cond0.tile_stipple_origin_y() {
            let (tile_stipple_y_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            tile_stipple_y_origin
        } else {
            Default::default()
        };
        let font: Font = if cond0.font() {
            let (font, sz): (Font, usize) = <Font>::from_bytes(&bytes[index..])?;
            index += sz;
            font
        } else {
            Default::default()
        };
        let subwindow_mode: SubwindowMode = if cond0.subwindow_mode() {
            let (subwindow_mode, sz): (SubwindowMode, usize) =
                <SubwindowMode>::from_bytes(&bytes[index..])?;
            index += sz;
            subwindow_mode
        } else {
            Default::default()
        };
        let graphics_exposures: Bool32 = if cond0.graphics_exposures() {
            let (graphics_exposures, sz): (Bool32, usize) = <Bool32>::from_bytes(&bytes[index..])?;
            index += sz;
            graphics_exposures
        } else {
            Default::default()
        };
        let clip_x_origin: Int32 = if cond0.clip_origin_x() {
            let (clip_x_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clip_x_origin
        } else {
            Default::default()
        };
        let clip_y_origin: Int32 = if cond0.clip_origin_y() {
            let (clip_y_origin, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            clip_y_origin
        } else {
            Default::default()
        };
        let clip_mask: Pixmap = if cond0.clip_mask() {
            let (clip_mask, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
            index += sz;
            clip_mask
        } else {
            Default::default()
        };
        let dash_offset: Card32 = if cond0.dash_offset() {
            let (dash_offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            dash_offset
        } else {
            Default::default()
        };
        let dashes: Card32 = if cond0.dash_list() {
            let (dashes, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            dashes
        } else {
            Default::default()
        };
        let arc_mode: ArcMode = if cond0.arc_mode() {
            let (arc_mode, sz): (ArcMode, usize) = <ArcMode>::from_bytes(&bytes[index..])?;
            index += sz;
            arc_mode
        } else {
            Default::default()
        };
        Some((
            ChangeGcRequest {
                req_type: req_type,
                length: length,
                gc: gc,
                value_mask: value_mask,
                function: function,
                plane_mask: plane_mask,
                foreground: foreground,
                background: background,
                line_width: line_width,
                line_style: line_style,
                cap_style: cap_style,
                join_style: join_style,
                fill_style: fill_style,
                fill_rule: fill_rule,
                tile: tile,
                stipple: stipple,
                tile_stipple_x_origin: tile_stipple_x_origin,
                tile_stipple_y_origin: tile_stipple_y_origin,
                font: font,
                subwindow_mode: subwindow_mode,
                graphics_exposures: graphics_exposures,
                clip_x_origin: clip_x_origin,
                clip_y_origin: clip_y_origin,
                clip_mask: clip_mask,
                dash_offset: dash_offset,
                dashes: dashes,
                arc_mode: arc_mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.gc.size()
            + self.value_mask.size()
            + self.function.size()
            + self.plane_mask.size()
            + self.foreground.size()
            + self.background.size()
            + self.line_width.size()
            + self.line_style.size()
            + self.cap_style.size()
            + self.join_style.size()
            + self.fill_style.size()
            + self.fill_rule.size()
            + self.tile.size()
            + self.stipple.size()
            + self.tile_stipple_x_origin.size()
            + self.tile_stipple_y_origin.size()
            + self.font.size()
            + self.subwindow_mode.size()
            + self.graphics_exposures.size()
            + self.clip_x_origin.size()
            + self.clip_y_origin.size()
            + self.clip_mask.size()
            + self.dash_offset.size()
            + self.dashes.size()
            + self.arc_mode.size()
    }
}
impl Request for ChangeGcRequest {
    const OPCODE: u8 = 56;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CopyGcRequest {
    pub req_type: u8,
    pub length: u16,
    pub src_gc: Gcontext,
    pub dst_gc: Gcontext,
    pub value_mask: Gc,
}
impl CopyGcRequest {}
impl AsByteSequence for CopyGcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.src_gc.as_bytes(&mut bytes[index..]);
        index += self.dst_gc.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CopyGcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Gc, usize) = <Gc>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyGcRequest {
                req_type: req_type,
                length: length,
                src_gc: src_gc,
                dst_gc: dst_gc,
                value_mask: value_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.src_gc.size()
            + self.dst_gc.size()
            + self.value_mask.size()
    }
}
impl Request for CopyGcRequest {
    const OPCODE: u8 = 57;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetDashesRequest {
    pub req_type: u8,
    pub length: u16,
    pub gc: Gcontext,
    pub dash_offset: Card16,
    pub dashes: Vec<Card8>,
}
impl SetDashesRequest {}
impl AsByteSequence for SetDashesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.dash_offset.as_bytes(&mut bytes[index..]);
        index += (self.dashes.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.dashes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDashesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dash_offset, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dashes, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            SetDashesRequest {
                req_type: req_type,
                length: length,
                gc: gc,
                dash_offset: dash_offset,
                dashes: dashes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.gc.size()
            + self.dash_offset.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.dashes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for SetDashesRequest {
    const OPCODE: u8 = 58;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetClipRectanglesRequest {
    pub req_type: u8,
    pub ordering: ClipOrdering,
    pub length: u16,
    pub gc: Gcontext,
    pub clip_x_origin: Int16,
    pub clip_y_origin: Int16,
    pub rectangles: Vec<Rectangle>,
}
impl SetClipRectanglesRequest {}
impl AsByteSequence for SetClipRectanglesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.ordering.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.clip_x_origin.as_bytes(&mut bytes[index..]);
        index += self.clip_y_origin.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetClipRectanglesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ordering, sz): (ClipOrdering, usize) = <ClipOrdering>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_x_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clip_y_origin, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rectangles, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            SetClipRectanglesRequest {
                req_type: req_type,
                ordering: ordering,
                length: length,
                gc: gc,
                clip_x_origin: clip_x_origin,
                clip_y_origin: clip_y_origin,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.ordering.size()
            + self.length.size()
            + self.gc.size()
            + self.clip_x_origin.size()
            + self.clip_y_origin.size()
            + {
                let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
                block_len + pad
            }
    }
}
impl Request for SetClipRectanglesRequest {
    const OPCODE: u8 = 59;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ClipOrdering {
    Unsorted = 0,
    YSorted = 1,
    YxSorted = 2,
    YxBanded = 3,
}
impl AsByteSequence for ClipOrdering {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Unsorted, sz)),
            1 => Some((Self::YSorted, sz)),
            2 => Some((Self::YxSorted, sz)),
            3 => Some((Self::YxBanded, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ClipOrdering {
    #[inline]
    fn default() -> ClipOrdering {
        ClipOrdering::Unsorted
    }
}
#[derive(Clone, Debug, Default)]
pub struct FreeGcRequest {
    pub req_type: u8,
    pub length: u16,
    pub gc: Gcontext,
}
impl FreeGcRequest {}
impl AsByteSequence for FreeGcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeGcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreeGcRequest {
                req_type: req_type,
                length: length,
                gc: gc,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.gc.size()
    }
}
impl Request for FreeGcRequest {
    const OPCODE: u8 = 60;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ClearAreaRequest {
    pub req_type: u8,
    pub exposures: bool,
    pub length: u16,
    pub window: Window,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
}
impl ClearAreaRequest {}
impl AsByteSequence for ClearAreaRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.exposures.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ClearAreaRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (exposures, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ClearAreaRequest {
                req_type: req_type,
                exposures: exposures,
                length: length,
                window: window,
                x: x,
                y: y,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.exposures.size()
            + self.length.size()
            + self.window.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
    }
}
impl Request for ClearAreaRequest {
    const OPCODE: u8 = 61;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CopyAreaRequest {
    pub req_type: u8,
    pub length: u16,
    pub src_drawable: Drawable,
    pub dst_drawable: Drawable,
    pub gc: Gcontext,
    pub src_x: Int16,
    pub src_y: Int16,
    pub dst_x: Int16,
    pub dst_y: Int16,
    pub width: Card16,
    pub height: Card16,
}
impl CopyAreaRequest {}
impl AsByteSequence for CopyAreaRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.src_drawable.as_bytes(&mut bytes[index..]);
        index += self.dst_drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CopyAreaRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyAreaRequest {
                req_type: req_type,
                length: length,
                src_drawable: src_drawable,
                dst_drawable: dst_drawable,
                gc: gc,
                src_x: src_x,
                src_y: src_y,
                dst_x: dst_x,
                dst_y: dst_y,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.src_drawable.size()
            + self.dst_drawable.size()
            + self.gc.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.dst_x.size()
            + self.dst_y.size()
            + self.width.size()
            + self.height.size()
    }
}
impl Request for CopyAreaRequest {
    const OPCODE: u8 = 62;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CopyPlaneRequest {
    pub req_type: u8,
    pub length: u16,
    pub src_drawable: Drawable,
    pub dst_drawable: Drawable,
    pub gc: Gcontext,
    pub src_x: Int16,
    pub src_y: Int16,
    pub dst_x: Int16,
    pub dst_y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub bit_plane: Card32,
}
impl CopyPlaneRequest {}
impl AsByteSequence for CopyPlaneRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.src_drawable.as_bytes(&mut bytes[index..]);
        index += self.dst_drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.bit_plane.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CopyPlaneRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bit_plane, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyPlaneRequest {
                req_type: req_type,
                length: length,
                src_drawable: src_drawable,
                dst_drawable: dst_drawable,
                gc: gc,
                src_x: src_x,
                src_y: src_y,
                dst_x: dst_x,
                dst_y: dst_y,
                width: width,
                height: height,
                bit_plane: bit_plane,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.src_drawable.size()
            + self.dst_drawable.size()
            + self.gc.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.dst_x.size()
            + self.dst_y.size()
            + self.width.size()
            + self.height.size()
            + self.bit_plane.size()
    }
}
impl Request for CopyPlaneRequest {
    const OPCODE: u8 = 63;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PolyPointRequest {
    pub req_type: u8,
    pub coordinate_mode: CoordMode,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub points: Vec<Point>,
}
impl PolyPointRequest {}
impl AsByteSequence for PolyPointRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.coordinate_mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.points, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyPointRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (coordinate_mode, sz): (CoordMode, usize) = <CoordMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (points, block_len): (Vec<Point>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        Some((
            PolyPointRequest {
                req_type: req_type,
                coordinate_mode: coordinate_mode,
                length: length,
                drawable: drawable,
                gc: gc,
                points: points,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.coordinate_mode.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + {
                let block_len: usize = self.points.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Point>());
                block_len + pad
            }
    }
}
impl Request for PolyPointRequest {
    const OPCODE: u8 = 64;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CoordMode {
    Origin = 0,
    Previous = 1,
}
impl AsByteSequence for CoordMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Origin, sz)),
            1 => Some((Self::Previous, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for CoordMode {
    #[inline]
    fn default() -> CoordMode {
        CoordMode::Origin
    }
}
#[derive(Clone, Debug, Default)]
pub struct PolyLineRequest {
    pub req_type: u8,
    pub coordinate_mode: CoordMode,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub points: Vec<Point>,
}
impl PolyLineRequest {}
impl AsByteSequence for PolyLineRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.coordinate_mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.points, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyLineRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (coordinate_mode, sz): (CoordMode, usize) = <CoordMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (points, block_len): (Vec<Point>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        Some((
            PolyLineRequest {
                req_type: req_type,
                coordinate_mode: coordinate_mode,
                length: length,
                drawable: drawable,
                gc: gc,
                points: points,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.coordinate_mode.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + {
                let block_len: usize = self.points.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Point>());
                block_len + pad
            }
    }
}
impl Request for PolyLineRequest {
    const OPCODE: u8 = 65;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct Segment {
    pub x1: Int16,
    pub y1: Int16,
    pub x2: Int16,
    pub y2: Int16,
}
impl Segment {}
impl AsByteSequence for Segment {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.x1.as_bytes(&mut bytes[index..]);
        index += self.y1.as_bytes(&mut bytes[index..]);
        index += self.x2.as_bytes(&mut bytes[index..]);
        index += self.y2.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Segment from byte buffer");
        let (x1, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y1, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x2, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y2, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Segment {
                x1: x1,
                y1: y1,
                x2: x2,
                y2: y2,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.x1.size() + self.y1.size() + self.x2.size() + self.y2.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct PolySegmentRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub segments: Vec<Segment>,
}
impl PolySegmentRequest {}
impl AsByteSequence for PolySegmentRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.segments, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Segment>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolySegmentRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (segments, block_len): (Vec<Segment>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Segment>());
        Some((
            PolySegmentRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                segments: segments,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.gc.size() + {
            let block_len: usize = self.segments.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Segment>());
            block_len + pad
        }
    }
}
impl Request for PolySegmentRequest {
    const OPCODE: u8 = 66;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PolyRectangleRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub rectangles: Vec<Rectangle>,
}
impl PolyRectangleRequest {}
impl AsByteSequence for PolyRectangleRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyRectangleRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rectangles, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            PolyRectangleRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.gc.size() + {
            let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
            block_len + pad
        }
    }
}
impl Request for PolyRectangleRequest {
    const OPCODE: u8 = 67;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PolyArcRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub arcs: Vec<Arc>,
}
impl PolyArcRequest {}
impl AsByteSequence for PolyArcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.arcs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Arc>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyArcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (arcs, block_len): (Vec<Arc>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Arc>());
        Some((
            PolyArcRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                arcs: arcs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.gc.size() + {
            let block_len: usize = self.arcs.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Arc>());
            block_len + pad
        }
    }
}
impl Request for PolyArcRequest {
    const OPCODE: u8 = 68;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct FillPolyRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub shape: PolyShape,
    pub coordinate_mode: CoordMode,
    pub points: Vec<Point>,
}
impl FillPolyRequest {}
impl AsByteSequence for FillPolyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.shape.as_bytes(&mut bytes[index..]);
        index += self.coordinate_mode.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.points, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FillPolyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shape, sz): (PolyShape, usize) = <PolyShape>::from_bytes(&bytes[index..])?;
        index += sz;
        let (coordinate_mode, sz): (CoordMode, usize) = <CoordMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (points, block_len): (Vec<Point>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        Some((
            FillPolyRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                shape: shape,
                coordinate_mode: coordinate_mode,
                points: points,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.shape.size()
            + self.coordinate_mode.size()
            + 2
            + {
                let block_len: usize = self.points.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Point>());
                block_len + pad
            }
    }
}
impl Request for FillPolyRequest {
    const OPCODE: u8 = 69;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum PolyShape {
    Complex = 0,
    Nonconvex = 1,
    Convex = 2,
}
impl AsByteSequence for PolyShape {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Complex, sz)),
            1 => Some((Self::Nonconvex, sz)),
            2 => Some((Self::Convex, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for PolyShape {
    #[inline]
    fn default() -> PolyShape {
        PolyShape::Complex
    }
}
#[derive(Clone, Debug, Default)]
pub struct PolyFillRectangleRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub rectangles: Vec<Rectangle>,
}
impl PolyFillRectangleRequest {}
impl AsByteSequence for PolyFillRectangleRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.rectangles, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyFillRectangleRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rectangles, block_len): (Vec<Rectangle>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
        Some((
            PolyFillRectangleRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                rectangles: rectangles,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.gc.size() + {
            let block_len: usize = self.rectangles.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rectangle>());
            block_len + pad
        }
    }
}
impl Request for PolyFillRectangleRequest {
    const OPCODE: u8 = 70;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PolyFillArcRequest {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub arcs: Vec<Arc>,
}
impl PolyFillArcRequest {}
impl AsByteSequence for PolyFillArcRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.arcs, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Arc>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyFillArcRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (arcs, block_len): (Vec<Arc>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Arc>());
        Some((
            PolyFillArcRequest {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                arcs: arcs,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.drawable.size() + self.gc.size() + {
            let block_len: usize = self.arcs.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Arc>());
            block_len + pad
        }
    }
}
impl Request for PolyFillArcRequest {
    const OPCODE: u8 = 71;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PutImageRequest {
    pub req_type: u8,
    pub format: ImageFormat,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub width: Card16,
    pub height: Card16,
    pub dst_x: Int16,
    pub dst_y: Int16,
    pub left_pad: Card8,
    pub depth: Card8,
    pub data: Vec<Byte>,
}
impl PutImageRequest {}
impl AsByteSequence for PutImageRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index += self.left_pad.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PutImageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (ImageFormat, usize) = <ImageFormat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (left_pad, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            PutImageRequest {
                req_type: req_type,
                format: format,
                length: length,
                drawable: drawable,
                gc: gc,
                width: width,
                height: height,
                dst_x: dst_x,
                dst_y: dst_y,
                left_pad: left_pad,
                depth: depth,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.format.size()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.width.size()
            + self.height.size()
            + self.dst_x.size()
            + self.dst_y.size()
            + self.left_pad.size()
            + self.depth.size()
            + 2
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for PutImageRequest {
    const OPCODE: u8 = 72;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ImageFormat {
    XyBitmap = 0,
    XyPixmap = 1,
    ZPixmap = 2,
}
impl AsByteSequence for ImageFormat {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::XyBitmap, sz)),
            1 => Some((Self::XyPixmap, sz)),
            2 => Some((Self::ZPixmap, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ImageFormat {
    #[inline]
    fn default() -> ImageFormat {
        ImageFormat::XyBitmap
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetImageRequest {
    pub req_type: u8,
    pub format: ImageFormat,
    pub length: u16,
    pub drawable: Drawable,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub plane_mask: Card32,
}
impl GetImageRequest {}
impl AsByteSequence for GetImageRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.plane_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetImageRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (ImageFormat, usize) = <ImageFormat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (plane_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetImageRequest {
                req_type: req_type,
                format: format,
                length: length,
                drawable: drawable,
                x: x,
                y: y,
                width: width,
                height: height,
                plane_mask: plane_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.format.size()
            + self.length.size()
            + self.drawable.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.plane_mask.size()
    }
}
impl Request for GetImageRequest {
    const OPCODE: u8 = 73;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetImageReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetImageReply {
    pub reply_type: u8,
    pub depth: Card8,
    pub sequence: u16,
    pub length: u32,
    pub visual: Visualid,
    pub data: Vec<Byte>,
}
impl GetImageReply {}
impl AsByteSequence for GetImageReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.depth.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.data, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetImageReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (depth, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (data, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize) * (4)) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            GetImageReply {
                reply_type: reply_type,
                depth: depth,
                sequence: sequence,
                length: length,
                visual: visual,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.depth.size()
            + self.sequence.size()
            + self.length.size()
            + self.visual.size()
            + 20
            + {
                let block_len: usize = self.data.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct PolyText8Request {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub x: Int16,
    pub y: Int16,
    pub items: Vec<Byte>,
}
impl PolyText8Request {}
impl AsByteSequence for PolyText8Request {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.items, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyText8Request from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (items, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            PolyText8Request {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                x: x,
                y: y,
                items: items,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.x.size()
            + self.y.size()
            + {
                let block_len: usize = self.items.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for PolyText8Request {
    const OPCODE: u8 = 74;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PolyText16Request {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub x: Int16,
    pub y: Int16,
    pub items: Vec<Byte>,
}
impl PolyText16Request {}
impl AsByteSequence for PolyText16Request {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.items, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PolyText16Request from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (items, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            PolyText16Request {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                x: x,
                y: y,
                items: items,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.x.size()
            + self.y.size()
            + {
                let block_len: usize = self.items.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for PolyText16Request {
    const OPCODE: u8 = 75;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ImageText8Request {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub x: Int16,
    pub y: Int16,
    pub string: String,
}
impl ImageText8Request {}
impl AsByteSequence for ImageText8Request {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += (self.string.len() as Byte).as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ImageText8Request from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            ImageText8Request {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                x: x,
                y: y,
                string: string,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + ::core::mem::size_of::<Byte>()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.x.size()
            + self.y.size()
            + {
                let block_len: usize = self.string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for ImageText8Request {
    const OPCODE: u8 = 76;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ImageText16Request {
    pub req_type: u8,
    pub length: u16,
    pub drawable: Drawable,
    pub gc: Gcontext,
    pub x: Int16,
    pub y: Int16,
    pub string: Vec<Char2b>,
}
impl ImageText16Request {}
impl AsByteSequence for ImageText16Request {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += (self.string.len() as Byte).as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.gc.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Char2b>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ImageText16Request from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gc, sz): (Gcontext, usize) = <Gcontext>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string, block_len): (Vec<Char2b>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Char2b>());
        Some((
            ImageText16Request {
                req_type: req_type,
                length: length,
                drawable: drawable,
                gc: gc,
                x: x,
                y: y,
                string: string,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + ::core::mem::size_of::<Byte>()
            + self.length.size()
            + self.drawable.size()
            + self.gc.size()
            + self.x.size()
            + self.y.size()
            + {
                let block_len: usize = self.string.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Char2b>());
                block_len + pad
            }
    }
}
impl Request for ImageText16Request {
    const OPCODE: u8 = 77;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CreateColormapRequest {
    pub req_type: u8,
    pub alloc: ColormapAlloc,
    pub length: u16,
    pub mid: Colormap,
    pub window: Window,
    pub visual: Visualid,
}
impl CreateColormapRequest {}
impl AsByteSequence for CreateColormapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.alloc.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mid.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.visual.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateColormapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alloc, sz): (ColormapAlloc, usize) = <ColormapAlloc>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mid, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual, sz): (Visualid, usize) = <Visualid>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateColormapRequest {
                req_type: req_type,
                alloc: alloc,
                length: length,
                mid: mid,
                window: window,
                visual: visual,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.alloc.size()
            + self.length.size()
            + self.mid.size()
            + self.window.size()
            + self.visual.size()
    }
}
impl Request for CreateColormapRequest {
    const OPCODE: u8 = 78;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColormapAlloc {
    None = 0,
    All = 1,
}
impl AsByteSequence for ColormapAlloc {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::None, sz)),
            1 => Some((Self::All, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ColormapAlloc {
    #[inline]
    fn default() -> ColormapAlloc {
        ColormapAlloc::None
    }
}
#[derive(Clone, Debug, Default)]
pub struct FreeColormapRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
}
impl FreeColormapRequest {}
impl AsByteSequence for FreeColormapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeColormapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreeColormapRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.cmap.size()
    }
}
impl Request for FreeColormapRequest {
    const OPCODE: u8 = 79;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct CopyColormapAndFreeRequest {
    pub req_type: u8,
    pub length: u16,
    pub mid: Colormap,
    pub src_cmap: Colormap,
}
impl CopyColormapAndFreeRequest {}
impl AsByteSequence for CopyColormapAndFreeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mid.as_bytes(&mut bytes[index..]);
        index += self.src_cmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CopyColormapAndFreeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mid, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CopyColormapAndFreeRequest {
                req_type: req_type,
                length: length,
                mid: mid,
                src_cmap: src_cmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.mid.size() + self.src_cmap.size()
    }
}
impl Request for CopyColormapAndFreeRequest {
    const OPCODE: u8 = 80;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct InstallColormapRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
}
impl InstallColormapRequest {}
impl AsByteSequence for InstallColormapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InstallColormapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InstallColormapRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.cmap.size()
    }
}
impl Request for InstallColormapRequest {
    const OPCODE: u8 = 81;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct UninstallColormapRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
}
impl UninstallColormapRequest {}
impl AsByteSequence for UninstallColormapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UninstallColormapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UninstallColormapRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.cmap.size()
    }
}
impl Request for UninstallColormapRequest {
    const OPCODE: u8 = 82;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ListInstalledColormapsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl ListInstalledColormapsRequest {}
impl AsByteSequence for ListInstalledColormapsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListInstalledColormapsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListInstalledColormapsRequest {
                req_type: req_type,
                length: length,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.window.size()
    }
}
impl Request for ListInstalledColormapsRequest {
    const OPCODE: u8 = 83;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListInstalledColormapsReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListInstalledColormapsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub cmaps: Vec<Colormap>,
}
impl ListInstalledColormapsReply {}
impl AsByteSequence for ListInstalledColormapsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.cmaps.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.cmaps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Colormap>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListInstalledColormapsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (cmaps, block_len): (Vec<Colormap>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Colormap>());
        Some((
            ListInstalledColormapsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                cmaps: cmaps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.cmaps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Colormap>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct AllocColorRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
    pub red: Card16,
    pub green: Card16,
    pub blue: Card16,
}
impl AllocColorRequest {}
impl AsByteSequence for AllocColorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocColorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            AllocColorRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
                red: red,
                green: green,
                blue: blue,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cmap.size()
            + self.red.size()
            + self.green.size()
            + self.blue.size()
            + 2
    }
}
impl Request for AllocColorRequest {
    const OPCODE: u8 = 84;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = AllocColorReply;
}
#[derive(Clone, Debug, Default)]
pub struct AllocColorReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red: Card16,
    pub green: Card16,
    pub blue: Card16,
    pub pixel: Card32,
}
impl AllocColorReply {}
impl AsByteSequence for AllocColorReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.pixel.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocColorReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AllocColorReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                red: red,
                green: green,
                blue: blue,
                pixel: pixel,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.red.size()
            + self.green.size()
            + self.blue.size()
            + 2
            + self.pixel.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct AllocNamedColorRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
    pub name: String,
}
impl AllocNamedColorRequest {}
impl AsByteSequence for AllocNamedColorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocNamedColorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            AllocNamedColorRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cmap.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for AllocNamedColorRequest {
    const OPCODE: u8 = 85;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = AllocNamedColorReply;
}
#[derive(Clone, Debug, Default)]
pub struct AllocNamedColorReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixel: Card32,
    pub exact_red: Card16,
    pub exact_green: Card16,
    pub exact_blue: Card16,
    pub visual_red: Card16,
    pub visual_green: Card16,
    pub visual_blue: Card16,
}
impl AllocNamedColorReply {}
impl AsByteSequence for AllocNamedColorReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.pixel.as_bytes(&mut bytes[index..]);
        index += self.exact_red.as_bytes(&mut bytes[index..]);
        index += self.exact_green.as_bytes(&mut bytes[index..]);
        index += self.exact_blue.as_bytes(&mut bytes[index..]);
        index += self.visual_red.as_bytes(&mut bytes[index..]);
        index += self.visual_green.as_bytes(&mut bytes[index..]);
        index += self.visual_blue.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocNamedColorReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (exact_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (exact_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (exact_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AllocNamedColorReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                pixel: pixel,
                exact_red: exact_red,
                exact_green: exact_green,
                exact_blue: exact_blue,
                visual_red: visual_red,
                visual_green: visual_green,
                visual_blue: visual_blue,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.pixel.size()
            + self.exact_red.size()
            + self.exact_green.size()
            + self.exact_blue.size()
            + self.visual_red.size()
            + self.visual_green.size()
            + self.visual_blue.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct AllocColorCellsRequest {
    pub req_type: u8,
    pub contiguous: bool,
    pub length: u16,
    pub cmap: Colormap,
    pub colors: Card16,
    pub planes: Card16,
}
impl AllocColorCellsRequest {}
impl AsByteSequence for AllocColorCellsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.contiguous.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index += self.colors.as_bytes(&mut bytes[index..]);
        index += self.planes.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocColorCellsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (contiguous, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (colors, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (planes, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AllocColorCellsRequest {
                req_type: req_type,
                contiguous: contiguous,
                length: length,
                cmap: cmap,
                colors: colors,
                planes: planes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.contiguous.size()
            + self.length.size()
            + self.cmap.size()
            + self.colors.size()
            + self.planes.size()
    }
}
impl Request for AllocColorCellsRequest {
    const OPCODE: u8 = 86;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = AllocColorCellsReply;
}
#[derive(Clone, Debug, Default)]
pub struct AllocColorCellsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub pixels: Vec<Card32>,
    pub masks: Vec<Card32>,
}
impl AllocColorCellsReply {}
impl AsByteSequence for AllocColorCellsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.pixels.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.masks.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.pixels, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.masks, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocColorCellsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (pixels, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (masks, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            AllocColorCellsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                pixels: pixels,
                masks: masks,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + 20
            + {
                let block_len: usize = self.pixels.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.masks.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct AllocColorPlanesRequest {
    pub req_type: u8,
    pub contiguous: bool,
    pub length: u16,
    pub cmap: Colormap,
    pub colors: Card16,
    pub reds: Card16,
    pub greens: Card16,
    pub blues: Card16,
}
impl AllocColorPlanesRequest {}
impl AsByteSequence for AllocColorPlanesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.contiguous.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index += self.colors.as_bytes(&mut bytes[index..]);
        index += self.reds.as_bytes(&mut bytes[index..]);
        index += self.greens.as_bytes(&mut bytes[index..]);
        index += self.blues.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocColorPlanesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (contiguous, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (colors, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reds, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (greens, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blues, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AllocColorPlanesRequest {
                req_type: req_type,
                contiguous: contiguous,
                length: length,
                cmap: cmap,
                colors: colors,
                reds: reds,
                greens: greens,
                blues: blues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.contiguous.size()
            + self.length.size()
            + self.cmap.size()
            + self.colors.size()
            + self.reds.size()
            + self.greens.size()
            + self.blues.size()
    }
}
impl Request for AllocColorPlanesRequest {
    const OPCODE: u8 = 87;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = AllocColorPlanesReply;
}
#[derive(Clone, Debug, Default)]
pub struct AllocColorPlanesReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub red_mask: Card32,
    pub green_mask: Card32,
    pub blue_mask: Card32,
    pub pixels: Vec<Card32>,
}
impl AllocColorPlanesReply {}
impl AsByteSequence for AllocColorPlanesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.pixels.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.red_mask.as_bytes(&mut bytes[index..]);
        index += self.green_mask.as_bytes(&mut bytes[index..]);
        index += self.blue_mask.as_bytes(&mut bytes[index..]);
        index += 8;
        let block_len: usize = vector_as_bytes(&self.pixels, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocColorPlanesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (red_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        let (pixels, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            AllocColorPlanesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                red_mask: red_mask,
                green_mask: green_mask,
                blue_mask: blue_mask,
                pixels: pixels,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + self.red_mask.size()
            + self.green_mask.size()
            + self.blue_mask.size()
            + 8
            + {
                let block_len: usize = self.pixels.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct FreeColorsRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
    pub plane_mask: Card32,
    pub pixels: Vec<Card32>,
}
impl FreeColorsRequest {}
impl AsByteSequence for FreeColorsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index += self.plane_mask.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.pixels, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeColorsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (plane_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixels, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            FreeColorsRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
                plane_mask: plane_mask,
                pixels: pixels,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cmap.size()
            + self.plane_mask.size()
            + {
                let block_len: usize = self.pixels.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for FreeColorsRequest {
    const OPCODE: u8 = 88;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct Coloritem {
    pub pixel: Card32,
    pub red: Card16,
    pub green: Card16,
    pub blue: Card16,
    pub flags: ColorFlag,
}
impl Coloritem {}
impl AsByteSequence for Coloritem {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.pixel.as_bytes(&mut bytes[index..]);
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Coloritem from byte buffer");
        let (pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (ColorFlag, usize) = <ColorFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            Coloritem {
                pixel: pixel,
                red: red,
                green: green,
                blue: blue,
                flags: flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.pixel.size()
            + self.red.size()
            + self.green.size()
            + self.blue.size()
            + self.flags.size()
            + 1
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ColorFlag {
    pub inner: u8,
}
impl ColorFlag {
    #[inline]
    pub fn red(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_red(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn green(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_green(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn blue(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_blue(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(red: bool, green: bool, blue: bool) -> Self {
        let mut inner: u8 = 0;
        if red {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if green {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if blue {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        ColorFlag { inner: inner }
    }
}
impl AsByteSequence for ColorFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((ColorFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct StoreColorsRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
    pub items: Vec<Coloritem>,
}
impl StoreColorsRequest {}
impl AsByteSequence for StoreColorsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.items, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Coloritem>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing StoreColorsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (items, block_len): (Vec<Coloritem>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Coloritem>());
        Some((
            StoreColorsRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
                items: items,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.cmap.size() + {
            let block_len: usize = self.items.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Coloritem>());
            block_len + pad
        }
    }
}
impl Request for StoreColorsRequest {
    const OPCODE: u8 = 89;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct StoreNamedColorRequest {
    pub req_type: u8,
    pub flags: ColorFlag,
    pub length: u16,
    pub cmap: Colormap,
    pub pixel: Card32,
    pub name: String,
}
impl StoreNamedColorRequest {}
impl AsByteSequence for StoreNamedColorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index += self.pixel.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing StoreNamedColorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (ColorFlag, usize) = <ColorFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixel, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            StoreNamedColorRequest {
                req_type: req_type,
                flags: flags,
                length: length,
                cmap: cmap,
                pixel: pixel,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.flags.size()
            + self.length.size()
            + self.cmap.size()
            + self.pixel.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for StoreNamedColorRequest {
    const OPCODE: u8 = 90;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct Rgb {
    pub red: Card16,
    pub green: Card16,
    pub blue: Card16,
}
impl Rgb {}
impl AsByteSequence for Rgb {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.red.as_bytes(&mut bytes[index..]);
        index += self.green.as_bytes(&mut bytes[index..]);
        index += self.blue.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Rgb from byte buffer");
        let (red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            Rgb {
                red: red,
                green: green,
                blue: blue,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.red.size() + self.green.size() + self.blue.size() + 2
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryColorsRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
    pub pixels: Vec<Card32>,
}
impl QueryColorsRequest {}
impl AsByteSequence for QueryColorsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.pixels, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryColorsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pixels, block_len): (Vec<Card32>, usize) =
            vector_from_bytes(&bytes[index..], ((length as usize * 4) - index) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            QueryColorsRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
                pixels: pixels,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.cmap.size() + {
            let block_len: usize = self.pixels.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
            block_len + pad
        }
    }
}
impl Request for QueryColorsRequest {
    const OPCODE: u8 = 91;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryColorsReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryColorsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub colors: Vec<Rgb>,
}
impl QueryColorsReply {}
impl AsByteSequence for QueryColorsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.colors.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.colors, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rgb>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryColorsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (colors, block_len): (Vec<Rgb>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Rgb>());
        Some((
            QueryColorsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                colors: colors,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.colors.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Rgb>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct LookupColorRequest {
    pub req_type: u8,
    pub length: u16,
    pub cmap: Colormap,
    pub name: String,
}
impl LookupColorRequest {}
impl AsByteSequence for LookupColorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cmap.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LookupColorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cmap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            LookupColorRequest {
                req_type: req_type,
                length: length,
                cmap: cmap,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cmap.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
    }
}
impl Request for LookupColorRequest {
    const OPCODE: u8 = 92;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = LookupColorReply;
}
#[derive(Clone, Debug, Default)]
pub struct LookupColorReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub exact_red: Card16,
    pub exact_green: Card16,
    pub exact_blue: Card16,
    pub visual_red: Card16,
    pub visual_green: Card16,
    pub visual_blue: Card16,
}
impl LookupColorReply {}
impl AsByteSequence for LookupColorReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.exact_red.as_bytes(&mut bytes[index..]);
        index += self.exact_green.as_bytes(&mut bytes[index..]);
        index += self.exact_blue.as_bytes(&mut bytes[index..]);
        index += self.visual_red.as_bytes(&mut bytes[index..]);
        index += self.visual_green.as_bytes(&mut bytes[index..]);
        index += self.visual_blue.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LookupColorReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (exact_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (exact_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (exact_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (visual_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            LookupColorReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                exact_red: exact_red,
                exact_green: exact_green,
                exact_blue: exact_blue,
                visual_red: visual_red,
                visual_green: visual_green,
                visual_blue: visual_blue,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.exact_red.size()
            + self.exact_green.size()
            + self.exact_blue.size()
            + self.visual_red.size()
            + self.visual_green.size()
            + self.visual_blue.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct CreateCursorRequest {
    pub req_type: u8,
    pub length: u16,
    pub cid: Cursor,
    pub source: Pixmap,
    pub mask: Pixmap,
    pub fore_red: Card16,
    pub fore_green: Card16,
    pub fore_blue: Card16,
    pub back_red: Card16,
    pub back_green: Card16,
    pub back_blue: Card16,
    pub x: Card16,
    pub y: Card16,
}
impl CreateCursorRequest {}
impl AsByteSequence for CreateCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cid.as_bytes(&mut bytes[index..]);
        index += self.source.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.fore_red.as_bytes(&mut bytes[index..]);
        index += self.fore_green.as_bytes(&mut bytes[index..]);
        index += self.fore_blue.as_bytes(&mut bytes[index..]);
        index += self.back_red.as_bytes(&mut bytes[index..]);
        index += self.back_green.as_bytes(&mut bytes[index..]);
        index += self.back_blue.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cid, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (Pixmap, usize) = <Pixmap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateCursorRequest {
                req_type: req_type,
                length: length,
                cid: cid,
                source: source,
                mask: mask,
                fore_red: fore_red,
                fore_green: fore_green,
                fore_blue: fore_blue,
                back_red: back_red,
                back_green: back_green,
                back_blue: back_blue,
                x: x,
                y: y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cid.size()
            + self.source.size()
            + self.mask.size()
            + self.fore_red.size()
            + self.fore_green.size()
            + self.fore_blue.size()
            + self.back_red.size()
            + self.back_green.size()
            + self.back_blue.size()
            + self.x.size()
            + self.y.size()
    }
}
impl Request for CreateCursorRequest {
    const OPCODE: u8 = 93;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
pub const PIXMAP_NONE: Pixmap = <Pixmap>::const_from_xid(0);
#[derive(Clone, Debug, Default)]
pub struct CreateGlyphCursorRequest {
    pub req_type: u8,
    pub length: u16,
    pub cid: Cursor,
    pub source_font: Font,
    pub mask_font: Font,
    pub source_char: Card16,
    pub mask_char: Card16,
    pub fore_red: Card16,
    pub fore_green: Card16,
    pub fore_blue: Card16,
    pub back_red: Card16,
    pub back_green: Card16,
    pub back_blue: Card16,
}
impl CreateGlyphCursorRequest {}
impl AsByteSequence for CreateGlyphCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cid.as_bytes(&mut bytes[index..]);
        index += self.source_font.as_bytes(&mut bytes[index..]);
        index += self.mask_font.as_bytes(&mut bytes[index..]);
        index += self.source_char.as_bytes(&mut bytes[index..]);
        index += self.mask_char.as_bytes(&mut bytes[index..]);
        index += self.fore_red.as_bytes(&mut bytes[index..]);
        index += self.fore_green.as_bytes(&mut bytes[index..]);
        index += self.fore_blue.as_bytes(&mut bytes[index..]);
        index += self.back_red.as_bytes(&mut bytes[index..]);
        index += self.back_green.as_bytes(&mut bytes[index..]);
        index += self.back_blue.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateGlyphCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cid, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source_font, sz): (Font, usize) = <Font>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_font, sz): (Font, usize) = <Font>::from_bytes(&bytes[index..])?;
        index += sz;
        let (source_char, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask_char, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateGlyphCursorRequest {
                req_type: req_type,
                length: length,
                cid: cid,
                source_font: source_font,
                mask_font: mask_font,
                source_char: source_char,
                mask_char: mask_char,
                fore_red: fore_red,
                fore_green: fore_green,
                fore_blue: fore_blue,
                back_red: back_red,
                back_green: back_green,
                back_blue: back_blue,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cid.size()
            + self.source_font.size()
            + self.mask_font.size()
            + self.source_char.size()
            + self.mask_char.size()
            + self.fore_red.size()
            + self.fore_green.size()
            + self.fore_blue.size()
            + self.back_red.size()
            + self.back_green.size()
            + self.back_blue.size()
    }
}
impl Request for CreateGlyphCursorRequest {
    const OPCODE: u8 = 94;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
pub const FONT_NONE: Font = <Font>::const_from_xid(0);
#[derive(Clone, Debug, Default)]
pub struct FreeCursorRequest {
    pub req_type: u8,
    pub length: u16,
    pub cursor: Cursor,
}
impl FreeCursorRequest {}
impl AsByteSequence for FreeCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FreeCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            FreeCursorRequest {
                req_type: req_type,
                length: length,
                cursor: cursor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.cursor.size()
    }
}
impl Request for FreeCursorRequest {
    const OPCODE: u8 = 95;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct RecolorCursorRequest {
    pub req_type: u8,
    pub length: u16,
    pub cursor: Cursor,
    pub fore_red: Card16,
    pub fore_green: Card16,
    pub fore_blue: Card16,
    pub back_red: Card16,
    pub back_green: Card16,
    pub back_blue: Card16,
}
impl RecolorCursorRequest {}
impl AsByteSequence for RecolorCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.fore_red.as_bytes(&mut bytes[index..]);
        index += self.fore_green.as_bytes(&mut bytes[index..]);
        index += self.fore_blue.as_bytes(&mut bytes[index..]);
        index += self.back_red.as_bytes(&mut bytes[index..]);
        index += self.back_green.as_bytes(&mut bytes[index..]);
        index += self.back_blue.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RecolorCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (fore_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_red, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_green, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (back_blue, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            RecolorCursorRequest {
                req_type: req_type,
                length: length,
                cursor: cursor,
                fore_red: fore_red,
                fore_green: fore_green,
                fore_blue: fore_blue,
                back_red: back_red,
                back_green: back_green,
                back_blue: back_blue,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.cursor.size()
            + self.fore_red.size()
            + self.fore_green.size()
            + self.fore_blue.size()
            + self.back_red.size()
            + self.back_green.size()
            + self.back_blue.size()
    }
}
impl Request for RecolorCursorRequest {
    const OPCODE: u8 = 96;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct QueryBestSizeRequest {
    pub req_type: u8,
    pub class: QueryShapeOf,
    pub length: u16,
    pub drawable: Drawable,
    pub width: Card16,
    pub height: Card16,
}
impl QueryBestSizeRequest {}
impl AsByteSequence for QueryBestSizeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.class.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryBestSizeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (class, sz): (QueryShapeOf, usize) = <QueryShapeOf>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryBestSizeRequest {
                req_type: req_type,
                class: class,
                length: length,
                drawable: drawable,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.class.size()
            + self.length.size()
            + self.drawable.size()
            + self.width.size()
            + self.height.size()
    }
}
impl Request for QueryBestSizeRequest {
    const OPCODE: u8 = 97;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryBestSizeReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryBestSizeReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub width: Card16,
    pub height: Card16,
}
impl QueryBestSizeReply {}
impl AsByteSequence for QueryBestSizeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryBestSizeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryBestSizeReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.width.size()
            + self.height.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum QueryShapeOf {
    LargestCursor = 0,
    FastestTile = 1,
    FastestStipple = 2,
}
impl AsByteSequence for QueryShapeOf {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::LargestCursor, sz)),
            1 => Some((Self::FastestTile, sz)),
            2 => Some((Self::FastestStipple, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for QueryShapeOf {
    #[inline]
    fn default() -> QueryShapeOf {
        QueryShapeOf::LargestCursor
    }
}
#[derive(Clone, Debug, Default)]
pub struct QueryExtensionRequest {
    pub req_type: u8,
    pub length: u16,
    pub name: String,
}
impl QueryExtensionRequest {}
impl AsByteSequence for QueryExtensionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryExtensionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (String, usize) = string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            QueryExtensionRequest {
                req_type: req_type,
                length: length,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card16>() + 2 + {
            let block_len: usize = self.name.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
impl Request for QueryExtensionRequest {
    const OPCODE: u8 = 98;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryExtensionReply;
}
#[derive(Clone, Debug, Default)]
pub struct QueryExtensionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub present: bool,
    pub major_opcode: Card8,
    pub first_event: Card8,
    pub first_error: Card8,
}
impl QueryExtensionReply {}
impl AsByteSequence for QueryExtensionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.present.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += self.first_event.as_bytes(&mut bytes[index..]);
        index += self.first_error.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryExtensionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (present, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_event, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_error, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            QueryExtensionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                present: present,
                major_opcode: major_opcode,
                first_event: first_event,
                first_error: first_error,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.present.size()
            + self.major_opcode.size()
            + self.first_event.size()
            + self.first_error.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct ListExtensionsRequest {
    pub req_type: u8,
    pub length: u16,
}
impl ListExtensionsRequest {}
impl AsByteSequence for ListExtensionsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListExtensionsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListExtensionsRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for ListExtensionsRequest {
    const OPCODE: u8 = 99;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListExtensionsReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListExtensionsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub names: Vec<Str>,
}
impl ListExtensionsReply {}
impl AsByteSequence for ListExtensionsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += (self.names.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListExtensionsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (names, block_len): (Vec<Str>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Str>());
        Some((
            ListExtensionsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                names: names,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + ::core::mem::size_of::<Card8>()
            + self.sequence.size()
            + self.length.size()
            + 24
            + {
                let block_len: usize = self.names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Str>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangeKeyboardMappingRequest {
    pub req_type: u8,
    pub keycode_count: Card8,
    pub length: u16,
    pub first_keycode: Keycode,
    pub keysyms_per_keycode: Card8,
    pub keysyms: Vec<Keysym>,
}
impl ChangeKeyboardMappingRequest {}
impl AsByteSequence for ChangeKeyboardMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.keycode_count.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.first_keycode.as_bytes(&mut bytes[index..]);
        index += self.keysyms_per_keycode.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeKeyboardMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycode_count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms_per_keycode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (keysyms, block_len): (Vec<Keysym>, usize) = vector_from_bytes(
            &bytes[index..],
            ((keycode_count as usize) * (keysyms_per_keycode as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        Some((
            ChangeKeyboardMappingRequest {
                req_type: req_type,
                keycode_count: keycode_count,
                length: length,
                first_keycode: first_keycode,
                keysyms_per_keycode: keysyms_per_keycode,
                keysyms: keysyms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.keycode_count.size()
            + self.length.size()
            + self.first_keycode.size()
            + self.keysyms_per_keycode.size()
            + 2
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
    }
}
impl Request for ChangeKeyboardMappingRequest {
    const OPCODE: u8 = 100;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetKeyboardMappingRequest {
    pub req_type: u8,
    pub length: u16,
    pub first_keycode: Keycode,
    pub count: Card8,
}
impl GetKeyboardMappingRequest {}
impl AsByteSequence for GetKeyboardMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.first_keycode.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetKeyboardMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetKeyboardMappingRequest {
                req_type: req_type,
                length: length,
                first_keycode: first_keycode,
                count: count,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.first_keycode.size()
            + self.count.size()
    }
}
impl Request for GetKeyboardMappingRequest {
    const OPCODE: u8 = 101;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetKeyboardMappingReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetKeyboardMappingReply {
    pub reply_type: u8,
    pub keysyms_per_keycode: Byte,
    pub sequence: u16,
    pub keysyms: Vec<Keysym>,
}
impl GetKeyboardMappingReply {}
impl AsByteSequence for GetKeyboardMappingReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.keysyms_per_keycode.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += (self.keysyms.len() as u32).as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetKeyboardMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms_per_keycode, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (keysyms, block_len): (Vec<Keysym>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        Some((
            GetKeyboardMappingReply {
                reply_type: reply_type,
                keysyms_per_keycode: keysyms_per_keycode,
                sequence: sequence,
                keysyms: keysyms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.keysyms_per_keycode.size()
            + self.sequence.size()
            + ::core::mem::size_of::<u32>()
            + 24
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangeKeyboardControlRequest {
    pub req_type: u8,
    pub length: u16,
    pub value_mask: Kb,
    pub key_click_percent: Int32,
    pub bell_percent: Int32,
    pub bell_pitch: Int32,
    pub bell_duration: Int32,
    pub led: Card32,
    pub led_mode: LedMode,
    pub key: Keycode32,
    pub auto_repeat_mode: AutoRepeatMode,
}
impl ChangeKeyboardControlRequest {}
impl AsByteSequence for ChangeKeyboardControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        let cond0 = (self.value_mask);
        if cond0.key_click_percent() {
            index += self.key_click_percent.as_bytes(&mut bytes[index..]);
        }
        if cond0.bell_percent() {
            index += self.bell_percent.as_bytes(&mut bytes[index..]);
        }
        if cond0.bell_pitch() {
            index += self.bell_pitch.as_bytes(&mut bytes[index..]);
        }
        if cond0.bell_duration() {
            index += self.bell_duration.as_bytes(&mut bytes[index..]);
        }
        if cond0.led() {
            index += self.led.as_bytes(&mut bytes[index..]);
        }
        if cond0.led_mode() {
            index += self.led_mode.as_bytes(&mut bytes[index..]);
        }
        if cond0.key() {
            index += self.key.as_bytes(&mut bytes[index..]);
        }
        if cond0.auto_repeat_mode() {
            index += self.auto_repeat_mode.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeKeyboardControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (Kb, usize) = <Kb>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (value_mask);
        let key_click_percent: Int32 = if cond0.key_click_percent() {
            let (key_click_percent, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            key_click_percent
        } else {
            Default::default()
        };
        let bell_percent: Int32 = if cond0.bell_percent() {
            let (bell_percent, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            bell_percent
        } else {
            Default::default()
        };
        let bell_pitch: Int32 = if cond0.bell_pitch() {
            let (bell_pitch, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            bell_pitch
        } else {
            Default::default()
        };
        let bell_duration: Int32 = if cond0.bell_duration() {
            let (bell_duration, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            bell_duration
        } else {
            Default::default()
        };
        let led: Card32 = if cond0.led() {
            let (led, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led
        } else {
            Default::default()
        };
        let led_mode: LedMode = if cond0.led_mode() {
            let (led_mode, sz): (LedMode, usize) = <LedMode>::from_bytes(&bytes[index..])?;
            index += sz;
            led_mode
        } else {
            Default::default()
        };
        let key: Keycode32 = if cond0.key() {
            let (key, sz): (Keycode32, usize) = <Keycode32>::from_bytes(&bytes[index..])?;
            index += sz;
            key
        } else {
            Default::default()
        };
        let auto_repeat_mode: AutoRepeatMode = if cond0.auto_repeat_mode() {
            let (auto_repeat_mode, sz): (AutoRepeatMode, usize) =
                <AutoRepeatMode>::from_bytes(&bytes[index..])?;
            index += sz;
            auto_repeat_mode
        } else {
            Default::default()
        };
        Some((
            ChangeKeyboardControlRequest {
                req_type: req_type,
                length: length,
                value_mask: value_mask,
                key_click_percent: key_click_percent,
                bell_percent: bell_percent,
                bell_pitch: bell_pitch,
                bell_duration: bell_duration,
                led: led,
                led_mode: led_mode,
                key: key,
                auto_repeat_mode: auto_repeat_mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.value_mask.size()
            + self.key_click_percent.size()
            + self.bell_percent.size()
            + self.bell_pitch.size()
            + self.bell_duration.size()
            + self.led.size()
            + self.led_mode.size()
            + self.key.size()
            + self.auto_repeat_mode.size()
    }
}
impl Request for ChangeKeyboardControlRequest {
    const OPCODE: u8 = 102;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Kb {
    pub inner: u32,
}
impl Kb {
    #[inline]
    pub fn key_click_percent(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_key_click_percent(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn bell_percent(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_bell_percent(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn bell_pitch(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_bell_pitch(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn bell_duration(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_bell_duration(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn led(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_led(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn led_mode(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_led_mode(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn key(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_key(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn auto_repeat_mode(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_auto_repeat_mode(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        key_click_percent: bool,
        bell_percent: bool,
        bell_pitch: bool,
        bell_duration: bool,
        led: bool,
        led_mode: bool,
        key: bool,
        auto_repeat_mode: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if key_click_percent {
            inner |= 1 << 0;
        } else {
            inner &= !(1 << 0);
        }
        if bell_percent {
            inner |= 1 << 1;
        } else {
            inner &= !(1 << 1);
        }
        if bell_pitch {
            inner |= 1 << 2;
        } else {
            inner &= !(1 << 2);
        }
        if bell_duration {
            inner |= 1 << 3;
        } else {
            inner &= !(1 << 3);
        }
        if led {
            inner |= 1 << 4;
        } else {
            inner &= !(1 << 4);
        }
        if led_mode {
            inner |= 1 << 5;
        } else {
            inner &= !(1 << 5);
        }
        if key {
            inner |= 1 << 6;
        } else {
            inner &= !(1 << 6);
        }
        if auto_repeat_mode {
            inner |= 1 << 7;
        } else {
            inner &= !(1 << 7);
        }
        Kb { inner: inner }
    }
}
impl AsByteSequence for Kb {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Kb { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetKeyboardControlRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetKeyboardControlRequest {}
impl AsByteSequence for GetKeyboardControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetKeyboardControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetKeyboardControlRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GetKeyboardControlRequest {
    const OPCODE: u8 = 103;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetKeyboardControlReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetKeyboardControlReply {
    pub reply_type: u8,
    pub global_auto_repeat: AutoRepeatMode,
    pub sequence: u16,
    pub length: u32,
    pub led_mask: Card32,
    pub key_click_percent: Card8,
    pub bell_percent: Card8,
    pub bell_pitch: Card16,
    pub bell_duration: Card16,
    pub auto_repeats: [Card8; 32],
}
impl GetKeyboardControlReply {}
impl AsByteSequence for GetKeyboardControlReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.global_auto_repeat.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.led_mask.as_bytes(&mut bytes[index..]);
        index += self.key_click_percent.as_bytes(&mut bytes[index..]);
        index += self.bell_percent.as_bytes(&mut bytes[index..]);
        index += self.bell_pitch.as_bytes(&mut bytes[index..]);
        index += self.bell_duration.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.auto_repeats.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetKeyboardControlReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (global_auto_repeat, sz): (AutoRepeatMode, usize) =
            <AutoRepeatMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key_click_percent, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_percent, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_pitch, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_duration, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (auto_repeats, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetKeyboardControlReply {
                reply_type: reply_type,
                global_auto_repeat: global_auto_repeat,
                sequence: sequence,
                length: length,
                led_mask: led_mask,
                key_click_percent: key_click_percent,
                bell_percent: bell_percent,
                bell_pitch: bell_pitch,
                bell_duration: bell_duration,
                auto_repeats: auto_repeats,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.global_auto_repeat.size()
            + self.sequence.size()
            + self.length.size()
            + self.led_mask.size()
            + self.key_click_percent.size()
            + self.bell_percent.size()
            + self.bell_pitch.size()
            + self.bell_duration.size()
            + 2
            + self.auto_repeats.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AutoRepeatMode {
    Off = 0,
    On = 1,
    Default = 2,
}
impl AsByteSequence for AutoRepeatMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Off, sz)),
            1 => Some((Self::On, sz)),
            2 => Some((Self::Default, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for AutoRepeatMode {
    #[inline]
    fn default() -> AutoRepeatMode {
        AutoRepeatMode::Off
    }
}
#[derive(Clone, Debug, Default)]
pub struct BellRequest {
    pub req_type: u8,
    pub percent: Int8,
    pub length: u16,
}
impl BellRequest {}
impl AsByteSequence for BellRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.percent.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BellRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BellRequest {
                req_type: req_type,
                percent: percent,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.percent.size() + self.length.size()
    }
}
impl Request for BellRequest {
    const OPCODE: u8 = 104;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ChangePointerControlRequest {
    pub req_type: u8,
    pub length: u16,
    pub acceleration_numerator: Int16,
    pub acceleration_denominator: Int16,
    pub threshold: Int16,
    pub do_acceleration: bool,
    pub do_threshold: bool,
}
impl ChangePointerControlRequest {}
impl AsByteSequence for ChangePointerControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.acceleration_numerator.as_bytes(&mut bytes[index..]);
        index += self.acceleration_denominator.as_bytes(&mut bytes[index..]);
        index += self.threshold.as_bytes(&mut bytes[index..]);
        index += self.do_acceleration.as_bytes(&mut bytes[index..]);
        index += self.do_threshold.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangePointerControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (acceleration_numerator, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (acceleration_denominator, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (threshold, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (do_acceleration, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (do_threshold, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ChangePointerControlRequest {
                req_type: req_type,
                length: length,
                acceleration_numerator: acceleration_numerator,
                acceleration_denominator: acceleration_denominator,
                threshold: threshold,
                do_acceleration: do_acceleration,
                do_threshold: do_threshold,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.acceleration_numerator.size()
            + self.acceleration_denominator.size()
            + self.threshold.size()
            + self.do_acceleration.size()
            + self.do_threshold.size()
    }
}
impl Request for ChangePointerControlRequest {
    const OPCODE: u8 = 105;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetPointerControlRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetPointerControlRequest {}
impl AsByteSequence for GetPointerControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPointerControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPointerControlRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GetPointerControlRequest {
    const OPCODE: u8 = 106;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPointerControlReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPointerControlReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub acceleration_numerator: Card16,
    pub acceleration_denominator: Card16,
    pub threshold: Card16,
}
impl GetPointerControlReply {}
impl AsByteSequence for GetPointerControlReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.acceleration_numerator.as_bytes(&mut bytes[index..]);
        index += self.acceleration_denominator.as_bytes(&mut bytes[index..]);
        index += self.threshold.as_bytes(&mut bytes[index..]);
        index += 18;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPointerControlReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (acceleration_numerator, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (acceleration_denominator, sz): (Card16, usize) =
            <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (threshold, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 18;
        Some((
            GetPointerControlReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                acceleration_numerator: acceleration_numerator,
                acceleration_denominator: acceleration_denominator,
                threshold: threshold,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.acceleration_numerator.size()
            + self.acceleration_denominator.size()
            + self.threshold.size()
            + 18
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetScreenSaverRequest {
    pub req_type: u8,
    pub length: u16,
    pub timeout: Int16,
    pub interval: Int16,
    pub prefer_blanking: Blanking,
    pub allow_exposures: Exposures,
}
impl SetScreenSaverRequest {}
impl AsByteSequence for SetScreenSaverRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timeout.as_bytes(&mut bytes[index..]);
        index += self.interval.as_bytes(&mut bytes[index..]);
        index += self.prefer_blanking.as_bytes(&mut bytes[index..]);
        index += self.allow_exposures.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetScreenSaverRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timeout, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (interval, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (prefer_blanking, sz): (Blanking, usize) = <Blanking>::from_bytes(&bytes[index..])?;
        index += sz;
        let (allow_exposures, sz): (Exposures, usize) = <Exposures>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetScreenSaverRequest {
                req_type: req_type,
                length: length,
                timeout: timeout,
                interval: interval,
                prefer_blanking: prefer_blanking,
                allow_exposures: allow_exposures,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.timeout.size()
            + self.interval.size()
            + self.prefer_blanking.size()
            + self.allow_exposures.size()
    }
}
impl Request for SetScreenSaverRequest {
    const OPCODE: u8 = 107;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Blanking {
    NotPreferred = 0,
    Preferred = 1,
    Default = 2,
}
impl AsByteSequence for Blanking {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NotPreferred, sz)),
            1 => Some((Self::Preferred, sz)),
            2 => Some((Self::Default, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Blanking {
    #[inline]
    fn default() -> Blanking {
        Blanking::NotPreferred
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Exposures {
    NotAllowed = 0,
    Allowed = 1,
    Default = 2,
}
impl AsByteSequence for Exposures {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NotAllowed, sz)),
            1 => Some((Self::Allowed, sz)),
            2 => Some((Self::Default, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Exposures {
    #[inline]
    fn default() -> Exposures {
        Exposures::NotAllowed
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetScreenSaverRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetScreenSaverRequest {}
impl AsByteSequence for GetScreenSaverRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenSaverRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetScreenSaverRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GetScreenSaverRequest {
    const OPCODE: u8 = 108;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetScreenSaverReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetScreenSaverReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub timeout: Card16,
    pub interval: Card16,
    pub prefer_blanking: Blanking,
    pub allow_exposures: Exposures,
}
impl GetScreenSaverReply {}
impl AsByteSequence for GetScreenSaverReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.timeout.as_bytes(&mut bytes[index..]);
        index += self.interval.as_bytes(&mut bytes[index..]);
        index += self.prefer_blanking.as_bytes(&mut bytes[index..]);
        index += self.allow_exposures.as_bytes(&mut bytes[index..]);
        index += 18;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetScreenSaverReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (interval, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (prefer_blanking, sz): (Blanking, usize) = <Blanking>::from_bytes(&bytes[index..])?;
        index += sz;
        let (allow_exposures, sz): (Exposures, usize) = <Exposures>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 18;
        Some((
            GetScreenSaverReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                timeout: timeout,
                interval: interval,
                prefer_blanking: prefer_blanking,
                allow_exposures: allow_exposures,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.timeout.size()
            + self.interval.size()
            + self.prefer_blanking.size()
            + self.allow_exposures.size()
            + 18
    }
}
#[derive(Clone, Debug, Default)]
pub struct ChangeHostsRequest {
    pub req_type: u8,
    pub mode: HostMode,
    pub length: u16,
    pub family: Family,
    pub address: Vec<Byte>,
}
impl ChangeHostsRequest {}
impl AsByteSequence for ChangeHostsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.family.as_bytes(&mut bytes[index..]);
        index += 1;
        index += (self.address.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.address, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeHostsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (HostMode, usize) = <HostMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (family, sz): (Family, usize) = <Family>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (address, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Byte>());
        Some((
            ChangeHostsRequest {
                req_type: req_type,
                mode: mode,
                length: length,
                family: family,
                address: address,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.mode.size()
            + self.length.size()
            + self.family.size()
            + 1
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.address.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Byte>());
                block_len + pad
            }
    }
}
impl Request for ChangeHostsRequest {
    const OPCODE: u8 = 109;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum HostMode {
    Insert = 0,
    Delete = 1,
}
impl AsByteSequence for HostMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Insert, sz)),
            1 => Some((Self::Delete, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for HostMode {
    #[inline]
    fn default() -> HostMode {
        HostMode::Insert
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Family {
    Internet = 0,
    DeCnet = 1,
    Chaos = 2,
    ServerInterpreted = 5,
    Internet6 = 6,
}
impl AsByteSequence for Family {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Internet, sz)),
            1 => Some((Self::DeCnet, sz)),
            2 => Some((Self::Chaos, sz)),
            5 => Some((Self::ServerInterpreted, sz)),
            6 => Some((Self::Internet6, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Family {
    #[inline]
    fn default() -> Family {
        Family::Internet
    }
}
#[derive(Clone, Debug, Default)]
pub struct Host {
    pub family: Family,
    pub address: Vec<Byte>,
}
impl Host {}
impl AsByteSequence for Host {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.family.as_bytes(&mut bytes[index..]);
        index += 1;
        index += (self.address.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.address, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Host from byte buffer");
        let (family, sz): (Family, usize) = <Family>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (address, block_len): (Vec<Byte>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            Host {
                family: family,
                address: address,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.family.size() + 1 + ::core::mem::size_of::<Card16>() + {
            let block_len: usize = self.address.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, 4);
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ListHostsRequest {
    pub req_type: u8,
    pub length: u16,
}
impl ListHostsRequest {}
impl AsByteSequence for ListHostsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListHostsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListHostsRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for ListHostsRequest {
    const OPCODE: u8 = 110;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListHostsReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListHostsReply {
    pub reply_type: u8,
    pub mode: AccessControl,
    pub sequence: u16,
    pub length: u32,
    pub hosts: Vec<Host>,
}
impl ListHostsReply {}
impl AsByteSequence for ListHostsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.hosts.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.hosts, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Host>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListHostsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (AccessControl, usize) = <AccessControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (hosts, block_len): (Vec<Host>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Host>());
        Some((
            ListHostsReply {
                reply_type: reply_type,
                mode: mode,
                sequence: sequence,
                length: length,
                hosts: hosts,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.mode.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.hosts.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Host>());
                block_len + pad
            }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum AccessControl {
    Disable = 0,
    Enable = 1,
}
impl AsByteSequence for AccessControl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Disable, sz)),
            1 => Some((Self::Enable, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for AccessControl {
    #[inline]
    fn default() -> AccessControl {
        AccessControl::Disable
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetAccessControlRequest {
    pub req_type: u8,
    pub mode: AccessControl,
    pub length: u16,
}
impl SetAccessControlRequest {}
impl AsByteSequence for SetAccessControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetAccessControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (AccessControl, usize) = <AccessControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetAccessControlRequest {
                req_type: req_type,
                mode: mode,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.mode.size() + self.length.size()
    }
}
impl Request for SetAccessControlRequest {
    const OPCODE: u8 = 111;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetCloseDownModeRequest {
    pub req_type: u8,
    pub mode: CloseDown,
    pub length: u16,
}
impl SetCloseDownModeRequest {}
impl AsByteSequence for SetCloseDownModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCloseDownModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (CloseDown, usize) = <CloseDown>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetCloseDownModeRequest {
                req_type: req_type,
                mode: mode,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.mode.size() + self.length.size()
    }
}
impl Request for SetCloseDownModeRequest {
    const OPCODE: u8 = 112;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CloseDown {
    DestroyAll = 0,
    RetainPermanent = 1,
    RetainTemporary = 2,
}
impl AsByteSequence for CloseDown {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::DestroyAll, sz)),
            1 => Some((Self::RetainPermanent, sz)),
            2 => Some((Self::RetainTemporary, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for CloseDown {
    #[inline]
    fn default() -> CloseDown {
        CloseDown::DestroyAll
    }
}
#[derive(Clone, Debug, Default)]
pub struct KillClientRequest {
    pub req_type: u8,
    pub length: u16,
    pub resource: Card32,
}
impl KillClientRequest {}
impl AsByteSequence for KillClientRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.resource.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KillClientRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resource, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KillClientRequest {
                req_type: req_type,
                length: length,
                resource: resource,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.resource.size()
    }
}
impl Request for KillClientRequest {
    const OPCODE: u8 = 113;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kill {
    AllTemporary = 0,
}
impl AsByteSequence for Kill {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::AllTemporary, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for Kill {
    #[inline]
    fn default() -> Kill {
        Kill::AllTemporary
    }
}
#[derive(Clone, Debug, Default)]
pub struct RotatePropertiesRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub delta: Int16,
    pub atoms: Vec<Atom>,
}
impl RotatePropertiesRequest {}
impl AsByteSequence for RotatePropertiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += (self.atoms.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.delta.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.atoms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RotatePropertiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delta, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (atoms, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            RotatePropertiesRequest {
                req_type: req_type,
                length: length,
                window: window,
                delta: delta,
                atoms: atoms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.window.size()
            + ::core::mem::size_of::<Card16>()
            + self.delta.size()
            + {
                let block_len: usize = self.atoms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
impl Request for RotatePropertiesRequest {
    const OPCODE: u8 = 114;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct ForceScreenSaverRequest {
    pub req_type: u8,
    pub mode: ScreenSaver,
    pub length: u16,
}
impl ForceScreenSaverRequest {}
impl AsByteSequence for ForceScreenSaverRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ForceScreenSaverRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (ScreenSaver, usize) = <ScreenSaver>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ForceScreenSaverRequest {
                req_type: req_type,
                mode: mode,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.mode.size() + self.length.size()
    }
}
impl Request for ForceScreenSaverRequest {
    const OPCODE: u8 = 115;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ScreenSaver {
    Reset = 0,
    Active = 1,
}
impl AsByteSequence for ScreenSaver {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Reset, sz)),
            1 => Some((Self::Active, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ScreenSaver {
    #[inline]
    fn default() -> ScreenSaver {
        ScreenSaver::Reset
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetPointerMappingRequest {
    pub req_type: u8,
    pub length: u16,
    pub map: Vec<Card8>,
}
impl SetPointerMappingRequest {}
impl AsByteSequence for SetPointerMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += (self.map.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.map, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPointerMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            SetPointerMappingRequest {
                req_type: req_type,
                length: length,
                map: map,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + ::core::mem::size_of::<Card8>() + self.length.size() + {
            let block_len: usize = self.map.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
            block_len + pad
        }
    }
}
impl Request for SetPointerMappingRequest {
    const OPCODE: u8 = 116;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetPointerMappingReply;
}
#[derive(Clone, Debug, Default)]
pub struct SetPointerMappingReply {
    pub reply_type: u8,
    pub status: MappingStatus,
    pub sequence: u16,
    pub length: u32,
}
impl SetPointerMappingReply {}
impl AsByteSequence for SetPointerMappingReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetPointerMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (MappingStatus, usize) = <MappingStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetPointerMappingReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.status.size() + self.sequence.size() + self.length.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MappingStatus {
    Success = 0,
    Busy = 1,
    Failure = 2,
}
impl AsByteSequence for MappingStatus {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Success, sz)),
            1 => Some((Self::Busy, sz)),
            2 => Some((Self::Failure, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for MappingStatus {
    #[inline]
    fn default() -> MappingStatus {
        MappingStatus::Success
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetPointerMappingRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetPointerMappingRequest {}
impl AsByteSequence for GetPointerMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPointerMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetPointerMappingRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GetPointerMappingRequest {
    const OPCODE: u8 = 117;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetPointerMappingReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetPointerMappingReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub map: Vec<Card8>,
}
impl GetPointerMappingReply {}
impl AsByteSequence for GetPointerMappingReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += (self.map.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.map, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetPointerMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (map, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            GetPointerMappingReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                map: map,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + ::core::mem::size_of::<Card8>()
            + self.sequence.size()
            + self.length.size()
            + 24
            + {
                let block_len: usize = self.map.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetModifierMappingRequest {
    pub req_type: u8,
    pub keycodes_per_modifier: Card8,
    pub length: u16,
    pub keycodes: Vec<Keycode>,
}
impl SetModifierMappingRequest {}
impl AsByteSequence for SetModifierMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.keycodes_per_modifier.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keycodes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keycode>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetModifierMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycodes_per_modifier, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycodes, block_len): (Vec<Keycode>, usize) = vector_from_bytes(
            &bytes[index..],
            ((keycodes_per_modifier as usize) * (8)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keycode>());
        Some((
            SetModifierMappingRequest {
                req_type: req_type,
                keycodes_per_modifier: keycodes_per_modifier,
                length: length,
                keycodes: keycodes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.keycodes_per_modifier.size() + self.length.size() + {
            let block_len: usize = self.keycodes.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keycode>());
            block_len + pad
        }
    }
}
impl Request for SetModifierMappingRequest {
    const OPCODE: u8 = 118;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetModifierMappingReply;
}
#[derive(Clone, Debug, Default)]
pub struct SetModifierMappingReply {
    pub reply_type: u8,
    pub status: MappingStatus,
    pub sequence: u16,
    pub length: u32,
}
impl SetModifierMappingReply {}
impl AsByteSequence for SetModifierMappingReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetModifierMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (MappingStatus, usize) = <MappingStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetModifierMappingReply {
                reply_type: reply_type,
                status: status,
                sequence: sequence,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size() + self.status.size() + self.sequence.size() + self.length.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetModifierMappingRequest {
    pub req_type: u8,
    pub length: u16,
}
impl GetModifierMappingRequest {}
impl AsByteSequence for GetModifierMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetModifierMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetModifierMappingRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for GetModifierMappingRequest {
    const OPCODE: u8 = 119;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetModifierMappingReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetModifierMappingReply {
    pub reply_type: u8,
    pub keycodes_per_modifier: Card8,
    pub sequence: u16,
    pub length: u32,
    pub keycodes: Vec<Keycode>,
}
impl GetModifierMappingReply {}
impl AsByteSequence for GetModifierMappingReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.keycodes_per_modifier.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 24;
        let block_len: usize = vector_as_bytes(&self.keycodes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keycode>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetModifierMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycodes_per_modifier, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 24;
        let (keycodes, block_len): (Vec<Keycode>, usize) = vector_from_bytes(
            &bytes[index..],
            ((keycodes_per_modifier as usize) * (8)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keycode>());
        Some((
            GetModifierMappingReply {
                reply_type: reply_type,
                keycodes_per_modifier: keycodes_per_modifier,
                sequence: sequence,
                length: length,
                keycodes: keycodes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.keycodes_per_modifier.size()
            + self.sequence.size()
            + self.length.size()
            + 24
            + {
                let block_len: usize = self.keycodes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keycode>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct NoOperationRequest {
    pub req_type: u8,
    pub length: u16,
}
impl NoOperationRequest {}
impl AsByteSequence for NoOperationRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NoOperationRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NoOperationRequest {
                req_type: req_type,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size()
    }
}
impl Request for NoOperationRequest {
    const OPCODE: u8 = 127;
    const EXTENSION: Option<&'static str> = None;
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum MapIndex {
    Shift = 0,
    Lock = 1,
    Control = 2,
    One = 3,
    Two = 4,
    Three = 5,
    Four = 6,
    Five = 7,
}
impl AsByteSequence for MapIndex {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Shift, sz)),
            1 => Some((Self::Lock, sz)),
            2 => Some((Self::Control, sz)),
            3 => Some((Self::One, sz)),
            4 => Some((Self::Two, sz)),
            5 => Some((Self::Three, sz)),
            6 => Some((Self::Four, sz)),
            7 => Some((Self::Five, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for MapIndex {
    #[inline]
    fn default() -> MapIndex {
        MapIndex::Shift
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SubwindowMode {
    ClipByChildren = 0,
    IncludeInferiors = 1,
}
impl AsByteSequence for SubwindowMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::ClipByChildren, sz)),
            1 => Some((Self::IncludeInferiors, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for SubwindowMode {
    #[inline]
    fn default() -> SubwindowMode {
        SubwindowMode::ClipByChildren
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FillStyle {
    Solid = 0,
    Tiled = 1,
    Stippled = 2,
    OpaqueStippled = 3,
}
impl AsByteSequence for FillStyle {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Solid, sz)),
            1 => Some((Self::Tiled, sz)),
            2 => Some((Self::Stippled, sz)),
            3 => Some((Self::OpaqueStippled, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for FillStyle {
    #[inline]
    fn default() -> FillStyle {
        FillStyle::Solid
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedMode {
    Off = 0,
    On = 1,
}
impl AsByteSequence for LedMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Off, sz)),
            1 => Some((Self::On, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for LedMode {
    #[inline]
    fn default() -> LedMode {
        LedMode::Off
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ButtonMask {
    pub inner: i32,
}
impl ButtonMask {
    #[inline]
    pub fn One(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_One(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn Two(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_Two(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn Three(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_Three(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn Four(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_Four(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn Five(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_Five(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn any(&self) -> bool {
        self.inner & (1 << 15) != 0
    }
    #[inline]
    pub fn set_any(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 15;
        } else {
            self.inner &= !(1 << 15);
        }
        self
    }
    #[inline]
    pub fn new(One: bool, Two: bool, Three: bool, Four: bool, Five: bool, any: bool) -> Self {
        let mut inner: i32 = 0;
        if One {
            inner |= 1 << 8;
        } else {
            inner &= !(1 << 8);
        }
        if Two {
            inner |= 1 << 9;
        } else {
            inner &= !(1 << 9);
        }
        if Three {
            inner |= 1 << 10;
        } else {
            inner &= !(1 << 10);
        }
        if Four {
            inner |= 1 << 11;
        } else {
            inner &= !(1 << 11);
        }
        if Five {
            inner |= 1 << 12;
        } else {
            inner &= !(1 << 12);
        }
        if any {
            inner |= 1 << 15;
        } else {
            inner &= !(1 << 15);
        }
        ButtonMask { inner: inner }
    }
}
impl AsByteSequence for ButtonMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((ButtonMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ArcMode {
    Chord = 0,
    PieSlice = 1,
}
impl AsByteSequence for ArcMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Chord, sz)),
            1 => Some((Self::PieSlice, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for ArcMode {
    #[inline]
    fn default() -> ArcMode {
        ArcMode::Chord
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum JoinStyle {
    Miter = 0,
    Round = 1,
    Bevel = 2,
}
impl AsByteSequence for JoinStyle {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Miter, sz)),
            1 => Some((Self::Round, sz)),
            2 => Some((Self::Bevel, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for JoinStyle {
    #[inline]
    fn default() -> JoinStyle {
        JoinStyle::Miter
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BackPixmap {
    None = 0,
    ParentRelative = 1,
}
impl AsByteSequence for BackPixmap {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::None, sz)),
            1 => Some((Self::ParentRelative, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for BackPixmap {
    #[inline]
    fn default() -> BackPixmap {
        BackPixmap::None
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Gx {
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    Noop = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equiv = 9,
    Invert = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}
impl AsByteSequence for Gx {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Clear, sz)),
            1 => Some((Self::And, sz)),
            2 => Some((Self::AndReverse, sz)),
            3 => Some((Self::Copy, sz)),
            4 => Some((Self::AndInverted, sz)),
            5 => Some((Self::Noop, sz)),
            6 => Some((Self::Xor, sz)),
            7 => Some((Self::Or, sz)),
            8 => Some((Self::Nor, sz)),
            9 => Some((Self::Equiv, sz)),
            10 => Some((Self::Invert, sz)),
            11 => Some((Self::OrReverse, sz)),
            12 => Some((Self::CopyInverted, sz)),
            13 => Some((Self::OrInverted, sz)),
            14 => Some((Self::Nand, sz)),
            15 => Some((Self::Set, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Gx {
    #[inline]
    fn default() -> Gx {
        Gx::Clear
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum CapStyle {
    NotLast = 0,
    Butt = 1,
    Round = 2,
    Projecting = 3,
}
impl AsByteSequence for CapStyle {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NotLast, sz)),
            1 => Some((Self::Butt, sz)),
            2 => Some((Self::Round, sz)),
            3 => Some((Self::Projecting, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for CapStyle {
    #[inline]
    fn default() -> CapStyle {
        CapStyle::NotLast
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LineStyle {
    Solid = 0,
    OnOffDash = 1,
    DoubleDash = 2,
}
impl AsByteSequence for LineStyle {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Solid, sz)),
            1 => Some((Self::OnOffDash, sz)),
            2 => Some((Self::DoubleDash, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for LineStyle {
    #[inline]
    fn default() -> LineStyle {
        LineStyle::Solid
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum FillRule {
    EvenOdd = 0,
    Winding = 1,
}
impl AsByteSequence for FillRule {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::EvenOdd, sz)),
            1 => Some((Self::Winding, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for FillRule {
    #[inline]
    fn default() -> FillRule {
        FillRule::EvenOdd
    }
}
#[derive(Clone, Debug, Default)]
pub struct CursorError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl CursorError {}
impl AsByteSequence for CursorError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CursorError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            CursorError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for CursorError {
    const OPCODE: u8 = 6;
}
#[derive(Clone, Debug, Default)]
pub struct AccessError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl AccessError {}
impl AsByteSequence for AccessError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AccessError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            AccessError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for AccessError {
    const OPCODE: u8 = 10;
}
#[derive(Clone, Debug, Default)]
pub struct ColormapError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl ColormapError {}
impl AsByteSequence for ColormapError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ColormapError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            ColormapError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for ColormapError {
    const OPCODE: u8 = 12;
}
#[derive(Clone, Debug, Default)]
pub struct FontError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl FontError {}
impl AsByteSequence for FontError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FontError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            FontError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for FontError {
    const OPCODE: u8 = 7;
}
#[derive(Clone, Debug, Default)]
pub struct IdChoiceError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl IdChoiceError {}
impl AsByteSequence for IdChoiceError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IdChoiceError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            IdChoiceError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for IdChoiceError {
    const OPCODE: u8 = 14;
}
#[derive(Clone, Debug, Default)]
pub struct GContextError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl GContextError {}
impl AsByteSequence for GContextError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GContextError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            GContextError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for GContextError {
    const OPCODE: u8 = 13;
}
#[derive(Clone, Debug, Default)]
pub struct ImplementationError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl ImplementationError {}
impl AsByteSequence for ImplementationError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ImplementationError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            ImplementationError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for ImplementationError {
    const OPCODE: u8 = 17;
}
#[derive(Clone, Debug, Default)]
pub struct ValueError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl ValueError {}
impl AsByteSequence for ValueError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ValueError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            ValueError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for ValueError {
    const OPCODE: u8 = 2;
}
#[derive(Clone, Debug, Default)]
pub struct MatchError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl MatchError {}
impl AsByteSequence for MatchError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MatchError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            MatchError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for MatchError {
    const OPCODE: u8 = 8;
}
#[derive(Clone, Debug, Default)]
pub struct WindowError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl WindowError {}
impl AsByteSequence for WindowError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing WindowError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            WindowError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for WindowError {
    const OPCODE: u8 = 3;
}
#[derive(Clone, Debug, Default)]
pub struct NameError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl NameError {}
impl AsByteSequence for NameError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NameError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            NameError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for NameError {
    const OPCODE: u8 = 15;
}
#[derive(Clone, Debug, Default)]
pub struct RequestError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl RequestError {}
impl AsByteSequence for RequestError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RequestError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            RequestError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for RequestError {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default)]
pub struct AllocError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl AllocError {}
impl AsByteSequence for AllocError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllocError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            AllocError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for AllocError {
    const OPCODE: u8 = 11;
}
#[derive(Clone, Debug, Default)]
pub struct PixmapError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl PixmapError {}
impl AsByteSequence for PixmapError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PixmapError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            PixmapError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for PixmapError {
    const OPCODE: u8 = 4;
}
#[derive(Clone, Debug, Default)]
pub struct DrawableError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl DrawableError {}
impl AsByteSequence for DrawableError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DrawableError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            DrawableError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for DrawableError {
    const OPCODE: u8 = 9;
}
#[derive(Clone, Debug, Default)]
pub struct LengthError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl LengthError {}
impl AsByteSequence for LengthError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LengthError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            LengthError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for LengthError {
    const OPCODE: u8 = 16;
}
#[derive(Clone, Debug, Default)]
pub struct AtomError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub bad_value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl AtomError {}
impl AsByteSequence for AtomError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.bad_value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AtomError from byte buffer");
        let (_error_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (error_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_code, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bad_value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            AtomError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                bad_value: bad_value,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self._error_type.size()
            + self.error_code.size()
            + self.major_code.size()
            + self.minor_code.size()
            + self.sequence.size()
            + self.bad_value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 1
    }
}
impl crate::auto::Error for AtomError {
    const OPCODE: u8 = 5;
}
#[derive(Clone, Debug, Default)]
pub struct KeyPressEvent {
    pub event_type: u8,
    pub detail: Keycode,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub event_x: Int16,
    pub event_y: Int16,
    pub state: KeyButMask,
    pub same_screen: bool,
}
impl KeyPressEvent {}
impl AsByteSequence for KeyPressEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            KeyPressEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                state: state,
                same_screen: same_screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.state.size()
            + self.same_screen.size()
            + 1
    }
}
impl crate::auto::Event for KeyPressEvent {
    const OPCODE: u8 = 2;
}
#[derive(Clone, Debug, Default)]
pub struct ConfigureRequestEvent {
    pub event_type: u8,
    pub stack_mode: StackMode,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
    pub sibling: Window,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub border_width: Card16,
    pub value_mask: ConfigWindow,
}
impl ConfigureRequestEvent {}
impl AsByteSequence for ConfigureRequestEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.stack_mode.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.parent.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.sibling.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.border_width.as_bytes(&mut bytes[index..]);
        index += self.value_mask.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConfigureRequestEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (stack_mode, sz): (StackMode, usize) = <StackMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parent, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sibling, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value_mask, sz): (ConfigWindow, usize) = <ConfigWindow>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ConfigureRequestEvent {
                event_type: event_type,
                stack_mode: stack_mode,
                sequence: sequence,
                parent: parent,
                window: window,
                sibling: sibling,
                x: x,
                y: y,
                width: width,
                height: height,
                border_width: border_width,
                value_mask: value_mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.stack_mode.size()
            + self.sequence.size()
            + self.parent.size()
            + self.window.size()
            + self.sibling.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.border_width.size()
            + self.value_mask.size()
    }
}
impl crate::auto::Event for ConfigureRequestEvent {
    const OPCODE: u8 = 23;
}
#[derive(Clone, Debug, Default)]
pub struct FocusOutEvent {
    pub event_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub event: Window,
    pub mode: NotifyMode,
}
impl FocusOutEvent {}
impl AsByteSequence for FocusOutEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FocusOutEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (NotifyDetail, usize) = <NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (NotifyMode, usize) = <NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            FocusOutEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                event: event,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.event.size()
            + self.mode.size()
            + 3
    }
}
impl crate::auto::Event for FocusOutEvent {
    const OPCODE: u8 = 10;
}
#[derive(Clone, Debug, Default)]
pub struct ConfigureNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub above_sibling: Window,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub border_width: Card16,
    pub override_redirect: bool,
}
impl ConfigureNotifyEvent {}
impl AsByteSequence for ConfigureNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.above_sibling.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.border_width.as_bytes(&mut bytes[index..]);
        index += self.override_redirect.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ConfigureNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (above_sibling, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (override_redirect, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ConfigureNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                above_sibling: above_sibling,
                x: x,
                y: y,
                width: width,
                height: height,
                border_width: border_width,
                override_redirect: override_redirect,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + self.above_sibling.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.border_width.size()
            + self.override_redirect.size()
    }
}
impl crate::auto::Event for ConfigureNotifyEvent {
    const OPCODE: u8 = 22;
}
#[derive(Clone, Debug, Default)]
pub struct MotionNotifyEvent {
    pub event_type: u8,
    pub detail: Motion,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub event_x: Int16,
    pub event_y: Int16,
    pub state: KeyButMask,
    pub same_screen: bool,
}
impl MotionNotifyEvent {}
impl AsByteSequence for MotionNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MotionNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Motion, usize) = <Motion>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            MotionNotifyEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                state: state,
                same_screen: same_screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.state.size()
            + self.same_screen.size()
            + 1
    }
}
impl crate::auto::Event for MotionNotifyEvent {
    const OPCODE: u8 = 6;
}
#[derive(Clone, Debug, Default)]
pub struct MappingNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub request: Mapping,
    pub first_keycode: Keycode,
    pub count: Card8,
}
impl MappingNotifyEvent {}
impl AsByteSequence for MappingNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.request.as_bytes(&mut bytes[index..]);
        index += self.first_keycode.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MappingNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request, sz): (Mapping, usize) = <Mapping>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            MappingNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                request: request,
                first_keycode: first_keycode,
                count: count,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.request.size()
            + self.first_keycode.size()
            + self.count.size()
    }
}
impl crate::auto::Event for MappingNotifyEvent {
    const OPCODE: u8 = 34;
}
#[derive(Clone, Debug, Default)]
pub struct PropertyNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub atom: Atom,
    pub time: Timestamp,
    pub state: Property,
}
impl PropertyNotifyEvent {}
impl AsByteSequence for PropertyNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.atom.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PropertyNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (atom, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Property, usize) = <Property>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            PropertyNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                window: window,
                atom: atom,
                time: time,
                state: state,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.window.size()
            + self.atom.size()
            + self.time.size()
            + self.state.size()
            + 3
    }
}
impl crate::auto::Event for PropertyNotifyEvent {
    const OPCODE: u8 = 28;
}
#[derive(Clone, Debug, Default)]
pub struct FocusInEvent {
    pub event_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub event: Window,
    pub mode: NotifyMode,
}
impl FocusInEvent {}
impl AsByteSequence for FocusInEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FocusInEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (NotifyDetail, usize) = <NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (NotifyMode, usize) = <NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            FocusInEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                event: event,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.event.size()
            + self.mode.size()
            + 3
    }
}
impl crate::auto::Event for FocusInEvent {
    const OPCODE: u8 = 9;
}
#[derive(Clone, Debug, Default)]
pub struct GravityNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub x: Int16,
    pub y: Int16,
}
impl GravityNotifyEvent {}
impl AsByteSequence for GravityNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GravityNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GravityNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                x: x,
                y: y,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + self.x.size()
            + self.y.size()
    }
}
impl crate::auto::Event for GravityNotifyEvent {
    const OPCODE: u8 = 24;
}
#[derive(Clone, Debug, Default)]
pub struct ClientMessageEvent {
    pub event_type: u8,
    pub format: Card8,
    pub sequence: u16,
    pub window: Window,
    pub ty: Atom,
    pub data: ClientMessageData,
}
impl ClientMessageEvent {}
impl AsByteSequence for ClientMessageEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.data.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ClientMessageEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, sz): (ClientMessageData, usize) =
            <ClientMessageData>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ClientMessageEvent {
                event_type: event_type,
                format: format,
                sequence: sequence,
                window: window,
                ty: ty,
                data: data,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.format.size()
            + self.sequence.size()
            + self.window.size()
            + self.ty.size()
            + self.data.size()
    }
}
impl crate::auto::Event for ClientMessageEvent {
    const OPCODE: u8 = 33;
}
#[derive(Clone, Debug, Default)]
pub struct CirculateNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub place: Place,
}
impl CirculateNotifyEvent {}
impl AsByteSequence for CirculateNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.place.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CirculateNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (place, sz): (Place, usize) = <Place>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CirculateNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                place: place,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + 4
            + self.place.size()
            + 3
    }
}
impl crate::auto::Event for CirculateNotifyEvent {
    const OPCODE: u8 = 26;
}
#[derive(Clone, Debug, Default)]
pub struct SelectionRequestEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub owner: Window,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
}
impl SelectionRequestEvent {}
impl AsByteSequence for SelectionRequestEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.owner.as_bytes(&mut bytes[index..]);
        index += self.requestor.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectionRequestEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (requestor, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectionRequestEvent {
                event_type: event_type,
                sequence: sequence,
                time: time,
                owner: owner,
                requestor: requestor,
                selection: selection,
                target: target,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.time.size()
            + self.owner.size()
            + self.requestor.size()
            + self.selection.size()
            + self.target.size()
            + self.property.size()
    }
}
impl crate::auto::Event for SelectionRequestEvent {
    const OPCODE: u8 = 30;
}
#[derive(Clone, Debug, Default)]
pub struct NoExposureEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub drawable: Drawable,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl NoExposureEvent {}
impl AsByteSequence for NoExposureEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NoExposureEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            NoExposureEvent {
                event_type: event_type,
                sequence: sequence,
                drawable: drawable,
                minor_opcode: minor_opcode,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.drawable.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
    }
}
impl crate::auto::Event for NoExposureEvent {
    const OPCODE: u8 = 14;
}
#[derive(Clone, Debug, Default)]
pub struct VisibilityNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub state: Visibility,
}
impl VisibilityNotifyEvent {}
impl AsByteSequence for VisibilityNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing VisibilityNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Visibility, usize) = <Visibility>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            VisibilityNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                window: window,
                state: state,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.window.size()
            + self.state.size()
            + 3
    }
}
impl crate::auto::Event for VisibilityNotifyEvent {
    const OPCODE: u8 = 15;
}
#[derive(Clone, Debug, Default)]
pub struct ButtonReleaseEvent {
    pub event_type: u8,
    pub detail: Button,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub event_x: Int16,
    pub event_y: Int16,
    pub state: KeyButMask,
    pub same_screen: bool,
}
impl ButtonReleaseEvent {}
impl AsByteSequence for ButtonReleaseEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ButtonReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Button, usize) = <Button>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            ButtonReleaseEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                state: state,
                same_screen: same_screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.state.size()
            + self.same_screen.size()
            + 1
    }
}
impl crate::auto::Event for ButtonReleaseEvent {
    const OPCODE: u8 = 5;
}
#[derive(Clone, Debug, Default)]
pub struct KeyReleaseEvent {
    pub event_type: u8,
    pub detail: Keycode,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub event_x: Int16,
    pub event_y: Int16,
    pub state: KeyButMask,
    pub same_screen: bool,
}
impl KeyReleaseEvent {}
impl AsByteSequence for KeyReleaseEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            KeyReleaseEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                state: state,
                same_screen: same_screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.state.size()
            + self.same_screen.size()
            + 1
    }
}
impl crate::auto::Event for KeyReleaseEvent {
    const OPCODE: u8 = 3;
}
#[derive(Clone, Debug, Default)]
pub struct SelectionNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub requestor: Window,
    pub selection: Atom,
    pub target: Atom,
    pub property: Atom,
}
impl SelectionNotifyEvent {}
impl AsByteSequence for SelectionNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.requestor.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index += self.target.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectionNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (requestor, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (target, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectionNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                time: time,
                requestor: requestor,
                selection: selection,
                target: target,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.time.size()
            + self.requestor.size()
            + self.selection.size()
            + self.target.size()
            + self.property.size()
    }
}
impl crate::auto::Event for SelectionNotifyEvent {
    const OPCODE: u8 = 31;
}
#[derive(Clone, Debug, Default)]
pub struct EnterNotifyEvent {
    pub event_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub event_x: Int16,
    pub event_y: Int16,
    pub state: KeyButMask,
    pub mode: NotifyMode,
    pub same_screen_focus: Byte,
}
impl EnterNotifyEvent {}
impl AsByteSequence for EnterNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.same_screen_focus.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing EnterNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (NotifyDetail, usize) = <NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (NotifyMode, usize) = <NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen_focus, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            EnterNotifyEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                state: state,
                mode: mode,
                same_screen_focus: same_screen_focus,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.state.size()
            + self.mode.size()
            + self.same_screen_focus.size()
    }
}
impl crate::auto::Event for EnterNotifyEvent {
    const OPCODE: u8 = 7;
}
#[derive(Clone, Debug, Default)]
pub struct MapRequestEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
}
impl MapRequestEvent {}
impl AsByteSequence for MapRequestEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.parent.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MapRequestEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parent, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            MapRequestEvent {
                event_type: event_type,
                sequence: sequence,
                parent: parent,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + 1 + self.sequence.size() + self.parent.size() + self.window.size()
    }
}
impl crate::auto::Event for MapRequestEvent {
    const OPCODE: u8 = 20;
}
#[derive(Clone, Debug, Default)]
pub struct GeGenericEvent {
    pub event_type: u8,
    pub sequence: u16,
}
impl GeGenericEvent {}
impl AsByteSequence for GeGenericEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 22;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GeGenericEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GeGenericEvent {
                event_type: event_type,
                sequence: sequence,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + 22 + self.sequence.size()
    }
}
impl crate::auto::Event for GeGenericEvent {
    const OPCODE: u8 = 35;
}
#[derive(Clone, Debug, Default)]
pub struct ExposeEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub x: Card16,
    pub y: Card16,
    pub width: Card16,
    pub height: Card16,
    pub count: Card16,
}
impl ExposeEvent {}
impl AsByteSequence for ExposeEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ExposeEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            ExposeEvent {
                event_type: event_type,
                sequence: sequence,
                window: window,
                x: x,
                y: y,
                width: width,
                height: height,
                count: count,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.window.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.count.size()
            + 2
    }
}
impl crate::auto::Event for ExposeEvent {
    const OPCODE: u8 = 12;
}
#[derive(Clone, Debug, Default)]
pub struct ReparentNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub parent: Window,
    pub x: Int16,
    pub y: Int16,
    pub override_redirect: bool,
}
impl ReparentNotifyEvent {}
impl AsByteSequence for ReparentNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.parent.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.override_redirect.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ReparentNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parent, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (override_redirect, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            ReparentNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                parent: parent,
                x: x,
                y: y,
                override_redirect: override_redirect,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + self.parent.size()
            + self.x.size()
            + self.y.size()
            + self.override_redirect.size()
            + 3
    }
}
impl crate::auto::Event for ReparentNotifyEvent {
    const OPCODE: u8 = 21;
}
#[derive(Clone, Debug, Default)]
pub struct DestroyNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
}
impl DestroyNotifyEvent {}
impl AsByteSequence for DestroyNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DestroyNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DestroyNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + 1 + self.sequence.size() + self.event.size() + self.window.size()
    }
}
impl crate::auto::Event for DestroyNotifyEvent {
    const OPCODE: u8 = 17;
}
#[derive(Clone, Debug, Default)]
pub struct UnmapNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub from_configure: bool,
}
impl UnmapNotifyEvent {}
impl AsByteSequence for UnmapNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.from_configure.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UnmapNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (from_configure, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            UnmapNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                from_configure: from_configure,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + self.from_configure.size()
            + 3
    }
}
impl crate::auto::Event for UnmapNotifyEvent {
    const OPCODE: u8 = 18;
}
#[derive(Clone, Debug, Default)]
pub struct CreateNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub parent: Window,
    pub window: Window,
    pub x: Int16,
    pub y: Int16,
    pub width: Card16,
    pub height: Card16,
    pub border_width: Card16,
    pub override_redirect: bool,
}
impl CreateNotifyEvent {}
impl AsByteSequence for CreateNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.parent.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.border_width.as_bytes(&mut bytes[index..]);
        index += self.override_redirect.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CreateNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (parent, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (border_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (override_redirect, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            CreateNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                parent: parent,
                window: window,
                x: x,
                y: y,
                width: width,
                height: height,
                border_width: border_width,
                override_redirect: override_redirect,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.parent.size()
            + self.window.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.border_width.size()
            + self.override_redirect.size()
    }
}
impl crate::auto::Event for CreateNotifyEvent {
    const OPCODE: u8 = 16;
}
#[derive(Clone, Debug, Default)]
pub struct LeaveNotifyEvent {
    pub event_type: u8,
    pub detail: NotifyDetail,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub event_x: Int16,
    pub event_y: Int16,
    pub state: KeyButMask,
    pub mode: NotifyMode,
    pub same_screen_focus: Byte,
}
impl LeaveNotifyEvent {}
impl AsByteSequence for LeaveNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.same_screen_focus.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LeaveNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (NotifyDetail, usize) = <NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (NotifyMode, usize) = <NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen_focus, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            LeaveNotifyEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                state: state,
                mode: mode,
                same_screen_focus: same_screen_focus,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.state.size()
            + self.mode.size()
            + self.same_screen_focus.size()
    }
}
impl crate::auto::Event for LeaveNotifyEvent {
    const OPCODE: u8 = 8;
}
#[derive(Clone, Debug, Default)]
pub struct SelectionClearEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub owner: Window,
    pub selection: Atom,
}
impl SelectionClearEvent {}
impl AsByteSequence for SelectionClearEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.owner.as_bytes(&mut bytes[index..]);
        index += self.selection.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectionClearEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (selection, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SelectionClearEvent {
                event_type: event_type,
                sequence: sequence,
                time: time,
                owner: owner,
                selection: selection,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.time.size()
            + self.owner.size()
            + self.selection.size()
    }
}
impl crate::auto::Event for SelectionClearEvent {
    const OPCODE: u8 = 29;
}
#[derive(Clone, Debug, Default)]
pub struct ButtonPressEvent {
    pub event_type: u8,
    pub detail: Button,
    pub sequence: u16,
    pub time: Timestamp,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Int16,
    pub root_y: Int16,
    pub event_x: Int16,
    pub event_y: Int16,
    pub state: KeyButMask,
    pub same_screen: bool,
}
impl ButtonPressEvent {}
impl AsByteSequence for ButtonPressEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ButtonPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Button, usize) = <Button>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            ButtonPressEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                state: state,
                same_screen: same_screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.detail.size()
            + self.sequence.size()
            + self.time.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.state.size()
            + self.same_screen.size()
            + 1
    }
}
impl crate::auto::Event for ButtonPressEvent {
    const OPCODE: u8 = 4;
}
#[derive(Clone, Debug, Default)]
pub struct GraphicsExposureEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub drawable: Drawable,
    pub x: Card16,
    pub y: Card16,
    pub width: Card16,
    pub height: Card16,
    pub minor_opcode: Card16,
    pub count: Card16,
    pub major_opcode: Card8,
}
impl GraphicsExposureEvent {}
impl AsByteSequence for GraphicsExposureEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.drawable.as_bytes(&mut bytes[index..]);
        index += self.x.as_bytes(&mut bytes[index..]);
        index += self.y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GraphicsExposureEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (drawable, sz): (Drawable, usize) = <Drawable>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GraphicsExposureEvent {
                event_type: event_type,
                sequence: sequence,
                drawable: drawable,
                x: x,
                y: y,
                width: width,
                height: height,
                minor_opcode: minor_opcode,
                count: count,
                major_opcode: major_opcode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.drawable.size()
            + self.x.size()
            + self.y.size()
            + self.width.size()
            + self.height.size()
            + self.minor_opcode.size()
            + self.count.size()
            + self.major_opcode.size()
            + 3
    }
}
impl crate::auto::Event for GraphicsExposureEvent {
    const OPCODE: u8 = 13;
}
#[derive(Clone, Debug, Default)]
pub struct ColormapNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub colormap: Colormap,
    pub new: bool,
    pub state: ColormapState,
}
impl ColormapNotifyEvent {}
impl AsByteSequence for ColormapNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.colormap.as_bytes(&mut bytes[index..]);
        index += self.new.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ColormapNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (colormap, sz): (Colormap, usize) = <Colormap>::from_bytes(&bytes[index..])?;
        index += sz;
        let (new, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (ColormapState, usize) = <ColormapState>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            ColormapNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                window: window,
                colormap: colormap,
                new: new,
                state: state,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.window.size()
            + self.colormap.size()
            + self.new.size()
            + self.state.size()
            + 2
    }
}
impl crate::auto::Event for ColormapNotifyEvent {
    const OPCODE: u8 = 32;
}
#[derive(Clone, Debug, Default)]
pub struct ResizeRequestEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub window: Window,
    pub width: Card16,
    pub height: Card16,
}
impl ResizeRequestEvent {}
impl AsByteSequence for ResizeRequestEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ResizeRequestEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ResizeRequestEvent {
                event_type: event_type,
                sequence: sequence,
                window: window,
                width: width,
                height: height,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.window.size()
            + self.width.size()
            + self.height.size()
    }
}
impl crate::auto::Event for ResizeRequestEvent {
    const OPCODE: u8 = 25;
}
#[derive(Clone, Debug, Default)]
pub struct KeymapNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub keys: [Card8; 31],
}
impl KeymapNotifyEvent {}
impl AsByteSequence for KeymapNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.keys.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeymapNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keys, sz): ([Card8; 31], usize) = <[Card8; 31]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KeymapNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                keys: keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + 1 + self.sequence.size() + self.keys.size()
    }
}
impl crate::auto::Event for KeymapNotifyEvent {
    const OPCODE: u8 = 11;
}
#[derive(Clone, Debug, Default)]
pub struct MapNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub override_redirect: bool,
}
impl MapNotifyEvent {}
impl AsByteSequence for MapNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.override_redirect.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MapNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (override_redirect, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            MapNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                override_redirect: override_redirect,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + self.override_redirect.size()
            + 3
    }
}
impl crate::auto::Event for MapNotifyEvent {
    const OPCODE: u8 = 19;
}
#[derive(Clone, Debug, Default)]
pub struct CirculateRequestEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub event: Window,
    pub window: Window,
    pub place: Place,
}
impl CirculateRequestEvent {}
impl AsByteSequence for CirculateRequestEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += 4;
        index += self.place.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CirculateRequestEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (place, sz): (Place, usize) = <Place>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CirculateRequestEvent {
                event_type: event_type,
                sequence: sequence,
                event: event,
                window: window,
                place: place,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + 1
            + self.sequence.size()
            + self.event.size()
            + self.window.size()
            + 4
            + self.place.size()
            + 3
    }
}
impl crate::auto::Event for CirculateRequestEvent {
    const OPCODE: u8 = 27;
}
