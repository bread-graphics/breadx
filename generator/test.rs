#[derive(Default, Debug)]
pub struct Char2b {
  pub byte1: Card8,
  pub byte2: Card8,
}
impl AsByteSequence for Char2b {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Card8>() + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.byte1.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.byte2.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Window {
  inner: XID,
}
impl XidType for Window {
  #[inline]
  fn xid(&self) -> XID {
    self.inner
  }
}
impl Window {
  #[inline]
  pub fn from_xid(xid: XID) -> Window {
    Window { inner: xid }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Pixmap {
  inner: XID,
}
impl XidType for Pixmap {
  #[inline]
  fn xid(&self) -> XID {
    self.inner
  }
}
impl Pixmap {
  #[inline]
  pub fn from_xid(xid: XID) -> Pixmap {
    Pixmap { inner: xid }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Cursor {
  inner: XID,
}
impl XidType for Cursor {
  #[inline]
  fn xid(&self) -> XID {
    self.inner
  }
}
impl Cursor {
  #[inline]
  pub fn from_xid(xid: XID) -> Cursor {
    Cursor { inner: xid }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Font {
  inner: XID,
}
impl XidType for Font {
  #[inline]
  fn xid(&self) -> XID {
    self.inner
  }
}
impl Font {
  #[inline]
  pub fn from_xid(xid: XID) -> Font {
    Font { inner: xid }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Gcontext {
  inner: XID,
}
impl XidType for Gcontext {
  #[inline]
  fn xid(&self) -> XID {
    self.inner
  }
}
impl Gcontext {
  #[inline]
  pub fn from_xid(xid: XID) -> Gcontext {
    Gcontext { inner: xid }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Colormap {
  inner: XID,
}
impl XidType for Colormap {
  #[inline]
  fn xid(&self) -> XID {
    self.inner
  }
}
impl Colormap {
  #[inline]
  pub fn from_xid(xid: XID) -> Colormap {
    Colormap { inner: xid }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
#[repr(transparent)]
pub struct Atom {
  inner: XID,
}
impl XidType for Atom {
  #[inline]
  fn xid(&self) -> XID {
    self.inner
  }
}
impl Atom {
  #[inline]
  pub fn from_xid(xid: XID) -> Atom {
    Atom { inner: xid }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
pub enum Drawable {
  Window(Window),
  Pixmap(Pixmap),
}
impl XidType for Drawable {
  #[inline]
  fn xid(&self) -> XID {
    match Self {
      self::Window(i) => i.xid(),
      self::Pixmap(i) => i.xid(),
    }
  }
}
#[derive(Default, Debug, Copy, Clone, Hash)]
pub enum Fontable {
  Font(Font),
  Gcontext(Gcontext),
}
impl XidType for Fontable {
  #[inline]
  fn xid(&self) -> XID {
    match Self {
      self::Font(i) => i.xid(),
      self::Gcontext(i) => i.xid(),
    }
  }
}
pub type Visualid = Card32;
pub type Timestamp = Card32;
pub type Keysym = Card32;
pub type Keycode = Card8;
pub type Button = Card8;
#[derive(Default, Debug)]
pub struct Point {
  pub x: Int16,
  pub y: Int16,
}
impl AsByteSequence for Point {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Int16>() + ::std::mem::size_of::<Int16>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.x.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.y.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
  }
}
#[derive(Default, Debug)]
pub struct Rectangle {
  pub x: Int16,
  pub y: Int16,
  pub width: Card16,
  pub height: Card16,
}
impl AsByteSequence for Rectangle {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.x.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.y.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.width.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.height.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
  }
}
#[derive(Default, Debug)]
pub struct Arc {
  pub x: Int16,
  pub y: Int16,
  pub width: Card16,
  pub height: Card16,
  pub angle1: Int16,
  pub angle2: Int16,
}
impl AsByteSequence for Arc {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.x.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.y.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.width.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.height.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.angle1.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.angle2.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
  }
}
#[derive(Default, Debug)]
pub struct Format {
  pub depth: Card8,
  pub bits_per_pixel: Card8,
  pub scanline_pad: Card8,
}
impl AsByteSequence for Format {
  #[inline]
  fn size() -> usize {
    5 + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.depth.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.bits_per_pixel.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.scanline_pad.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    index += 5;
  }
}
#[derive(Default, Debug)]
pub struct Visualtype {
  pub visual_id: Visualid,
  pub class: Card8,
  pub bits_per_rgb_value: Card8,
  pub colormap_entries: Card16,
  pub red_mask: Card32,
  pub green_mask: Card32,
  pub blue_mask: Card32,
}
impl AsByteSequence for Visualtype {
  #[inline]
  fn size() -> usize {
    4 + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Visualid>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.visual_id.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Visualid>();
    self.class.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.bits_per_rgb_value.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.colormap_entries.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.red_mask.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.green_mask.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.blue_mask.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    index += 4;
  }
}
#[derive(Default, Debug)]
pub struct Depth {
  pub depth: Card8,
  pub visuals: Vec<Visualtype>,
}
impl AsByteSequence for Depth {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Visualtype>() + 4 + 2 + 1 + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.depth.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    index += 1;
    self.visuals.len().as_bytes(&mut bytes[index..]);
    index += 2;
    index += 4;
    self.visuals.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Visualtype>();
  }
}
#[derive(Default, Debug)]
pub struct Screen {
  pub root: Window,
  pub default_colormap: Colormap,
  pub white_pixel: Card32,
  pub black_pixel: Card32,
  pub current_input_masks: Card32,
  pub width_in_pixels: Card16,
  pub height_in_pixels: Card16,
  pub width_in_millimeters: Card16,
  pub height_in_millimeters: Card16,
  pub min_installed_maps: Card16,
  pub max_installed_maps: Card16,
  pub root_visual: Visualid,
  pub backing_stores: Byte,
  pub save_unders: Bool,
  pub root_depth: Card8,
  pub allowed_depths: Vec<Depth>,
}
impl AsByteSequence for Screen {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Depth>()
      + 2
      + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Bool>()
      + ::std::mem::size_of::<Byte>()
      + ::std::mem::size_of::<Visualid>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Colormap>()
      + ::std::mem::size_of::<Window>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.root.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Window>();
    self.default_colormap.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Colormap>();
    self.white_pixel.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.black_pixel.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.current_input_masks.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.width_in_pixels.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.height_in_pixels.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.width_in_millimeters.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.height_in_millimeters.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.min_installed_maps.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.max_installed_maps.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.root_visual.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Visualid>();
    self.backing_stores.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Byte>();
    self.save_unders.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Bool>();
    self.root_depth.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.allowed_depths.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.allowed_depths.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Depth>();
  }
}
#[derive(Default, Debug)]
pub struct SetupRequest {
  pub byte_order: Card8,
  pub protocol_major_version: Card16,
  pub protocol_minor_version: Card16,
  pub authorization_protocol_name: String,
  pub authorization_protocol_data: String,
}
impl AsByteSequence for SetupRequest {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Char>()
      + ::std::mem::size_of::<*mut Char>()
      + 2
      + 2
      + 2
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + 1
      + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.byte_order.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    index += 1;
    self.protocol_major_version.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.protocol_minor_version.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self
      .authorization_protocol_name
      .len()
      .as_bytes(&mut bytes[index..]);
    index += 2;
    self
      .authorization_protocol_data
      .len()
      .as_bytes(&mut bytes[index..]);
    index += 2;
    index += 2;
    self
      .authorization_protocol_name
      .as_ptr()
      .as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Char>();
    self
      .authorization_protocol_data
      .as_ptr()
      .as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Char>();
  }
}
#[derive(Default, Debug)]
pub struct SetupFailed {
  pub status: Card8,
  pub protocol_major_version: Card16,
  pub protocol_minor_version: Card16,
  pub length: Card16,
  pub reason: String,
}
impl AsByteSequence for SetupFailed {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Char>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + 2
      + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.status.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.reason.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.protocol_major_version.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.protocol_minor_version.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.length.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.reason.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Char>();
  }
}
#[derive(Default, Debug)]
pub struct SetupAuthenticate {
  pub status: Card8,
  pub reason: String,
}
impl AsByteSequence for SetupAuthenticate {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Char>() + 2 + 5 + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.status.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    index += 5;
    self.reason.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.reason.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Char>();
  }
}
#[derive(Default, Debug)]
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
  pub image_byte_order: Card8,
  pub bitmap_format_bit_order: Card8,
  pub bitmap_format_scanline_unit: Card8,
  pub bitmap_format_scanline_pad: Card8,
  pub min_keycode: Keycode,
  pub max_keycode: Keycode,
  pub vendor: String,
  pub pixmap_formats: Vec<Format>,
  pub roots: Vec<Screen>,
}
impl AsByteSequence for Setup {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Screen>()
      + ::std::mem::size_of::<*mut Format>()
      + ::std::mem::size_of::<*mut Char>()
      + 4
      + ::std::mem::size_of::<Keycode>()
      + ::std::mem::size_of::<Keycode>()
      + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Card8>()
      + ::std::mem::size_of::<Card8>()
      + 2
      + 2
      + ::std::mem::size_of::<Card16>()
      + 2
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card32>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + 1
      + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.status.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    index += 1;
    self.protocol_major_version.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.protocol_minor_version.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.length.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.release_number.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.resource_id_base.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.resource_id_mask.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.motion_buffer_size.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.vendor.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.maximum_request_length.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.roots.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.pixmap_formats.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.image_byte_order.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.bitmap_format_bit_order.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self
      .bitmap_format_scanline_unit
      .as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self
      .bitmap_format_scanline_pad
      .as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    self.min_keycode.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Keycode>();
    self.max_keycode.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Keycode>();
    index += 4;
    self.vendor.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Char>();
    self.pixmap_formats.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Format>();
    self.roots.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Screen>();
  }
}
#[derive(Default, Debug)]
pub struct Timecoord {
  pub time: Timestamp,
  pub x: Int16,
  pub y: Int16,
}
impl AsByteSequence for Timecoord {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Timestamp>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.time.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Timestamp>();
    self.x.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.y.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
  }
}
#[derive(Default, Debug)]
pub struct Fontprop {
  pub name: Atom,
  pub value: Card32,
}
impl AsByteSequence for Fontprop {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Card32>() + ::std::mem::size_of::<Atom>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.name.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Atom>();
    self.value.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
  }
}
#[derive(Default, Debug)]
pub struct Charinfo {
  pub left_side_bearing: Int16,
  pub right_side_bearing: Int16,
  pub character_width: Int16,
  pub ascent: Int16,
  pub descent: Int16,
  pub attributes: Card16,
}
impl AsByteSequence for Charinfo {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.left_side_bearing.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.right_side_bearing.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.character_width.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.ascent.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.descent.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.attributes.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
  }
}
impl AsByteSequence for Str {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Char>() + 2
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.name.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.name.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Char>();
  }
}
#[derive(Default, Debug)]
pub struct Segment {
  pub x1: Int16,
  pub y1: Int16,
  pub x2: Int16,
  pub y2: Int16,
}
impl AsByteSequence for Segment {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
      + ::std::mem::size_of::<Int16>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.x1.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.y1.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.x2.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
    self.y2.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Int16>();
  }
}
#[derive(Default, Debug)]
pub struct Coloritem {
  pub pixel: Card32,
  pub red: Card16,
  pub green: Card16,
  pub blue: Card16,
  pub flags: Byte,
}
impl AsByteSequence for Coloritem {
  #[inline]
  fn size() -> usize {
    1 + ::std::mem::size_of::<Byte>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card32>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.pixel.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card32>();
    self.red.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.green.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.blue.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.flags.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Byte>();
    index += 1;
  }
}
#[derive(Default, Debug)]
pub struct Rgb {
  pub red: Card16,
  pub green: Card16,
  pub blue: Card16,
}
impl AsByteSequence for Rgb {
  #[inline]
  fn size() -> usize {
    2 + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
      + ::std::mem::size_of::<Card16>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.red.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.green.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    self.blue.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card16>();
    index += 2;
  }
}
#[derive(Default, Debug)]
pub struct Host {
  pub family: Card8,
  pub address: Vec<Byte>,
}
impl AsByteSequence for Host {
  #[inline]
  fn size() -> usize {
    ::std::mem::size_of::<*mut Byte>() + 2 + 1 + ::std::mem::size_of::<Card8>()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut index = 0;
    self.family.as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<Card8>();
    index += 1;
    self.address.len().as_bytes(&mut bytes[index..]);
    index += 2;
    self.address.as_ptr().as_bytes(&mut bytes[index..]);
    index += ::std::mem::size_of::<*mut Byte>();
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
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
  fn size(&self) -> usize {
    <u8>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u8).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum BackingStore {
  NotUseful = 0,
  WhenMapped = 1,
  Always = 2,
}
impl AsByteSequence for BackingStore {
  #[inline]
  fn size(&self) -> usize {
    <u8>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u8).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum ImageOrder {
  LSBFirst = 0,
  MSBFirst = 1,
}
impl AsByteSequence for ImageOrder {
  #[inline]
  fn size(&self) -> usize {
    <u8>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u8).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Family {
  Internet = 0,
  DECnet = 1,
  Chaos = 2,
  ServerInterpreted = 5,
  Internet6 = 6,
}
impl AsByteSequence for Family {
  #[inline]
  fn size(&self) -> usize {
    <u8>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u8).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct EventMask {
  pub key_press: bool,
  pub key_release: bool,
  pub button_press: bool,
  pub button_release: bool,
  pub enter_window: bool,
  pub leave_window: bool,
  pub pointer_motion: bool,
  pub pointer_motion_hint: bool,
  pub button1_motion: bool,
  pub button2_motion: bool,
  pub button3_motion: bool,
  pub button4_motion: bool,
  pub button5_motion: bool,
  pub button_motion: bool,
  pub keymap_state: bool,
  pub exposure: bool,
  pub visibility_change: bool,
  pub structure_notify: bool,
  pub resize_redirect: bool,
  pub substructure_notify: bool,
  pub substructure_redirect: bool,
  pub focus_change: bool,
  pub property_change: bool,
  pub color_map_change: bool,
  pub owner_grab_button: bool,
}
impl AsByteSequence for EventMask {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.key_press { 1 << 0 } else { 0 };
    asb |= if self.key_release { 1 << 1 } else { 0 };
    asb |= if self.button_press { 1 << 2 } else { 0 };
    asb |= if self.button_release { 1 << 3 } else { 0 };
    asb |= if self.enter_window { 1 << 4 } else { 0 };
    asb |= if self.leave_window { 1 << 5 } else { 0 };
    asb |= if self.pointer_motion { 1 << 6 } else { 0 };
    asb |= if self.pointer_motion_hint { 1 << 7 } else { 0 };
    asb |= if self.button1_motion { 1 << 8 } else { 0 };
    asb |= if self.button2_motion { 1 << 9 } else { 0 };
    asb |= if self.button3_motion { 1 << 10 } else { 0 };
    asb |= if self.button4_motion { 1 << 11 } else { 0 };
    asb |= if self.button5_motion { 1 << 12 } else { 0 };
    asb |= if self.button_motion { 1 << 13 } else { 0 };
    asb |= if self.keymap_state { 1 << 14 } else { 0 };
    asb |= if self.exposure { 1 << 15 } else { 0 };
    asb |= if self.visibility_change { 1 << 16 } else { 0 };
    asb |= if self.structure_notify { 1 << 17 } else { 0 };
    asb |= if self.resize_redirect { 1 << 18 } else { 0 };
    asb |= if self.substructure_notify { 1 << 19 } else { 0 };
    asb |= if self.substructure_redirect {
      1 << 20
    } else {
      0
    };
    asb |= if self.focus_change { 1 << 21 } else { 0 };
    asb |= if self.property_change { 1 << 22 } else { 0 };
    asb |= if self.color_map_change { 1 << 23 } else { 0 };
    asb |= if self.owner_grab_button { 1 << 24 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct ModMask {
  pub shift: bool,
  pub lock: bool,
  pub control: bool,
  pub one: bool,
  pub two: bool,
  pub three: bool,
  pub four: bool,
  pub five: bool,
  pub any: bool,
}
impl AsByteSequence for ModMask {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.shift { 1 << 0 } else { 0 };
    asb |= if self.lock { 1 << 1 } else { 0 };
    asb |= if self.control { 1 << 2 } else { 0 };
    asb |= if self.one { 1 << 3 } else { 0 };
    asb |= if self.two { 1 << 4 } else { 0 };
    asb |= if self.three { 1 << 5 } else { 0 };
    asb |= if self.four { 1 << 6 } else { 0 };
    asb |= if self.five { 1 << 7 } else { 0 };
    asb |= if self.any { 1 << 15 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct KeyButMask {
  pub shift: bool,
  pub lock: bool,
  pub control: bool,
  pub mod1: bool,
  pub mod2: bool,
  pub mod3: bool,
  pub mod4: bool,
  pub mod5: bool,
  pub button1: bool,
  pub button2: bool,
  pub button3: bool,
  pub button4: bool,
  pub button5: bool,
}
impl AsByteSequence for KeyButMask {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.shift { 1 << 0 } else { 0 };
    asb |= if self.lock { 1 << 1 } else { 0 };
    asb |= if self.control { 1 << 2 } else { 0 };
    asb |= if self.mod1 { 1 << 3 } else { 0 };
    asb |= if self.mod2 { 1 << 4 } else { 0 };
    asb |= if self.mod3 { 1 << 5 } else { 0 };
    asb |= if self.mod4 { 1 << 6 } else { 0 };
    asb |= if self.mod5 { 1 << 7 } else { 0 };
    asb |= if self.button1 { 1 << 8 } else { 0 };
    asb |= if self.button2 { 1 << 9 } else { 0 };
    asb |= if self.button3 { 1 << 10 } else { 0 };
    asb |= if self.button4 { 1 << 11 } else { 0 };
    asb |= if self.button5 { 1 << 12 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Window {
  None = 0,
}
impl AsByteSequence for Window {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct ButtonMask {
  pub one: bool,
  pub two: bool,
  pub three: bool,
  pub four: bool,
  pub five: bool,
  pub any: bool,
}
impl AsByteSequence for ButtonMask {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.one { 1 << 8 } else { 0 };
    asb |= if self.two { 1 << 9 } else { 0 };
    asb |= if self.three { 1 << 10 } else { 0 };
    asb |= if self.four { 1 << 11 } else { 0 };
    asb |= if self.five { 1 << 12 } else { 0 };
    asb |= if self.any { 1 << 15 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Motion {
  Normal = 0,
  Hint = 1,
}
impl AsByteSequence for Motion {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
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
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum NotifyMode {
  Normal = 0,
  Grab = 1,
  Ungrab = 2,
  WhileGrabbed = 3,
}
impl AsByteSequence for NotifyMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Visibility {
  Unobscured = 0,
  PartiallyObscured = 1,
  FullyObscured = 2,
}
impl AsByteSequence for Visibility {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Place {
  OnTop = 0,
  OnBottom = 1,
}
impl AsByteSequence for Place {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Property {
  NewValue = 0,
  Delete = 1,
}
impl AsByteSequence for Property {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Time {
  CurrentTime = 0,
}
impl AsByteSequence for Time {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Atom {
  None = 0,
  Any = 0,
  PRIMARY = 1,
  SECONDARY = 2,
  ARC = 3,
  ATOM = 4,
  BITMAP = 5,
  CARDINAL = 6,
  COLORMAP = 7,
  CURSOR = 8,
  CUT_BUFFER0 = 9,
  CUT_BUFFER1 = 10,
  CUT_BUFFER2 = 11,
  CUT_BUFFER3 = 12,
  CUT_BUFFER4 = 13,
  CUT_BUFFER5 = 14,
  CUT_BUFFER6 = 15,
  CUT_BUFFER7 = 16,
  DRAWABLE = 17,
  FONT = 18,
  INTEGER = 19,
  PIXMAP = 20,
  POINT = 21,
  RECTANGLE = 22,
  RESOURCE_MANAGER = 23,
  RGB_COLOR_MAP = 24,
  RGB_BEST_MAP = 25,
  RGB_BLUE_MAP = 26,
  RGB_DEFAULT_MAP = 27,
  RGB_GRAY_MAP = 28,
  RGB_GREEN_MAP = 29,
  RGB_RED_MAP = 30,
  STRING = 31,
  VISUALID = 32,
  WINDOW = 33,
  WM_COMMAND = 34,
  WM_HINTS = 35,
  WM_CLIENT_MACHINE = 36,
  WM_ICON_NAME = 37,
  WM_ICON_SIZE = 38,
  WM_NAME = 39,
  WM_NORMAL_HINTS = 40,
  WM_SIZE_HINTS = 41,
  WM_ZOOM_HINTS = 42,
  MIN_SPACE = 43,
  NORM_SPACE = 44,
  MAX_SPACE = 45,
  END_SPACE = 46,
  SUPERSCRIPT_X = 47,
  SUPERSCRIPT_Y = 48,
  SUBSCRIPT_X = 49,
  SUBSCRIPT_Y = 50,
  UNDERLINE_POSITION = 51,
  UNDERLINE_THICKNESS = 52,
  STRIKEOUT_ASCENT = 53,
  STRIKEOUT_DESCENT = 54,
  ITALIC_ANGLE = 55,
  X_HEIGHT = 56,
  QUAD_WIDTH = 57,
  WEIGHT = 58,
  POINT_SIZE = 59,
  RESOLUTION = 60,
  COPYRIGHT = 61,
  NOTICE = 62,
  FONT_NAME = 63,
  FAMILY_NAME = 64,
  FULL_NAME = 65,
  CAP_HEIGHT = 66,
  WM_CLASS = 67,
  WM_TRANSIENT_FOR = 68,
}
impl AsByteSequence for Atom {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ColormapState {
  Uninstalled = 0,
  Installed = 1,
}
impl AsByteSequence for ColormapState {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Colormap {
  None = 0,
}
impl AsByteSequence for Colormap {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Mapping {
  Modifier = 0,
  Keyboard = 1,
  Pointer = 2,
}
impl AsByteSequence for Mapping {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum WindowClass {
  CopyFromParent = 0,
  InputOutput = 1,
  InputOnly = 2,
}
impl AsByteSequence for WindowClass {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct CW {
  pub back_pixmap: bool,
  pub back_pixel: bool,
  pub border_pixmap: bool,
  pub border_pixel: bool,
  pub bit_gravity: bool,
  pub win_gravity: bool,
  pub backing_store: bool,
  pub backing_planes: bool,
  pub backing_pixel: bool,
  pub override_redirect: bool,
  pub save_under: bool,
  pub event_mask: bool,
  pub dont_propagate: bool,
  pub colormap: bool,
  pub cursor: bool,
}
impl AsByteSequence for CW {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.back_pixmap { 1 << 0 } else { 0 };
    asb |= if self.back_pixel { 1 << 1 } else { 0 };
    asb |= if self.border_pixmap { 1 << 2 } else { 0 };
    asb |= if self.border_pixel { 1 << 3 } else { 0 };
    asb |= if self.bit_gravity { 1 << 4 } else { 0 };
    asb |= if self.win_gravity { 1 << 5 } else { 0 };
    asb |= if self.backing_store { 1 << 6 } else { 0 };
    asb |= if self.backing_planes { 1 << 7 } else { 0 };
    asb |= if self.backing_pixel { 1 << 8 } else { 0 };
    asb |= if self.override_redirect { 1 << 9 } else { 0 };
    asb |= if self.save_under { 1 << 10 } else { 0 };
    asb |= if self.event_mask { 1 << 11 } else { 0 };
    asb |= if self.dont_propagate { 1 << 12 } else { 0 };
    asb |= if self.colormap { 1 << 13 } else { 0 };
    asb |= if self.cursor { 1 << 14 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum BackPixmap {
  None = 0,
  ParentRelative = 1,
}
impl AsByteSequence for BackPixmap {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Gravity {
  BitForget = 0,
  WinUnmap = 0,
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
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum MapState {
  Unmapped = 0,
  Unviewable = 1,
  Viewable = 2,
}
impl AsByteSequence for MapState {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum SetMode {
  Insert = 0,
  Delete = 1,
}
impl AsByteSequence for SetMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct ConfigWindow {
  pub x: bool,
  pub y: bool,
  pub width: bool,
  pub height: bool,
  pub border_width: bool,
  pub sibling: bool,
  pub stack_mode: bool,
}
impl AsByteSequence for ConfigWindow {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.x { 1 << 0 } else { 0 };
    asb |= if self.y { 1 << 1 } else { 0 };
    asb |= if self.width { 1 << 2 } else { 0 };
    asb |= if self.height { 1 << 3 } else { 0 };
    asb |= if self.border_width { 1 << 4 } else { 0 };
    asb |= if self.sibling { 1 << 5 } else { 0 };
    asb |= if self.stack_mode { 1 << 6 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum StackMode {
  Above = 0,
  Below = 1,
  TopIf = 2,
  BottomIf = 3,
  Opposite = 4,
}
impl AsByteSequence for StackMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Circulate {
  RaiseLowest = 0,
  LowerHighest = 1,
}
impl AsByteSequence for Circulate {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum PropMode {
  Replace = 0,
  Prepend = 1,
  Append = 2,
}
impl AsByteSequence for PropMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GetPropertyType {
  Any = 0,
}
impl AsByteSequence for GetPropertyType {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum SendEventDest {
  PointerWindow = 0,
  ItemFocus = 1,
}
impl AsByteSequence for SendEventDest {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GrabMode {
  Sync = 0,
  Async = 1,
}
impl AsByteSequence for GrabMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GrabStatus {
  Success = 0,
  AlreadyGrabbed = 1,
  InvalidTime = 2,
  NotViewable = 3,
  Frozen = 4,
}
impl AsByteSequence for GrabStatus {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Cursor {
  None = 0,
}
impl AsByteSequence for Cursor {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
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
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Grab {
  Any = 0,
}
impl AsByteSequence for Grab {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
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
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum InputFocus {
  None = 0,
  PointerRoot = 1,
  Parent = 2,
  FollowKeyboard = 3,
}
impl AsByteSequence for InputFocus {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum FontDraw {
  LeftToRight = 0,
  RightToLeft = 1,
}
impl AsByteSequence for FontDraw {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct GC {
  pub function: bool,
  pub plane_mask: bool,
  pub foreground: bool,
  pub background: bool,
  pub line_width: bool,
  pub line_style: bool,
  pub cap_style: bool,
  pub join_style: bool,
  pub fill_style: bool,
  pub fill_rule: bool,
  pub tile: bool,
  pub stipple: bool,
  pub tile_stipple_origin_x: bool,
  pub tile_stipple_origin_y: bool,
  pub font: bool,
  pub subwindow_mode: bool,
  pub graphics_exposures: bool,
  pub clip_origin_x: bool,
  pub clip_origin_y: bool,
  pub clip_mask: bool,
  pub dash_offset: bool,
  pub dash_list: bool,
  pub arc_mode: bool,
}
impl AsByteSequence for GC {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.function { 1 << 0 } else { 0 };
    asb |= if self.plane_mask { 1 << 1 } else { 0 };
    asb |= if self.foreground { 1 << 2 } else { 0 };
    asb |= if self.background { 1 << 3 } else { 0 };
    asb |= if self.line_width { 1 << 4 } else { 0 };
    asb |= if self.line_style { 1 << 5 } else { 0 };
    asb |= if self.cap_style { 1 << 6 } else { 0 };
    asb |= if self.join_style { 1 << 7 } else { 0 };
    asb |= if self.fill_style { 1 << 8 } else { 0 };
    asb |= if self.fill_rule { 1 << 9 } else { 0 };
    asb |= if self.tile { 1 << 10 } else { 0 };
    asb |= if self.stipple { 1 << 11 } else { 0 };
    asb |= if self.tile_stipple_origin_x {
      1 << 12
    } else {
      0
    };
    asb |= if self.tile_stipple_origin_y {
      1 << 13
    } else {
      0
    };
    asb |= if self.font { 1 << 14 } else { 0 };
    asb |= if self.subwindow_mode { 1 << 15 } else { 0 };
    asb |= if self.graphics_exposures { 1 << 16 } else { 0 };
    asb |= if self.clip_origin_x { 1 << 17 } else { 0 };
    asb |= if self.clip_origin_y { 1 << 18 } else { 0 };
    asb |= if self.clip_mask { 1 << 19 } else { 0 };
    asb |= if self.dash_offset { 1 << 20 } else { 0 };
    asb |= if self.dash_list { 1 << 21 } else { 0 };
    asb |= if self.arc_mode { 1 << 22 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum GX {
  clear = 0,
  and = 1,
  andReverse = 2,
  copy = 3,
  andInverted = 4,
  noop = 5,
  xor = 6,
  or = 7,
  nor = 8,
  equiv = 9,
  invert = 10,
  orReverse = 11,
  copyInverted = 12,
  orInverted = 13,
  nand = 14,
  set = 15,
}
impl AsByteSequence for GX {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LineStyle {
  Solid = 0,
  OnOffDash = 1,
  DoubleDash = 2,
}
impl AsByteSequence for LineStyle {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CapStyle {
  NotLast = 0,
  Butt = 1,
  Round = 2,
  Projecting = 3,
}
impl AsByteSequence for CapStyle {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum JoinStyle {
  Miter = 0,
  Round = 1,
  Bevel = 2,
}
impl AsByteSequence for JoinStyle {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum FillStyle {
  Solid = 0,
  Tiled = 1,
  Stippled = 2,
  OpaqueStippled = 3,
}
impl AsByteSequence for FillStyle {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum FillRule {
  EvenOdd = 0,
  Winding = 1,
}
impl AsByteSequence for FillRule {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum SubwindowMode {
  ClipByChildren = 0,
  IncludeInferiors = 1,
}
impl AsByteSequence for SubwindowMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ArcMode {
  Chord = 0,
  PieSlice = 1,
}
impl AsByteSequence for ArcMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ClipOrdering {
  Unsorted = 0,
  YSorted = 1,
  YXSorted = 2,
  YXBanded = 3,
}
impl AsByteSequence for ClipOrdering {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CoordMode {
  Origin = 0,
  Previous = 1,
}
impl AsByteSequence for CoordMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum PolyShape {
  Complex = 0,
  Nonconvex = 1,
  Convex = 2,
}
impl AsByteSequence for PolyShape {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ImageFormat {
  XYBitmap = 0,
  XYPixmap = 1,
  ZPixmap = 2,
}
impl AsByteSequence for ImageFormat {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ColormapAlloc {
  None = 0,
  All = 1,
}
impl AsByteSequence for ColormapAlloc {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct ColorFlag {
  pub red: bool,
  pub green: bool,
  pub blue: bool,
}
impl AsByteSequence for ColorFlag {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.red { 1 << 0 } else { 0 };
    asb |= if self.green { 1 << 1 } else { 0 };
    asb |= if self.blue { 1 << 2 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Pixmap {
  None = 0,
}
impl AsByteSequence for Pixmap {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Font {
  None = 0,
}
impl AsByteSequence for Font {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum QueryShapeOf {
  LargestCursor = 0,
  FastestTile = 1,
  FastestStipple = 2,
}
impl AsByteSequence for QueryShapeOf {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, Default)]
pub struct KB {
  pub key_click_percent: bool,
  pub bell_percent: bool,
  pub bell_pitch: bool,
  pub bell_duration: bool,
  pub led: bool,
  pub led_mode: bool,
  pub key: bool,
  pub auto_repeat_mode: bool,
}
impl AsByteSequence for KB {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  fn as_bytes(&self, bytes: &mut [u8]) {
    let mut asb: u32 = 0;
    asb |= if self.key_click_percent { 1 << 0 } else { 0 };
    asb |= if self.bell_percent { 1 << 1 } else { 0 };
    asb |= if self.bell_pitch { 1 << 2 } else { 0 };
    asb |= if self.bell_duration { 1 << 3 } else { 0 };
    asb |= if self.led { 1 << 4 } else { 0 };
    asb |= if self.led_mode { 1 << 5 } else { 0 };
    asb |= if self.key { 1 << 6 } else { 0 };
    asb |= if self.auto_repeat_mode { 1 << 7 } else { 0 };
    asb.as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum LedMode {
  Off = 0,
  On = 1,
}
impl AsByteSequence for LedMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum AutoRepeatMode {
  Off = 0,
  On = 1,
  Default = 2,
}
impl AsByteSequence for AutoRepeatMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Blanking {
  NotPreferred = 0,
  Preferred = 1,
  Default = 2,
}
impl AsByteSequence for Blanking {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Exposures {
  NotAllowed = 0,
  Allowed = 1,
  Default = 2,
}
impl AsByteSequence for Exposures {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum HostMode {
  Insert = 0,
  Delete = 1,
}
impl AsByteSequence for HostMode {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum AccessControl {
  Disable = 0,
  Enable = 1,
}
impl AsByteSequence for AccessControl {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum CloseDown {
  DestroyAll = 0,
  RetainPermanent = 1,
  RetainTemporary = 2,
}
impl AsByteSequence for CloseDown {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum Kill {
  AllTemporary = 0,
}
impl AsByteSequence for Kill {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum ScreenSaver {
  Reset = 0,
  Active = 1,
}
impl AsByteSequence for ScreenSaver {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
pub enum MappingStatus {
  Success = 0,
  Busy = 1,
  Failure = 2,
}
impl AsByteSequence for MappingStatus {
  #[inline]
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u32)]
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
  fn size(&self) -> usize {
    <u32>::size()
  }
  #[inline]
  fn as_bytes(&self, bytes: &mut [u8]) {
    (self as u32).as_bytes(bytes);
  }
}
