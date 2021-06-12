// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xfixes::*;
use super::xproto::*;
pub type EventClass = Card32;
pub type KeyCode = Card8;
pub type DeviceId = Card16;
pub type Fp1616 = Int32;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct Fp3232 {
    pub integral: Int32,
    pub frac: Card32,
}
impl Fp3232 {}
impl AsByteSequence for Fp3232 {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.integral.as_bytes(&mut bytes[index..]);
        index += self.frac.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Fp3232 from byte buffer");
        let (integral, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (frac, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Fp3232 {
                integral: integral,
                frac: frac,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.integral.size() + self.frac.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetExtensionVersionRequest<'a> {
    pub req_type: u8,
    pub length: u16,
    pub name: Cow<'a, str>,
}
impl<'a> GetExtensionVersionRequest {}
impl<'a> AsByteSequence for GetExtensionVersionRequest<'a> {
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
        log::trace!("Deserializing GetExtensionVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((
            GetExtensionVersionRequest {
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
impl Request for GetExtensionVersionRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetExtensionVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetExtensionVersionReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub server_major: Card16,
    pub server_minor: Card16,
    pub present: bool,
}
impl GetExtensionVersionReply {}
impl AsByteSequence for GetExtensionVersionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.server_major.as_bytes(&mut bytes[index..]);
        index += self.server_minor.as_bytes(&mut bytes[index..]);
        index += self.present.as_bytes(&mut bytes[index..]);
        index += 19;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetExtensionVersionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_major, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_minor, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (present, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 19;
        Some((
            GetExtensionVersionReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                server_major: server_major,
                server_minor: server_minor,
                present: present,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.server_major.size()
            + self.server_minor.size()
            + self.present.size()
            + 19
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceInfo {
    pub device_type: Atom,
    pub device_id: Card8,
    pub num_class_info: Card8,
    pub device_use: DeviceUse,
}
impl DeviceInfo {}
impl AsByteSequence for DeviceInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.device_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.num_class_info.as_bytes(&mut bytes[index..]);
        index += self.device_use.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceInfo from byte buffer");
        let (device_type, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_class_info, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_use, sz): (DeviceUse, usize) = <DeviceUse>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            DeviceInfo {
                device_type: device_type,
                device_id: device_id,
                num_class_info: num_class_info,
                device_use: device_use,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.device_type.size()
            + self.device_id.size()
            + self.num_class_info.size()
            + self.device_use.size()
            + 1
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DeviceUse {
    IsXPointer = 0,
    IsXKeyboard = 1,
    IsXExtensionDevice = 2,
    IsXExtensionKeyboard = 3,
    IsXExtensionPointer = 4,
}
impl AsByteSequence for DeviceUse {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::IsXPointer, sz)),
            1 => Some((Self::IsXKeyboard, sz)),
            2 => Some((Self::IsXExtensionDevice, sz)),
            3 => Some((Self::IsXExtensionKeyboard, sz)),
            4 => Some((Self::IsXExtensionPointer, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for DeviceUse {
    #[inline]
    fn default() -> DeviceUse {
        DeviceUse::IsXPointer
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct KeyInfo {
    pub class_id: InputClass,
    pub len: Card8,
    pub min_keycode: KeyCode,
    pub max_keycode: KeyCode,
    pub num_keys: Card16,
}
impl KeyInfo {}
impl AsByteSequence for KeyInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.min_keycode.as_bytes(&mut bytes[index..]);
        index += self.max_keycode.as_bytes(&mut bytes[index..]);
        index += self.num_keys.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyInfo from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_keycode, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_keycode, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_keys, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            KeyInfo {
                class_id: class_id,
                len: len,
                min_keycode: min_keycode,
                max_keycode: max_keycode,
                num_keys: num_keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.len.size()
            + self.min_keycode.size()
            + self.max_keycode.size()
            + self.num_keys.size()
            + 2
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum InputClass {
    Key = 0,
    Button = 1,
    Valuator = 2,
    Feedback = 3,
    Proximity = 4,
    Focus = 5,
    Other = 6,
}
impl AsByteSequence for InputClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Key, sz)),
            1 => Some((Self::Button, sz)),
            2 => Some((Self::Valuator, sz)),
            3 => Some((Self::Feedback, sz)),
            4 => Some((Self::Proximity, sz)),
            5 => Some((Self::Focus, sz)),
            6 => Some((Self::Other, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for InputClass {
    #[inline]
    fn default() -> InputClass {
        InputClass::Key
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ButtonInfo {
    pub class_id: InputClass,
    pub len: Card8,
    pub num_buttons: Card16,
}
impl ButtonInfo {}
impl AsByteSequence for ButtonInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.num_buttons.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ButtonInfo from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_buttons, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ButtonInfo {
                class_id: class_id,
                len: len,
                num_buttons: num_buttons,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size() + self.len.size() + self.num_buttons.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AxisInfo {
    pub resolution: Card32,
    pub minimum: Int32,
    pub maximum: Int32,
}
impl AxisInfo {}
impl AsByteSequence for AxisInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.resolution.as_bytes(&mut bytes[index..]);
        index += self.minimum.as_bytes(&mut bytes[index..]);
        index += self.maximum.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AxisInfo from byte buffer");
        let (resolution, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minimum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (maximum, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AxisInfo {
                resolution: resolution,
                minimum: minimum,
                maximum: maximum,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.resolution.size() + self.minimum.size() + self.maximum.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ValuatorInfo<'b> {
    pub class_id: InputClass,
    pub len: Card8,
    pub mode: ValuatorMode,
    pub motion_size: Card32,
    pub axes: Cow<'b, [AxisInfo]>,
}
impl<'b> ValuatorInfo {}
impl<'b> AsByteSequence for ValuatorInfo<'b> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += (self.axes.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.motion_size.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.axes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AxisInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ValuatorInfo from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (ValuatorMode, usize) = <ValuatorMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (motion_size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (axes, block_len): (Cow<'static, [AxisInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AxisInfo>());
        Some((
            ValuatorInfo {
                class_id: class_id,
                len: len,
                mode: mode,
                motion_size: motion_size,
                axes: axes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.len.size()
            + ::core::mem::size_of::<Card8>()
            + self.mode.size()
            + self.motion_size.size()
            + {
                let block_len: usize = self.axes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<AxisInfo>());
                block_len + pad
            }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ValuatorMode {
    Relative = 0,
    Absolute = 1,
}
impl AsByteSequence for ValuatorMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Relative, sz)),
            1 => Some((Self::Absolute, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ValuatorMode {
    #[inline]
    fn default() -> ValuatorMode {
        ValuatorMode::Relative
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InputInfo<'c> {
    pub class_id: InputClass,
    pub len: Card8,
    pub min_keycode: KeyCode,
    pub max_keycode: KeyCode,
    pub num_keys: Card16,
    pub num_buttons: Card16,
    pub mode: ValuatorMode,
    pub motion_size: Card32,
    pub axes: Cow<'c, [AxisInfo]>,
}
impl<'c> InputInfo {}
impl<'c> AsByteSequence for InputInfo<'c> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        let cond0 = (self.class_id);
        if cond0 == InputClass::Key {
            index += self.min_keycode.as_bytes(&mut bytes[index..]);
        }
        if cond0 == InputClass::Key {
            index += self.max_keycode.as_bytes(&mut bytes[index..]);
        }
        if cond0 == InputClass::Key {
            index += self.num_keys.as_bytes(&mut bytes[index..]);
        }
        index += 2;
        if cond0 == InputClass::Button {
            index += self.num_buttons.as_bytes(&mut bytes[index..]);
        }
        index += (self.axes.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0 == InputClass::Valuator {
            index += self.mode.as_bytes(&mut bytes[index..]);
        }
        if cond0 == InputClass::Valuator {
            index += self.motion_size.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.axes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AxisInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InputInfo from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (class_id);
        let min_keycode: KeyCode = if cond0 == InputClass::Key {
            let (min_keycode, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
            index += sz;
            min_keycode
        } else {
            Default::default()
        };
        let max_keycode: KeyCode = if cond0 == InputClass::Key {
            let (max_keycode, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
            index += sz;
            max_keycode
        } else {
            Default::default()
        };
        let num_keys: Card16 = if cond0 == InputClass::Key {
            let (num_keys, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            num_keys
        } else {
            Default::default()
        };
        index += 2;
        let num_buttons: Card16 = if cond0 == InputClass::Button {
            let (num_buttons, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            num_buttons
        } else {
            Default::default()
        };
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let mode: ValuatorMode = if cond0 == InputClass::Valuator {
            let (mode, sz): (ValuatorMode, usize) = <ValuatorMode>::from_bytes(&bytes[index..])?;
            index += sz;
            mode
        } else {
            Default::default()
        };
        let motion_size: Card32 = if cond0 == InputClass::Valuator {
            let (motion_size, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            motion_size
        } else {
            Default::default()
        };
        let (axes, block_len): (Cow<'static, [AxisInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<AxisInfo>());
        Some((
            InputInfo {
                class_id: class_id,
                len: len,
                min_keycode: min_keycode,
                max_keycode: max_keycode,
                num_keys: num_keys,
                num_buttons: num_buttons,
                mode: mode,
                motion_size: motion_size,
                axes: axes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.len.size()
            + self.min_keycode.size()
            + self.max_keycode.size()
            + self.num_keys.size()
            + 2
            + self.num_buttons.size()
            + ::core::mem::size_of::<Card8>()
            + self.mode.size()
            + self.motion_size.size()
            + {
                let block_len: usize = self.axes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<AxisInfo>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceName<'d> {
    pub string: Cow<'d, str>,
}
impl<'d> DeviceName {}
impl<'d> AsByteSequence for DeviceName<'d> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += (self.string.len() as Card8).as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceName from byte buffer");
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        Some((DeviceName { string: string }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<Card8>() + {
            let block_len: usize = self.string.len();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListInputDevicesRequest {
    pub req_type: u8,
    pub length: u16,
}
impl ListInputDevicesRequest {}
impl AsByteSequence for ListInputDevicesRequest {
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
        log::trace!("Deserializing ListInputDevicesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListInputDevicesRequest {
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
impl Request for ListInputDevicesRequest {
    const OPCODE: u8 = 2;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListInputDevicesReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListInputDevicesReply<'e, 'g, 'f, 'h> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub devices_len: Card8,
    pub devices: Cow<'e, [DeviceInfo]>,
    pub infos: Cow<'g, [InputInfo<'f>]>,
    pub names: Cow<'h, [Str]>,
}
impl<'e, 'g, 'f, 'h> ListInputDevicesReply {}
impl<'e, 'g, 'f, 'h> AsByteSequence for ListInputDevicesReply<'e, 'g, 'f, 'h> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.devices_len.as_bytes(&mut bytes[index..]);
        index += 23;
        let block_len: usize = vector_as_bytes(&self.devices, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DeviceInfo>());
        let block_len: usize = vector_as_bytes(&self.infos, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<InputInfo<'f>>());
        let block_len: usize = vector_as_bytes(&self.names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListInputDevicesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (devices_len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        let (devices, block_len): (Cow<'static, [DeviceInfo]>, usize) =
            vector_from_bytes(&bytes[index..], (devices_len as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DeviceInfo>());
        let (infos, block_len): (Cow<'static, [InputInfo<'f>]>, usize) = vector_from_bytes(
            &bytes[index..],
            (devices
                .iter()
                .map(|a| (a.num_class_info as usize) as usize)
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<InputInfo<'f>>());
        let (names, block_len): (Cow<'static, [Str]>, usize) =
            vector_from_bytes(&bytes[index..], (devices_len as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            ListInputDevicesReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                devices_len: devices_len,
                devices: devices,
                infos: infos,
                names: names,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.devices_len.size()
            + 23
            + {
                let block_len: usize = self.devices.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<DeviceInfo>());
                block_len + pad
            }
            + {
                let block_len: usize = self.infos.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<InputInfo<'f>>());
                block_len + pad
            }
            + {
                let block_len: usize = self.names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
pub type EventTypeBase = Card8;
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InputClassInfo {
    pub class_id: InputClass,
    pub event_type_base: EventTypeBase,
}
impl InputClassInfo {}
impl AsByteSequence for InputClassInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.event_type_base.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InputClassInfo from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_type_base, sz): (EventTypeBase, usize) =
            <EventTypeBase>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            InputClassInfo {
                class_id: class_id,
                event_type_base: event_type_base,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size() + self.event_type_base.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OpenDeviceRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl OpenDeviceRequest {}
impl AsByteSequence for OpenDeviceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OpenDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            OpenDeviceRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for OpenDeviceRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = OpenDeviceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct OpenDeviceReply<'i> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub class_info: Cow<'i, [InputClassInfo]>,
}
impl<'i> OpenDeviceReply {}
impl<'i> AsByteSequence for OpenDeviceReply<'i> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.class_info.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 23;
        let block_len: usize = vector_as_bytes(&self.class_info, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OpenDeviceReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        let (class_info, block_len): (Cow<'static, [InputClassInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            OpenDeviceReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                class_info: class_info,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card8>()
            + 23
            + {
                let block_len: usize = self.class_info.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct CloseDeviceRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl CloseDeviceRequest {}
impl AsByteSequence for CloseDeviceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CloseDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            CloseDeviceRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for CloseDeviceRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceModeRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
    pub mode: ValuatorMode,
}
impl SetDeviceModeRequest {}
impl AsByteSequence for SetDeviceModeRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceModeRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (ValuatorMode, usize) = <ValuatorMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            SetDeviceModeRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + self.mode.size() + 2
    }
}
impl Request for SetDeviceModeRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetDeviceModeReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceModeReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: GrabStatus,
}
impl SetDeviceModeReply {}
impl AsByteSequence for SetDeviceModeReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceModeReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            SetDeviceModeReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SelectExtensionEventRequest<'j> {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub classes: Cow<'j, [EventClass]>,
}
impl<'j> SelectExtensionEventRequest {}
impl<'j> AsByteSequence for SelectExtensionEventRequest<'j> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectExtensionEventRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            SelectExtensionEventRequest {
                req_type: req_type,
                length: length,
                window: window,
                classes: classes,
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
            + 2
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
impl Request for SelectExtensionEventRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectedExtensionEventsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetSelectedExtensionEventsRequest {}
impl AsByteSequence for GetSelectedExtensionEventsRequest {
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
        log::trace!("Deserializing GetSelectedExtensionEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetSelectedExtensionEventsRequest {
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
impl Request for GetSelectedExtensionEventsRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetSelectedExtensionEventsReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetSelectedExtensionEventsReply<'k, 'l> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub this_classes: Cow<'k, [EventClass]>,
    pub all_classes: Cow<'l, [EventClass]>,
}
impl<'k, 'l> GetSelectedExtensionEventsReply {}
impl<'k, 'l> AsByteSequence for GetSelectedExtensionEventsReply<'k, 'l> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.this_classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.all_classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 20;
        let block_len: usize = vector_as_bytes(&self.this_classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        let block_len: usize = vector_as_bytes(&self.all_classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetSelectedExtensionEventsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        let (this_classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        let (all_classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            GetSelectedExtensionEventsReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                this_classes: this_classes,
                all_classes: all_classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + 20
            + {
                let block_len: usize = self.this_classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
            + {
                let block_len: usize = self.all_classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeDeviceDontPropagateListRequest<'m> {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub mode: PropagateMode,
    pub classes: Cow<'m, [EventClass]>,
}
impl<'m> ChangeDeviceDontPropagateListRequest {}
impl<'m> AsByteSequence for ChangeDeviceDontPropagateListRequest<'m> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 1;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeDeviceDontPropagateListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (PropagateMode, usize) = <PropagateMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            ChangeDeviceDontPropagateListRequest {
                req_type: req_type,
                length: length,
                window: window,
                mode: mode,
                classes: classes,
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
            + self.mode.size()
            + 1
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
impl Request for ChangeDeviceDontPropagateListRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum PropagateMode {
    AddToList = 0,
    DeleteFromList = 1,
}
impl AsByteSequence for PropagateMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::AddToList, sz)),
            1 => Some((Self::DeleteFromList, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for PropagateMode {
    #[inline]
    fn default() -> PropagateMode {
        PropagateMode::AddToList
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceDontPropagateListRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl GetDeviceDontPropagateListRequest {}
impl AsByteSequence for GetDeviceDontPropagateListRequest {
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
        log::trace!("Deserializing GetDeviceDontPropagateListRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDeviceDontPropagateListRequest {
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
impl Request for GetDeviceDontPropagateListRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceDontPropagateListReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceDontPropagateListReply<'n> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub classes: Cow<'n, [EventClass]>,
}
impl<'n> GetDeviceDontPropagateListReply {}
impl<'n> AsByteSequence for GetDeviceDontPropagateListReply<'n> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceDontPropagateListReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            GetDeviceDontPropagateListReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeKeyboardDeviceRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl ChangeKeyboardDeviceRequest {}
impl AsByteSequence for ChangeKeyboardDeviceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeKeyboardDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            ChangeKeyboardDeviceRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for ChangeKeyboardDeviceRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ChangeKeyboardDeviceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeKeyboardDeviceReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: GrabStatus,
}
impl ChangeKeyboardDeviceReply {}
impl AsByteSequence for ChangeKeyboardDeviceReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeKeyboardDeviceReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            ChangeKeyboardDeviceReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangePointerDeviceRequest {
    pub req_type: u8,
    pub length: u16,
    pub x_axis: Card8,
    pub y_axis: Card8,
    pub device_id: Card8,
}
impl ChangePointerDeviceRequest {}
impl AsByteSequence for ChangePointerDeviceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.x_axis.as_bytes(&mut bytes[index..]);
        index += self.y_axis.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangePointerDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_axis, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_axis, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            ChangePointerDeviceRequest {
                req_type: req_type,
                length: length,
                x_axis: x_axis,
                y_axis: y_axis,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.x_axis.size()
            + self.y_axis.size()
            + self.device_id.size()
            + 1
    }
}
impl Request for ChangePointerDeviceRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ChangePointerDeviceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangePointerDeviceReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: GrabStatus,
}
impl ChangePointerDeviceReply {}
impl AsByteSequence for ChangePointerDeviceReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangePointerDeviceReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            ChangePointerDeviceReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GrabDeviceRequest<'o> {
    pub req_type: u8,
    pub length: u16,
    pub grab_window: Window,
    pub time: Timestamp,
    pub this_device_mode: GrabMode,
    pub other_device_mode: GrabMode,
    pub owner_events: bool,
    pub device_id: Card8,
    pub classes: Cow<'o, [EventClass]>,
}
impl<'o> GrabDeviceRequest {}
impl<'o> AsByteSequence for GrabDeviceRequest<'o> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.this_device_mode.as_bytes(&mut bytes[index..]);
        index += self.other_device_mode.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (this_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (other_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            GrabDeviceRequest {
                req_type: req_type,
                length: length,
                grab_window: grab_window,
                time: time,
                this_device_mode: this_device_mode,
                other_device_mode: other_device_mode,
                owner_events: owner_events,
                device_id: device_id,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.grab_window.size()
            + self.time.size()
            + ::core::mem::size_of::<Card16>()
            + self.this_device_mode.size()
            + self.other_device_mode.size()
            + self.owner_events.size()
            + self.device_id.size()
            + 2
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
impl Request for GrabDeviceRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GrabDeviceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GrabDeviceReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: GrabStatus,
}
impl GrabDeviceReply {}
impl AsByteSequence for GrabDeviceReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabDeviceReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            GrabDeviceReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct UngrabDeviceRequest {
    pub req_type: u8,
    pub length: u16,
    pub time: Timestamp,
    pub device_id: Card8,
}
impl UngrabDeviceRequest {}
impl AsByteSequence for UngrabDeviceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            UngrabDeviceRequest {
                req_type: req_type,
                length: length,
                time: time,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.time.size() + self.device_id.size() + 3
    }
}
impl Request for UngrabDeviceRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GrabDeviceKeyRequest<'p> {
    pub req_type: u8,
    pub length: u16,
    pub grab_window: Window,
    pub modifiers: ModMask,
    pub modifier_device: Card8,
    pub grabbed_device: Card8,
    pub key: Card8,
    pub this_device_mode: GrabMode,
    pub other_device_mode: GrabMode,
    pub owner_events: bool,
    pub classes: Cow<'p, [EventClass]>,
}
impl<'p> GrabDeviceKeyRequest {}
impl<'p> AsByteSequence for GrabDeviceKeyRequest<'p> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += self.modifier_device.as_bytes(&mut bytes[index..]);
        index += self.grabbed_device.as_bytes(&mut bytes[index..]);
        index += self.key.as_bytes(&mut bytes[index..]);
        index += self.this_device_mode.as_bytes(&mut bytes[index..]);
        index += self.other_device_mode.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabDeviceKeyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifier_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grabbed_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (this_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (other_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            GrabDeviceKeyRequest {
                req_type: req_type,
                length: length,
                grab_window: grab_window,
                modifiers: modifiers,
                modifier_device: modifier_device,
                grabbed_device: grabbed_device,
                key: key,
                this_device_mode: this_device_mode,
                other_device_mode: other_device_mode,
                owner_events: owner_events,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.grab_window.size()
            + ::core::mem::size_of::<Card16>()
            + self.modifiers.size()
            + self.modifier_device.size()
            + self.grabbed_device.size()
            + self.key.size()
            + self.this_device_mode.size()
            + self.other_device_mode.size()
            + self.owner_events.size()
            + 2
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
impl Request for GrabDeviceKeyRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ModifierDevice {
    UseXKeyboard = 255,
}
impl AsByteSequence for ModifierDevice {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            255 => Some((Self::UseXKeyboard, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ModifierDevice {
    #[inline]
    fn default() -> ModifierDevice {
        ModifierDevice::UseXKeyboard
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct UngrabDeviceKeyRequest {
    pub req_type: u8,
    pub length: u16,
    pub grab_window: Window,
    pub modifiers: ModMask,
    pub modifier_device: Card8,
    pub key: Card8,
    pub grabbed_device: Card8,
}
impl UngrabDeviceKeyRequest {}
impl AsByteSequence for UngrabDeviceKeyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += self.modifier_device.as_bytes(&mut bytes[index..]);
        index += self.key.as_bytes(&mut bytes[index..]);
        index += self.grabbed_device.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabDeviceKeyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifier_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grabbed_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UngrabDeviceKeyRequest {
                req_type: req_type,
                length: length,
                grab_window: grab_window,
                modifiers: modifiers,
                modifier_device: modifier_device,
                key: key,
                grabbed_device: grabbed_device,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.grab_window.size()
            + self.modifiers.size()
            + self.modifier_device.size()
            + self.key.size()
            + self.grabbed_device.size()
    }
}
impl Request for UngrabDeviceKeyRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GrabDeviceButtonRequest<'q> {
    pub req_type: u8,
    pub length: u16,
    pub grab_window: Window,
    pub grabbed_device: Card8,
    pub modifier_device: Card8,
    pub modifiers: ModMask,
    pub this_device_mode: GrabMode,
    pub other_device_mode: GrabMode,
    pub button: Card8,
    pub owner_events: bool,
    pub classes: Cow<'q, [EventClass]>,
}
impl<'q> GrabDeviceButtonRequest {}
impl<'q> AsByteSequence for GrabDeviceButtonRequest<'q> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.grabbed_device.as_bytes(&mut bytes[index..]);
        index += self.modifier_device.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += self.this_device_mode.as_bytes(&mut bytes[index..]);
        index += self.other_device_mode.as_bytes(&mut bytes[index..]);
        index += self.button.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabDeviceButtonRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grabbed_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifier_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (this_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (other_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            GrabDeviceButtonRequest {
                req_type: req_type,
                length: length,
                grab_window: grab_window,
                grabbed_device: grabbed_device,
                modifier_device: modifier_device,
                modifiers: modifiers,
                this_device_mode: this_device_mode,
                other_device_mode: other_device_mode,
                button: button,
                owner_events: owner_events,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.grab_window.size()
            + self.grabbed_device.size()
            + self.modifier_device.size()
            + ::core::mem::size_of::<Card16>()
            + self.modifiers.size()
            + self.this_device_mode.size()
            + self.other_device_mode.size()
            + self.button.size()
            + self.owner_events.size()
            + 2
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
impl Request for GrabDeviceButtonRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct UngrabDeviceButtonRequest {
    pub req_type: u8,
    pub length: u16,
    pub grab_window: Window,
    pub modifiers: ModMask,
    pub modifier_device: Card8,
    pub button: Card8,
    pub grabbed_device: Card8,
}
impl UngrabDeviceButtonRequest {}
impl AsByteSequence for UngrabDeviceButtonRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += self.modifier_device.as_bytes(&mut bytes[index..]);
        index += self.button.as_bytes(&mut bytes[index..]);
        index += self.grabbed_device.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UngrabDeviceButtonRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (modifier_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grabbed_device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            UngrabDeviceButtonRequest {
                req_type: req_type,
                length: length,
                grab_window: grab_window,
                modifiers: modifiers,
                modifier_device: modifier_device,
                button: button,
                grabbed_device: grabbed_device,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.grab_window.size()
            + self.modifiers.size()
            + self.modifier_device.size()
            + self.button.size()
            + self.grabbed_device.size()
            + 3
    }
}
impl Request for UngrabDeviceButtonRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AllowDeviceEventsRequest {
    pub req_type: u8,
    pub length: u16,
    pub time: Timestamp,
    pub mode: DeviceInputMode,
    pub device_id: Card8,
}
impl AllowDeviceEventsRequest {}
impl AsByteSequence for AllowDeviceEventsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AllowDeviceEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (DeviceInputMode, usize) = <DeviceInputMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            AllowDeviceEventsRequest {
                req_type: req_type,
                length: length,
                time: time,
                mode: mode,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.time.size()
            + self.mode.size()
            + self.device_id.size()
            + 2
    }
}
impl Request for AllowDeviceEventsRequest {
    const OPCODE: u8 = 19;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DeviceInputMode {
    AsyncThisDevice = 0,
    SyncThisDevice = 1,
    ReplayThisDevice = 2,
    AsyncOtherDevices = 3,
    AsyncAll = 4,
    SyncAll = 5,
}
impl AsByteSequence for DeviceInputMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::AsyncThisDevice, sz)),
            1 => Some((Self::SyncThisDevice, sz)),
            2 => Some((Self::ReplayThisDevice, sz)),
            3 => Some((Self::AsyncOtherDevices, sz)),
            4 => Some((Self::AsyncAll, sz)),
            5 => Some((Self::SyncAll, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for DeviceInputMode {
    #[inline]
    fn default() -> DeviceInputMode {
        DeviceInputMode::AsyncThisDevice
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceFocusRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl GetDeviceFocusRequest {}
impl AsByteSequence for GetDeviceFocusRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceFocusRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GetDeviceFocusRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for GetDeviceFocusRequest {
    const OPCODE: u8 = 20;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceFocusReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceFocusReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub focus: Window,
    pub time: Timestamp,
    pub revert_to: InputFocus,
}
impl GetDeviceFocusReply {}
impl AsByteSequence for GetDeviceFocusReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.revert_to.as_bytes(&mut bytes[index..]);
        index += 15;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceFocusReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (revert_to, sz): (InputFocus, usize) = <InputFocus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 15;
        Some((
            GetDeviceFocusReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                focus: focus,
                time: time,
                revert_to: revert_to,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.focus.size()
            + self.time.size()
            + self.revert_to.size()
            + 15
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceFocusRequest {
    pub req_type: u8,
    pub length: u16,
    pub focus: Window,
    pub time: Timestamp,
    pub revert_to: InputFocus,
    pub device_id: Card8,
}
impl SetDeviceFocusRequest {}
impl AsByteSequence for SetDeviceFocusRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.revert_to.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceFocusRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (revert_to, sz): (InputFocus, usize) = <InputFocus>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            SetDeviceFocusRequest {
                req_type: req_type,
                length: length,
                focus: focus,
                time: time,
                revert_to: revert_to,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.focus.size()
            + self.time.size()
            + self.revert_to.size()
            + self.device_id.size()
            + 2
    }
}
impl Request for SetDeviceFocusRequest {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct KbdFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub pitch: Card16,
    pub duration: Card16,
    pub led_mask: Card32,
    pub led_values: Card32,
    pub global_auto_repeat: bool,
    pub click: Card8,
    pub percent: Card8,
    pub auto_repeats: [Card8; 32],
}
impl KbdFeedbackState {}
impl AsByteSequence for KbdFeedbackState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.pitch.as_bytes(&mut bytes[index..]);
        index += self.duration.as_bytes(&mut bytes[index..]);
        index += self.led_mask.as_bytes(&mut bytes[index..]);
        index += self.led_values.as_bytes(&mut bytes[index..]);
        index += self.global_auto_repeat.as_bytes(&mut bytes[index..]);
        index += self.click.as_bytes(&mut bytes[index..]);
        index += self.percent.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.auto_repeats.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KbdFeedbackState from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pitch, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (duration, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_values, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (global_auto_repeat, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (click, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (percent, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (auto_repeats, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KbdFeedbackState {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                pitch: pitch,
                duration: duration,
                led_mask: led_mask,
                led_values: led_values,
                global_auto_repeat: global_auto_repeat,
                click: click,
                percent: percent,
                auto_repeats: auto_repeats,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.pitch.size()
            + self.duration.size()
            + self.led_mask.size()
            + self.led_values.size()
            + self.global_auto_repeat.size()
            + self.click.size()
            + self.percent.size()
            + 1
            + self.auto_repeats.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum FeedbackClass {
    Keyboard = 0,
    Pointer = 1,
    String = 2,
    Integer = 3,
    Led = 4,
    Bell = 5,
}
impl AsByteSequence for FeedbackClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Keyboard, sz)),
            1 => Some((Self::Pointer, sz)),
            2 => Some((Self::String, sz)),
            3 => Some((Self::Integer, sz)),
            4 => Some((Self::Led, sz)),
            5 => Some((Self::Bell, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for FeedbackClass {
    #[inline]
    fn default() -> FeedbackClass {
        FeedbackClass::Keyboard
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct PtrFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub accel_num: Card16,
    pub accel_denom: Card16,
    pub threshold: Card16,
}
impl PtrFeedbackState {}
impl AsByteSequence for PtrFeedbackState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.accel_num.as_bytes(&mut bytes[index..]);
        index += self.accel_denom.as_bytes(&mut bytes[index..]);
        index += self.threshold.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PtrFeedbackState from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (accel_num, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (accel_denom, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (threshold, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PtrFeedbackState {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                accel_num: accel_num,
                accel_denom: accel_denom,
                threshold: threshold,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + 2
            + self.accel_num.size()
            + self.accel_denom.size()
            + self.threshold.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct IntegerFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub resolution: Card32,
    pub min_value: Int32,
    pub max_value: Int32,
}
impl IntegerFeedbackState {}
impl AsByteSequence for IntegerFeedbackState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.resolution.as_bytes(&mut bytes[index..]);
        index += self.min_value.as_bytes(&mut bytes[index..]);
        index += self.max_value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IntegerFeedbackState from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resolution, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_value, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_value, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IntegerFeedbackState {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                resolution: resolution,
                min_value: min_value,
                max_value: max_value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.resolution.size()
            + self.min_value.size()
            + self.max_value.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct StringFeedbackState<'r> {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub max_symbols: Card16,
    pub keysyms: Cow<'r, [Keysym]>,
}
impl<'r> StringFeedbackState {}
impl<'r> AsByteSequence for StringFeedbackState<'r> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.max_symbols.as_bytes(&mut bytes[index..]);
        index += (self.keysyms.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing StringFeedbackState from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_symbols, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms, block_len): (Cow<'static, [Keysym]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        Some((
            StringFeedbackState {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                max_symbols: max_symbols,
                keysyms: keysyms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.max_symbols.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct BellFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub percent: Card8,
    pub pitch: Card16,
    pub duration: Card16,
}
impl BellFeedbackState {}
impl AsByteSequence for BellFeedbackState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.percent.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.pitch.as_bytes(&mut bytes[index..]);
        index += self.duration.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BellFeedbackState from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (percent, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (pitch, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (duration, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BellFeedbackState {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                percent: percent,
                pitch: pitch,
                duration: duration,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.percent.size()
            + 3
            + self.pitch.size()
            + self.duration.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct LedFeedbackState {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub led_mask: Card32,
    pub led_values: Card32,
}
impl LedFeedbackState {}
impl AsByteSequence for LedFeedbackState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.led_mask.as_bytes(&mut bytes[index..]);
        index += self.led_values.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LedFeedbackState from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_values, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            LedFeedbackState {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                led_mask: led_mask,
                led_values: led_values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.led_mask.size()
            + self.led_values.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct FeedbackState<'s> {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub pitch: Card16,
    pub duration: Card16,
    pub led_mask: Card32,
    pub led_values: Card32,
    pub global_auto_repeat: bool,
    pub click: Card8,
    pub percent: Card8,
    pub auto_repeats: [Card8; 32],
    pub accel_num: Card16,
    pub accel_denom: Card16,
    pub threshold: Card16,
    pub max_symbols: Card16,
    pub keysyms: Cow<'s, [Keysym]>,
    pub resolution: Card32,
    pub min_value: Int32,
    pub max_value: Int32,
    pub led_mask_: Card32,
    pub led_values_: Card32,
    pub percent_: Card8,
    pub pitch_: Card16,
    pub duration_: Card16,
}
impl<'s> FeedbackState {}
impl<'s> AsByteSequence for FeedbackState<'s> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        let cond0 = (self.class_id);
        if cond0 == FeedbackClass::Keyboard {
            index += self.pitch.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.duration.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.led_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.led_values.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.global_auto_repeat.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.click.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.percent.as_bytes(&mut bytes[index..]);
        }
        index += 1;
        index += self.auto_repeats.as_bytes(&mut bytes[index..]);
        index += 2;
        if cond0 == FeedbackClass::Pointer {
            index += self.accel_num.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Pointer {
            index += self.accel_denom.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Pointer {
            index += self.threshold.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::String {
            index += self.max_symbols.as_bytes(&mut bytes[index..]);
        }
        index += (self.keysyms.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        if cond0 == FeedbackClass::Integer {
            index += self.resolution.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Integer {
            index += self.min_value.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Integer {
            index += self.max_value.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Led {
            index += self.led_mask_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Led {
            index += self.led_values_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Bell {
            index += self.percent_.as_bytes(&mut bytes[index..]);
        }
        index += 3;
        if cond0 == FeedbackClass::Bell {
            index += self.pitch_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Bell {
            index += self.duration_.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FeedbackState from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (class_id);
        let pitch: Card16 = if cond0 == FeedbackClass::Keyboard {
            let (pitch, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            pitch
        } else {
            Default::default()
        };
        let duration: Card16 = if cond0 == FeedbackClass::Keyboard {
            let (duration, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            duration
        } else {
            Default::default()
        };
        let led_mask: Card32 = if cond0 == FeedbackClass::Keyboard {
            let (led_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_mask
        } else {
            Default::default()
        };
        let led_values: Card32 = if cond0 == FeedbackClass::Keyboard {
            let (led_values, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_values
        } else {
            Default::default()
        };
        let global_auto_repeat: bool = if cond0 == FeedbackClass::Keyboard {
            let (global_auto_repeat, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
            index += sz;
            global_auto_repeat
        } else {
            Default::default()
        };
        let click: Card8 = if cond0 == FeedbackClass::Keyboard {
            let (click, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            click
        } else {
            Default::default()
        };
        let percent: Card8 = if cond0 == FeedbackClass::Keyboard {
            let (percent, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            percent
        } else {
            Default::default()
        };
        index += 1;
        let (auto_repeats, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let accel_num: Card16 = if cond0 == FeedbackClass::Pointer {
            let (accel_num, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            accel_num
        } else {
            Default::default()
        };
        let accel_denom: Card16 = if cond0 == FeedbackClass::Pointer {
            let (accel_denom, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            accel_denom
        } else {
            Default::default()
        };
        let threshold: Card16 = if cond0 == FeedbackClass::Pointer {
            let (threshold, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            threshold
        } else {
            Default::default()
        };
        let max_symbols: Card16 = if cond0 == FeedbackClass::String {
            let (max_symbols, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            max_symbols
        } else {
            Default::default()
        };
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms, block_len): (Cow<'static, [Keysym]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        let resolution: Card32 = if cond0 == FeedbackClass::Integer {
            let (resolution, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            resolution
        } else {
            Default::default()
        };
        let min_value: Int32 = if cond0 == FeedbackClass::Integer {
            let (min_value, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            min_value
        } else {
            Default::default()
        };
        let max_value: Int32 = if cond0 == FeedbackClass::Integer {
            let (max_value, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            max_value
        } else {
            Default::default()
        };
        let led_mask_: Card32 = if cond0 == FeedbackClass::Led {
            let (led_mask_, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_mask_
        } else {
            Default::default()
        };
        let led_values_: Card32 = if cond0 == FeedbackClass::Led {
            let (led_values_, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_values_
        } else {
            Default::default()
        };
        let percent_: Card8 = if cond0 == FeedbackClass::Bell {
            let (percent_, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            percent_
        } else {
            Default::default()
        };
        index += 3;
        let pitch_: Card16 = if cond0 == FeedbackClass::Bell {
            let (pitch_, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            pitch_
        } else {
            Default::default()
        };
        let duration_: Card16 = if cond0 == FeedbackClass::Bell {
            let (duration_, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            duration_
        } else {
            Default::default()
        };
        Some((
            FeedbackState {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                pitch: pitch,
                duration: duration,
                led_mask: led_mask,
                led_values: led_values,
                global_auto_repeat: global_auto_repeat,
                click: click,
                percent: percent,
                auto_repeats: auto_repeats,
                accel_num: accel_num,
                accel_denom: accel_denom,
                threshold: threshold,
                max_symbols: max_symbols,
                keysyms: keysyms,
                resolution: resolution,
                min_value: min_value,
                max_value: max_value,
                led_mask_: led_mask_,
                led_values_: led_values_,
                percent_: percent_,
                pitch_: pitch_,
                duration_: duration_,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.pitch.size()
            + self.duration.size()
            + self.led_mask.size()
            + self.led_values.size()
            + self.global_auto_repeat.size()
            + self.click.size()
            + self.percent.size()
            + 1
            + self.auto_repeats.size()
            + 2
            + self.accel_num.size()
            + self.accel_denom.size()
            + self.threshold.size()
            + self.max_symbols.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
            + self.resolution.size()
            + self.min_value.size()
            + self.max_value.size()
            + self.led_mask_.size()
            + self.led_values_.size()
            + self.percent_.size()
            + 3
            + self.pitch_.size()
            + self.duration_.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetFeedbackControlRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl GetFeedbackControlRequest {}
impl AsByteSequence for GetFeedbackControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFeedbackControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GetFeedbackControlRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for GetFeedbackControlRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetFeedbackControlReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetFeedbackControlReply<'u, 't> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub feedbacks: Cow<'u, [FeedbackState<'t>]>,
}
impl<'u, 't> GetFeedbackControlReply {}
impl<'u, 't> AsByteSequence for GetFeedbackControlReply<'u, 't> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.feedbacks.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.feedbacks, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<FeedbackState<'t>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetFeedbackControlReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (feedbacks, block_len): (Cow<'static, [FeedbackState<'t>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<FeedbackState<'t>>());
        Some((
            GetFeedbackControlReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                feedbacks: feedbacks,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + 22
            + {
                let block_len: usize = self.feedbacks.iter().map(|i| i.size()).sum();
                let pad: usize =
                    buffer_pad(block_len, ::core::mem::align_of::<FeedbackState<'t>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct KbdFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub key: KeyCode,
    pub auto_repeat_mode: Card8,
    pub key_click_percent: Int8,
    pub bell_percent: Int8,
    pub bell_pitch: Int16,
    pub bell_duration: Int16,
    pub led_mask: Card32,
    pub led_values: Card32,
}
impl KbdFeedbackCtl {}
impl AsByteSequence for KbdFeedbackCtl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.key.as_bytes(&mut bytes[index..]);
        index += self.auto_repeat_mode.as_bytes(&mut bytes[index..]);
        index += self.key_click_percent.as_bytes(&mut bytes[index..]);
        index += self.bell_percent.as_bytes(&mut bytes[index..]);
        index += self.bell_pitch.as_bytes(&mut bytes[index..]);
        index += self.bell_duration.as_bytes(&mut bytes[index..]);
        index += self.led_mask.as_bytes(&mut bytes[index..]);
        index += self.led_values.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KbdFeedbackCtl from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (auto_repeat_mode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key_click_percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_pitch, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_duration, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_values, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KbdFeedbackCtl {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                key: key,
                auto_repeat_mode: auto_repeat_mode,
                key_click_percent: key_click_percent,
                bell_percent: bell_percent,
                bell_pitch: bell_pitch,
                bell_duration: bell_duration,
                led_mask: led_mask,
                led_values: led_values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.key.size()
            + self.auto_repeat_mode.size()
            + self.key_click_percent.size()
            + self.bell_percent.size()
            + self.bell_pitch.size()
            + self.bell_duration.size()
            + self.led_mask.size()
            + self.led_values.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct PtrFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub num: Int16,
    pub denom: Int16,
    pub threshold: Int16,
}
impl PtrFeedbackCtl {}
impl AsByteSequence for PtrFeedbackCtl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.num.as_bytes(&mut bytes[index..]);
        index += self.denom.as_bytes(&mut bytes[index..]);
        index += self.threshold.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PtrFeedbackCtl from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (num, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (denom, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (threshold, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PtrFeedbackCtl {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                num: num,
                denom: denom,
                threshold: threshold,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + 2
            + self.num.size()
            + self.denom.size()
            + self.threshold.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct IntegerFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub int_to_display: Int32,
}
impl IntegerFeedbackCtl {}
impl AsByteSequence for IntegerFeedbackCtl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.int_to_display.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IntegerFeedbackCtl from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (int_to_display, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IntegerFeedbackCtl {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                int_to_display: int_to_display,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.int_to_display.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct StringFeedbackCtl<'v> {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub keysyms: Cow<'v, [Keysym]>,
}
impl<'v> StringFeedbackCtl {}
impl<'v> AsByteSequence for StringFeedbackCtl<'v> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += 2;
        index += (self.keysyms.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing StringFeedbackCtl from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms, block_len): (Cow<'static, [Keysym]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        Some((
            StringFeedbackCtl {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                keysyms: keysyms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + 2
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct BellFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub percent: Int8,
    pub pitch: Int16,
    pub duration: Int16,
}
impl BellFeedbackCtl {}
impl AsByteSequence for BellFeedbackCtl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.percent.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.pitch.as_bytes(&mut bytes[index..]);
        index += self.duration.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BellFeedbackCtl from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (pitch, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (duration, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BellFeedbackCtl {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                percent: percent,
                pitch: pitch,
                duration: duration,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.percent.size()
            + 3
            + self.pitch.size()
            + self.duration.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct LedFeedbackCtl {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub led_mask: Card32,
    pub led_values: Card32,
}
impl LedFeedbackCtl {}
impl AsByteSequence for LedFeedbackCtl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.led_mask.as_bytes(&mut bytes[index..]);
        index += self.led_values.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LedFeedbackCtl from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_values, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            LedFeedbackCtl {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                led_mask: led_mask,
                led_values: led_values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.led_mask.size()
            + self.led_values.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct FeedbackCtl<'w> {
    pub class_id: FeedbackClass,
    pub feedback_id: Card8,
    pub len: Card16,
    pub key: KeyCode,
    pub auto_repeat_mode: Card8,
    pub key_click_percent: Int8,
    pub bell_percent: Int8,
    pub bell_pitch: Int16,
    pub bell_duration: Int16,
    pub led_mask: Card32,
    pub led_values: Card32,
    pub num: Int16,
    pub denom: Int16,
    pub threshold: Int16,
    pub keysyms: Cow<'w, [Keysym]>,
    pub int_to_display: Int32,
    pub led_mask_: Card32,
    pub led_values_: Card32,
    pub percent: Int8,
    pub pitch: Int16,
    pub duration: Int16,
}
impl<'w> FeedbackCtl {}
impl<'w> AsByteSequence for FeedbackCtl<'w> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        let cond0 = (self.class_id);
        if cond0 == FeedbackClass::Keyboard {
            index += self.key.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.auto_repeat_mode.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.key_click_percent.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.bell_percent.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.bell_pitch.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.bell_duration.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.led_mask.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Keyboard {
            index += self.led_values.as_bytes(&mut bytes[index..]);
        }
        index += 2;
        if cond0 == FeedbackClass::Pointer {
            index += self.num.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Pointer {
            index += self.denom.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Pointer {
            index += self.threshold.as_bytes(&mut bytes[index..]);
        }
        index += (self.keysyms.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        if cond0 == FeedbackClass::Integer {
            index += self.int_to_display.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Led {
            index += self.led_mask_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Led {
            index += self.led_values_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Bell {
            index += self.percent.as_bytes(&mut bytes[index..]);
        }
        index += 3;
        if cond0 == FeedbackClass::Bell {
            index += self.pitch.as_bytes(&mut bytes[index..]);
        }
        if cond0 == FeedbackClass::Bell {
            index += self.duration.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FeedbackCtl from byte buffer");
        let (class_id, sz): (FeedbackClass, usize) = <FeedbackClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (class_id);
        let key: KeyCode = if cond0 == FeedbackClass::Keyboard {
            let (key, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
            index += sz;
            key
        } else {
            Default::default()
        };
        let auto_repeat_mode: Card8 = if cond0 == FeedbackClass::Keyboard {
            let (auto_repeat_mode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            auto_repeat_mode
        } else {
            Default::default()
        };
        let key_click_percent: Int8 = if cond0 == FeedbackClass::Keyboard {
            let (key_click_percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
            index += sz;
            key_click_percent
        } else {
            Default::default()
        };
        let bell_percent: Int8 = if cond0 == FeedbackClass::Keyboard {
            let (bell_percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
            index += sz;
            bell_percent
        } else {
            Default::default()
        };
        let bell_pitch: Int16 = if cond0 == FeedbackClass::Keyboard {
            let (bell_pitch, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
            index += sz;
            bell_pitch
        } else {
            Default::default()
        };
        let bell_duration: Int16 = if cond0 == FeedbackClass::Keyboard {
            let (bell_duration, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
            index += sz;
            bell_duration
        } else {
            Default::default()
        };
        let led_mask: Card32 = if cond0 == FeedbackClass::Keyboard {
            let (led_mask, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_mask
        } else {
            Default::default()
        };
        let led_values: Card32 = if cond0 == FeedbackClass::Keyboard {
            let (led_values, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_values
        } else {
            Default::default()
        };
        index += 2;
        let num: Int16 = if cond0 == FeedbackClass::Pointer {
            let (num, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
            index += sz;
            num
        } else {
            Default::default()
        };
        let denom: Int16 = if cond0 == FeedbackClass::Pointer {
            let (denom, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
            index += sz;
            denom
        } else {
            Default::default()
        };
        let threshold: Int16 = if cond0 == FeedbackClass::Pointer {
            let (threshold, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
            index += sz;
            threshold
        } else {
            Default::default()
        };
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms, block_len): (Cow<'static, [Keysym]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        let int_to_display: Int32 = if cond0 == FeedbackClass::Integer {
            let (int_to_display, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            int_to_display
        } else {
            Default::default()
        };
        let led_mask_: Card32 = if cond0 == FeedbackClass::Led {
            let (led_mask_, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_mask_
        } else {
            Default::default()
        };
        let led_values_: Card32 = if cond0 == FeedbackClass::Led {
            let (led_values_, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            led_values_
        } else {
            Default::default()
        };
        let percent: Int8 = if cond0 == FeedbackClass::Bell {
            let (percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
            index += sz;
            percent
        } else {
            Default::default()
        };
        index += 3;
        let pitch: Int16 = if cond0 == FeedbackClass::Bell {
            let (pitch, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
            index += sz;
            pitch
        } else {
            Default::default()
        };
        let duration: Int16 = if cond0 == FeedbackClass::Bell {
            let (duration, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
            index += sz;
            duration
        } else {
            Default::default()
        };
        Some((
            FeedbackCtl {
                class_id: class_id,
                feedback_id: feedback_id,
                len: len,
                key: key,
                auto_repeat_mode: auto_repeat_mode,
                key_click_percent: key_click_percent,
                bell_percent: bell_percent,
                bell_pitch: bell_pitch,
                bell_duration: bell_duration,
                led_mask: led_mask,
                led_values: led_values,
                num: num,
                denom: denom,
                threshold: threshold,
                keysyms: keysyms,
                int_to_display: int_to_display,
                led_mask_: led_mask_,
                led_values_: led_values_,
                percent: percent,
                pitch: pitch,
                duration: duration,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.feedback_id.size()
            + self.len.size()
            + self.key.size()
            + self.auto_repeat_mode.size()
            + self.key_click_percent.size()
            + self.bell_percent.size()
            + self.bell_pitch.size()
            + self.bell_duration.size()
            + self.led_mask.size()
            + self.led_values.size()
            + 2
            + self.num.size()
            + self.denom.size()
            + self.threshold.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
            + self.int_to_display.size()
            + self.led_mask_.size()
            + self.led_values_.size()
            + self.percent.size()
            + 3
            + self.pitch.size()
            + self.duration.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeFeedbackControlRequest<'x> {
    pub req_type: u8,
    pub length: u16,
    pub mask: ChangeFeedbackControlMask,
    pub device_id: Card8,
    pub feedback_id: Card8,
    pub feedback: FeedbackCtl<'x>,
}
impl<'x> ChangeFeedbackControlRequest {}
impl<'x> AsByteSequence for ChangeFeedbackControlRequest<'x> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.feedback.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeFeedbackControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (ChangeFeedbackControlMask, usize) =
            <ChangeFeedbackControlMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (feedback, sz): (FeedbackCtl<'x>, usize) =
            <FeedbackCtl<'x>>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ChangeFeedbackControlRequest {
                req_type: req_type,
                length: length,
                mask: mask,
                device_id: device_id,
                feedback_id: feedback_id,
                feedback: feedback,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.mask.size()
            + self.device_id.size()
            + self.feedback_id.size()
            + 2
            + self.feedback.size()
    }
}
impl Request for ChangeFeedbackControlRequest {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChangeFeedbackControlMask {
    pub inner: u32,
}
impl ChangeFeedbackControlMask {
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
    pub fn percent(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_percent(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn pitch(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_pitch(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn duration(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_duration(&mut self, val: bool) -> &mut Self {
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
        percent: bool,
        pitch: bool,
        duration: bool,
        led: bool,
        led_mode: bool,
        key: bool,
        auto_repeat_mode: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if key_click_percent {
            inner |= 1 << 0;
        }
        if percent {
            inner |= 1 << 1;
        }
        if pitch {
            inner |= 1 << 2;
        }
        if duration {
            inner |= 1 << 3;
        }
        if led {
            inner |= 1 << 4;
        }
        if led_mode {
            inner |= 1 << 5;
        }
        if key {
            inner |= 1 << 6;
        }
        if auto_repeat_mode {
            inner |= 1 << 7;
        }
        ChangeFeedbackControlMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const KEY_CLICK_PERCENT: Self = Self { inner: 1 };
    pub const PERCENT: Self = Self { inner: 2 };
    pub const PITCH: Self = Self { inner: 4 };
    pub const DURATION: Self = Self { inner: 8 };
    pub const LED: Self = Self { inner: 16 };
    pub const LED_MODE: Self = Self { inner: 32 };
    pub const KEY: Self = Self { inner: 64 };
    pub const AUTO_REPEAT_MODE: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for ChangeFeedbackControlMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((ChangeFeedbackControlMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ChangeFeedbackControlMask {
    type Output = ChangeFeedbackControlMask;
    #[inline]
    fn not(self) -> ChangeFeedbackControlMask {
        ChangeFeedbackControlMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ChangeFeedbackControlMask {
    type Output = ChangeFeedbackControlMask;
    #[inline]
    fn bitand(self, rhs: ChangeFeedbackControlMask) -> ChangeFeedbackControlMask {
        ChangeFeedbackControlMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ChangeFeedbackControlMask {
    type Output = ChangeFeedbackControlMask;
    #[inline]
    fn bitor(self, rhs: ChangeFeedbackControlMask) -> ChangeFeedbackControlMask {
        ChangeFeedbackControlMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ChangeFeedbackControlMask {
    type Output = ChangeFeedbackControlMask;
    #[inline]
    fn bitxor(self, rhs: ChangeFeedbackControlMask) -> ChangeFeedbackControlMask {
        ChangeFeedbackControlMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceKeyMappingRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
    pub first_keycode: KeyCode,
    pub count: Card8,
}
impl GetDeviceKeyMappingRequest {}
impl AsByteSequence for GetDeviceKeyMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.first_keycode.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceKeyMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_keycode, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            GetDeviceKeyMappingRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
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
            + self.device_id.size()
            + self.first_keycode.size()
            + self.count.size()
            + 1
    }
}
impl Request for GetDeviceKeyMappingRequest {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceKeyMappingReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceKeyMappingReply<'y> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub keysyms_per_keycode: Card8,
    pub keysyms: Cow<'y, [Keysym]>,
}
impl<'y> GetDeviceKeyMappingReply {}
impl<'y> AsByteSequence for GetDeviceKeyMappingReply<'y> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.keysyms_per_keycode.as_bytes(&mut bytes[index..]);
        index += 23;
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceKeyMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms_per_keycode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        let (keysyms, block_len): (Cow<'static, [Keysym]>, usize) =
            vector_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        Some((
            GetDeviceKeyMappingReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                keysyms_per_keycode: keysyms_per_keycode,
                keysyms: keysyms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.keysyms_per_keycode.size()
            + 23
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeDeviceKeyMappingRequest<'z> {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
    pub first_keycode: KeyCode,
    pub keysyms_per_keycode: Card8,
    pub keycode_count: Card8,
    pub keysyms: Cow<'z, [Keysym]>,
}
impl<'z> ChangeDeviceKeyMappingRequest {}
impl<'z> AsByteSequence for ChangeDeviceKeyMappingRequest<'z> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.first_keycode.as_bytes(&mut bytes[index..]);
        index += self.keysyms_per_keycode.as_bytes(&mut bytes[index..]);
        index += self.keycode_count.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keysyms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeDeviceKeyMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_keycode, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms_per_keycode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycode_count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keysyms, block_len): (Cow<'static, [Keysym]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((keycode_count as usize) * (keysyms_per_keycode as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        Some((
            ChangeDeviceKeyMappingRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
                first_keycode: first_keycode,
                keysyms_per_keycode: keysyms_per_keycode,
                keycode_count: keycode_count,
                keysyms: keysyms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.device_id.size()
            + self.first_keycode.size()
            + self.keysyms_per_keycode.size()
            + self.keycode_count.size()
            + {
                let block_len: usize = self.keysyms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
    }
}
impl Request for ChangeDeviceKeyMappingRequest {
    const OPCODE: u8 = 25;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceModifierMappingRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl GetDeviceModifierMappingRequest {}
impl AsByteSequence for GetDeviceModifierMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceModifierMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GetDeviceModifierMappingRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for GetDeviceModifierMappingRequest {
    const OPCODE: u8 = 26;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceModifierMappingReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceModifierMappingReply<'ab> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub keycodes_per_modifier: Card8,
    pub keymaps: Cow<'ab, [Card8]>,
}
impl<'ab> GetDeviceModifierMappingReply {}
impl<'ab> AsByteSequence for GetDeviceModifierMappingReply<'ab> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.keycodes_per_modifier.as_bytes(&mut bytes[index..]);
        index += 23;
        let block_len: usize = vector_as_bytes(&self.keymaps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceModifierMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycodes_per_modifier, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        let (keymaps, block_len): (Cow<'static, [Card8]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((keycodes_per_modifier as usize) * (8)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            GetDeviceModifierMappingReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                keycodes_per_modifier: keycodes_per_modifier,
                keymaps: keymaps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.keycodes_per_modifier.size()
            + 23
            + {
                let block_len: usize = self.keymaps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceModifierMappingRequest<'bb> {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
    pub keycodes_per_modifier: Card8,
    pub keymaps: Cow<'bb, [Card8]>,
}
impl<'bb> SetDeviceModifierMappingRequest {}
impl<'bb> AsByteSequence for SetDeviceModifierMappingRequest<'bb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.keycodes_per_modifier.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.keymaps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceModifierMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycodes_per_modifier, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (keymaps, block_len): (Cow<'static, [Card8]>, usize) = vector_from_bytes(
            &bytes[index..],
            ((keycodes_per_modifier as usize) * (8)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            SetDeviceModifierMappingRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
                keycodes_per_modifier: keycodes_per_modifier,
                keymaps: keymaps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.device_id.size()
            + self.keycodes_per_modifier.size()
            + 2
            + {
                let block_len: usize = self.keymaps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for SetDeviceModifierMappingRequest {
    const OPCODE: u8 = 27;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetDeviceModifierMappingReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceModifierMappingReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: MappingStatus,
}
impl SetDeviceModifierMappingReply {}
impl AsByteSequence for SetDeviceModifierMappingReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceModifierMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (MappingStatus, usize) = <MappingStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            SetDeviceModifierMappingReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceButtonMappingRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl GetDeviceButtonMappingRequest {}
impl AsByteSequence for GetDeviceButtonMappingRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceButtonMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GetDeviceButtonMappingRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for GetDeviceButtonMappingRequest {
    const OPCODE: u8 = 28;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceButtonMappingReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceButtonMappingReply<'cb> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub map: Cow<'cb, [Card8]>,
}
impl<'cb> GetDeviceButtonMappingReply {}
impl<'cb> AsByteSequence for GetDeviceButtonMappingReply<'cb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.map.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 23;
        let block_len: usize = vector_as_bytes(&self.map, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceButtonMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        let (map, block_len): (Cow<'static, [Card8]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            GetDeviceButtonMappingReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
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
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card8>()
            + 23
            + {
                let block_len: usize = self.map.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceButtonMappingRequest<'db> {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
    pub map: Cow<'db, [Card8]>,
}
impl<'db> SetDeviceButtonMappingRequest {}
impl<'db> AsByteSequence for SetDeviceButtonMappingRequest<'db> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += (self.map.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.map, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceButtonMappingRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (map, block_len): (Cow<'static, [Card8]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        Some((
            SetDeviceButtonMappingRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
                map: map,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.device_id.size()
            + ::core::mem::size_of::<Card8>()
            + 2
            + {
                let block_len: usize = self.map.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
    }
}
impl Request for SetDeviceButtonMappingRequest {
    const OPCODE: u8 = 29;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetDeviceButtonMappingReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceButtonMappingReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: MappingStatus,
}
impl SetDeviceButtonMappingReply {}
impl AsByteSequence for SetDeviceButtonMappingReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceButtonMappingReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (MappingStatus, usize) = <MappingStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            SetDeviceButtonMappingReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct KeyState {
    pub class_id: InputClass,
    pub len: Card8,
    pub num_keys: Card8,
    pub keys: [Card8; 32],
}
impl KeyState {}
impl AsByteSequence for KeyState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.num_keys.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.keys.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyState from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (keys, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KeyState {
                class_id: class_id,
                len: len,
                num_keys: num_keys,
                keys: keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size() + self.len.size() + self.num_keys.size() + 1 + self.keys.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ButtonState {
    pub class_id: InputClass,
    pub len: Card8,
    pub num_buttons: Card8,
    pub buttons: [Card8; 32],
}
impl ButtonState {}
impl AsByteSequence for ButtonState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.num_buttons.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.buttons.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ButtonState from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_buttons, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (buttons, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ButtonState {
                class_id: class_id,
                len: len,
                num_buttons: num_buttons,
                buttons: buttons,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size() + self.len.size() + self.num_buttons.size() + 1 + self.buttons.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ValuatorState<'eb> {
    pub class_id: InputClass,
    pub len: Card8,
    pub mode: ValuatorStateModeMask,
    pub valuators: Cow<'eb, [Int32]>,
}
impl<'eb> ValuatorState {}
impl<'eb> AsByteSequence for ValuatorState<'eb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += (self.valuators.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.valuators, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ValuatorState from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (ValuatorStateModeMask, usize) =
            <ValuatorStateModeMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (valuators, block_len): (Cow<'static, [Int32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            ValuatorState {
                class_id: class_id,
                len: len,
                mode: mode,
                valuators: valuators,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.len.size()
            + ::core::mem::size_of::<Card8>()
            + self.mode.size()
            + {
                let block_len: usize = self.valuators.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ValuatorStateModeMask {
    pub inner: u8,
}
impl ValuatorStateModeMask {
    #[inline]
    pub fn device_mode_absolute(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_device_mode_absolute(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn out_of_proximity(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_out_of_proximity(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(device_mode_absolute: bool, out_of_proximity: bool) -> Self {
        let mut inner: u8 = 0;
        if device_mode_absolute {
            inner |= 1 << 0;
        }
        if out_of_proximity {
            inner |= 1 << 1;
        }
        ValuatorStateModeMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const DEVICE_MODE_ABSOLUTE: Self = Self { inner: 1 };
    pub const OUT_OF_PROXIMITY: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for ValuatorStateModeMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((ValuatorStateModeMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ValuatorStateModeMask {
    type Output = ValuatorStateModeMask;
    #[inline]
    fn not(self) -> ValuatorStateModeMask {
        ValuatorStateModeMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ValuatorStateModeMask {
    type Output = ValuatorStateModeMask;
    #[inline]
    fn bitand(self, rhs: ValuatorStateModeMask) -> ValuatorStateModeMask {
        ValuatorStateModeMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ValuatorStateModeMask {
    type Output = ValuatorStateModeMask;
    #[inline]
    fn bitor(self, rhs: ValuatorStateModeMask) -> ValuatorStateModeMask {
        ValuatorStateModeMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ValuatorStateModeMask {
    type Output = ValuatorStateModeMask;
    #[inline]
    fn bitxor(self, rhs: ValuatorStateModeMask) -> ValuatorStateModeMask {
        ValuatorStateModeMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct InputState<'fb> {
    pub class_id: InputClass,
    pub len: Card8,
    pub num_keys: Card8,
    pub keys: [Card8; 32],
    pub num_buttons: Card8,
    pub buttons: [Card8; 32],
    pub mode: ValuatorStateModeMask,
    pub valuators: Cow<'fb, [Int32]>,
}
impl<'fb> InputState {}
impl<'fb> AsByteSequence for InputState<'fb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.class_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        let cond0 = (self.class_id);
        if cond0 == InputClass::Key {
            index += self.num_keys.as_bytes(&mut bytes[index..]);
        }
        index += 1;
        index += self.keys.as_bytes(&mut bytes[index..]);
        if cond0 == InputClass::Button {
            index += self.num_buttons.as_bytes(&mut bytes[index..]);
        }
        index += self.buttons.as_bytes(&mut bytes[index..]);
        index += (self.valuators.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0 == InputClass::Valuator {
            index += self.mode.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.valuators, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing InputState from byte buffer");
        let (class_id, sz): (InputClass, usize) = <InputClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (class_id);
        let num_keys: Card8 = if cond0 == InputClass::Key {
            let (num_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            num_keys
        } else {
            Default::default()
        };
        index += 1;
        let (keys, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        let num_buttons: Card8 = if cond0 == InputClass::Button {
            let (num_buttons, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            num_buttons
        } else {
            Default::default()
        };
        let (buttons, sz): ([Card8; 32], usize) = <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let mode: ValuatorStateModeMask = if cond0 == InputClass::Valuator {
            let (mode, sz): (ValuatorStateModeMask, usize) =
                <ValuatorStateModeMask>::from_bytes(&bytes[index..])?;
            index += sz;
            mode
        } else {
            Default::default()
        };
        let (valuators, block_len): (Cow<'static, [Int32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            InputState {
                class_id: class_id,
                len: len,
                num_keys: num_keys,
                keys: keys,
                num_buttons: num_buttons,
                buttons: buttons,
                mode: mode,
                valuators: valuators,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.class_id.size()
            + self.len.size()
            + self.num_keys.size()
            + 1
            + self.keys.size()
            + self.num_buttons.size()
            + self.buttons.size()
            + ::core::mem::size_of::<Card8>()
            + self.mode.size()
            + {
                let block_len: usize = self.valuators.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryDeviceStateRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl QueryDeviceStateRequest {}
impl AsByteSequence for QueryDeviceStateRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryDeviceStateRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            QueryDeviceStateRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for QueryDeviceStateRequest {
    const OPCODE: u8 = 30;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = QueryDeviceStateReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct QueryDeviceStateReply<'hb, 'gb> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub classes: Cow<'hb, [InputState<'gb>]>,
}
impl<'hb, 'gb> QueryDeviceStateReply {}
impl<'hb, 'gb> AsByteSequence for QueryDeviceStateReply<'hb, 'gb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 23;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<InputState<'gb>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing QueryDeviceStateReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        let (classes, block_len): (Cow<'static, [InputState<'gb>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<InputState<'gb>>());
        Some((
            QueryDeviceStateReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card8>()
            + 23
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<InputState<'gb>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceBellRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
    pub feedback_id: Card8,
    pub feedback_class: Card8,
    pub percent: Int8,
}
impl DeviceBellRequest {}
impl AsByteSequence for DeviceBellRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_id.as_bytes(&mut bytes[index..]);
        index += self.feedback_class.as_bytes(&mut bytes[index..]);
        index += self.percent.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceBellRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (feedback_class, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceBellRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
                feedback_id: feedback_id,
                feedback_class: feedback_class,
                percent: percent,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.device_id.size()
            + self.feedback_id.size()
            + self.feedback_class.size()
            + self.percent.size()
    }
}
impl Request for DeviceBellRequest {
    const OPCODE: u8 = 32;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceValuatorsRequest<'ib> {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
    pub first_valuator: Card8,
    pub valuators: Cow<'ib, [Int32]>,
}
impl<'ib> SetDeviceValuatorsRequest {}
impl<'ib> AsByteSequence for SetDeviceValuatorsRequest<'ib> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.first_valuator.as_bytes(&mut bytes[index..]);
        index += (self.valuators.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 1;
        let block_len: usize = vector_as_bytes(&self.valuators, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceValuatorsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_valuator, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (valuators, block_len): (Cow<'static, [Int32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Int32>());
        Some((
            SetDeviceValuatorsRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
                first_valuator: first_valuator,
                valuators: valuators,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.device_id.size()
            + self.first_valuator.size()
            + ::core::mem::size_of::<Card8>()
            + 1
            + {
                let block_len: usize = self.valuators.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Int32>());
                block_len + pad
            }
    }
}
impl Request for SetDeviceValuatorsRequest {
    const OPCODE: u8 = 33;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetDeviceValuatorsReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SetDeviceValuatorsReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: GrabStatus,
}
impl SetDeviceValuatorsReply {}
impl AsByteSequence for SetDeviceValuatorsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceValuatorsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            SetDeviceValuatorsReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceResolutionState<'jb, 'kb, 'lb> {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub num_valuators: Card32,
    pub resolution_values: Cow<'jb, [Card32]>,
    pub resolution_min: Cow<'kb, [Card32]>,
    pub resolution_max: Cow<'lb, [Card32]>,
}
impl<'jb, 'kb, 'lb> DeviceResolutionState {}
impl<'jb, 'kb, 'lb> AsByteSequence for DeviceResolutionState<'jb, 'kb, 'lb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.num_valuators.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.resolution_values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.resolution_min, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.resolution_max, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceResolutionState from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_valuators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resolution_values, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_valuators as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (resolution_min, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_valuators as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (resolution_max, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_valuators as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            DeviceResolutionState {
                control_id: control_id,
                len: len,
                num_valuators: num_valuators,
                resolution_values: resolution_values,
                resolution_min: resolution_min,
                resolution_max: resolution_max,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.num_valuators.size()
            + {
                let block_len: usize = self.resolution_values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.resolution_min.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.resolution_max.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DeviceControl {
    Resolution = 1,
    AbsCalib = 2,
    Core = 3,
    Enable = 4,
    AbsArea = 5,
}
impl AsByteSequence for DeviceControl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::Resolution, sz)),
            2 => Some((Self::AbsCalib, sz)),
            3 => Some((Self::Core, sz)),
            4 => Some((Self::Enable, sz)),
            5 => Some((Self::AbsArea, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for DeviceControl {
    #[inline]
    fn default() -> DeviceControl {
        DeviceControl::Resolution
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceAbsCalibState {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub min_x: Int32,
    pub max_x: Int32,
    pub min_y: Int32,
    pub max_y: Int32,
    pub flip_x: Card32,
    pub flip_y: Card32,
    pub rotation: Card32,
    pub button_threshold: Card32,
}
impl DeviceAbsCalibState {}
impl AsByteSequence for DeviceAbsCalibState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.min_x.as_bytes(&mut bytes[index..]);
        index += self.max_x.as_bytes(&mut bytes[index..]);
        index += self.min_y.as_bytes(&mut bytes[index..]);
        index += self.max_y.as_bytes(&mut bytes[index..]);
        index += self.flip_x.as_bytes(&mut bytes[index..]);
        index += self.flip_y.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += self.button_threshold.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceAbsCalibState from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flip_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flip_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_threshold, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceAbsCalibState {
                control_id: control_id,
                len: len,
                min_x: min_x,
                max_x: max_x,
                min_y: min_y,
                max_y: max_y,
                flip_x: flip_x,
                flip_y: flip_y,
                rotation: rotation,
                button_threshold: button_threshold,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.min_x.size()
            + self.max_x.size()
            + self.min_y.size()
            + self.max_y.size()
            + self.flip_x.size()
            + self.flip_y.size()
            + self.rotation.size()
            + self.button_threshold.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceAbsAreaState {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub offset_x: Card32,
    pub offset_y: Card32,
    pub width: Card32,
    pub height: Card32,
    pub screen: Card32,
    pub following: Card32,
}
impl DeviceAbsAreaState {}
impl AsByteSequence for DeviceAbsAreaState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.offset_x.as_bytes(&mut bytes[index..]);
        index += self.offset_y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.following.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceAbsAreaState from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (following, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceAbsAreaState {
                control_id: control_id,
                len: len,
                offset_x: offset_x,
                offset_y: offset_y,
                width: width,
                height: height,
                screen: screen,
                following: following,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.offset_x.size()
            + self.offset_y.size()
            + self.width.size()
            + self.height.size()
            + self.screen.size()
            + self.following.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceCoreState {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub status: Card8,
    pub iscore: Card8,
}
impl DeviceCoreState {}
impl AsByteSequence for DeviceCoreState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += self.iscore.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceCoreState from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (iscore, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            DeviceCoreState {
                control_id: control_id,
                len: len,
                status: status,
                iscore: iscore,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size() + self.len.size() + self.status.size() + self.iscore.size() + 2
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceEnableState {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub enable: Card8,
}
impl DeviceEnableState {}
impl AsByteSequence for DeviceEnableState {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.enable.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceEnableState from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enable, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            DeviceEnableState {
                control_id: control_id,
                len: len,
                enable: enable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size() + self.len.size() + self.enable.size() + 3
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceState<'mb, 'nb, 'ob> {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub num_valuators: Card32,
    pub resolution_values: Cow<'mb, [Card32]>,
    pub resolution_min: Cow<'nb, [Card32]>,
    pub resolution_max: Cow<'ob, [Card32]>,
    pub min_x: Int32,
    pub max_x: Int32,
    pub min_y: Int32,
    pub max_y: Int32,
    pub flip_x: Card32,
    pub flip_y: Card32,
    pub rotation: Card32,
    pub button_threshold: Card32,
    pub status: Card8,
    pub iscore: Card8,
    pub enable: Card8,
    pub offset_x: Card32,
    pub offset_y: Card32,
    pub width: Card32,
    pub height: Card32,
    pub screen: Card32,
    pub following: Card32,
}
impl<'mb, 'nb, 'ob> DeviceState {}
impl<'mb, 'nb, 'ob> AsByteSequence for DeviceState<'mb, 'nb, 'ob> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        let cond0 = (self.control_id);
        if cond0 == DeviceControl::Resolution {
            index += self.num_valuators.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.resolution_values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.resolution_min, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.resolution_max, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        if cond0 == DeviceControl::AbsCalib {
            index += self.min_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.max_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.min_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.max_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.flip_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.flip_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.rotation.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.button_threshold.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::Core {
            index += self.status.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::Core {
            index += self.iscore.as_bytes(&mut bytes[index..]);
        }
        index += 2;
        if cond0 == DeviceControl::Enable {
            index += self.enable.as_bytes(&mut bytes[index..]);
        }
        index += 3;
        if cond0 == DeviceControl::AbsArea {
            index += self.offset_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.offset_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.width.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.height.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.screen.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.following.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceState from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (control_id);
        let num_valuators: Card32 = if cond0 == DeviceControl::Resolution {
            let (num_valuators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            num_valuators
        } else {
            Default::default()
        };
        let (resolution_values, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_valuators as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (resolution_min, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_valuators as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (resolution_max, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_valuators as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let min_x: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (min_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            min_x
        } else {
            Default::default()
        };
        let max_x: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (max_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            max_x
        } else {
            Default::default()
        };
        let min_y: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (min_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            min_y
        } else {
            Default::default()
        };
        let max_y: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (max_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            max_y
        } else {
            Default::default()
        };
        let flip_x: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (flip_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            flip_x
        } else {
            Default::default()
        };
        let flip_y: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (flip_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            flip_y
        } else {
            Default::default()
        };
        let rotation: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (rotation, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            rotation
        } else {
            Default::default()
        };
        let button_threshold: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (button_threshold, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            button_threshold
        } else {
            Default::default()
        };
        let status: Card8 = if cond0 == DeviceControl::Core {
            let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            status
        } else {
            Default::default()
        };
        let iscore: Card8 = if cond0 == DeviceControl::Core {
            let (iscore, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            iscore
        } else {
            Default::default()
        };
        index += 2;
        let enable: Card8 = if cond0 == DeviceControl::Enable {
            let (enable, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            enable
        } else {
            Default::default()
        };
        index += 3;
        let offset_x: Card32 = if cond0 == DeviceControl::AbsArea {
            let (offset_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            offset_x
        } else {
            Default::default()
        };
        let offset_y: Card32 = if cond0 == DeviceControl::AbsArea {
            let (offset_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            offset_y
        } else {
            Default::default()
        };
        let width: Card32 = if cond0 == DeviceControl::AbsArea {
            let (width, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            width
        } else {
            Default::default()
        };
        let height: Card32 = if cond0 == DeviceControl::AbsArea {
            let (height, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            height
        } else {
            Default::default()
        };
        let screen: Card32 = if cond0 == DeviceControl::AbsArea {
            let (screen, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            screen
        } else {
            Default::default()
        };
        let following: Card32 = if cond0 == DeviceControl::AbsArea {
            let (following, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            following
        } else {
            Default::default()
        };
        Some((
            DeviceState {
                control_id: control_id,
                len: len,
                num_valuators: num_valuators,
                resolution_values: resolution_values,
                resolution_min: resolution_min,
                resolution_max: resolution_max,
                min_x: min_x,
                max_x: max_x,
                min_y: min_y,
                max_y: max_y,
                flip_x: flip_x,
                flip_y: flip_y,
                rotation: rotation,
                button_threshold: button_threshold,
                status: status,
                iscore: iscore,
                enable: enable,
                offset_x: offset_x,
                offset_y: offset_y,
                width: width,
                height: height,
                screen: screen,
                following: following,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.num_valuators.size()
            + {
                let block_len: usize = self.resolution_values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.resolution_min.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.resolution_max.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + self.min_x.size()
            + self.max_x.size()
            + self.min_y.size()
            + self.max_y.size()
            + self.flip_x.size()
            + self.flip_y.size()
            + self.rotation.size()
            + self.button_threshold.size()
            + self.status.size()
            + self.iscore.size()
            + 2
            + self.enable.size()
            + 3
            + self.offset_x.size()
            + self.offset_y.size()
            + self.width.size()
            + self.height.size()
            + self.screen.size()
            + self.following.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceControlRequest {
    pub req_type: u8,
    pub length: u16,
    pub control_id: DeviceControl,
    pub device_id: Card8,
}
impl GetDeviceControlRequest {}
impl AsByteSequence for GetDeviceControlRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            GetDeviceControlRequest {
                req_type: req_type,
                length: length,
                control_id: control_id,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.control_id.size()
            + self.device_id.size()
            + 1
    }
}
impl Request for GetDeviceControlRequest {
    const OPCODE: u8 = 34;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceControlReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDeviceControlReply<'pb, 'qb, 'rb> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: Card8,
    pub control: DeviceState<'pb, 'qb, 'rb>,
}
impl<'pb, 'qb, 'rb> GetDeviceControlReply {}
impl<'pb, 'qb, 'rb> AsByteSequence for GetDeviceControlReply<'pb, 'qb, 'rb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index += self.control.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceControlReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        let (control, sz): (DeviceState<'pb, 'qb, 'rb>, usize) =
            <DeviceState<'pb, 'qb, 'rb>>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDeviceControlReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
                control: control,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
            + self.control.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceResolutionCtl<'sb> {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub first_valuator: Card8,
    pub resolution_values: Cow<'sb, [Card32]>,
}
impl<'sb> DeviceResolutionCtl {}
impl<'sb> AsByteSequence for DeviceResolutionCtl<'sb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.first_valuator.as_bytes(&mut bytes[index..]);
        index += (self.resolution_values.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.resolution_values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceResolutionCtl from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_valuator, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (resolution_values, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            DeviceResolutionCtl {
                control_id: control_id,
                len: len,
                first_valuator: first_valuator,
                resolution_values: resolution_values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.first_valuator.size()
            + ::core::mem::size_of::<Card8>()
            + 2
            + {
                let block_len: usize = self.resolution_values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceAbsCalibCtl {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub min_x: Int32,
    pub max_x: Int32,
    pub min_y: Int32,
    pub max_y: Int32,
    pub flip_x: Card32,
    pub flip_y: Card32,
    pub rotation: Card32,
    pub button_threshold: Card32,
}
impl DeviceAbsCalibCtl {}
impl AsByteSequence for DeviceAbsCalibCtl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.min_x.as_bytes(&mut bytes[index..]);
        index += self.max_x.as_bytes(&mut bytes[index..]);
        index += self.min_y.as_bytes(&mut bytes[index..]);
        index += self.max_y.as_bytes(&mut bytes[index..]);
        index += self.flip_x.as_bytes(&mut bytes[index..]);
        index += self.flip_y.as_bytes(&mut bytes[index..]);
        index += self.rotation.as_bytes(&mut bytes[index..]);
        index += self.button_threshold.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceAbsCalibCtl from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flip_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flip_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (rotation, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_threshold, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceAbsCalibCtl {
                control_id: control_id,
                len: len,
                min_x: min_x,
                max_x: max_x,
                min_y: min_y,
                max_y: max_y,
                flip_x: flip_x,
                flip_y: flip_y,
                rotation: rotation,
                button_threshold: button_threshold,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.min_x.size()
            + self.max_x.size()
            + self.min_y.size()
            + self.max_y.size()
            + self.flip_x.size()
            + self.flip_y.size()
            + self.rotation.size()
            + self.button_threshold.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceAbsAreaCtrl {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub offset_x: Card32,
    pub offset_y: Card32,
    pub width: Int32,
    pub height: Int32,
    pub screen: Int32,
    pub following: Card32,
}
impl DeviceAbsAreaCtrl {}
impl AsByteSequence for DeviceAbsAreaCtrl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.offset_x.as_bytes(&mut bytes[index..]);
        index += self.offset_y.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += self.height.as_bytes(&mut bytes[index..]);
        index += self.screen.as_bytes(&mut bytes[index..]);
        index += self.following.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceAbsAreaCtrl from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (height, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (screen, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (following, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceAbsAreaCtrl {
                control_id: control_id,
                len: len,
                offset_x: offset_x,
                offset_y: offset_y,
                width: width,
                height: height,
                screen: screen,
                following: following,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.offset_x.size()
            + self.offset_y.size()
            + self.width.size()
            + self.height.size()
            + self.screen.size()
            + self.following.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceCoreCtrl {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub status: Card8,
}
impl DeviceCoreCtrl {}
impl AsByteSequence for DeviceCoreCtrl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceCoreCtrl from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            DeviceCoreCtrl {
                control_id: control_id,
                len: len,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size() + self.len.size() + self.status.size() + 3
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceEnableCtrl {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub enable: Card8,
}
impl DeviceEnableCtrl {}
impl AsByteSequence for DeviceEnableCtrl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.enable.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceEnableCtrl from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enable, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            DeviceEnableCtrl {
                control_id: control_id,
                len: len,
                enable: enable,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size() + self.len.size() + self.enable.size() + 3
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceCtl<'tb> {
    pub control_id: DeviceControl,
    pub len: Card16,
    pub first_valuator: Card8,
    pub resolution_values: Cow<'tb, [Card32]>,
    pub min_x: Int32,
    pub max_x: Int32,
    pub min_y: Int32,
    pub max_y: Int32,
    pub flip_x: Card32,
    pub flip_y: Card32,
    pub rotation: Card32,
    pub button_threshold: Card32,
    pub status: Card8,
    pub enable: Card8,
    pub offset_x: Card32,
    pub offset_y: Card32,
    pub width: Int32,
    pub height: Int32,
    pub screen: Int32,
    pub following: Card32,
}
impl<'tb> DeviceCtl {}
impl<'tb> AsByteSequence for DeviceCtl<'tb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        let cond0 = (self.control_id);
        if cond0 == DeviceControl::Resolution {
            index += self.first_valuator.as_bytes(&mut bytes[index..]);
        }
        index += (self.resolution_values.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.resolution_values, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        if cond0 == DeviceControl::AbsCalib {
            index += self.min_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.max_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.min_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.max_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.flip_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.flip_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.rotation.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsCalib {
            index += self.button_threshold.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::Core {
            index += self.status.as_bytes(&mut bytes[index..]);
        }
        index += 3;
        if cond0 == DeviceControl::Enable {
            index += self.enable.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.offset_x.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.offset_y.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.width.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.height.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.screen.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceControl::AbsArea {
            index += self.following.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceCtl from byte buffer");
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (control_id);
        let first_valuator: Card8 = if cond0 == DeviceControl::Resolution {
            let (first_valuator, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            first_valuator
        } else {
            Default::default()
        };
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (resolution_values, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let min_x: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (min_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            min_x
        } else {
            Default::default()
        };
        let max_x: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (max_x, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            max_x
        } else {
            Default::default()
        };
        let min_y: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (min_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            min_y
        } else {
            Default::default()
        };
        let max_y: Int32 = if cond0 == DeviceControl::AbsCalib {
            let (max_y, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            max_y
        } else {
            Default::default()
        };
        let flip_x: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (flip_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            flip_x
        } else {
            Default::default()
        };
        let flip_y: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (flip_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            flip_y
        } else {
            Default::default()
        };
        let rotation: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (rotation, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            rotation
        } else {
            Default::default()
        };
        let button_threshold: Card32 = if cond0 == DeviceControl::AbsCalib {
            let (button_threshold, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            button_threshold
        } else {
            Default::default()
        };
        let status: Card8 = if cond0 == DeviceControl::Core {
            let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            status
        } else {
            Default::default()
        };
        index += 3;
        let enable: Card8 = if cond0 == DeviceControl::Enable {
            let (enable, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            enable
        } else {
            Default::default()
        };
        let offset_x: Card32 = if cond0 == DeviceControl::AbsArea {
            let (offset_x, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            offset_x
        } else {
            Default::default()
        };
        let offset_y: Card32 = if cond0 == DeviceControl::AbsArea {
            let (offset_y, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            offset_y
        } else {
            Default::default()
        };
        let width: Int32 = if cond0 == DeviceControl::AbsArea {
            let (width, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            width
        } else {
            Default::default()
        };
        let height: Int32 = if cond0 == DeviceControl::AbsArea {
            let (height, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            height
        } else {
            Default::default()
        };
        let screen: Int32 = if cond0 == DeviceControl::AbsArea {
            let (screen, sz): (Int32, usize) = <Int32>::from_bytes(&bytes[index..])?;
            index += sz;
            screen
        } else {
            Default::default()
        };
        let following: Card32 = if cond0 == DeviceControl::AbsArea {
            let (following, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            following
        } else {
            Default::default()
        };
        Some((
            DeviceCtl {
                control_id: control_id,
                len: len,
                first_valuator: first_valuator,
                resolution_values: resolution_values,
                min_x: min_x,
                max_x: max_x,
                min_y: min_y,
                max_y: max_y,
                flip_x: flip_x,
                flip_y: flip_y,
                rotation: rotation,
                button_threshold: button_threshold,
                status: status,
                enable: enable,
                offset_x: offset_x,
                offset_y: offset_y,
                width: width,
                height: height,
                screen: screen,
                following: following,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.control_id.size()
            + self.len.size()
            + self.first_valuator.size()
            + ::core::mem::size_of::<Card8>()
            + 2
            + {
                let block_len: usize = self.resolution_values.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + self.min_x.size()
            + self.max_x.size()
            + self.min_y.size()
            + self.max_y.size()
            + self.flip_x.size()
            + self.flip_y.size()
            + self.rotation.size()
            + self.button_threshold.size()
            + self.status.size()
            + 3
            + self.enable.size()
            + self.offset_x.size()
            + self.offset_y.size()
            + self.width.size()
            + self.height.size()
            + self.screen.size()
            + self.following.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeDeviceControlRequest<'ub> {
    pub req_type: u8,
    pub length: u16,
    pub control_id: DeviceControl,
    pub device_id: Card8,
    pub control: DeviceCtl<'ub>,
}
impl<'ub> ChangeDeviceControlRequest {}
impl<'ub> AsByteSequence for ChangeDeviceControlRequest<'ub> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.control_id.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.control.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeDeviceControlRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (control_id, sz): (DeviceControl, usize) =
            <DeviceControl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (control, sz): (DeviceCtl<'ub>, usize) = <DeviceCtl<'ub>>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ChangeDeviceControlRequest {
                req_type: req_type,
                length: length,
                control_id: control_id,
                device_id: device_id,
                control: control,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.control_id.size()
            + self.device_id.size()
            + 1
            + self.control.size()
    }
}
impl Request for ChangeDeviceControlRequest {
    const OPCODE: u8 = 35;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ChangeDeviceControlReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeDeviceControlReply {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub status: Card8,
}
impl ChangeDeviceControlReply {}
impl AsByteSequence for ChangeDeviceControlReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeDeviceControlReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            ChangeDeviceControlReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.status.size()
            + 23
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListDevicePropertiesRequest {
    pub req_type: u8,
    pub length: u16,
    pub device_id: Card8,
}
impl ListDevicePropertiesRequest {}
impl AsByteSequence for ListDevicePropertiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListDevicePropertiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            ListDevicePropertiesRequest {
                req_type: req_type,
                length: length,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.device_id.size() + 3
    }
}
impl Request for ListDevicePropertiesRequest {
    const OPCODE: u8 = 36;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListDevicePropertiesReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ListDevicePropertiesReply<'vb> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub atoms: Cow<'vb, [Atom]>,
}
impl<'vb> ListDevicePropertiesReply {}
impl<'vb> AsByteSequence for ListDevicePropertiesReply<'vb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
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
        log::trace!("Deserializing ListDevicePropertiesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 22;
        let (atoms, block_len): (Cow<'static, [Atom]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            ListDevicePropertiesReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
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
            + self.xi_reply_type.size()
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
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeDevicePropertyRequest<'wb, 'xb, 'yb> {
    pub req_type: u8,
    pub length: u16,
    pub property: Atom,
    pub ty: Atom,
    pub device_id: Card8,
    pub format: PropertyFormat,
    pub mode: PropMode,
    pub num_items: Card32,
    pub data8: Cow<'wb, [Card8]>,
    pub data16: Cow<'xb, [Card16]>,
    pub data32: Cow<'yb, [Card32]>,
}
impl<'wb, 'xb, 'yb> ChangeDevicePropertyRequest {}
impl<'wb, 'xb, 'yb> AsByteSequence for ChangeDevicePropertyRequest<'wb, 'xb, 'yb> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.num_items.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data8, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.data16, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.data32, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeDevicePropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (PropertyFormat, usize) = <PropertyFormat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (PropMode, usize) = <PropMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (num_items, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data8, block_len): (Cow<'static, [Card8]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (data16, block_len): (Cow<'static, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (data32, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            ChangeDevicePropertyRequest {
                req_type: req_type,
                length: length,
                property: property,
                ty: ty,
                device_id: device_id,
                format: format,
                mode: mode,
                num_items: num_items,
                data8: data8,
                data16: data16,
                data32: data32,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.property.size()
            + self.ty.size()
            + self.device_id.size()
            + self.format.size()
            + self.mode.size()
            + 1
            + self.num_items.size()
            + {
                let block_len: usize = self.data8.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.data16.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.data32.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for ChangeDevicePropertyRequest {
    const OPCODE: u8 = 37;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum PropertyFormat {
    EightBits = 8,
    SixteenBits = 16,
    ThirtyTwoBits = 32,
}
impl AsByteSequence for PropertyFormat {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            8 => Some((Self::EightBits, sz)),
            16 => Some((Self::SixteenBits, sz)),
            32 => Some((Self::ThirtyTwoBits, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for PropertyFormat {
    #[inline]
    fn default() -> PropertyFormat {
        PropertyFormat::EightBits
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeleteDevicePropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub property: Atom,
    pub device_id: Card8,
}
impl DeleteDevicePropertyRequest {}
impl AsByteSequence for DeleteDevicePropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeleteDevicePropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            DeleteDevicePropertyRequest {
                req_type: req_type,
                length: length,
                property: property,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.property.size()
            + self.device_id.size()
            + 3
    }
}
impl Request for DeleteDevicePropertyRequest {
    const OPCODE: u8 = 38;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDevicePropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub property: Atom,
    pub ty: Atom,
    pub offset: Card32,
    pub len: Card32,
    pub device_id: Card8,
    pub delete: bool,
}
impl GetDevicePropertyRequest {}
impl AsByteSequence for GetDevicePropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.offset.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.delete.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDevicePropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delete, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetDevicePropertyRequest {
                req_type: req_type,
                length: length,
                property: property,
                ty: ty,
                offset: offset,
                len: len,
                device_id: device_id,
                delete: delete,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.property.size()
            + self.ty.size()
            + self.offset.size()
            + self.len.size()
            + self.device_id.size()
            + self.delete.size()
            + 2
    }
}
impl Request for GetDevicePropertyRequest {
    const OPCODE: u8 = 39;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDevicePropertyReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GetDevicePropertyReply<'zb, 'ac, 'bc> {
    pub reply_type: u8,
    pub xi_reply_type: Card8,
    pub sequence: u16,
    pub length: u32,
    pub ty: Atom,
    pub bytes_after: Card32,
    pub num_items: Card32,
    pub format: PropertyFormat,
    pub device_id: Card8,
    pub data8: Cow<'zb, [Card8]>,
    pub data16: Cow<'ac, [Card16]>,
    pub data32: Cow<'bc, [Card32]>,
}
impl<'zb, 'ac, 'bc> GetDevicePropertyReply {}
impl<'zb, 'ac, 'bc> AsByteSequence for GetDevicePropertyReply<'zb, 'ac, 'bc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.xi_reply_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.bytes_after.as_bytes(&mut bytes[index..]);
        index += self.num_items.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 10;
        let block_len: usize = vector_as_bytes(&self.data8, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.data16, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.data32, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDevicePropertyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xi_reply_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_after, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_items, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (PropertyFormat, usize) = <PropertyFormat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 10;
        let (data8, block_len): (Cow<'static, [Card8]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (data16, block_len): (Cow<'static, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (data32, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            GetDevicePropertyReply {
                reply_type: reply_type,
                xi_reply_type: xi_reply_type,
                sequence: sequence,
                length: length,
                ty: ty,
                bytes_after: bytes_after,
                num_items: num_items,
                format: format,
                device_id: device_id,
                data8: data8,
                data16: data16,
                data32: data32,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.xi_reply_type.size()
            + self.sequence.size()
            + self.length.size()
            + self.ty.size()
            + self.bytes_after.size()
            + self.num_items.size()
            + self.format.size()
            + self.device_id.size()
            + 10
            + {
                let block_len: usize = self.data8.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.data16.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.data32.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GroupInfo {
    pub base: Card8,
    pub latched: Card8,
    pub locked: Card8,
    pub effective: Card8,
}
impl GroupInfo {}
impl AsByteSequence for GroupInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.base.as_bytes(&mut bytes[index..]);
        index += self.latched.as_bytes(&mut bytes[index..]);
        index += self.locked.as_bytes(&mut bytes[index..]);
        index += self.effective.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GroupInfo from byte buffer");
        let (base, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (latched, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (locked, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (effective, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GroupInfo {
                base: base,
                latched: latched,
                locked: locked,
                effective: effective,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.base.size() + self.latched.size() + self.locked.size() + self.effective.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ModifierInfo {
    pub base: Card32,
    pub latched: Card32,
    pub locked: Card32,
    pub effective: Card32,
}
impl ModifierInfo {}
impl AsByteSequence for ModifierInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.base.as_bytes(&mut bytes[index..]);
        index += self.latched.as_bytes(&mut bytes[index..]);
        index += self.locked.as_bytes(&mut bytes[index..]);
        index += self.effective.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ModifierInfo from byte buffer");
        let (base, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (latched, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (locked, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (effective, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ModifierInfo {
                base: base,
                latched: latched,
                locked: locked,
                effective: effective,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.base.size() + self.latched.size() + self.locked.size() + self.effective.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiQueryPointerRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub deviceid: DeviceId,
}
impl XiQueryPointerRequest {}
impl AsByteSequence for XiQueryPointerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiQueryPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiQueryPointerRequest {
                req_type: req_type,
                length: length,
                window: window,
                deviceid: deviceid,
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
            + self.deviceid.size()
            + 2
    }
}
impl Request for XiQueryPointerRequest {
    const OPCODE: u8 = 40;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiQueryPointerReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiQueryPointerReply<'cc> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub root: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub win_x: Fp1616,
    pub win_y: Fp1616,
    pub same_screen: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Cow<'cc, [Card32]>,
}
impl<'cc> XiQueryPointerReply {}
impl<'cc> AsByteSequence for XiQueryPointerReply<'cc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.win_x.as_bytes(&mut bytes[index..]);
        index += self.win_y.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += (self.buttons.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.buttons, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiQueryPointerReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (win_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (win_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buttons, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            XiQueryPointerReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                root: root,
                child: child,
                root_x: root_x,
                root_y: root_y,
                win_x: win_x,
                win_y: win_y,
                same_screen: same_screen,
                mods: mods,
                group: group,
                buttons: buttons,
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
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.win_x.size()
            + self.win_y.size()
            + self.same_screen.size()
            + ::core::mem::size_of::<Card16>()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.buttons.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum Device {
    All = 0,
    AllMaster = 1,
}
impl AsByteSequence for Device {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::All, sz)),
            1 => Some((Self::AllMaster, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for Device {
    #[inline]
    fn default() -> Device {
        Device::All
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiWarpPointerRequest {
    pub req_type: u8,
    pub length: u16,
    pub src_win: Window,
    pub dst_win: Window,
    pub src_x: Fp1616,
    pub src_y: Fp1616,
    pub src_width: Card16,
    pub src_height: Card16,
    pub dst_x: Fp1616,
    pub dst_y: Fp1616,
    pub deviceid: DeviceId,
}
impl XiWarpPointerRequest {}
impl AsByteSequence for XiWarpPointerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.src_win.as_bytes(&mut bytes[index..]);
        index += self.dst_win.as_bytes(&mut bytes[index..]);
        index += self.src_x.as_bytes(&mut bytes[index..]);
        index += self.src_y.as_bytes(&mut bytes[index..]);
        index += self.src_width.as_bytes(&mut bytes[index..]);
        index += self.src_height.as_bytes(&mut bytes[index..]);
        index += self.dst_x.as_bytes(&mut bytes[index..]);
        index += self.dst_y.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiWarpPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_win, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_win, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_width, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (src_height, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dst_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiWarpPointerRequest {
                req_type: req_type,
                length: length,
                src_win: src_win,
                dst_win: dst_win,
                src_x: src_x,
                src_y: src_y,
                src_width: src_width,
                src_height: src_height,
                dst_x: dst_x,
                dst_y: dst_y,
                deviceid: deviceid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.src_win.size()
            + self.dst_win.size()
            + self.src_x.size()
            + self.src_y.size()
            + self.src_width.size()
            + self.src_height.size()
            + self.dst_x.size()
            + self.dst_y.size()
            + self.deviceid.size()
            + 2
    }
}
impl Request for XiWarpPointerRequest {
    const OPCODE: u8 = 41;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiChangeCursorRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub cursor: Cursor,
    pub deviceid: DeviceId,
}
impl XiChangeCursorRequest {}
impl AsByteSequence for XiChangeCursorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiChangeCursorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiChangeCursorRequest {
                req_type: req_type,
                length: length,
                window: window,
                cursor: cursor,
                deviceid: deviceid,
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
            + self.cursor.size()
            + self.deviceid.size()
            + 2
    }
}
impl Request for XiChangeCursorRequest {
    const OPCODE: u8 = 42;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AddMaster<'dc> {
    pub ty: HierarchyChangeType,
    pub len: Card16,
    pub send_core: bool,
    pub enable: bool,
    pub name: Cow<'dc, str>,
}
impl<'dc> AddMaster {}
impl<'dc> AsByteSequence for AddMaster<'dc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.send_core.as_bytes(&mut bytes[index..]);
        index += self.enable.as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AddMaster from byte buffer");
        let (ty, sz): (HierarchyChangeType, usize) =
            <HierarchyChangeType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (send_core, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enable, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        Some((
            AddMaster {
                ty: ty,
                len: len,
                send_core: send_core,
                enable: enable,
                name: name,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + ::core::mem::size_of::<Card16>()
            + self.send_core.size()
            + self.enable.size()
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum HierarchyChangeType {
    AddMaster = 1,
    RemoveMaster = 2,
    AttachSlave = 3,
    DetachSlave = 4,
}
impl AsByteSequence for HierarchyChangeType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::AddMaster, sz)),
            2 => Some((Self::RemoveMaster, sz)),
            3 => Some((Self::AttachSlave, sz)),
            4 => Some((Self::DetachSlave, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for HierarchyChangeType {
    #[inline]
    fn default() -> HierarchyChangeType {
        HierarchyChangeType::AddMaster
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RemoveMaster {
    pub ty: HierarchyChangeType,
    pub len: Card16,
    pub deviceid: DeviceId,
    pub return_mode: ChangeMode,
    pub return_pointer: DeviceId,
    pub return_keyboard: DeviceId,
}
impl RemoveMaster {}
impl AsByteSequence for RemoveMaster {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.return_mode.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.return_pointer.as_bytes(&mut bytes[index..]);
        index += self.return_keyboard.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RemoveMaster from byte buffer");
        let (ty, sz): (HierarchyChangeType, usize) =
            <HierarchyChangeType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (return_mode, sz): (ChangeMode, usize) = <ChangeMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (return_pointer, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (return_keyboard, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            RemoveMaster {
                ty: ty,
                len: len,
                deviceid: deviceid,
                return_mode: return_mode,
                return_pointer: return_pointer,
                return_keyboard: return_keyboard,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + self.deviceid.size()
            + self.return_mode.size()
            + 1
            + self.return_pointer.size()
            + self.return_keyboard.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ChangeMode {
    Attach = 1,
    Float = 2,
}
impl AsByteSequence for ChangeMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::Attach, sz)),
            2 => Some((Self::Float, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ChangeMode {
    #[inline]
    fn default() -> ChangeMode {
        ChangeMode::Attach
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct AttachSlave {
    pub ty: HierarchyChangeType,
    pub len: Card16,
    pub deviceid: DeviceId,
    pub master: DeviceId,
}
impl AttachSlave {}
impl AsByteSequence for AttachSlave {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.master.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AttachSlave from byte buffer");
        let (ty, sz): (HierarchyChangeType, usize) =
            <HierarchyChangeType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (master, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            AttachSlave {
                ty: ty,
                len: len,
                deviceid: deviceid,
                master: master,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.len.size() + self.deviceid.size() + self.master.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DetachSlave {
    pub ty: HierarchyChangeType,
    pub len: Card16,
    pub deviceid: DeviceId,
}
impl DetachSlave {}
impl AsByteSequence for DetachSlave {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DetachSlave from byte buffer");
        let (ty, sz): (HierarchyChangeType, usize) =
            <HierarchyChangeType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            DetachSlave {
                ty: ty,
                len: len,
                deviceid: deviceid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.len.size() + self.deviceid.size() + 2
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct HierarchyChange<'ec> {
    pub ty: HierarchyChangeType,
    pub len: Card16,
    pub send_core: bool,
    pub enable: bool,
    pub name: Cow<'ec, str>,
    pub deviceid: DeviceId,
    pub return_mode: ChangeMode,
    pub return_pointer: DeviceId,
    pub return_keyboard: DeviceId,
    pub deviceid_: DeviceId,
    pub master: DeviceId,
    pub deviceid__: DeviceId,
}
impl<'ec> HierarchyChange {}
impl<'ec> AsByteSequence for HierarchyChange<'ec> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        let cond0 = (self.ty);
        if cond0 == HierarchyChangeType::AddMaster {
            index += self.send_core.as_bytes(&mut bytes[index..]);
        }
        if cond0 == HierarchyChangeType::AddMaster {
            index += self.enable.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index += 4;
        if cond0 == HierarchyChangeType::RemoveMaster {
            index += self.deviceid.as_bytes(&mut bytes[index..]);
        }
        if cond0 == HierarchyChangeType::RemoveMaster {
            index += self.return_mode.as_bytes(&mut bytes[index..]);
        }
        index += 1;
        if cond0 == HierarchyChangeType::RemoveMaster {
            index += self.return_pointer.as_bytes(&mut bytes[index..]);
        }
        if cond0 == HierarchyChangeType::RemoveMaster {
            index += self.return_keyboard.as_bytes(&mut bytes[index..]);
        }
        if cond0 == HierarchyChangeType::AttachSlave {
            index += self.deviceid_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == HierarchyChangeType::AttachSlave {
            index += self.master.as_bytes(&mut bytes[index..]);
        }
        if cond0 == HierarchyChangeType::DetachSlave {
            index += self.deviceid__.as_bytes(&mut bytes[index..]);
        }
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing HierarchyChange from byte buffer");
        let (ty, sz): (HierarchyChangeType, usize) =
            <HierarchyChangeType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (ty);
        let send_core: bool = if cond0 == HierarchyChangeType::AddMaster {
            let (send_core, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
            index += sz;
            send_core
        } else {
            Default::default()
        };
        let enable: bool = if cond0 == HierarchyChangeType::AddMaster {
            let (enable, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
            index += sz;
            enable
        } else {
            Default::default()
        };
        let (name, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        index += 4;
        let deviceid: DeviceId = if cond0 == HierarchyChangeType::RemoveMaster {
            let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
            index += sz;
            deviceid
        } else {
            Default::default()
        };
        let return_mode: ChangeMode = if cond0 == HierarchyChangeType::RemoveMaster {
            let (return_mode, sz): (ChangeMode, usize) = <ChangeMode>::from_bytes(&bytes[index..])?;
            index += sz;
            return_mode
        } else {
            Default::default()
        };
        index += 1;
        let return_pointer: DeviceId = if cond0 == HierarchyChangeType::RemoveMaster {
            let (return_pointer, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
            index += sz;
            return_pointer
        } else {
            Default::default()
        };
        let return_keyboard: DeviceId = if cond0 == HierarchyChangeType::RemoveMaster {
            let (return_keyboard, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
            index += sz;
            return_keyboard
        } else {
            Default::default()
        };
        let deviceid_: DeviceId = if cond0 == HierarchyChangeType::AttachSlave {
            let (deviceid_, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
            index += sz;
            deviceid_
        } else {
            Default::default()
        };
        let master: DeviceId = if cond0 == HierarchyChangeType::AttachSlave {
            let (master, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
            index += sz;
            master
        } else {
            Default::default()
        };
        let deviceid__: DeviceId = if cond0 == HierarchyChangeType::DetachSlave {
            let (deviceid__, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
            index += sz;
            deviceid__
        } else {
            Default::default()
        };
        index += 2;
        Some((
            HierarchyChange {
                ty: ty,
                len: len,
                send_core: send_core,
                enable: enable,
                name: name,
                deviceid: deviceid,
                return_mode: return_mode,
                return_pointer: return_pointer,
                return_keyboard: return_keyboard,
                deviceid_: deviceid_,
                master: master,
                deviceid__: deviceid__,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + ::core::mem::size_of::<Card16>()
            + self.send_core.size()
            + self.enable.size()
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
            + 4
            + self.deviceid.size()
            + self.return_mode.size()
            + 1
            + self.return_pointer.size()
            + self.return_keyboard.size()
            + self.deviceid_.size()
            + self.master.size()
            + self.deviceid__.size()
            + 2
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiChangeHierarchyRequest<'gc, 'fc> {
    pub req_type: u8,
    pub length: u16,
    pub changes: Cow<'gc, [HierarchyChange<'fc>]>,
}
impl<'gc, 'fc> XiChangeHierarchyRequest {}
impl<'gc, 'fc> AsByteSequence for XiChangeHierarchyRequest<'gc, 'fc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.changes.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 3;
        let block_len: usize = vector_as_bytes(&self.changes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<HierarchyChange<'fc>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiChangeHierarchyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (changes, block_len): (Cow<'static, [HierarchyChange<'fc>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<HierarchyChange<'fc>>());
        Some((
            XiChangeHierarchyRequest {
                req_type: req_type,
                length: length,
                changes: changes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card8>() + 3 + {
            let block_len: usize = self.changes.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<HierarchyChange<'fc>>());
            block_len + pad
        }
    }
}
impl Request for XiChangeHierarchyRequest {
    const OPCODE: u8 = 43;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiSetClientPointerRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub deviceid: DeviceId,
}
impl XiSetClientPointerRequest {}
impl AsByteSequence for XiSetClientPointerRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiSetClientPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiSetClientPointerRequest {
                req_type: req_type,
                length: length,
                window: window,
                deviceid: deviceid,
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
            + self.deviceid.size()
            + 2
    }
}
impl Request for XiSetClientPointerRequest {
    const OPCODE: u8 = 44;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetClientPointerRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl XiGetClientPointerRequest {}
impl AsByteSequence for XiGetClientPointerRequest {
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
        log::trace!("Deserializing XiGetClientPointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            XiGetClientPointerRequest {
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
impl Request for XiGetClientPointerRequest {
    const OPCODE: u8 = 45;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiGetClientPointerReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetClientPointerReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub set: bool,
    pub deviceid: DeviceId,
}
impl XiGetClientPointerReply {}
impl AsByteSequence for XiGetClientPointerReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.set.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGetClientPointerReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (set, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            XiGetClientPointerReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                set: set,
                deviceid: deviceid,
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
            + self.set.size()
            + self.deviceid.size()
            + 20
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct EventMask<'hc> {
    pub deviceid: DeviceId,
    pub mask: Cow<'hc, [Card32]>,
}
impl<'hc> EventMask {}
impl<'hc> AsByteSequence for EventMask<'hc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += (self.mask.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing EventMask from byte buffer");
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            EventMask {
                deviceid: deviceid,
                mask: mask,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.deviceid.size() + ::core::mem::size_of::<Card16>() + {
            let block_len: usize = self.mask.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiSelectEventsRequest<'jc, 'ic> {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub masks: Cow<'jc, [EventMask<'ic>]>,
}
impl<'jc, 'ic> XiSelectEventsRequest {}
impl<'jc, 'ic> AsByteSequence for XiSelectEventsRequest<'jc, 'ic> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += (self.masks.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.masks, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventMask<'ic>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiSelectEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (masks, block_len): (Cow<'static, [EventMask<'ic>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventMask<'ic>>());
        Some((
            XiSelectEventsRequest {
                req_type: req_type,
                length: length,
                window: window,
                masks: masks,
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
            + 2
            + {
                let block_len: usize = self.masks.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventMask<'ic>>());
                block_len + pad
            }
    }
}
impl Request for XiSelectEventsRequest {
    const OPCODE: u8 = 46;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiQueryVersionRequest {
    pub req_type: u8,
    pub length: u16,
    pub major_version: Card16,
    pub minor_version: Card16,
}
impl XiQueryVersionRequest {}
impl AsByteSequence for XiQueryVersionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiQueryVersionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            XiQueryVersionRequest {
                req_type: req_type,
                length: length,
                major_version: major_version,
                minor_version: minor_version,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.major_version.size()
            + self.minor_version.size()
    }
}
impl Request for XiQueryVersionRequest {
    const OPCODE: u8 = 47;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiQueryVersionReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiQueryVersionReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub major_version: Card16,
    pub minor_version: Card16,
}
impl XiQueryVersionReply {}
impl AsByteSequence for XiQueryVersionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.major_version.as_bytes(&mut bytes[index..]);
        index += self.minor_version.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiQueryVersionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_version, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            XiQueryVersionReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                major_version: major_version,
                minor_version: minor_version,
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
            + self.major_version.size()
            + self.minor_version.size()
            + 20
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ButtonClass<'kc, 'lc> {
    pub ty: DeviceClassType,
    pub len: Card16,
    pub sourceid: DeviceId,
    pub num_buttons: Card16,
    pub state: Cow<'kc, [Card32]>,
    pub labels: Cow<'lc, [Atom]>,
}
impl<'kc, 'lc> ButtonClass {}
impl<'kc, 'lc> AsByteSequence for ButtonClass<'kc, 'lc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.num_buttons.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.state, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.labels, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ButtonClass from byte buffer");
        let (ty, sz): (DeviceClassType, usize) = <DeviceClassType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_buttons, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, block_len): (Cow<'static, [Card32]>, usize) = vector_from_bytes(
            &bytes[index..],
            (((num_buttons as usize) + (31)) / (32)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (labels, block_len): (Cow<'static, [Atom]>, usize) =
            vector_from_bytes(&bytes[index..], (num_buttons as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            ButtonClass {
                ty: ty,
                len: len,
                sourceid: sourceid,
                num_buttons: num_buttons,
                state: state,
                labels: labels,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + self.sourceid.size()
            + self.num_buttons.size()
            + {
                let block_len: usize = self.state.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.labels.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DeviceClassType {
    Key = 0,
    Button = 1,
    Valuator = 2,
    Scroll = 3,
    Touch = 8,
}
impl AsByteSequence for DeviceClassType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Key, sz)),
            1 => Some((Self::Button, sz)),
            2 => Some((Self::Valuator, sz)),
            3 => Some((Self::Scroll, sz)),
            8 => Some((Self::Touch, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for DeviceClassType {
    #[inline]
    fn default() -> DeviceClassType {
        DeviceClassType::Key
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct KeyClass<'mc> {
    pub ty: DeviceClassType,
    pub len: Card16,
    pub sourceid: DeviceId,
    pub keys: Cow<'mc, [Card32]>,
}
impl<'mc> KeyClass {}
impl<'mc> AsByteSequence for KeyClass<'mc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.keys.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keys, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyClass from byte buffer");
        let (ty, sz): (DeviceClassType, usize) = <DeviceClassType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keys, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            KeyClass {
                ty: ty,
                len: len,
                sourceid: sourceid,
                keys: keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.keys.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ScrollClass {
    pub ty: DeviceClassType,
    pub len: Card16,
    pub sourceid: DeviceId,
    pub number: Card16,
    pub scroll_type: ScrollType,
    pub flags: ScrollFlags,
    pub increment: Fp3232,
}
impl ScrollClass {}
impl AsByteSequence for ScrollClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.number.as_bytes(&mut bytes[index..]);
        index += self.scroll_type.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.increment.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ScrollClass from byte buffer");
        let (ty, sz): (DeviceClassType, usize) = <DeviceClassType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (number, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (scroll_type, sz): (ScrollType, usize) = <ScrollType>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (ScrollFlags, usize) = <ScrollFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (increment, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ScrollClass {
                ty: ty,
                len: len,
                sourceid: sourceid,
                number: number,
                scroll_type: scroll_type,
                flags: flags,
                increment: increment,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + self.sourceid.size()
            + self.number.size()
            + self.scroll_type.size()
            + 2
            + self.flags.size()
            + self.increment.size()
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ScrollType {
    Vertical = 1,
    Horizontal = 2,
}
impl AsByteSequence for ScrollType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::Vertical, sz)),
            2 => Some((Self::Horizontal, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for ScrollType {
    #[inline]
    fn default() -> ScrollType {
        ScrollType::Vertical
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScrollFlags {
    pub inner: u32,
}
impl ScrollFlags {
    #[inline]
    pub fn no_emulation(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_no_emulation(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn preferred(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_preferred(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(no_emulation: bool, preferred: bool) -> Self {
        let mut inner: u32 = 0;
        if no_emulation {
            inner |= 1 << 0;
        }
        if preferred {
            inner |= 1 << 1;
        }
        ScrollFlags { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const NO_EMULATION: Self = Self { inner: 1 };
    pub const PREFERRED: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for ScrollFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((ScrollFlags { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ScrollFlags {
    type Output = ScrollFlags;
    #[inline]
    fn not(self) -> ScrollFlags {
        ScrollFlags { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ScrollFlags {
    type Output = ScrollFlags;
    #[inline]
    fn bitand(self, rhs: ScrollFlags) -> ScrollFlags {
        ScrollFlags {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ScrollFlags {
    type Output = ScrollFlags;
    #[inline]
    fn bitor(self, rhs: ScrollFlags) -> ScrollFlags {
        ScrollFlags {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ScrollFlags {
    type Output = ScrollFlags;
    #[inline]
    fn bitxor(self, rhs: ScrollFlags) -> ScrollFlags {
        ScrollFlags {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct TouchClass {
    pub ty: DeviceClassType,
    pub len: Card16,
    pub sourceid: DeviceId,
    pub mode: TouchMode,
    pub num_touches: Card8,
}
impl TouchClass {}
impl AsByteSequence for TouchClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.num_touches.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TouchClass from byte buffer");
        let (ty, sz): (DeviceClassType, usize) = <DeviceClassType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (TouchMode, usize) = <TouchMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_touches, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            TouchClass {
                ty: ty,
                len: len,
                sourceid: sourceid,
                mode: mode,
                num_touches: num_touches,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + self.sourceid.size()
            + self.mode.size()
            + self.num_touches.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TouchMode {
    Direct = 1,
    Dependent = 2,
}
impl AsByteSequence for TouchMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::Direct, sz)),
            2 => Some((Self::Dependent, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for TouchMode {
    #[inline]
    fn default() -> TouchMode {
        TouchMode::Direct
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ValuatorClass {
    pub ty: DeviceClassType,
    pub len: Card16,
    pub sourceid: DeviceId,
    pub number: Card16,
    pub label: Atom,
    pub min: Fp3232,
    pub max: Fp3232,
    pub value: Fp3232,
    pub resolution: Card32,
    pub mode: ValuatorMode,
}
impl ValuatorClass {}
impl AsByteSequence for ValuatorClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.number.as_bytes(&mut bytes[index..]);
        index += self.label.as_bytes(&mut bytes[index..]);
        index += self.min.as_bytes(&mut bytes[index..]);
        index += self.max.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index += self.resolution.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ValuatorClass from byte buffer");
        let (ty, sz): (DeviceClassType, usize) = <DeviceClassType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (number, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (label, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        let (resolution, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (ValuatorMode, usize) = <ValuatorMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            ValuatorClass {
                ty: ty,
                len: len,
                sourceid: sourceid,
                number: number,
                label: label,
                min: min,
                max: max,
                value: value,
                resolution: resolution,
                mode: mode,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + self.sourceid.size()
            + self.number.size()
            + self.label.size()
            + self.min.size()
            + self.max.size()
            + self.value.size()
            + self.resolution.size()
            + self.mode.size()
            + 3
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceClass<'nc, 'oc, 'pc> {
    pub ty: DeviceClassType,
    pub len: Card16,
    pub sourceid: DeviceId,
    pub keys: Cow<'nc, [Card32]>,
    pub num_buttons: Card16,
    pub state: Cow<'oc, [Card32]>,
    pub labels: Cow<'pc, [Atom]>,
    pub number: Card16,
    pub label: Atom,
    pub min: Fp3232,
    pub max: Fp3232,
    pub value: Fp3232,
    pub resolution: Card32,
    pub mode: ValuatorMode,
    pub number_: Card16,
    pub scroll_type: ScrollType,
    pub flags: ScrollFlags,
    pub increment: Fp3232,
    pub mode_: TouchMode,
    pub num_touches: Card8,
}
impl<'nc, 'oc, 'pc> DeviceClass {}
impl<'nc, 'oc, 'pc> AsByteSequence for DeviceClass<'nc, 'oc, 'pc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.keys.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.keys, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let cond0 = (self.ty);
        if cond0 == DeviceClassType::Button {
            index += self.num_buttons.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.state, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.labels, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        if cond0 == DeviceClassType::Valuator {
            index += self.number.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Valuator {
            index += self.label.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Valuator {
            index += self.min.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Valuator {
            index += self.max.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Valuator {
            index += self.value.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Valuator {
            index += self.resolution.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Valuator {
            index += self.mode.as_bytes(&mut bytes[index..]);
        }
        index += 3;
        if cond0 == DeviceClassType::Scroll {
            index += self.number_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Scroll {
            index += self.scroll_type.as_bytes(&mut bytes[index..]);
        }
        index += 2;
        if cond0 == DeviceClassType::Scroll {
            index += self.flags.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Scroll {
            index += self.increment.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Touch {
            index += self.mode_.as_bytes(&mut bytes[index..]);
        }
        if cond0 == DeviceClassType::Touch {
            index += self.num_touches.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceClass from byte buffer");
        let (ty, sz): (DeviceClassType, usize) = <DeviceClassType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keys, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let cond0 = (ty);
        let num_buttons: Card16 = if cond0 == DeviceClassType::Button {
            let (num_buttons, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            num_buttons
        } else {
            Default::default()
        };
        let (state, block_len): (Cow<'static, [Card32]>, usize) = vector_from_bytes(
            &bytes[index..],
            (((num_buttons as usize) + (31)) / (32)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (labels, block_len): (Cow<'static, [Atom]>, usize) =
            vector_from_bytes(&bytes[index..], (num_buttons as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let number: Card16 = if cond0 == DeviceClassType::Valuator {
            let (number, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            number
        } else {
            Default::default()
        };
        let label: Atom = if cond0 == DeviceClassType::Valuator {
            let (label, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            label
        } else {
            Default::default()
        };
        let min: Fp3232 = if cond0 == DeviceClassType::Valuator {
            let (min, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
            index += sz;
            min
        } else {
            Default::default()
        };
        let max: Fp3232 = if cond0 == DeviceClassType::Valuator {
            let (max, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
            index += sz;
            max
        } else {
            Default::default()
        };
        let value: Fp3232 = if cond0 == DeviceClassType::Valuator {
            let (value, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
            index += sz;
            value
        } else {
            Default::default()
        };
        let resolution: Card32 = if cond0 == DeviceClassType::Valuator {
            let (resolution, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            resolution
        } else {
            Default::default()
        };
        let mode: ValuatorMode = if cond0 == DeviceClassType::Valuator {
            let (mode, sz): (ValuatorMode, usize) = <ValuatorMode>::from_bytes(&bytes[index..])?;
            index += sz;
            mode
        } else {
            Default::default()
        };
        index += 3;
        let number_: Card16 = if cond0 == DeviceClassType::Scroll {
            let (number_, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            number_
        } else {
            Default::default()
        };
        let scroll_type: ScrollType = if cond0 == DeviceClassType::Scroll {
            let (scroll_type, sz): (ScrollType, usize) = <ScrollType>::from_bytes(&bytes[index..])?;
            index += sz;
            scroll_type
        } else {
            Default::default()
        };
        index += 2;
        let flags: ScrollFlags = if cond0 == DeviceClassType::Scroll {
            let (flags, sz): (ScrollFlags, usize) = <ScrollFlags>::from_bytes(&bytes[index..])?;
            index += sz;
            flags
        } else {
            Default::default()
        };
        let increment: Fp3232 = if cond0 == DeviceClassType::Scroll {
            let (increment, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
            index += sz;
            increment
        } else {
            Default::default()
        };
        let mode_: TouchMode = if cond0 == DeviceClassType::Touch {
            let (mode_, sz): (TouchMode, usize) = <TouchMode>::from_bytes(&bytes[index..])?;
            index += sz;
            mode_
        } else {
            Default::default()
        };
        let num_touches: Card8 = if cond0 == DeviceClassType::Touch {
            let (num_touches, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            num_touches
        } else {
            Default::default()
        };
        Some((
            DeviceClass {
                ty: ty,
                len: len,
                sourceid: sourceid,
                keys: keys,
                num_buttons: num_buttons,
                state: state,
                labels: labels,
                number: number,
                label: label,
                min: min,
                max: max,
                value: value,
                resolution: resolution,
                mode: mode,
                number_: number_,
                scroll_type: scroll_type,
                flags: flags,
                increment: increment,
                mode_: mode_,
                num_touches: num_touches,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.len.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.keys.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + self.num_buttons.size()
            + {
                let block_len: usize = self.state.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.labels.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + self.number.size()
            + self.label.size()
            + self.min.size()
            + self.max.size()
            + self.value.size()
            + self.resolution.size()
            + self.mode.size()
            + 3
            + self.number_.size()
            + self.scroll_type.size()
            + 2
            + self.flags.size()
            + self.increment.size()
            + self.mode_.size()
            + self.num_touches.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiDeviceInfo<'qc, 'uc, 'rc, 'sc, 'tc> {
    pub deviceid: DeviceId,
    pub ty: DeviceType,
    pub attachment: DeviceId,
    pub enabled: bool,
    pub name: Cow<'qc, str>,
    pub classes: Cow<'uc, [DeviceClass<'rc, 'sc, 'tc>]>,
}
impl<'qc, 'uc, 'rc, 'sc, 'tc> XiDeviceInfo {}
impl<'qc, 'uc, 'rc, 'sc, 'tc> AsByteSequence for XiDeviceInfo<'qc, 'uc, 'rc, 'sc, 'tc> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.attachment.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.enabled.as_bytes(&mut bytes[index..]);
        index += 1;
        let block_len: usize = string_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<DeviceClass<'rc, 'sc, 'tc>>(),
        );
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiDeviceInfo from byte buffer");
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (DeviceType, usize) = <DeviceType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attachment, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (name, block_len): (Cow<'static, str>, usize) =
            string_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (classes, block_len): (Cow<'static, [DeviceClass<'rc, 'sc, 'tc>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<DeviceClass<'rc, 'sc, 'tc>>(),
        );
        Some((
            XiDeviceInfo {
                deviceid: deviceid,
                ty: ty,
                attachment: attachment,
                enabled: enabled,
                name: name,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.deviceid.size()
            + self.ty.size()
            + self.attachment.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.enabled.size()
            + 1
            + {
                let block_len: usize = self.name.len();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(
                    block_len,
                    ::core::mem::align_of::<DeviceClass<'rc, 'sc, 'tc>>(),
                );
                block_len + pad
            }
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DeviceType {
    MasterPointer = 1,
    MasterKeyboard = 2,
    SlavePointer = 3,
    SlaveKeyboard = 4,
    FloatingSlave = 5,
}
impl AsByteSequence for DeviceType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::MasterPointer, sz)),
            2 => Some((Self::MasterKeyboard, sz)),
            3 => Some((Self::SlavePointer, sz)),
            4 => Some((Self::SlaveKeyboard, sz)),
            5 => Some((Self::FloatingSlave, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for DeviceType {
    #[inline]
    fn default() -> DeviceType {
        DeviceType::MasterPointer
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiQueryDeviceRequest {
    pub req_type: u8,
    pub length: u16,
    pub deviceid: DeviceId,
}
impl XiQueryDeviceRequest {}
impl AsByteSequence for XiQueryDeviceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiQueryDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiQueryDeviceRequest {
                req_type: req_type,
                length: length,
                deviceid: deviceid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.deviceid.size() + 2
    }
}
impl Request for XiQueryDeviceRequest {
    const OPCODE: u8 = 48;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiQueryDeviceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiQueryDeviceReply<'ad, 'vc, 'wc, 'xc, 'yc, 'zc> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub infos: Cow<'ad, [XiDeviceInfo<'vc, 'wc, 'xc, 'yc, 'zc>]>,
}
impl<'ad, 'vc, 'wc, 'xc, 'yc, 'zc> XiQueryDeviceReply {}
impl<'ad, 'vc, 'wc, 'xc, 'yc, 'zc> AsByteSequence
    for XiQueryDeviceReply<'ad, 'vc, 'wc, 'xc, 'yc, 'zc>
{
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.infos.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.infos, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<XiDeviceInfo<'vc, 'wc, 'xc, 'yc, 'zc>>(),
        );
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiQueryDeviceReply from byte buffer");
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
        let (infos, block_len): (Cow<'static, [XiDeviceInfo<'vc, 'wc, 'xc, 'yc, 'zc>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<XiDeviceInfo<'vc, 'wc, 'xc, 'yc, 'zc>>(),
        );
        Some((
            XiQueryDeviceReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                infos: infos,
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
                let block_len: usize = self.infos.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(
                    block_len,
                    ::core::mem::align_of::<XiDeviceInfo<'vc, 'wc, 'xc, 'yc, 'zc>>(),
                );
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiSetFocusRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub time: Timestamp,
    pub deviceid: DeviceId,
}
impl XiSetFocusRequest {}
impl AsByteSequence for XiSetFocusRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiSetFocusRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiSetFocusRequest {
                req_type: req_type,
                length: length,
                window: window,
                time: time,
                deviceid: deviceid,
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
            + self.time.size()
            + self.deviceid.size()
            + 2
    }
}
impl Request for XiSetFocusRequest {
    const OPCODE: u8 = 49;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetFocusRequest {
    pub req_type: u8,
    pub length: u16,
    pub deviceid: DeviceId,
}
impl XiGetFocusRequest {}
impl AsByteSequence for XiGetFocusRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGetFocusRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiGetFocusRequest {
                req_type: req_type,
                length: length,
                deviceid: deviceid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.deviceid.size() + 2
    }
}
impl Request for XiGetFocusRequest {
    const OPCODE: u8 = 50;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiGetFocusReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetFocusReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub focus: Window,
}
impl XiGetFocusReply {}
impl AsByteSequence for XiGetFocusReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGetFocusReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            XiGetFocusReply {
                reply_type: reply_type,
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
            + 1
            + self.sequence.size()
            + self.length.size()
            + self.focus.size()
            + 20
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGrabDeviceRequest<'bd> {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
    pub time: Timestamp,
    pub cursor: Cursor,
    pub deviceid: DeviceId,
    pub mode: GrabMode,
    pub paired_device_mode: GrabMode,
    pub owner_events: GrabOwner,
    pub mask: Cow<'bd, [Card32]>,
}
impl<'bd> XiGrabDeviceRequest {}
impl<'bd> AsByteSequence for XiGrabDeviceRequest<'bd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.paired_device_mode.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += 1;
        index += (self.mask.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGrabDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (paired_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (GrabOwner, usize) = <GrabOwner>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            XiGrabDeviceRequest {
                req_type: req_type,
                length: length,
                window: window,
                time: time,
                cursor: cursor,
                deviceid: deviceid,
                mode: mode,
                paired_device_mode: paired_device_mode,
                owner_events: owner_events,
                mask: mask,
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
            + self.time.size()
            + self.cursor.size()
            + self.deviceid.size()
            + self.mode.size()
            + self.paired_device_mode.size()
            + self.owner_events.size()
            + 1
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for XiGrabDeviceRequest {
    const OPCODE: u8 = 51;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiGrabDeviceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGrabDeviceReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub status: GrabStatus,
}
impl XiGrabDeviceReply {}
impl AsByteSequence for XiGrabDeviceReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGrabDeviceReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            XiGrabDeviceReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                status: status,
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
            + self.status.size()
            + 23
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GrabOwner {
    NoOwner = 0,
    Owner = 1,
}
impl AsByteSequence for GrabOwner {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NoOwner, sz)),
            1 => Some((Self::Owner, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for GrabOwner {
    #[inline]
    fn default() -> GrabOwner {
        GrabOwner::NoOwner
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiUngrabDeviceRequest {
    pub req_type: u8,
    pub length: u16,
    pub time: Timestamp,
    pub deviceid: DeviceId,
}
impl XiUngrabDeviceRequest {}
impl AsByteSequence for XiUngrabDeviceRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiUngrabDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiUngrabDeviceRequest {
                req_type: req_type,
                length: length,
                time: time,
                deviceid: deviceid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.time.size() + self.deviceid.size() + 2
    }
}
impl Request for XiUngrabDeviceRequest {
    const OPCODE: u8 = 52;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiAllowEventsRequest {
    pub req_type: u8,
    pub length: u16,
    pub time: Timestamp,
    pub deviceid: DeviceId,
    pub event_mode: EventMode,
    pub touchid: Card32,
    pub grab_window: Window,
}
impl XiAllowEventsRequest {}
impl AsByteSequence for XiAllowEventsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.event_mode.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.touchid.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiAllowEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_mode, sz): (EventMode, usize) = <EventMode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (touchid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            XiAllowEventsRequest {
                req_type: req_type,
                length: length,
                time: time,
                deviceid: deviceid,
                event_mode: event_mode,
                touchid: touchid,
                grab_window: grab_window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.time.size()
            + self.deviceid.size()
            + self.event_mode.size()
            + 1
            + self.touchid.size()
            + self.grab_window.size()
    }
}
impl Request for XiAllowEventsRequest {
    const OPCODE: u8 = 53;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum EventMode {
    AsyncDevice = 0,
    SyncDevice = 1,
    ReplayDevice = 2,
    AsyncPairedDevice = 3,
    AsyncPair = 4,
    SyncPair = 5,
    AcceptTouch = 6,
    RejectTouch = 7,
}
impl AsByteSequence for EventMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::AsyncDevice, sz)),
            1 => Some((Self::SyncDevice, sz)),
            2 => Some((Self::ReplayDevice, sz)),
            3 => Some((Self::AsyncPairedDevice, sz)),
            4 => Some((Self::AsyncPair, sz)),
            5 => Some((Self::SyncPair, sz)),
            6 => Some((Self::AcceptTouch, sz)),
            7 => Some((Self::RejectTouch, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for EventMode {
    #[inline]
    fn default() -> EventMode {
        EventMode::AsyncDevice
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct GrabModifierInfo {
    pub modifiers: Card32,
    pub status: GrabStatus,
}
impl GrabModifierInfo {}
impl AsByteSequence for GrabModifierInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.modifiers.as_bytes(&mut bytes[index..]);
        index += self.status.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GrabModifierInfo from byte buffer");
        let (modifiers, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (status, sz): (GrabStatus, usize) = <GrabStatus>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GrabModifierInfo {
                modifiers: modifiers,
                status: status,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.modifiers.size() + self.status.size() + 3
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModifierMask {
    pub inner: u32,
}
impl ModifierMask {
    #[inline]
    pub fn any(&self) -> bool {
        self.inner & (1 << 31) != 0
    }
    #[inline]
    pub fn set_any(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 31;
        } else {
            self.inner &= !(1 << 31);
        }
        self
    }
    #[inline]
    pub fn new(any: bool) -> Self {
        let mut inner: u32 = 0;
        if any {
            inner |= 1 << 31;
        }
        ModifierMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const ANY: Self = Self { inner: 2147483648 };
    pub const COMPLETE: Self = Self { inner: 2147483648 };
}
impl AsByteSequence for ModifierMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((ModifierMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ModifierMask {
    type Output = ModifierMask;
    #[inline]
    fn not(self) -> ModifierMask {
        ModifierMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ModifierMask {
    type Output = ModifierMask;
    #[inline]
    fn bitand(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ModifierMask {
    type Output = ModifierMask;
    #[inline]
    fn bitor(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ModifierMask {
    type Output = ModifierMask;
    #[inline]
    fn bitxor(self, rhs: ModifierMask) -> ModifierMask {
        ModifierMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiPassiveGrabDeviceRequest<'dd, 'ed> {
    pub req_type: u8,
    pub length: u16,
    pub time: Timestamp,
    pub grab_window: Window,
    pub cursor: Cursor,
    pub detail: Card32,
    pub deviceid: DeviceId,
    pub grab_type: GrabType,
    pub grab_mode: GrabMode22,
    pub paired_device_mode: GrabMode,
    pub owner_events: GrabOwner,
    pub mask: Cow<'dd, [Card32]>,
    pub modifiers: Cow<'ed, [Card32]>,
}
impl<'dd, 'ed> XiPassiveGrabDeviceRequest {}
impl<'dd, 'ed> AsByteSequence for XiPassiveGrabDeviceRequest<'dd, 'ed> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.cursor.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += (self.modifiers.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.grab_type.as_bytes(&mut bytes[index..]);
        index += self.grab_mode.as_bytes(&mut bytes[index..]);
        index += self.paired_device_mode.as_bytes(&mut bytes[index..]);
        index += self.owner_events.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.modifiers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiPassiveGrabDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (cursor, sz): (Cursor, usize) = <Cursor>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_type, sz): (GrabType, usize) = <GrabType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_mode, sz): (GrabMode22, usize) = <GrabMode22>::from_bytes(&bytes[index..])?;
        index += sz;
        let (paired_device_mode, sz): (GrabMode, usize) = <GrabMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (owner_events, sz): (GrabOwner, usize) = <GrabOwner>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (modifiers, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            XiPassiveGrabDeviceRequest {
                req_type: req_type,
                length: length,
                time: time,
                grab_window: grab_window,
                cursor: cursor,
                detail: detail,
                deviceid: deviceid,
                grab_type: grab_type,
                grab_mode: grab_mode,
                paired_device_mode: paired_device_mode,
                owner_events: owner_events,
                mask: mask,
                modifiers: modifiers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.time.size()
            + self.grab_window.size()
            + self.cursor.size()
            + self.detail.size()
            + self.deviceid.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.grab_type.size()
            + self.grab_mode.size()
            + self.paired_device_mode.size()
            + self.owner_events.size()
            + 2
            + {
                let block_len: usize = self.mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.modifiers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for XiPassiveGrabDeviceRequest {
    const OPCODE: u8 = 54;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiPassiveGrabDeviceReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiPassiveGrabDeviceReply<'cd> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub modifiers: Cow<'cd, [GrabModifierInfo]>,
}
impl<'cd> XiPassiveGrabDeviceReply {}
impl<'cd> AsByteSequence for XiPassiveGrabDeviceReply<'cd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.modifiers.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.modifiers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<GrabModifierInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiPassiveGrabDeviceReply from byte buffer");
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
        let (modifiers, block_len): (Cow<'static, [GrabModifierInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<GrabModifierInfo>());
        Some((
            XiPassiveGrabDeviceReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                modifiers: modifiers,
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
                let block_len: usize = self.modifiers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<GrabModifierInfo>());
                block_len + pad
            }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GrabType {
    Button = 0,
    Keycode = 1,
    Enter = 2,
    FocusIn = 3,
    TouchBegin = 4,
}
impl AsByteSequence for GrabType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Button, sz)),
            1 => Some((Self::Keycode, sz)),
            2 => Some((Self::Enter, sz)),
            3 => Some((Self::FocusIn, sz)),
            4 => Some((Self::TouchBegin, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for GrabType {
    #[inline]
    fn default() -> GrabType {
        GrabType::Button
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum GrabMode22 {
    Sync = 0,
    Async = 1,
    Touch = 2,
}
impl AsByteSequence for GrabMode22 {
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
            2 => Some((Self::Touch, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for GrabMode22 {
    #[inline]
    fn default() -> GrabMode22 {
        GrabMode22::Sync
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiPassiveUngrabDeviceRequest<'fd> {
    pub req_type: u8,
    pub length: u16,
    pub grab_window: Window,
    pub detail: Card32,
    pub deviceid: DeviceId,
    pub grab_type: GrabType,
    pub modifiers: Cow<'fd, [Card32]>,
}
impl<'fd> XiPassiveUngrabDeviceRequest {}
impl<'fd> AsByteSequence for XiPassiveUngrabDeviceRequest<'fd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.grab_window.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += (self.modifiers.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.grab_type.as_bytes(&mut bytes[index..]);
        index += 3;
        let block_len: usize = vector_as_bytes(&self.modifiers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiPassiveUngrabDeviceRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_type, sz): (GrabType, usize) = <GrabType>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (modifiers, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            XiPassiveUngrabDeviceRequest {
                req_type: req_type,
                length: length,
                grab_window: grab_window,
                detail: detail,
                deviceid: deviceid,
                grab_type: grab_type,
                modifiers: modifiers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.grab_window.size()
            + self.detail.size()
            + self.deviceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.grab_type.size()
            + 3
            + {
                let block_len: usize = self.modifiers.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for XiPassiveUngrabDeviceRequest {
    const OPCODE: u8 = 55;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiListPropertiesRequest {
    pub req_type: u8,
    pub length: u16,
    pub deviceid: DeviceId,
}
impl XiListPropertiesRequest {}
impl AsByteSequence for XiListPropertiesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiListPropertiesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            XiListPropertiesRequest {
                req_type: req_type,
                length: length,
                deviceid: deviceid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + self.deviceid.size() + 2
    }
}
impl Request for XiListPropertiesRequest {
    const OPCODE: u8 = 56;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiListPropertiesReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiListPropertiesReply<'gd> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub properties: Cow<'gd, [Atom]>,
}
impl<'gd> XiListPropertiesReply {}
impl<'gd> AsByteSequence for XiListPropertiesReply<'gd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.properties.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.properties, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiListPropertiesReply from byte buffer");
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
        let (properties, block_len): (Cow<'static, [Atom]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            XiListPropertiesReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                properties: properties,
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
                let block_len: usize = self.properties.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiChangePropertyRequest<'hd, 'id, 'jd> {
    pub req_type: u8,
    pub length: u16,
    pub deviceid: DeviceId,
    pub mode: PropMode,
    pub format: PropertyFormat,
    pub property: Atom,
    pub ty: Atom,
    pub num_items: Card32,
    pub data8: Cow<'hd, [Card8]>,
    pub data16: Cow<'id, [Card16]>,
    pub data32: Cow<'jd, [Card32]>,
}
impl<'hd, 'id, 'jd> XiChangePropertyRequest {}
impl<'hd, 'id, 'jd> AsByteSequence for XiChangePropertyRequest<'hd, 'id, 'jd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.num_items.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.data8, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.data16, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.data32, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiChangePropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (PropMode, usize) = <PropMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (PropertyFormat, usize) = <PropertyFormat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_items, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data8, block_len): (Cow<'static, [Card8]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (data16, block_len): (Cow<'static, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (data32, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            XiChangePropertyRequest {
                req_type: req_type,
                length: length,
                deviceid: deviceid,
                mode: mode,
                format: format,
                property: property,
                ty: ty,
                num_items: num_items,
                data8: data8,
                data16: data16,
                data32: data32,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.deviceid.size()
            + self.mode.size()
            + self.format.size()
            + self.property.size()
            + self.ty.size()
            + self.num_items.size()
            + {
                let block_len: usize = self.data8.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.data16.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.data32.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl Request for XiChangePropertyRequest {
    const OPCODE: u8 = 57;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiDeletePropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub deviceid: DeviceId,
    pub property: Atom,
}
impl XiDeletePropertyRequest {}
impl AsByteSequence for XiDeletePropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.property.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiDeletePropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            XiDeletePropertyRequest {
                req_type: req_type,
                length: length,
                deviceid: deviceid,
                property: property,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.deviceid.size()
            + 2
            + self.property.size()
    }
}
impl Request for XiDeletePropertyRequest {
    const OPCODE: u8 = 58;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetPropertyRequest {
    pub req_type: u8,
    pub length: u16,
    pub deviceid: DeviceId,
    pub delete: bool,
    pub property: Atom,
    pub ty: Atom,
    pub offset: Card32,
    pub len: Card32,
}
impl XiGetPropertyRequest {}
impl AsByteSequence for XiGetPropertyRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.delete.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.offset.as_bytes(&mut bytes[index..]);
        index += self.len.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGetPropertyRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (delete, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (offset, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            XiGetPropertyRequest {
                req_type: req_type,
                length: length,
                deviceid: deviceid,
                delete: delete,
                property: property,
                ty: ty,
                offset: offset,
                len: len,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.deviceid.size()
            + self.delete.size()
            + 1
            + self.property.size()
            + self.ty.size()
            + self.offset.size()
            + self.len.size()
    }
}
impl Request for XiGetPropertyRequest {
    const OPCODE: u8 = 59;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiGetPropertyReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetPropertyReply<'kd, 'ld, 'md> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub ty: Atom,
    pub bytes_after: Card32,
    pub num_items: Card32,
    pub format: PropertyFormat,
    pub data8: Cow<'kd, [Card8]>,
    pub data16: Cow<'ld, [Card16]>,
    pub data32: Cow<'md, [Card32]>,
}
impl<'kd, 'ld, 'md> XiGetPropertyReply {}
impl<'kd, 'ld, 'md> AsByteSequence for XiGetPropertyReply<'kd, 'ld, 'md> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.bytes_after.as_bytes(&mut bytes[index..]);
        index += self.num_items.as_bytes(&mut bytes[index..]);
        index += self.format.as_bytes(&mut bytes[index..]);
        index += 11;
        let block_len: usize = vector_as_bytes(&self.data8, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.data16, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let block_len: usize = vector_as_bytes(&self.data32, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGetPropertyReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bytes_after, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_items, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (format, sz): (PropertyFormat, usize) = <PropertyFormat>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 11;
        let (data8, block_len): (Cow<'static, [Card8]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (data16, block_len): (Cow<'static, [Card16]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card16>());
        let (data32, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], (num_items as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            XiGetPropertyReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                ty: ty,
                bytes_after: bytes_after,
                num_items: num_items,
                format: format,
                data8: data8,
                data16: data16,
                data32: data32,
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
            + self.ty.size()
            + self.bytes_after.size()
            + self.num_items.size()
            + self.format.size()
            + 11
            + {
                let block_len: usize = self.data8.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.data16.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card16>());
                block_len + pad
            }
            + {
                let block_len: usize = self.data32.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetSelectedEventsRequest {
    pub req_type: u8,
    pub length: u16,
    pub window: Window,
}
impl XiGetSelectedEventsRequest {}
impl AsByteSequence for XiGetSelectedEventsRequest {
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
        log::trace!("Deserializing XiGetSelectedEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            XiGetSelectedEventsRequest {
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
impl Request for XiGetSelectedEventsRequest {
    const OPCODE: u8 = 60;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = XiGetSelectedEventsReply;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiGetSelectedEventsReply<'od, 'nd> {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub masks: Cow<'od, [EventMask<'nd>]>,
}
impl<'od, 'nd> XiGetSelectedEventsReply {}
impl<'od, 'nd> AsByteSequence for XiGetSelectedEventsReply<'od, 'nd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.masks.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 22;
        let block_len: usize = vector_as_bytes(&self.masks, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventMask<'nd>>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiGetSelectedEventsReply from byte buffer");
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
        let (masks, block_len): (Cow<'static, [EventMask<'nd>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventMask<'nd>>());
        Some((
            XiGetSelectedEventsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
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
            + 22
            + {
                let block_len: usize = self.masks.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventMask<'nd>>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct BarrierReleasePointerInfo {
    pub deviceid: DeviceId,
    pub barrier: Barrier,
    pub eventid: Card32,
}
impl BarrierReleasePointerInfo {}
impl AsByteSequence for BarrierReleasePointerInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.barrier.as_bytes(&mut bytes[index..]);
        index += self.eventid.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BarrierReleasePointerInfo from byte buffer");
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (barrier, sz): (Barrier, usize) = <Barrier>::from_bytes(&bytes[index..])?;
        index += sz;
        let (eventid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BarrierReleasePointerInfo {
                deviceid: deviceid,
                barrier: barrier,
                eventid: eventid,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.deviceid.size() + 2 + self.barrier.size() + self.eventid.size()
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct XiBarrierReleasePointerRequest<'pd> {
    pub req_type: u8,
    pub length: u16,
    pub barriers: Cow<'pd, [BarrierReleasePointerInfo]>,
}
impl<'pd> XiBarrierReleasePointerRequest {}
impl<'pd> AsByteSequence for XiBarrierReleasePointerRequest<'pd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.barriers.len() as Card32).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.barriers, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<BarrierReleasePointerInfo>(),
        );
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing XiBarrierReleasePointerRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (barriers, block_len): (Cow<'static, [BarrierReleasePointerInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<BarrierReleasePointerInfo>(),
        );
        Some((
            XiBarrierReleasePointerRequest {
                req_type: req_type,
                length: length,
                barriers: barriers,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + 1 + self.length.size() + ::core::mem::size_of::<Card32>() + {
            let block_len: usize = self.barriers.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(
                block_len,
                ::core::mem::align_of::<BarrierReleasePointerInfo>(),
            );
            block_len + pad
        }
    }
}
impl Request for XiBarrierReleasePointerRequest {
    const OPCODE: u8 = 61;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ClassesReportedMask {
    pub inner: u8,
}
impl ClassesReportedMask {
    #[inline]
    pub fn reporting_keys(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_reporting_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn reporting_buttons(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_reporting_buttons(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn reporting_valuators(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_reporting_valuators(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn device_mode_absolute(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_device_mode_absolute(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn out_of_proximity(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_out_of_proximity(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        reporting_keys: bool,
        reporting_buttons: bool,
        reporting_valuators: bool,
        device_mode_absolute: bool,
        out_of_proximity: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if reporting_keys {
            inner |= 1 << 0;
        }
        if reporting_buttons {
            inner |= 1 << 1;
        }
        if reporting_valuators {
            inner |= 1 << 2;
        }
        if device_mode_absolute {
            inner |= 1 << 6;
        }
        if out_of_proximity {
            inner |= 1 << 7;
        }
        ClassesReportedMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const REPORTING_KEYS: Self = Self { inner: 1 };
    pub const REPORTING_BUTTONS: Self = Self { inner: 2 };
    pub const REPORTING_VALUATORS: Self = Self { inner: 4 };
    pub const DEVICE_MODE_ABSOLUTE: Self = Self { inner: 64 };
    pub const OUT_OF_PROXIMITY: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 199 };
}
impl AsByteSequence for ClassesReportedMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((ClassesReportedMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ClassesReportedMask {
    type Output = ClassesReportedMask;
    #[inline]
    fn not(self) -> ClassesReportedMask {
        ClassesReportedMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ClassesReportedMask {
    type Output = ClassesReportedMask;
    #[inline]
    fn bitand(self, rhs: ClassesReportedMask) -> ClassesReportedMask {
        ClassesReportedMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ClassesReportedMask {
    type Output = ClassesReportedMask;
    #[inline]
    fn bitor(self, rhs: ClassesReportedMask) -> ClassesReportedMask {
        ClassesReportedMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ClassesReportedMask {
    type Output = ClassesReportedMask;
    #[inline]
    fn bitxor(self, rhs: ClassesReportedMask) -> ClassesReportedMask {
        ClassesReportedMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ChangeDevice {
    NewPointer = 0,
    NewKeyboard = 1,
}
impl AsByteSequence for ChangeDevice {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NewPointer, sz)),
            1 => Some((Self::NewKeyboard, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ChangeDevice {
    #[inline]
    fn default() -> ChangeDevice {
        ChangeDevice::NewPointer
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum DeviceChange {
    Added = 0,
    Removed = 1,
    Enabled = 2,
    Disabled = 3,
    Unrecoverable = 4,
    ControlChanged = 5,
}
impl AsByteSequence for DeviceChange {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Added, sz)),
            1 => Some((Self::Removed, sz)),
            2 => Some((Self::Enabled, sz)),
            3 => Some((Self::Disabled, sz)),
            4 => Some((Self::Unrecoverable, sz)),
            5 => Some((Self::ControlChanged, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for DeviceChange {
    #[inline]
    fn default() -> DeviceChange {
        DeviceChange::Added
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum ChangeReason {
    SlaveSwitch = 1,
    DeviceChange = 2,
}
impl AsByteSequence for ChangeReason {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::SlaveSwitch, sz)),
            2 => Some((Self::DeviceChange, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for ChangeReason {
    #[inline]
    fn default() -> ChangeReason {
        ChangeReason::SlaveSwitch
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct KeyEventFlags {
    pub inner: u32,
}
impl KeyEventFlags {
    #[inline]
    pub fn key_repeat(&self) -> bool {
        self.inner & (1 << 16) != 0
    }
    #[inline]
    pub fn set_key_repeat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 16;
        } else {
            self.inner &= !(1 << 16);
        }
        self
    }
    #[inline]
    pub fn new(key_repeat: bool) -> Self {
        let mut inner: u32 = 0;
        if key_repeat {
            inner |= 1 << 16;
        }
        KeyEventFlags { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const KEY_REPEAT: Self = Self { inner: 65536 };
    pub const COMPLETE: Self = Self { inner: 65536 };
}
impl AsByteSequence for KeyEventFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((KeyEventFlags { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for KeyEventFlags {
    type Output = KeyEventFlags;
    #[inline]
    fn not(self) -> KeyEventFlags {
        KeyEventFlags { inner: !self.inner }
    }
}
impl core::ops::BitAnd for KeyEventFlags {
    type Output = KeyEventFlags;
    #[inline]
    fn bitand(self, rhs: KeyEventFlags) -> KeyEventFlags {
        KeyEventFlags {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for KeyEventFlags {
    type Output = KeyEventFlags;
    #[inline]
    fn bitor(self, rhs: KeyEventFlags) -> KeyEventFlags {
        KeyEventFlags {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for KeyEventFlags {
    type Output = KeyEventFlags;
    #[inline]
    fn bitxor(self, rhs: KeyEventFlags) -> KeyEventFlags {
        KeyEventFlags {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PointerEventFlags {
    pub inner: u32,
}
impl PointerEventFlags {
    #[inline]
    pub fn pointer_emulated(&self) -> bool {
        self.inner & (1 << 16) != 0
    }
    #[inline]
    pub fn set_pointer_emulated(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 16;
        } else {
            self.inner &= !(1 << 16);
        }
        self
    }
    #[inline]
    pub fn new(pointer_emulated: bool) -> Self {
        let mut inner: u32 = 0;
        if pointer_emulated {
            inner |= 1 << 16;
        }
        PointerEventFlags { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const POINTER_EMULATED: Self = Self { inner: 65536 };
    pub const COMPLETE: Self = Self { inner: 65536 };
}
impl AsByteSequence for PointerEventFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((PointerEventFlags { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for PointerEventFlags {
    type Output = PointerEventFlags;
    #[inline]
    fn not(self) -> PointerEventFlags {
        PointerEventFlags { inner: !self.inner }
    }
}
impl core::ops::BitAnd for PointerEventFlags {
    type Output = PointerEventFlags;
    #[inline]
    fn bitand(self, rhs: PointerEventFlags) -> PointerEventFlags {
        PointerEventFlags {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for PointerEventFlags {
    type Output = PointerEventFlags;
    #[inline]
    fn bitor(self, rhs: PointerEventFlags) -> PointerEventFlags {
        PointerEventFlags {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for PointerEventFlags {
    type Output = PointerEventFlags;
    #[inline]
    fn bitxor(self, rhs: PointerEventFlags) -> PointerEventFlags {
        PointerEventFlags {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct HierarchyInfo {
    pub deviceid: DeviceId,
    pub attachment: DeviceId,
    pub ty: DeviceType,
    pub enabled: bool,
    pub flags: HierarchyMask,
}
impl HierarchyInfo {}
impl AsByteSequence for HierarchyInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.attachment.as_bytes(&mut bytes[index..]);
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.enabled.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing HierarchyInfo from byte buffer");
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (attachment, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ty, sz): (DeviceType, usize) = <DeviceType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (HierarchyMask, usize) = <HierarchyMask>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            HierarchyInfo {
                deviceid: deviceid,
                attachment: attachment,
                ty: ty,
                enabled: enabled,
                flags: flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.deviceid.size()
            + self.attachment.size()
            + self.ty.size()
            + self.enabled.size()
            + 2
            + self.flags.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct HierarchyMask {
    pub inner: u32,
}
impl HierarchyMask {
    #[inline]
    pub fn master_added(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_master_added(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn master_removed(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_master_removed(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn slave_added(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_slave_added(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn slave_removed(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_slave_removed(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn slave_attached(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_slave_attached(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn slave_detached(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_slave_detached(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn device_enabled(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_device_enabled(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn device_disabled(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_device_disabled(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        master_added: bool,
        master_removed: bool,
        slave_added: bool,
        slave_removed: bool,
        slave_attached: bool,
        slave_detached: bool,
        device_enabled: bool,
        device_disabled: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if master_added {
            inner |= 1 << 0;
        }
        if master_removed {
            inner |= 1 << 1;
        }
        if slave_added {
            inner |= 1 << 2;
        }
        if slave_removed {
            inner |= 1 << 3;
        }
        if slave_attached {
            inner |= 1 << 4;
        }
        if slave_detached {
            inner |= 1 << 5;
        }
        if device_enabled {
            inner |= 1 << 6;
        }
        if device_disabled {
            inner |= 1 << 7;
        }
        HierarchyMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const MASTER_ADDED: Self = Self { inner: 1 };
    pub const MASTER_REMOVED: Self = Self { inner: 2 };
    pub const SLAVE_ADDED: Self = Self { inner: 4 };
    pub const SLAVE_REMOVED: Self = Self { inner: 8 };
    pub const SLAVE_ATTACHED: Self = Self { inner: 16 };
    pub const SLAVE_DETACHED: Self = Self { inner: 32 };
    pub const DEVICE_ENABLED: Self = Self { inner: 64 };
    pub const DEVICE_DISABLED: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for HierarchyMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((HierarchyMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for HierarchyMask {
    type Output = HierarchyMask;
    #[inline]
    fn not(self) -> HierarchyMask {
        HierarchyMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for HierarchyMask {
    type Output = HierarchyMask;
    #[inline]
    fn bitand(self, rhs: HierarchyMask) -> HierarchyMask {
        HierarchyMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for HierarchyMask {
    type Output = HierarchyMask;
    #[inline]
    fn bitor(self, rhs: HierarchyMask) -> HierarchyMask {
        HierarchyMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for HierarchyMask {
    type Output = HierarchyMask;
    #[inline]
    fn bitxor(self, rhs: HierarchyMask) -> HierarchyMask {
        HierarchyMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum PropertyFlag {
    Deleted = 0,
    Created = 1,
    Modified = 2,
}
impl AsByteSequence for PropertyFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Deleted, sz)),
            1 => Some((Self::Created, sz)),
            2 => Some((Self::Modified, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for PropertyFlag {
    #[inline]
    fn default() -> PropertyFlag {
        PropertyFlag::Deleted
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct TouchEventFlags {
    pub inner: u32,
}
impl TouchEventFlags {
    #[inline]
    pub fn touch_pending_end(&self) -> bool {
        self.inner & (1 << 16) != 0
    }
    #[inline]
    pub fn set_touch_pending_end(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 16;
        } else {
            self.inner &= !(1 << 16);
        }
        self
    }
    #[inline]
    pub fn touch_emulating_pointer(&self) -> bool {
        self.inner & (1 << 17) != 0
    }
    #[inline]
    pub fn set_touch_emulating_pointer(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 17;
        } else {
            self.inner &= !(1 << 17);
        }
        self
    }
    #[inline]
    pub fn new(touch_pending_end: bool, touch_emulating_pointer: bool) -> Self {
        let mut inner: u32 = 0;
        if touch_pending_end {
            inner |= 1 << 16;
        }
        if touch_emulating_pointer {
            inner |= 1 << 17;
        }
        TouchEventFlags { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const TOUCH_PENDING_END: Self = Self { inner: 65536 };
    pub const TOUCH_EMULATING_POINTER: Self = Self { inner: 131072 };
    pub const COMPLETE: Self = Self { inner: 196608 };
}
impl AsByteSequence for TouchEventFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((TouchEventFlags { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for TouchEventFlags {
    type Output = TouchEventFlags;
    #[inline]
    fn not(self) -> TouchEventFlags {
        TouchEventFlags { inner: !self.inner }
    }
}
impl core::ops::BitAnd for TouchEventFlags {
    type Output = TouchEventFlags;
    #[inline]
    fn bitand(self, rhs: TouchEventFlags) -> TouchEventFlags {
        TouchEventFlags {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for TouchEventFlags {
    type Output = TouchEventFlags;
    #[inline]
    fn bitor(self, rhs: TouchEventFlags) -> TouchEventFlags {
        TouchEventFlags {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for TouchEventFlags {
    type Output = TouchEventFlags;
    #[inline]
    fn bitxor(self, rhs: TouchEventFlags) -> TouchEventFlags {
        TouchEventFlags {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum TouchOwnershipFlags {
    None = 0,
}
impl AsByteSequence for TouchOwnershipFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::None, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u32>()
    }
}
impl Default for TouchOwnershipFlags {
    #[inline]
    fn default() -> TouchOwnershipFlags {
        TouchOwnershipFlags::None
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BarrierFlags {
    pub inner: u32,
}
impl BarrierFlags {
    #[inline]
    pub fn pointer_released(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_pointer_released(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn device_is_grabbed(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_device_is_grabbed(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(pointer_released: bool, device_is_grabbed: bool) -> Self {
        let mut inner: u32 = 0;
        if pointer_released {
            inner |= 1 << 0;
        }
        if device_is_grabbed {
            inner |= 1 << 1;
        }
        BarrierFlags { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const POINTER_RELEASED: Self = Self { inner: 1 };
    pub const DEVICE_IS_GRABBED: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for BarrierFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((BarrierFlags { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for BarrierFlags {
    type Output = BarrierFlags;
    #[inline]
    fn not(self) -> BarrierFlags {
        BarrierFlags { inner: !self.inner }
    }
}
impl core::ops::BitAnd for BarrierFlags {
    type Output = BarrierFlags;
    #[inline]
    fn bitand(self, rhs: BarrierFlags) -> BarrierFlags {
        BarrierFlags {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for BarrierFlags {
    type Output = BarrierFlags;
    #[inline]
    fn bitor(self, rhs: BarrierFlags) -> BarrierFlags {
        BarrierFlags {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for BarrierFlags {
    type Output = BarrierFlags;
    #[inline]
    fn bitxor(self, rhs: BarrierFlags) -> BarrierFlags {
        BarrierFlags {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct SendExtensionEventRequest<'qd, 'rd> {
    pub req_type: u8,
    pub length: u16,
    pub destination: Window,
    pub device_id: Card8,
    pub propagate: bool,
    pub events: Cow<'qd, [EventForSend]>,
    pub classes: Cow<'rd, [EventClass]>,
}
impl<'qd, 'rd> SendExtensionEventRequest {}
impl<'qd, 'rd> AsByteSequence for SendExtensionEventRequest<'qd, 'rd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.destination.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.propagate.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.events.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 3;
        let block_len: usize = vector_as_bytes(&self.events, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventForSend>());
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SendExtensionEventRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (destination, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (propagate, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (events, block_len): (Cow<'static, [EventForSend]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventForSend>());
        let (classes, block_len): (Cow<'static, [EventClass]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
        Some((
            SendExtensionEventRequest {
                req_type: req_type,
                length: length,
                destination: destination,
                device_id: device_id,
                propagate: propagate,
                events: events,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + 1
            + self.length.size()
            + self.destination.size()
            + self.device_id.size()
            + self.propagate.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card8>()
            + 3
            + {
                let block_len: usize = self.events.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventForSend>());
                block_len + pad
            }
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<EventClass>());
                block_len + pad
            }
    }
}
impl Request for SendExtensionEventRequest {
    const OPCODE: u8 = 31;
    const EXTENSION: Option<&'static str> = Some("XInputExtension");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MoreEventsMask {
    pub inner: i32,
}
impl MoreEventsMask {
    #[inline]
    pub fn more_events(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_more_events(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(more_events: bool) -> Self {
        let mut inner: i32 = 0;
        if more_events {
            inner |= 1 << 7;
        }
        MoreEventsMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const MORE_EVENTS: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 128 };
}
impl AsByteSequence for MoreEventsMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((MoreEventsMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for MoreEventsMask {
    type Output = MoreEventsMask;
    #[inline]
    fn not(self) -> MoreEventsMask {
        MoreEventsMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for MoreEventsMask {
    type Output = MoreEventsMask;
    #[inline]
    fn bitand(self, rhs: MoreEventsMask) -> MoreEventsMask {
        MoreEventsMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for MoreEventsMask {
    type Output = MoreEventsMask;
    #[inline]
    fn bitor(self, rhs: MoreEventsMask) -> MoreEventsMask {
        MoreEventsMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for MoreEventsMask {
    type Output = MoreEventsMask;
    #[inline]
    fn bitxor(self, rhs: MoreEventsMask) -> MoreEventsMask {
        MoreEventsMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
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
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
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
        ::core::mem::size_of::<i32>()
    }
}
impl Default for NotifyDetail {
    #[inline]
    fn default() -> NotifyDetail {
        NotifyDetail::Ancestor
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, PartialOrd)]
pub enum NotifyMode {
    Normal = 0,
    Grab = 1,
    Ungrab = 2,
    WhileGrabbed = 3,
    PassiveGrab = 4,
    PassiveUngrab = 5,
}
impl AsByteSequence for NotifyMode {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Normal, sz)),
            1 => Some((Self::Grab, sz)),
            2 => Some((Self::Ungrab, sz)),
            3 => Some((Self::WhileGrabbed, sz)),
            4 => Some((Self::PassiveGrab, sz)),
            5 => Some((Self::PassiveUngrab, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for NotifyMode {
    #[inline]
    fn default() -> NotifyMode {
        NotifyMode::Normal
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct XiEventMask {
    pub inner: i32,
}
impl XiEventMask {
    #[inline]
    pub fn device_changed(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_device_changed(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn key_press(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_key_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn key_release(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_key_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn button_press(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_button_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn button_release(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_button_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn motion(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn enter(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_enter(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn leave(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_leave(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn focus_in(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_focus_in(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn focus_out(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_focus_out(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn hierarchy(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_hierarchy(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn property(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_property(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn raw_key_press(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_raw_key_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn raw_key_release(&self) -> bool {
        self.inner & (1 << 14) != 0
    }
    #[inline]
    pub fn set_raw_key_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 14;
        } else {
            self.inner &= !(1 << 14);
        }
        self
    }
    #[inline]
    pub fn raw_button_press(&self) -> bool {
        self.inner & (1 << 15) != 0
    }
    #[inline]
    pub fn set_raw_button_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 15;
        } else {
            self.inner &= !(1 << 15);
        }
        self
    }
    #[inline]
    pub fn raw_button_release(&self) -> bool {
        self.inner & (1 << 16) != 0
    }
    #[inline]
    pub fn set_raw_button_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 16;
        } else {
            self.inner &= !(1 << 16);
        }
        self
    }
    #[inline]
    pub fn raw_motion(&self) -> bool {
        self.inner & (1 << 17) != 0
    }
    #[inline]
    pub fn set_raw_motion(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 17;
        } else {
            self.inner &= !(1 << 17);
        }
        self
    }
    #[inline]
    pub fn touch_begin(&self) -> bool {
        self.inner & (1 << 18) != 0
    }
    #[inline]
    pub fn set_touch_begin(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 18;
        } else {
            self.inner &= !(1 << 18);
        }
        self
    }
    #[inline]
    pub fn touch_update(&self) -> bool {
        self.inner & (1 << 19) != 0
    }
    #[inline]
    pub fn set_touch_update(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 19;
        } else {
            self.inner &= !(1 << 19);
        }
        self
    }
    #[inline]
    pub fn touch_end(&self) -> bool {
        self.inner & (1 << 20) != 0
    }
    #[inline]
    pub fn set_touch_end(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 20;
        } else {
            self.inner &= !(1 << 20);
        }
        self
    }
    #[inline]
    pub fn touch_ownership(&self) -> bool {
        self.inner & (1 << 21) != 0
    }
    #[inline]
    pub fn set_touch_ownership(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 21;
        } else {
            self.inner &= !(1 << 21);
        }
        self
    }
    #[inline]
    pub fn raw_touch_begin(&self) -> bool {
        self.inner & (1 << 22) != 0
    }
    #[inline]
    pub fn set_raw_touch_begin(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 22;
        } else {
            self.inner &= !(1 << 22);
        }
        self
    }
    #[inline]
    pub fn raw_touch_update(&self) -> bool {
        self.inner & (1 << 23) != 0
    }
    #[inline]
    pub fn set_raw_touch_update(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 23;
        } else {
            self.inner &= !(1 << 23);
        }
        self
    }
    #[inline]
    pub fn raw_touch_end(&self) -> bool {
        self.inner & (1 << 24) != 0
    }
    #[inline]
    pub fn set_raw_touch_end(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 24;
        } else {
            self.inner &= !(1 << 24);
        }
        self
    }
    #[inline]
    pub fn barrier_hit(&self) -> bool {
        self.inner & (1 << 25) != 0
    }
    #[inline]
    pub fn set_barrier_hit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 25;
        } else {
            self.inner &= !(1 << 25);
        }
        self
    }
    #[inline]
    pub fn barrier_leave(&self) -> bool {
        self.inner & (1 << 26) != 0
    }
    #[inline]
    pub fn set_barrier_leave(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 26;
        } else {
            self.inner &= !(1 << 26);
        }
        self
    }
    #[inline]
    pub fn new(
        device_changed: bool,
        key_press: bool,
        key_release: bool,
        button_press: bool,
        button_release: bool,
        motion: bool,
        enter: bool,
        leave: bool,
        focus_in: bool,
        focus_out: bool,
        hierarchy: bool,
        property: bool,
        raw_key_press: bool,
        raw_key_release: bool,
        raw_button_press: bool,
        raw_button_release: bool,
        raw_motion: bool,
        touch_begin: bool,
        touch_update: bool,
        touch_end: bool,
        touch_ownership: bool,
        raw_touch_begin: bool,
        raw_touch_update: bool,
        raw_touch_end: bool,
        barrier_hit: bool,
        barrier_leave: bool,
    ) -> Self {
        let mut inner: i32 = 0;
        if device_changed {
            inner |= 1 << 1;
        }
        if key_press {
            inner |= 1 << 2;
        }
        if key_release {
            inner |= 1 << 3;
        }
        if button_press {
            inner |= 1 << 4;
        }
        if button_release {
            inner |= 1 << 5;
        }
        if motion {
            inner |= 1 << 6;
        }
        if enter {
            inner |= 1 << 7;
        }
        if leave {
            inner |= 1 << 8;
        }
        if focus_in {
            inner |= 1 << 9;
        }
        if focus_out {
            inner |= 1 << 10;
        }
        if hierarchy {
            inner |= 1 << 11;
        }
        if property {
            inner |= 1 << 12;
        }
        if raw_key_press {
            inner |= 1 << 13;
        }
        if raw_key_release {
            inner |= 1 << 14;
        }
        if raw_button_press {
            inner |= 1 << 15;
        }
        if raw_button_release {
            inner |= 1 << 16;
        }
        if raw_motion {
            inner |= 1 << 17;
        }
        if touch_begin {
            inner |= 1 << 18;
        }
        if touch_update {
            inner |= 1 << 19;
        }
        if touch_end {
            inner |= 1 << 20;
        }
        if touch_ownership {
            inner |= 1 << 21;
        }
        if raw_touch_begin {
            inner |= 1 << 22;
        }
        if raw_touch_update {
            inner |= 1 << 23;
        }
        if raw_touch_end {
            inner |= 1 << 24;
        }
        if barrier_hit {
            inner |= 1 << 25;
        }
        if barrier_leave {
            inner |= 1 << 26;
        }
        XiEventMask { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const DEVICE_CHANGED: Self = Self { inner: 2 };
    pub const KEY_PRESS: Self = Self { inner: 4 };
    pub const KEY_RELEASE: Self = Self { inner: 8 };
    pub const BUTTON_PRESS: Self = Self { inner: 16 };
    pub const BUTTON_RELEASE: Self = Self { inner: 32 };
    pub const MOTION: Self = Self { inner: 64 };
    pub const ENTER: Self = Self { inner: 128 };
    pub const LEAVE: Self = Self { inner: 256 };
    pub const FOCUS_IN: Self = Self { inner: 512 };
    pub const FOCUS_OUT: Self = Self { inner: 1024 };
    pub const HIERARCHY: Self = Self { inner: 2048 };
    pub const PROPERTY: Self = Self { inner: 4096 };
    pub const RAW_KEY_PRESS: Self = Self { inner: 8192 };
    pub const RAW_KEY_RELEASE: Self = Self { inner: 16384 };
    pub const RAW_BUTTON_PRESS: Self = Self { inner: 32768 };
    pub const RAW_BUTTON_RELEASE: Self = Self { inner: 65536 };
    pub const RAW_MOTION: Self = Self { inner: 131072 };
    pub const TOUCH_BEGIN: Self = Self { inner: 262144 };
    pub const TOUCH_UPDATE: Self = Self { inner: 524288 };
    pub const TOUCH_END: Self = Self { inner: 1048576 };
    pub const TOUCH_OWNERSHIP: Self = Self { inner: 2097152 };
    pub const RAW_TOUCH_BEGIN: Self = Self { inner: 4194304 };
    pub const RAW_TOUCH_UPDATE: Self = Self { inner: 8388608 };
    pub const RAW_TOUCH_END: Self = Self { inner: 16777216 };
    pub const BARRIER_HIT: Self = Self { inner: 33554432 };
    pub const BARRIER_LEAVE: Self = Self { inner: 67108864 };
    pub const COMPLETE: Self = Self { inner: 134217726 };
}
impl AsByteSequence for XiEventMask {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((XiEventMask { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for XiEventMask {
    type Output = XiEventMask;
    #[inline]
    fn not(self) -> XiEventMask {
        XiEventMask { inner: !self.inner }
    }
}
impl core::ops::BitAnd for XiEventMask {
    type Output = XiEventMask;
    #[inline]
    fn bitand(self, rhs: XiEventMask) -> XiEventMask {
        XiEventMask {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for XiEventMask {
    type Output = XiEventMask;
    #[inline]
    fn bitor(self, rhs: XiEventMask) -> XiEventMask {
        XiEventMask {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for XiEventMask {
    type Output = XiEventMask;
    #[inline]
    fn bitxor(self, rhs: XiEventMask) -> XiEventMask {
        XiEventMask {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct BarrierHitEvent {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub eventid: Card32,
    pub root: Window,
    pub event: Window,
    pub barrier: Barrier,
    pub dtime: Card32,
    pub flags: BarrierFlags,
    pub sourceid: DeviceId,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub dx: Fp3232,
    pub dy: Fp3232,
}
impl BarrierHitEvent {}
impl AsByteSequence for BarrierHitEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.eventid.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.barrier.as_bytes(&mut bytes[index..]);
        index += self.dtime.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.dx.as_bytes(&mut bytes[index..]);
        index += self.dy.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BarrierHitEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (eventid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (barrier, sz): (Barrier, usize) = <Barrier>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dtime, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (BarrierFlags, usize) = <BarrierFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dx, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dy, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BarrierHitEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                eventid: eventid,
                root: root,
                event: event,
                barrier: barrier,
                dtime: dtime,
                flags: flags,
                sourceid: sourceid,
                root_x: root_x,
                root_y: root_y,
                dx: dx,
                dy: dy,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.eventid.size()
            + self.root.size()
            + self.event.size()
            + self.barrier.size()
            + self.dtime.size()
            + self.flags.size()
            + self.sourceid.size()
            + 2
            + self.root_x.size()
            + self.root_y.size()
            + self.dx.size()
            + self.dy.size()
    }
}
impl crate::auto::Event for BarrierHitEvent {
    const OPCODE: u8 = 25;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct BarrierLeaveEvent {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub eventid: Card32,
    pub root: Window,
    pub event: Window,
    pub barrier: Barrier,
    pub dtime: Card32,
    pub flags: BarrierFlags,
    pub sourceid: DeviceId,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub dx: Fp3232,
    pub dy: Fp3232,
}
impl BarrierLeaveEvent {}
impl AsByteSequence for BarrierLeaveEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.eventid.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.barrier.as_bytes(&mut bytes[index..]);
        index += self.dtime.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.dx.as_bytes(&mut bytes[index..]);
        index += self.dy.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BarrierLeaveEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (eventid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (barrier, sz): (Barrier, usize) = <Barrier>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dtime, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (BarrierFlags, usize) = <BarrierFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dx, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dy, sz): (Fp3232, usize) = <Fp3232>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BarrierLeaveEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                eventid: eventid,
                root: root,
                event: event,
                barrier: barrier,
                dtime: dtime,
                flags: flags,
                sourceid: sourceid,
                root_x: root_x,
                root_y: root_y,
                dx: dx,
                dy: dy,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.eventid.size()
            + self.root.size()
            + self.event.size()
            + self.barrier.size()
            + self.dtime.size()
            + self.flags.size()
            + self.sourceid.size()
            + 2
            + self.root_x.size()
            + self.root_y.size()
            + self.dx.size()
            + self.dy.size()
    }
}
impl crate::auto::Event for BarrierLeaveEvent {
    const OPCODE: u8 = 26;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ButtonPressEvent<'sd, 'td, 'ud> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: PointerEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'sd, [Card32]>,
    pub valuator_mask: Cow<'td, [Card32]>,
    pub axisvalues: Cow<'ud, [Fp3232]>,
}
impl<'sd, 'td, 'ud> ButtonPressEvent {}
impl<'sd, 'td, 'ud> AsByteSequence for ButtonPressEvent<'sd, 'td, 'ud> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ButtonPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (PointerEventFlags, usize) =
            <PointerEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            ButtonPressEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for ButtonPressEvent {
    const OPCODE: u8 = 4;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ButtonReleaseEvent<'vd, 'wd, 'xd> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: PointerEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'vd, [Card32]>,
    pub valuator_mask: Cow<'wd, [Card32]>,
    pub axisvalues: Cow<'xd, [Fp3232]>,
}
impl<'vd, 'wd, 'xd> ButtonReleaseEvent {}
impl<'vd, 'wd, 'xd> AsByteSequence for ButtonReleaseEvent<'vd, 'wd, 'xd> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ButtonReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (PointerEventFlags, usize) =
            <PointerEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            ButtonReleaseEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for ButtonReleaseEvent {
    const OPCODE: u8 = 5;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ChangeDeviceNotifyEvent {
    pub event_type: u8,
    pub device_id: Byte,
    pub sequence: u16,
    pub time: Timestamp,
    pub request: ChangeDevice,
}
impl ChangeDeviceNotifyEvent {}
impl AsByteSequence for ChangeDeviceNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.request.as_bytes(&mut bytes[index..]);
        index += 23;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ChangeDeviceNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request, sz): (ChangeDevice, usize) = <ChangeDevice>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 23;
        Some((
            ChangeDeviceNotifyEvent {
                event_type: event_type,
                device_id: device_id,
                sequence: sequence,
                time: time,
                request: request,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.time.size()
            + self.request.size()
            + 23
    }
}
impl crate::auto::Event for ChangeDeviceNotifyEvent {
    const OPCODE: u8 = 12;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceButtonPressEvent {
    pub event_type: u8,
    pub detail: Byte,
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
    pub device_id: Card8,
}
impl DeviceButtonPressEvent {}
impl AsByteSequence for DeviceButtonPressEvent {
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
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceButtonPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
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
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceButtonPressEvent {
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
                device_id: device_id,
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
            + self.device_id.size()
    }
}
impl crate::auto::Event for DeviceButtonPressEvent {
    const OPCODE: u8 = 3;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceButtonReleaseEvent {
    pub event_type: u8,
    pub detail: Byte,
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
    pub device_id: Card8,
}
impl DeviceButtonReleaseEvent {}
impl AsByteSequence for DeviceButtonReleaseEvent {
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
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceButtonReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
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
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceButtonReleaseEvent {
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
                device_id: device_id,
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
            + self.device_id.size()
    }
}
impl crate::auto::Event for DeviceButtonReleaseEvent {
    const OPCODE: u8 = 4;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceButtonStateNotifyEvent {
    pub event_type: u8,
    pub device_id: Byte,
    pub sequence: u16,
    pub buttons: [Card8; 28],
}
impl DeviceButtonStateNotifyEvent {}
impl AsByteSequence for DeviceButtonStateNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.buttons.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceButtonStateNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buttons, sz): ([Card8; 28], usize) = <[Card8; 28]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceButtonStateNotifyEvent {
                event_type: event_type,
                device_id: device_id,
                sequence: sequence,
                buttons: buttons,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + self.device_id.size() + self.sequence.size() + self.buttons.size()
    }
}
impl crate::auto::Event for DeviceButtonStateNotifyEvent {
    const OPCODE: u8 = 14;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceChangedEvent<'be, 'yd, 'zd, 'ae> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub sourceid: DeviceId,
    pub reason: ChangeReason,
    pub classes: Cow<'be, [DeviceClass<'yd, 'zd, 'ae>]>,
}
impl<'be, 'yd, 'zd, 'ae> DeviceChangedEvent {}
impl<'be, 'yd, 'zd, 'ae> AsByteSequence for DeviceChangedEvent<'be, 'yd, 'zd, 'ae> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += (self.classes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.reason.as_bytes(&mut bytes[index..]);
        index += 11;
        let block_len: usize = vector_as_bytes(&self.classes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<DeviceClass<'yd, 'zd, 'ae>>(),
        );
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceChangedEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reason, sz): (ChangeReason, usize) = <ChangeReason>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 11;
        let (classes, block_len): (Cow<'static, [DeviceClass<'yd, 'zd, 'ae>]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(
            block_len,
            ::core::mem::align_of::<DeviceClass<'yd, 'zd, 'ae>>(),
        );
        Some((
            DeviceChangedEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                sourceid: sourceid,
                reason: reason,
                classes: classes,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + self.reason.size()
            + 11
            + {
                let block_len: usize = self.classes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(
                    block_len,
                    ::core::mem::align_of::<DeviceClass<'yd, 'zd, 'ae>>(),
                );
                block_len + pad
            }
    }
}
impl crate::auto::Event for DeviceChangedEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceFocusInEvent {
    pub event_type: u8,
    pub detail: super::xproto::NotifyDetail,
    pub sequence: u16,
    pub time: Timestamp,
    pub window: Window,
    pub mode: super::xproto::NotifyMode,
    pub device_id: Card8,
}
impl DeviceFocusInEvent {}
impl AsByteSequence for DeviceFocusInEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 18;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceFocusInEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (super::xproto::NotifyDetail, usize) =
            <super::xproto::NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (super::xproto::NotifyMode, usize) =
            <super::xproto::NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 18;
        Some((
            DeviceFocusInEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                window: window,
                mode: mode,
                device_id: device_id,
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
            + self.window.size()
            + self.mode.size()
            + self.device_id.size()
            + 18
    }
}
impl crate::auto::Event for DeviceFocusInEvent {
    const OPCODE: u8 = 6;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceFocusOutEvent {
    pub event_type: u8,
    pub detail: super::xproto::NotifyDetail,
    pub sequence: u16,
    pub time: Timestamp,
    pub window: Window,
    pub mode: super::xproto::NotifyMode,
    pub device_id: Card8,
}
impl DeviceFocusOutEvent {}
impl AsByteSequence for DeviceFocusOutEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 18;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceFocusOutEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (super::xproto::NotifyDetail, usize) =
            <super::xproto::NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (super::xproto::NotifyMode, usize) =
            <super::xproto::NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 18;
        Some((
            DeviceFocusOutEvent {
                event_type: event_type,
                detail: detail,
                sequence: sequence,
                time: time,
                window: window,
                mode: mode,
                device_id: device_id,
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
            + self.window.size()
            + self.mode.size()
            + self.device_id.size()
            + 18
    }
}
impl crate::auto::Event for DeviceFocusOutEvent {
    const OPCODE: u8 = 7;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceKeyPressEvent {
    pub event_type: u8,
    pub detail: Byte,
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
    pub device_id: Card8,
}
impl DeviceKeyPressEvent {}
impl AsByteSequence for DeviceKeyPressEvent {
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
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceKeyPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
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
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceKeyPressEvent {
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
                device_id: device_id,
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
            + self.device_id.size()
    }
}
impl crate::auto::Event for DeviceKeyPressEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceKeyReleaseEvent {
    pub event_type: u8,
    pub detail: Byte,
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
    pub device_id: Card8,
}
impl DeviceKeyReleaseEvent {}
impl AsByteSequence for DeviceKeyReleaseEvent {
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
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceKeyReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
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
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceKeyReleaseEvent {
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
                device_id: device_id,
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
            + self.device_id.size()
    }
}
impl crate::auto::Event for DeviceKeyReleaseEvent {
    const OPCODE: u8 = 2;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceKeyStateNotifyEvent {
    pub event_type: u8,
    pub device_id: Byte,
    pub sequence: u16,
    pub keys: [Card8; 28],
}
impl DeviceKeyStateNotifyEvent {}
impl AsByteSequence for DeviceKeyStateNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.keys.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceKeyStateNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keys, sz): ([Card8; 28], usize) = <[Card8; 28]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceKeyStateNotifyEvent {
                event_type: event_type,
                device_id: device_id,
                sequence: sequence,
                keys: keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size() + self.device_id.size() + self.sequence.size() + self.keys.size()
    }
}
impl crate::auto::Event for DeviceKeyStateNotifyEvent {
    const OPCODE: u8 = 13;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceMappingNotifyEvent {
    pub event_type: u8,
    pub device_id: Byte,
    pub sequence: u16,
    pub request: Mapping,
    pub first_keycode: KeyCode,
    pub count: Card8,
    pub time: Timestamp,
}
impl DeviceMappingNotifyEvent {}
impl AsByteSequence for DeviceMappingNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.request.as_bytes(&mut bytes[index..]);
        index += self.first_keycode.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.time.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceMappingNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request, sz): (Mapping, usize) = <Mapping>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_keycode, sz): (KeyCode, usize) = <KeyCode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            DeviceMappingNotifyEvent {
                event_type: event_type,
                device_id: device_id,
                sequence: sequence,
                request: request,
                first_keycode: first_keycode,
                count: count,
                time: time,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.request.size()
            + self.first_keycode.size()
            + self.count.size()
            + 1
            + self.time.size()
            + 20
    }
}
impl crate::auto::Event for DeviceMappingNotifyEvent {
    const OPCODE: u8 = 11;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceMotionNotifyEvent {
    pub event_type: u8,
    pub detail: Byte,
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
    pub device_id: Card8,
}
impl DeviceMotionNotifyEvent {}
impl AsByteSequence for DeviceMotionNotifyEvent {
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
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceMotionNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
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
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceMotionNotifyEvent {
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
                device_id: device_id,
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
            + self.device_id.size()
    }
}
impl crate::auto::Event for DeviceMotionNotifyEvent {
    const OPCODE: u8 = 5;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DevicePresenceNotifyEvent {
    pub event_type: u8,
    pub sequence: u16,
    pub time: Timestamp,
    pub devchange: DeviceChange,
    pub device_id: Byte,
    pub control: Card16,
}
impl DevicePresenceNotifyEvent {}
impl AsByteSequence for DevicePresenceNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.devchange.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.control.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DevicePresenceNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (devchange, sz): (DeviceChange, usize) = <DeviceChange>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (control, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            DevicePresenceNotifyEvent {
                event_type: event_type,
                sequence: sequence,
                time: time,
                devchange: devchange,
                device_id: device_id,
                control: control,
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
            + self.devchange.size()
            + self.device_id.size()
            + self.control.size()
            + 20
    }
}
impl crate::auto::Event for DevicePresenceNotifyEvent {
    const OPCODE: u8 = 15;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DevicePropertyNotifyEvent {
    pub event_type: u8,
    pub state: Property,
    pub sequence: u16,
    pub time: Timestamp,
    pub property: Atom,
    pub device_id: Card8,
}
impl DevicePropertyNotifyEvent {}
impl AsByteSequence for DevicePropertyNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += 19;
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DevicePropertyNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Property, usize) = <Property>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 19;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DevicePropertyNotifyEvent {
                event_type: event_type,
                state: state,
                sequence: sequence,
                time: time,
                property: property,
                device_id: device_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.state.size()
            + self.sequence.size()
            + self.time.size()
            + self.property.size()
            + 19
            + self.device_id.size()
    }
}
impl crate::auto::Event for DevicePropertyNotifyEvent {
    const OPCODE: u8 = 16;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceStateNotifyEvent {
    pub event_type: u8,
    pub device_id: Byte,
    pub sequence: u16,
    pub time: Timestamp,
    pub num_keys: Card8,
    pub num_buttons: Card8,
    pub num_valuators: Card8,
    pub classes_reported: ClassesReportedMask,
    pub buttons: [Card8; 4],
    pub keys: [Card8; 4],
    pub valuators: [Card32; 3],
}
impl DeviceStateNotifyEvent {}
impl AsByteSequence for DeviceStateNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.num_keys.as_bytes(&mut bytes[index..]);
        index += self.num_buttons.as_bytes(&mut bytes[index..]);
        index += self.num_valuators.as_bytes(&mut bytes[index..]);
        index += self.classes_reported.as_bytes(&mut bytes[index..]);
        index += self.buttons.as_bytes(&mut bytes[index..]);
        index += self.keys.as_bytes(&mut bytes[index..]);
        index += self.valuators.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceStateNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_buttons, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_valuators, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (classes_reported, sz): (ClassesReportedMask, usize) =
            <ClassesReportedMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buttons, sz): ([Card8; 4], usize) = <[Card8; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keys, sz): ([Card8; 4], usize) = <[Card8; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (valuators, sz): ([Card32; 3], usize) = <[Card32; 3]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceStateNotifyEvent {
                event_type: event_type,
                device_id: device_id,
                sequence: sequence,
                time: time,
                num_keys: num_keys,
                num_buttons: num_buttons,
                num_valuators: num_valuators,
                classes_reported: classes_reported,
                buttons: buttons,
                keys: keys,
                valuators: valuators,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.time.size()
            + self.num_keys.size()
            + self.num_buttons.size()
            + self.num_valuators.size()
            + self.classes_reported.size()
            + self.buttons.size()
            + self.keys.size()
            + self.valuators.size()
    }
}
impl crate::auto::Event for DeviceStateNotifyEvent {
    const OPCODE: u8 = 10;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct DeviceValuatorEvent {
    pub event_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub device_state: Card16,
    pub num_valuators: Card8,
    pub first_valuator: Card8,
    pub valuators: [Int32; 6],
}
impl DeviceValuatorEvent {}
impl AsByteSequence for DeviceValuatorEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.device_state.as_bytes(&mut bytes[index..]);
        index += self.num_valuators.as_bytes(&mut bytes[index..]);
        index += self.first_valuator.as_bytes(&mut bytes[index..]);
        index += self.valuators.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceValuatorEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_state, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_valuators, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_valuator, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (valuators, sz): ([Int32; 6], usize) = <[Int32; 6]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            DeviceValuatorEvent {
                event_type: event_type,
                device_id: device_id,
                sequence: sequence,
                device_state: device_state,
                num_valuators: num_valuators,
                first_valuator: first_valuator,
                valuators: valuators,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.device_state.size()
            + self.num_valuators.size()
            + self.first_valuator.size()
            + self.valuators.size()
    }
}
impl crate::auto::Event for DeviceValuatorEvent {
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct EnterEvent<'ce> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub sourceid: DeviceId,
    pub mode: super::xinput::NotifyMode,
    pub detail: super::xinput::NotifyDetail,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Cow<'ce, [Card32]>,
}
impl<'ce> EnterEvent {}
impl<'ce> AsByteSequence for EnterEvent<'ce> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += (self.buttons.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.buttons, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing EnterEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (super::xinput::NotifyMode, usize) =
            <super::xinput::NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (super::xinput::NotifyDetail, usize) =
            <super::xinput::NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buttons, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            EnterEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                sourceid: sourceid,
                mode: mode,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                same_screen: same_screen,
                focus: focus,
                mods: mods,
                group: group,
                buttons: buttons,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.sourceid.size()
            + self.mode.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.same_screen.size()
            + self.focus.size()
            + ::core::mem::size_of::<Card16>()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.buttons.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for EnterEvent {
    const OPCODE: u8 = 7;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct FocusInEvent<'de> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub sourceid: DeviceId,
    pub mode: super::xinput::NotifyMode,
    pub detail: super::xinput::NotifyDetail,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Cow<'de, [Card32]>,
}
impl<'de> FocusInEvent {}
impl<'de> AsByteSequence for FocusInEvent<'de> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += (self.buttons.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.buttons, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FocusInEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (super::xinput::NotifyMode, usize) =
            <super::xinput::NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (super::xinput::NotifyDetail, usize) =
            <super::xinput::NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buttons, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            FocusInEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                sourceid: sourceid,
                mode: mode,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                same_screen: same_screen,
                focus: focus,
                mods: mods,
                group: group,
                buttons: buttons,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.sourceid.size()
            + self.mode.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.same_screen.size()
            + self.focus.size()
            + ::core::mem::size_of::<Card16>()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.buttons.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for FocusInEvent {
    const OPCODE: u8 = 9;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct FocusOutEvent<'ee> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub sourceid: DeviceId,
    pub mode: super::xinput::NotifyMode,
    pub detail: super::xinput::NotifyDetail,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Cow<'ee, [Card32]>,
}
impl<'ee> FocusOutEvent {}
impl<'ee> AsByteSequence for FocusOutEvent<'ee> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += (self.buttons.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.buttons, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing FocusOutEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (super::xinput::NotifyMode, usize) =
            <super::xinput::NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (super::xinput::NotifyDetail, usize) =
            <super::xinput::NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buttons, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            FocusOutEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                sourceid: sourceid,
                mode: mode,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                same_screen: same_screen,
                focus: focus,
                mods: mods,
                group: group,
                buttons: buttons,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.sourceid.size()
            + self.mode.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.same_screen.size()
            + self.focus.size()
            + ::core::mem::size_of::<Card16>()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.buttons.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for FocusOutEvent {
    const OPCODE: u8 = 10;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct HierarchyEvent<'fe> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub flags: HierarchyMask,
    pub infos: Cow<'fe, [HierarchyInfo]>,
}
impl<'fe> HierarchyEvent {}
impl<'fe> AsByteSequence for HierarchyEvent<'fe> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += (self.infos.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 10;
        let block_len: usize = vector_as_bytes(&self.infos, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<HierarchyInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing HierarchyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (HierarchyMask, usize) = <HierarchyMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 10;
        let (infos, block_len): (Cow<'static, [HierarchyInfo]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<HierarchyInfo>());
        Some((
            HierarchyEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                flags: flags,
                infos: infos,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.flags.size()
            + ::core::mem::size_of::<Card16>()
            + 10
            + {
                let block_len: usize = self.infos.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<HierarchyInfo>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for HierarchyEvent {
    const OPCODE: u8 = 11;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct KeyPressEvent<'ge, 'he, 'ie> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: KeyEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'ge, [Card32]>,
    pub valuator_mask: Cow<'he, [Card32]>,
    pub axisvalues: Cow<'ie, [Fp3232]>,
}
impl<'ge, 'he, 'ie> KeyPressEvent {}
impl<'ge, 'he, 'ie> AsByteSequence for KeyPressEvent<'ge, 'he, 'ie> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (KeyEventFlags, usize) = <KeyEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            KeyPressEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for KeyPressEvent {
    const OPCODE: u8 = 2;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct KeyReleaseEvent<'je, 'ke, 'le> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: KeyEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'je, [Card32]>,
    pub valuator_mask: Cow<'ke, [Card32]>,
    pub axisvalues: Cow<'le, [Fp3232]>,
}
impl<'je, 'ke, 'le> KeyReleaseEvent {}
impl<'je, 'ke, 'le> AsByteSequence for KeyReleaseEvent<'je, 'ke, 'le> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (KeyEventFlags, usize) = <KeyEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            KeyReleaseEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for KeyReleaseEvent {
    const OPCODE: u8 = 3;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct LeaveEvent<'me> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub sourceid: DeviceId,
    pub mode: super::xinput::NotifyMode,
    pub detail: super::xinput::NotifyDetail,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub same_screen: bool,
    pub focus: bool,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub buttons: Cow<'me, [Card32]>,
}
impl<'me> LeaveEvent {}
impl<'me> AsByteSequence for LeaveEvent<'me> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += self.mode.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += self.same_screen.as_bytes(&mut bytes[index..]);
        index += self.focus.as_bytes(&mut bytes[index..]);
        index += (self.buttons.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.buttons, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LeaveEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mode, sz): (super::xinput::NotifyMode, usize) =
            <super::xinput::NotifyMode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (super::xinput::NotifyDetail, usize) =
            <super::xinput::NotifyDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (same_screen, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (focus, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (buttons, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        Some((
            LeaveEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                sourceid: sourceid,
                mode: mode,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                same_screen: same_screen,
                focus: focus,
                mods: mods,
                group: group,
                buttons: buttons,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.sourceid.size()
            + self.mode.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + self.same_screen.size()
            + self.focus.size()
            + ::core::mem::size_of::<Card16>()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.buttons.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for LeaveEvent {
    const OPCODE: u8 = 8;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct MotionEvent<'ne, 'oe, 'pe> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: PointerEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'ne, [Card32]>,
    pub valuator_mask: Cow<'oe, [Card32]>,
    pub axisvalues: Cow<'pe, [Fp3232]>,
}
impl<'ne, 'oe, 'pe> MotionEvent {}
impl<'ne, 'oe, 'pe> AsByteSequence for MotionEvent<'ne, 'oe, 'pe> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MotionEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (PointerEventFlags, usize) =
            <PointerEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            MotionEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for MotionEvent {
    const OPCODE: u8 = 6;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct PropertyEvent {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub property: Atom,
    pub what: PropertyFlag,
}
impl PropertyEvent {}
impl AsByteSequence for PropertyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.property.as_bytes(&mut bytes[index..]);
        index += self.what.as_bytes(&mut bytes[index..]);
        index += 11;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PropertyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (property, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (what, sz): (PropertyFlag, usize) = <PropertyFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 11;
        Some((
            PropertyEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                property: property,
                what: what,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.property.size()
            + self.what.size()
            + 11
    }
}
impl crate::auto::Event for PropertyEvent {
    const OPCODE: u8 = 12;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ProximityInEvent {
    pub event_type: u8,
    pub detail: Byte,
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
    pub device_id: Card8,
}
impl ProximityInEvent {}
impl AsByteSequence for ProximityInEvent {
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
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ProximityInEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
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
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ProximityInEvent {
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
                device_id: device_id,
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
            + self.device_id.size()
    }
}
impl crate::auto::Event for ProximityInEvent {
    const OPCODE: u8 = 8;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct ProximityOutEvent {
    pub event_type: u8,
    pub detail: Byte,
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
    pub device_id: Card8,
}
impl ProximityOutEvent {}
impl AsByteSequence for ProximityOutEvent {
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
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ProximityOutEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Byte, usize) = <Byte>::from_bytes(&bytes[index..])?;
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
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ProximityOutEvent {
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
                device_id: device_id,
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
            + self.device_id.size()
    }
}
impl crate::auto::Event for ProximityOutEvent {
    const OPCODE: u8 = 9;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawButtonPressEvent<'qe, 're, 'se> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: PointerEventFlags,
    pub valuator_mask: Cow<'qe, [Card32]>,
    pub axisvalues: Cow<'re, [Fp3232]>,
    pub axisvalues_raw: Cow<'se, [Fp3232]>,
}
impl<'qe, 're, 'se> RawButtonPressEvent {}
impl<'qe, 're, 'se> AsByteSequence for RawButtonPressEvent<'qe, 're, 'se> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawButtonPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (PointerEventFlags, usize) =
            <PointerEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawButtonPressEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawButtonPressEvent {
    const OPCODE: u8 = 15;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawButtonReleaseEvent<'te, 'ue, 've> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: PointerEventFlags,
    pub valuator_mask: Cow<'te, [Card32]>,
    pub axisvalues: Cow<'ue, [Fp3232]>,
    pub axisvalues_raw: Cow<'ve, [Fp3232]>,
}
impl<'te, 'ue, 've> RawButtonReleaseEvent {}
impl<'te, 'ue, 've> AsByteSequence for RawButtonReleaseEvent<'te, 'ue, 've> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawButtonReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (PointerEventFlags, usize) =
            <PointerEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawButtonReleaseEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawButtonReleaseEvent {
    const OPCODE: u8 = 16;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawKeyPressEvent<'we, 'xe, 'ye> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: KeyEventFlags,
    pub valuator_mask: Cow<'we, [Card32]>,
    pub axisvalues: Cow<'xe, [Fp3232]>,
    pub axisvalues_raw: Cow<'ye, [Fp3232]>,
}
impl<'we, 'xe, 'ye> RawKeyPressEvent {}
impl<'we, 'xe, 'ye> AsByteSequence for RawKeyPressEvent<'we, 'xe, 'ye> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawKeyPressEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (KeyEventFlags, usize) = <KeyEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawKeyPressEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawKeyPressEvent {
    const OPCODE: u8 = 13;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawKeyReleaseEvent<'ze, 'af, 'bf> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: KeyEventFlags,
    pub valuator_mask: Cow<'ze, [Card32]>,
    pub axisvalues: Cow<'af, [Fp3232]>,
    pub axisvalues_raw: Cow<'bf, [Fp3232]>,
}
impl<'ze, 'af, 'bf> RawKeyReleaseEvent {}
impl<'ze, 'af, 'bf> AsByteSequence for RawKeyReleaseEvent<'ze, 'af, 'bf> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawKeyReleaseEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (KeyEventFlags, usize) = <KeyEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawKeyReleaseEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawKeyReleaseEvent {
    const OPCODE: u8 = 14;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawMotionEvent<'cf, 'df, 'ef> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: PointerEventFlags,
    pub valuator_mask: Cow<'cf, [Card32]>,
    pub axisvalues: Cow<'df, [Fp3232]>,
    pub axisvalues_raw: Cow<'ef, [Fp3232]>,
}
impl<'cf, 'df, 'ef> RawMotionEvent {}
impl<'cf, 'df, 'ef> AsByteSequence for RawMotionEvent<'cf, 'df, 'ef> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawMotionEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (PointerEventFlags, usize) =
            <PointerEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawMotionEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawMotionEvent {
    const OPCODE: u8 = 17;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawTouchBeginEvent<'ff, 'gf, 'hf> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: TouchEventFlags,
    pub valuator_mask: Cow<'ff, [Card32]>,
    pub axisvalues: Cow<'gf, [Fp3232]>,
    pub axisvalues_raw: Cow<'hf, [Fp3232]>,
}
impl<'ff, 'gf, 'hf> RawTouchBeginEvent {}
impl<'ff, 'gf, 'hf> AsByteSequence for RawTouchBeginEvent<'ff, 'gf, 'hf> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawTouchBeginEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (TouchEventFlags, usize) = <TouchEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawTouchBeginEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawTouchBeginEvent {
    const OPCODE: u8 = 22;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawTouchEndEvent<'if, 'jf, 'kf> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: TouchEventFlags,
    pub valuator_mask: Cow<'if, [Card32]>,
    pub axisvalues: Cow<'jf, [Fp3232]>,
    pub axisvalues_raw: Cow<'kf, [Fp3232]>,
}
impl<'if, 'jf, 'kf> RawTouchEndEvent {}
impl<'if, 'jf, 'kf> AsByteSequence for RawTouchEndEvent<'if, 'jf, 'kf> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawTouchEndEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (TouchEventFlags, usize) = <TouchEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawTouchEndEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawTouchEndEvent {
    const OPCODE: u8 = 24;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct RawTouchUpdateEvent<'lf, 'mf, 'nf> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub sourceid: DeviceId,
    pub flags: TouchEventFlags,
    pub valuator_mask: Cow<'lf, [Card32]>,
    pub axisvalues: Cow<'mf, [Fp3232]>,
    pub axisvalues_raw: Cow<'nf, [Fp3232]>,
}
impl<'lf, 'mf, 'nf> RawTouchUpdateEvent {}
impl<'lf, 'mf, 'nf> AsByteSequence for RawTouchUpdateEvent<'lf, 'mf, 'nf> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 4;
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let block_len: usize = vector_as_bytes(&self.axisvalues_raw, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RawTouchUpdateEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (TouchEventFlags, usize) = <TouchEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        let (axisvalues_raw, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            RawTouchUpdateEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                sourceid: sourceid,
                flags: flags,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
                axisvalues_raw: axisvalues_raw,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.sourceid.size()
            + ::core::mem::size_of::<Card16>()
            + self.flags.size()
            + 4
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues_raw.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for RawTouchUpdateEvent {
    const OPCODE: u8 = 23;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct TouchBeginEvent<'of, 'pf, 'qf> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: TouchEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'of, [Card32]>,
    pub valuator_mask: Cow<'pf, [Card32]>,
    pub axisvalues: Cow<'qf, [Fp3232]>,
}
impl<'of, 'pf, 'qf> TouchBeginEvent {}
impl<'of, 'pf, 'qf> AsByteSequence for TouchBeginEvent<'of, 'pf, 'qf> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TouchBeginEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (TouchEventFlags, usize) = <TouchEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            TouchBeginEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for TouchBeginEvent {
    const OPCODE: u8 = 18;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct TouchEndEvent<'rf, 'sf, 'tf> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: TouchEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'rf, [Card32]>,
    pub valuator_mask: Cow<'sf, [Card32]>,
    pub axisvalues: Cow<'tf, [Fp3232]>,
}
impl<'rf, 'sf, 'tf> TouchEndEvent {}
impl<'rf, 'sf, 'tf> AsByteSequence for TouchEndEvent<'rf, 'sf, 'tf> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TouchEndEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (TouchEventFlags, usize) = <TouchEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            TouchEndEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for TouchEndEvent {
    const OPCODE: u8 = 20;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct TouchOwnershipEvent {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub touchid: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub sourceid: DeviceId,
    pub flags: TouchOwnershipFlags,
}
impl TouchOwnershipEvent {}
impl AsByteSequence for TouchOwnershipEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.touchid.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 8;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TouchOwnershipEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (touchid, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (TouchOwnershipFlags, usize) =
            <TouchOwnershipFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        Some((
            TouchOwnershipEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                touchid: touchid,
                root: root,
                event: event,
                child: child,
                sourceid: sourceid,
                flags: flags,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.touchid.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + 8
    }
}
impl crate::auto::Event for TouchOwnershipEvent {
    const OPCODE: u8 = 21;
}
#[derive(Clone, Debug, Default, PartialEq, PartialOrd)]
pub struct TouchUpdateEvent<'uf, 'vf, 'wf> {
    pub event_type: u8,
    pub deviceid: DeviceId,
    pub sequence: u16,
    pub time: Timestamp,
    pub detail: Card32,
    pub root: Window,
    pub event: Window,
    pub child: Window,
    pub root_x: Fp1616,
    pub root_y: Fp1616,
    pub event_x: Fp1616,
    pub event_y: Fp1616,
    pub sourceid: DeviceId,
    pub flags: TouchEventFlags,
    pub mods: ModifierInfo,
    pub group: GroupInfo,
    pub button_mask: Cow<'uf, [Card32]>,
    pub valuator_mask: Cow<'vf, [Card32]>,
    pub axisvalues: Cow<'wf, [Fp3232]>,
}
impl<'uf, 'vf, 'wf> TouchUpdateEvent {}
impl<'uf, 'vf, 'wf> AsByteSequence for TouchUpdateEvent<'uf, 'vf, 'wf> {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.deviceid.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.detail.as_bytes(&mut bytes[index..]);
        index += self.root.as_bytes(&mut bytes[index..]);
        index += self.event.as_bytes(&mut bytes[index..]);
        index += self.child.as_bytes(&mut bytes[index..]);
        index += self.root_x.as_bytes(&mut bytes[index..]);
        index += self.root_y.as_bytes(&mut bytes[index..]);
        index += self.event_x.as_bytes(&mut bytes[index..]);
        index += self.event_y.as_bytes(&mut bytes[index..]);
        index += (self.button_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.valuator_mask.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.sourceid.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.button_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.valuator_mask, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let block_len: usize = vector_as_bytes(&self.axisvalues, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing TouchUpdateEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (deviceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detail, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (child, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (root_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_x, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_y, sz): (Fp1616, usize) = <Fp1616>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sourceid, sz): (DeviceId, usize) = <DeviceId>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (flags, sz): (TouchEventFlags, usize) = <TouchEventFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModifierInfo, usize) = <ModifierInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (GroupInfo, usize) = <GroupInfo>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (valuator_mask, block_len): (Cow<'static, [Card32]>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card32>());
        let (axisvalues, block_len): (Cow<'static, [Fp3232]>, usize) = vector_from_bytes(
            &bytes[index..],
            (valuator_mask
                .iter()
                .map(|a| {
                    ((TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize"))
                        .count_ones()) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
        Some((
            TouchUpdateEvent {
                event_type: event_type,
                deviceid: deviceid,
                sequence: sequence,
                time: time,
                detail: detail,
                root: root,
                event: event,
                child: child,
                root_x: root_x,
                root_y: root_y,
                event_x: event_x,
                event_y: event_y,
                sourceid: sourceid,
                flags: flags,
                mods: mods,
                group: group,
                button_mask: button_mask,
                valuator_mask: valuator_mask,
                axisvalues: axisvalues,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.deviceid.size()
            + self.sequence.size()
            + self.time.size()
            + self.detail.size()
            + self.root.size()
            + self.event.size()
            + self.child.size()
            + self.root_x.size()
            + self.root_y.size()
            + self.event_x.size()
            + self.event_y.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.sourceid.size()
            + 2
            + self.flags.size()
            + self.mods.size()
            + self.group.size()
            + {
                let block_len: usize = self.button_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.valuator_mask.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card32>());
                block_len + pad
            }
            + {
                let block_len: usize = self.axisvalues.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Fp3232>());
                block_len + pad
            }
    }
}
impl crate::auto::Event for TouchUpdateEvent {
    const OPCODE: u8 = 19;
}
