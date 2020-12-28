// This file was automatically generated.
// It is considered to be licensed under the MIT and Apache 2.0 licenses.

#![allow(warnings)]

use super::prelude::*;

use super::xproto::*;
pub type DeviceSpec = Card16;
pub type LedClassSpec = Card16;
pub type BellClassSpec = Card16;
pub type IdSpec = Card16;
#[derive(Clone, Debug, Default)]
pub struct IndicatorMap {
    pub flags: ImFlag,
    pub which_groups: ImGroupsWhich,
    pub groups: SetOfGroup,
    pub which_mods: ImModsWhich,
    pub mods: ModMask,
    pub real_mods: ModMask,
    pub vmods: VMod,
    pub ctrls: BoolCtrl,
}
impl IndicatorMap {}
impl AsByteSequence for IndicatorMap {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.which_groups.as_bytes(&mut bytes[index..]);
        index += self.groups.as_bytes(&mut bytes[index..]);
        index += self.which_mods.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.real_mods.as_bytes(&mut bytes[index..]);
        index += self.vmods.as_bytes(&mut bytes[index..]);
        index += self.ctrls.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IndicatorMap from byte buffer");
        let (flags, sz): (ImFlag, usize) = <ImFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (which_groups, sz): (ImGroupsWhich, usize) =
            <ImGroupsWhich>::from_bytes(&bytes[index..])?;
        index += sz;
        let (groups, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        let (which_mods, sz): (ImModsWhich, usize) = <ImModsWhich>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ctrls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            IndicatorMap {
                flags: flags,
                which_groups: which_groups,
                groups: groups,
                which_mods: which_mods,
                mods: mods,
                real_mods: real_mods,
                vmods: vmods,
                ctrls: ctrls,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.flags.size()
            + self.which_groups.size()
            + self.groups.size()
            + self.which_mods.size()
            + self.mods.size()
            + self.real_mods.size()
            + self.vmods.size()
            + self.ctrls.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImFlag {
    pub inner: u8,
}
impl ImFlag {
    #[inline]
    pub fn led_drives_kb(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_led_drives_kb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn no_automatic(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_no_automatic(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn no_explicit(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_no_explicit(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(led_drives_kb: bool, no_automatic: bool, no_explicit: bool) -> Self {
        let mut inner: u8 = 0;
        if led_drives_kb {
            inner |= 1 << 5;
        }
        if no_automatic {
            inner |= 1 << 6;
        }
        if no_explicit {
            inner |= 1 << 7;
        }
        ImFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const LED_DRIVES_KB: Self = Self { inner: 32 };
    pub const NO_AUTOMATIC: Self = Self { inner: 64 };
    pub const NO_EXPLICIT: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 224 };
}
impl AsByteSequence for ImFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((ImFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ImFlag {
    type Output = ImFlag;
    #[inline]
    fn not(self) -> ImFlag {
        ImFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ImFlag {
    type Output = ImFlag;
    #[inline]
    fn bitand(self, rhs: ImFlag) -> ImFlag {
        ImFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ImFlag {
    type Output = ImFlag;
    #[inline]
    fn bitor(self, rhs: ImFlag) -> ImFlag {
        ImFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ImFlag {
    type Output = ImFlag;
    #[inline]
    fn bitxor(self, rhs: ImFlag) -> ImFlag {
        ImFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImGroupsWhich {
    pub inner: u8,
}
impl ImGroupsWhich {
    #[inline]
    pub fn use_base(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_use_base(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn use_latched(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_use_latched(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn use_locked(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_use_locked(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn use_effective(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_use_effective(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn use_compat(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_use_compat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn new(
        use_base: bool,
        use_latched: bool,
        use_locked: bool,
        use_effective: bool,
        use_compat: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if use_base {
            inner |= 1 << 0;
        }
        if use_latched {
            inner |= 1 << 1;
        }
        if use_locked {
            inner |= 1 << 2;
        }
        if use_effective {
            inner |= 1 << 3;
        }
        if use_compat {
            inner |= 1 << 4;
        }
        ImGroupsWhich { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const USE_BASE: Self = Self { inner: 1 };
    pub const USE_LATCHED: Self = Self { inner: 2 };
    pub const USE_LOCKED: Self = Self { inner: 4 };
    pub const USE_EFFECTIVE: Self = Self { inner: 8 };
    pub const USE_COMPAT: Self = Self { inner: 16 };
    pub const COMPLETE: Self = Self { inner: 31 };
}
impl AsByteSequence for ImGroupsWhich {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((ImGroupsWhich { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ImGroupsWhich {
    type Output = ImGroupsWhich;
    #[inline]
    fn not(self) -> ImGroupsWhich {
        ImGroupsWhich { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ImGroupsWhich {
    type Output = ImGroupsWhich;
    #[inline]
    fn bitand(self, rhs: ImGroupsWhich) -> ImGroupsWhich {
        ImGroupsWhich {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ImGroupsWhich {
    type Output = ImGroupsWhich;
    #[inline]
    fn bitor(self, rhs: ImGroupsWhich) -> ImGroupsWhich {
        ImGroupsWhich {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ImGroupsWhich {
    type Output = ImGroupsWhich;
    #[inline]
    fn bitxor(self, rhs: ImGroupsWhich) -> ImGroupsWhich {
        ImGroupsWhich {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetOfGroup {
    pub inner: u8,
}
impl SetOfGroup {
    #[inline]
    pub fn group1(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_group1(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn group2(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_group2(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn group3(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_group3(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn group4(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_group4(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn new(group1: bool, group2: bool, group3: bool, group4: bool) -> Self {
        let mut inner: u8 = 0;
        if group1 {
            inner |= 1 << 0;
        }
        if group2 {
            inner |= 1 << 1;
        }
        if group3 {
            inner |= 1 << 2;
        }
        if group4 {
            inner |= 1 << 3;
        }
        SetOfGroup { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const GROUP1: Self = Self { inner: 1 };
    pub const GROUP2: Self = Self { inner: 2 };
    pub const GROUP3: Self = Self { inner: 4 };
    pub const GROUP4: Self = Self { inner: 8 };
    pub const COMPLETE: Self = Self { inner: 15 };
}
impl AsByteSequence for SetOfGroup {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((SetOfGroup { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SetOfGroup {
    type Output = SetOfGroup;
    #[inline]
    fn not(self) -> SetOfGroup {
        SetOfGroup { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SetOfGroup {
    type Output = SetOfGroup;
    #[inline]
    fn bitand(self, rhs: SetOfGroup) -> SetOfGroup {
        SetOfGroup {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SetOfGroup {
    type Output = SetOfGroup;
    #[inline]
    fn bitor(self, rhs: SetOfGroup) -> SetOfGroup {
        SetOfGroup {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SetOfGroup {
    type Output = SetOfGroup;
    #[inline]
    fn bitxor(self, rhs: SetOfGroup) -> SetOfGroup {
        SetOfGroup {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ImModsWhich {
    pub inner: u8,
}
impl ImModsWhich {
    #[inline]
    pub fn use_base(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_use_base(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn use_latched(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_use_latched(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn use_locked(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_use_locked(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn use_effective(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_use_effective(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn use_compat(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_use_compat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn new(
        use_base: bool,
        use_latched: bool,
        use_locked: bool,
        use_effective: bool,
        use_compat: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if use_base {
            inner |= 1 << 0;
        }
        if use_latched {
            inner |= 1 << 1;
        }
        if use_locked {
            inner |= 1 << 2;
        }
        if use_effective {
            inner |= 1 << 3;
        }
        if use_compat {
            inner |= 1 << 4;
        }
        ImModsWhich { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const USE_BASE: Self = Self { inner: 1 };
    pub const USE_LATCHED: Self = Self { inner: 2 };
    pub const USE_LOCKED: Self = Self { inner: 4 };
    pub const USE_EFFECTIVE: Self = Self { inner: 8 };
    pub const USE_COMPAT: Self = Self { inner: 16 };
    pub const COMPLETE: Self = Self { inner: 31 };
}
impl AsByteSequence for ImModsWhich {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((ImModsWhich { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ImModsWhich {
    type Output = ImModsWhich;
    #[inline]
    fn not(self) -> ImModsWhich {
        ImModsWhich { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ImModsWhich {
    type Output = ImModsWhich;
    #[inline]
    fn bitand(self, rhs: ImModsWhich) -> ImModsWhich {
        ImModsWhich {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ImModsWhich {
    type Output = ImModsWhich;
    #[inline]
    fn bitor(self, rhs: ImModsWhich) -> ImModsWhich {
        ImModsWhich {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ImModsWhich {
    type Output = ImModsWhich;
    #[inline]
    fn bitxor(self, rhs: ImModsWhich) -> ImModsWhich {
        ImModsWhich {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VMod {
    pub inner: u16,
}
impl VMod {
    #[inline]
    pub fn Zero(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_Zero(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn One(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_One(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn Two(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_Two(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn Three(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_Three(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn Four(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_Four(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn Five(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_Five(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn Six(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_Six(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn Seven(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_Seven(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn Eight(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_Eight(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn Nine(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_Nine(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn Ten(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_Ten(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn Eleven(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_Eleven(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn Twelve(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_Twelve(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn Thirteen(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_Thirteen(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn Fourteen(&self) -> bool {
        self.inner & (1 << 14) != 0
    }
    #[inline]
    pub fn set_Fourteen(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 14;
        } else {
            self.inner &= !(1 << 14);
        }
        self
    }
    #[inline]
    pub fn Fifteen(&self) -> bool {
        self.inner & (1 << 15) != 0
    }
    #[inline]
    pub fn set_Fifteen(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 15;
        } else {
            self.inner &= !(1 << 15);
        }
        self
    }
    #[inline]
    pub fn new(
        Zero: bool,
        One: bool,
        Two: bool,
        Three: bool,
        Four: bool,
        Five: bool,
        Six: bool,
        Seven: bool,
        Eight: bool,
        Nine: bool,
        Ten: bool,
        Eleven: bool,
        Twelve: bool,
        Thirteen: bool,
        Fourteen: bool,
        Fifteen: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if Zero {
            inner |= 1 << 0;
        }
        if One {
            inner |= 1 << 1;
        }
        if Two {
            inner |= 1 << 2;
        }
        if Three {
            inner |= 1 << 3;
        }
        if Four {
            inner |= 1 << 4;
        }
        if Five {
            inner |= 1 << 5;
        }
        if Six {
            inner |= 1 << 6;
        }
        if Seven {
            inner |= 1 << 7;
        }
        if Eight {
            inner |= 1 << 8;
        }
        if Nine {
            inner |= 1 << 9;
        }
        if Ten {
            inner |= 1 << 10;
        }
        if Eleven {
            inner |= 1 << 11;
        }
        if Twelve {
            inner |= 1 << 12;
        }
        if Thirteen {
            inner |= 1 << 13;
        }
        if Fourteen {
            inner |= 1 << 14;
        }
        if Fifteen {
            inner |= 1 << 15;
        }
        VMod { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const ZERO: Self = Self { inner: 1 };
    pub const ONE: Self = Self { inner: 2 };
    pub const TWO: Self = Self { inner: 4 };
    pub const THREE: Self = Self { inner: 8 };
    pub const FOUR: Self = Self { inner: 16 };
    pub const FIVE: Self = Self { inner: 32 };
    pub const SIX: Self = Self { inner: 64 };
    pub const SEVEN: Self = Self { inner: 128 };
    pub const EIGHT: Self = Self { inner: 256 };
    pub const NINE: Self = Self { inner: 512 };
    pub const TEN: Self = Self { inner: 1024 };
    pub const ELEVEN: Self = Self { inner: 2048 };
    pub const TWELVE: Self = Self { inner: 4096 };
    pub const THIRTEEN: Self = Self { inner: 8192 };
    pub const FOURTEEN: Self = Self { inner: 16384 };
    pub const FIFTEEN: Self = Self { inner: 32768 };
    pub const COMPLETE: Self = Self { inner: 65535 };
}
impl AsByteSequence for VMod {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((VMod { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for VMod {
    type Output = VMod;
    #[inline]
    fn not(self) -> VMod {
        VMod { inner: !self.inner }
    }
}
impl core::ops::BitAnd for VMod {
    type Output = VMod;
    #[inline]
    fn bitand(self, rhs: VMod) -> VMod {
        VMod {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for VMod {
    type Output = VMod;
    #[inline]
    fn bitor(self, rhs: VMod) -> VMod {
        VMod {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for VMod {
    type Output = VMod;
    #[inline]
    fn bitxor(self, rhs: VMod) -> VMod {
        VMod {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoolCtrl {
    pub inner: u32,
}
impl BoolCtrl {
    #[inline]
    pub fn repeat_keys(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_repeat_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn slow_keys(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_slow_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn bounce_keys(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_bounce_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn sticky_keys(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_sticky_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn mouse_keys(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_mouse_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn mouse_keys_accel(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_mouse_keys_accel(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn access_x_keys(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_access_x_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn access_x_timeout_mask(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_access_x_timeout_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn access_x_feedback_mask(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_access_x_feedback_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn audible_bell_mask(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_audible_bell_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn overlay1_mask(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_overlay1_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn overlay2_mask(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_overlay2_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn ignore_group_lock_mask(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_ignore_group_lock_mask(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn new(
        repeat_keys: bool,
        slow_keys: bool,
        bounce_keys: bool,
        sticky_keys: bool,
        mouse_keys: bool,
        mouse_keys_accel: bool,
        access_x_keys: bool,
        access_x_timeout_mask: bool,
        access_x_feedback_mask: bool,
        audible_bell_mask: bool,
        overlay1_mask: bool,
        overlay2_mask: bool,
        ignore_group_lock_mask: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if repeat_keys {
            inner |= 1 << 0;
        }
        if slow_keys {
            inner |= 1 << 1;
        }
        if bounce_keys {
            inner |= 1 << 2;
        }
        if sticky_keys {
            inner |= 1 << 3;
        }
        if mouse_keys {
            inner |= 1 << 4;
        }
        if mouse_keys_accel {
            inner |= 1 << 5;
        }
        if access_x_keys {
            inner |= 1 << 6;
        }
        if access_x_timeout_mask {
            inner |= 1 << 7;
        }
        if access_x_feedback_mask {
            inner |= 1 << 8;
        }
        if audible_bell_mask {
            inner |= 1 << 9;
        }
        if overlay1_mask {
            inner |= 1 << 10;
        }
        if overlay2_mask {
            inner |= 1 << 11;
        }
        if ignore_group_lock_mask {
            inner |= 1 << 12;
        }
        BoolCtrl { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const REPEAT_KEYS: Self = Self { inner: 1 };
    pub const SLOW_KEYS: Self = Self { inner: 2 };
    pub const BOUNCE_KEYS: Self = Self { inner: 4 };
    pub const STICKY_KEYS: Self = Self { inner: 8 };
    pub const MOUSE_KEYS: Self = Self { inner: 16 };
    pub const MOUSE_KEYS_ACCEL: Self = Self { inner: 32 };
    pub const ACCESS_X_KEYS: Self = Self { inner: 64 };
    pub const ACCESS_X_TIMEOUT_MASK: Self = Self { inner: 128 };
    pub const ACCESS_X_FEEDBACK_MASK: Self = Self { inner: 256 };
    pub const AUDIBLE_BELL_MASK: Self = Self { inner: 512 };
    pub const OVERLAY1_MASK: Self = Self { inner: 1024 };
    pub const OVERLAY2_MASK: Self = Self { inner: 2048 };
    pub const IGNORE_GROUP_LOCK_MASK: Self = Self { inner: 4096 };
    pub const COMPLETE: Self = Self { inner: 8191 };
}
impl AsByteSequence for BoolCtrl {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((BoolCtrl { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for BoolCtrl {
    type Output = BoolCtrl;
    #[inline]
    fn not(self) -> BoolCtrl {
        BoolCtrl { inner: !self.inner }
    }
}
impl core::ops::BitAnd for BoolCtrl {
    type Output = BoolCtrl;
    #[inline]
    fn bitand(self, rhs: BoolCtrl) -> BoolCtrl {
        BoolCtrl {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for BoolCtrl {
    type Output = BoolCtrl;
    #[inline]
    fn bitor(self, rhs: BoolCtrl) -> BoolCtrl {
        BoolCtrl {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for BoolCtrl {
    type Output = BoolCtrl;
    #[inline]
    fn bitxor(self, rhs: BoolCtrl) -> BoolCtrl {
        BoolCtrl {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ModDef {
    pub mask: ModMask,
    pub real_mods: ModMask,
    pub vmods: VMod,
}
impl ModDef {}
impl AsByteSequence for ModDef {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.real_mods.as_bytes(&mut bytes[index..]);
        index += self.vmods.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ModDef from byte buffer");
        let (mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ModDef {
                mask: mask,
                real_mods: real_mods,
                vmods: vmods,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.mask.size() + self.real_mods.size() + self.vmods.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct KeyName {
    pub name: [c_char; 4],
}
impl KeyName {}
impl AsByteSequence for KeyName {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.name.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyName from byte buffer");
        let (name, sz): ([c_char; 4], usize) = <[c_char; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((KeyName { name: name }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.name.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct KeyAlias {
    pub real: [c_char; 4],
    pub alias: [c_char; 4],
}
impl KeyAlias {}
impl AsByteSequence for KeyAlias {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.real.as_bytes(&mut bytes[index..]);
        index += self.alias.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyAlias from byte buffer");
        let (real, sz): ([c_char; 4], usize) = <[c_char; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (alias, sz): ([c_char; 4], usize) = <[c_char; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KeyAlias {
                real: real,
                alias: alias,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.real.size() + self.alias.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct CountedString16 {
    pub length: Card16,
    pub string: String,
    pub alignment_pad: Vec<Void>,
}
impl CountedString16 {}
impl AsByteSequence for CountedString16 {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = string_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let block_len: usize = vector_as_bytes(&self.alignment_pad, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CountedString16 from byte buffer");
        let (length, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string, block_len): (String, usize) =
            string_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<c_char>());
        let (alignment_pad, block_len): (Vec<Void>, usize) = vector_from_bytes(
            &bytes[index..],
            ((((length as usize) + (5)) & (!(3))) - ((length as usize) + (2))) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Void>());
        Some((
            CountedString16 {
                length: length,
                string: string,
                alignment_pad: alignment_pad,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.length.size()
            + {
                let block_len: usize = self.string.len();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<c_char>());
                block_len + pad
            }
            + {
                let block_len: usize = self.alignment_pad.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Void>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct KtMapEntry {
    pub active: bool,
    pub mods_mask: ModMask,
    pub level: Card8,
    pub mods_mods: ModMask,
    pub mods_vmods: VMod,
}
impl KtMapEntry {}
impl AsByteSequence for KtMapEntry {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.active.as_bytes(&mut bytes[index..]);
        index += self.mods_mask.as_bytes(&mut bytes[index..]);
        index += self.level.as_bytes(&mut bytes[index..]);
        index += self.mods_mods.as_bytes(&mut bytes[index..]);
        index += self.mods_vmods.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KtMapEntry from byte buffer");
        let (active, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods_mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (level, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods_vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            KtMapEntry {
                active: active,
                mods_mask: mods_mask,
                level: level,
                mods_mods: mods_mods,
                mods_vmods: mods_vmods,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.active.size()
            + self.mods_mask.size()
            + self.level.size()
            + self.mods_mods.size()
            + self.mods_vmods.size()
            + 2
    }
}
#[derive(Clone, Debug, Default)]
pub struct KeyType {
    pub mods_mask: ModMask,
    pub mods_mods: ModMask,
    pub mods_vmods: VMod,
    pub num_levels: Card8,
    pub n_map_entries: Card8,
    pub has_preserve: bool,
    pub map: Vec<KtMapEntry>,
    pub preserve: Vec<ModDef>,
}
impl KeyType {}
impl AsByteSequence for KeyType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.mods_mask.as_bytes(&mut bytes[index..]);
        index += self.mods_mods.as_bytes(&mut bytes[index..]);
        index += self.mods_vmods.as_bytes(&mut bytes[index..]);
        index += self.num_levels.as_bytes(&mut bytes[index..]);
        index += self.n_map_entries.as_bytes(&mut bytes[index..]);
        index += self.has_preserve.as_bytes(&mut bytes[index..]);
        index += 1;
        let block_len: usize = vector_as_bytes(&self.map, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KtMapEntry>());
        let block_len: usize = vector_as_bytes(&self.preserve, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyType from byte buffer");
        let (mods_mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods_vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_levels, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_map_entries, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (has_preserve, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (map, block_len): (Vec<KtMapEntry>, usize) =
            vector_from_bytes(&bytes[index..], (n_map_entries as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KtMapEntry>());
        let (preserve, block_len): (Vec<ModDef>, usize) = vector_from_bytes(
            &bytes[index..],
            ((has_preserve as usize) * (n_map_entries as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        Some((
            KeyType {
                mods_mask: mods_mask,
                mods_mods: mods_mods,
                mods_vmods: mods_vmods,
                num_levels: num_levels,
                n_map_entries: n_map_entries,
                has_preserve: has_preserve,
                map: map,
                preserve: preserve,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.mods_mask.size()
            + self.mods_mods.size()
            + self.mods_vmods.size()
            + self.num_levels.size()
            + self.n_map_entries.size()
            + self.has_preserve.size()
            + 1
            + {
                let block_len: usize = self.map.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KtMapEntry>());
                block_len + pad
            }
            + {
                let block_len: usize = self.preserve.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct KeySymMap {
    pub kt_index: [Card8; 4],
    pub group_info: Card8,
    pub width: Card8,
    pub syms: Vec<Keysym>,
}
impl KeySymMap {}
impl AsByteSequence for KeySymMap {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.kt_index.as_bytes(&mut bytes[index..]);
        index += self.group_info.as_bytes(&mut bytes[index..]);
        index += self.width.as_bytes(&mut bytes[index..]);
        index += (self.syms.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.syms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeySymMap from byte buffer");
        let (kt_index, sz): ([Card8; 4], usize) = <[Card8; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group_info, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (width, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (syms, block_len): (Vec<Keysym>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
        Some((
            KeySymMap {
                kt_index: kt_index,
                group_info: group_info,
                width: width,
                syms: syms,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.kt_index.size()
            + self.group_info.size()
            + self.width.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.syms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Keysym>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct CommonBehavior {
    pub ty: Card8,
    pub data: Card8,
}
impl CommonBehavior {}
impl AsByteSequence for CommonBehavior {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.data.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CommonBehavior from byte buffer");
        let (ty, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((CommonBehavior { ty: ty, data: data }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.data.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct DefaultBehavior {
    pub ty: Card8,
}
impl DefaultBehavior {}
impl AsByteSequence for DefaultBehavior {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DefaultBehavior from byte buffer");
        let (ty, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((DefaultBehavior { ty: ty }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + 1
    }
}
pub type LockBehavior = DefaultBehavior;
#[derive(Clone, Debug, Default)]
pub struct RadioGroupBehavior {
    pub ty: Card8,
    pub group: Card8,
}
impl RadioGroupBehavior {}
impl AsByteSequence for RadioGroupBehavior {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing RadioGroupBehavior from byte buffer");
        let (ty, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            RadioGroupBehavior {
                ty: ty,
                group: group,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.group.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct OverlayBehavior {
    pub ty: Card8,
    pub key: Keycode,
}
impl OverlayBehavior {}
impl AsByteSequence for OverlayBehavior {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.key.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OverlayBehavior from byte buffer");
        let (ty, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((OverlayBehavior { ty: ty, key: key }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.key.size()
    }
}
pub type PermamentLockBehavior = LockBehavior;
pub type PermamentRadioGroupBehavior = RadioGroupBehavior;
pub type PermamentOverlayBehavior = OverlayBehavior;
#[derive(Clone, Debug, Default)]
pub struct SetBehavior {
    pub keycode: Keycode,
    pub behavior: Behavior,
}
impl SetBehavior {}
impl AsByteSequence for SetBehavior {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += self.behavior.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetBehavior from byte buffer");
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (behavior, sz): (Behavior, usize) = <Behavior>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            SetBehavior {
                keycode: keycode,
                behavior: behavior,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.keycode.size() + self.behavior.size() + 1
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetExplicit {
    pub keycode: Keycode,
    pub explicit: Explicit,
}
impl SetExplicit {}
impl AsByteSequence for SetExplicit {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += self.explicit.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetExplicit from byte buffer");
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (explicit, sz): (Explicit, usize) = <Explicit>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetExplicit {
                keycode: keycode,
                explicit: explicit,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.keycode.size() + self.explicit.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Explicit {
    pub inner: u8,
}
impl Explicit {
    #[inline]
    pub fn key_type1(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_key_type1(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn key_type2(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_key_type2(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn key_type3(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_key_type3(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn key_type4(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_key_type4(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn interpret(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_interpret(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn auto_repeat(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_auto_repeat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn behavior(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_behavior(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn v_mod_map(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_v_mod_map(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        key_type1: bool,
        key_type2: bool,
        key_type3: bool,
        key_type4: bool,
        interpret: bool,
        auto_repeat: bool,
        behavior: bool,
        v_mod_map: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if key_type1 {
            inner |= 1 << 0;
        }
        if key_type2 {
            inner |= 1 << 1;
        }
        if key_type3 {
            inner |= 1 << 2;
        }
        if key_type4 {
            inner |= 1 << 3;
        }
        if interpret {
            inner |= 1 << 4;
        }
        if auto_repeat {
            inner |= 1 << 5;
        }
        if behavior {
            inner |= 1 << 6;
        }
        if v_mod_map {
            inner |= 1 << 7;
        }
        Explicit { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const KEY_TYPE1: Self = Self { inner: 1 };
    pub const KEY_TYPE2: Self = Self { inner: 2 };
    pub const KEY_TYPE3: Self = Self { inner: 4 };
    pub const KEY_TYPE4: Self = Self { inner: 8 };
    pub const INTERPRET: Self = Self { inner: 16 };
    pub const AUTO_REPEAT: Self = Self { inner: 32 };
    pub const BEHAVIOR: Self = Self { inner: 64 };
    pub const V_MOD_MAP: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for Explicit {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((Explicit { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Explicit {
    type Output = Explicit;
    #[inline]
    fn not(self) -> Explicit {
        Explicit { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Explicit {
    type Output = Explicit;
    #[inline]
    fn bitand(self, rhs: Explicit) -> Explicit {
        Explicit {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Explicit {
    type Output = Explicit;
    #[inline]
    fn bitor(self, rhs: Explicit) -> Explicit {
        Explicit {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Explicit {
    type Output = Explicit;
    #[inline]
    fn bitxor(self, rhs: Explicit) -> Explicit {
        Explicit {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct KeyModMap {
    pub keycode: Keycode,
    pub mods: ModMask,
}
impl KeyModMap {}
impl AsByteSequence for KeyModMap {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyModMap from byte buffer");
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KeyModMap {
                keycode: keycode,
                mods: mods,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.keycode.size() + self.mods.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct KeyVModMap {
    pub keycode: Keycode,
    pub vmods: VMod,
}
impl KeyVModMap {}
impl AsByteSequence for KeyVModMap {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.vmods.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyVModMap from byte buffer");
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KeyVModMap {
                keycode: keycode,
                vmods: vmods,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.keycode.size() + 1 + self.vmods.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct KtSetMapEntry {
    pub level: Card8,
    pub real_mods: ModMask,
    pub virtual_mods: VMod,
}
impl KtSetMapEntry {}
impl AsByteSequence for KtSetMapEntry {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.level.as_bytes(&mut bytes[index..]);
        index += self.real_mods.as_bytes(&mut bytes[index..]);
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KtSetMapEntry from byte buffer");
        let (level, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            KtSetMapEntry {
                level: level,
                real_mods: real_mods,
                virtual_mods: virtual_mods,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.level.size() + self.real_mods.size() + self.virtual_mods.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetKeyType {
    pub mask: ModMask,
    pub real_mods: ModMask,
    pub virtual_mods: VMod,
    pub num_levels: Card8,
    pub n_map_entries: Card8,
    pub preserve: bool,
    pub entries: Vec<KtSetMapEntry>,
    pub preserve_entries: Vec<KtSetMapEntry>,
}
impl SetKeyType {}
impl AsByteSequence for SetKeyType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.real_mods.as_bytes(&mut bytes[index..]);
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        index += self.num_levels.as_bytes(&mut bytes[index..]);
        index += self.n_map_entries.as_bytes(&mut bytes[index..]);
        index += self.preserve.as_bytes(&mut bytes[index..]);
        index += 1;
        let block_len: usize = vector_as_bytes(&self.entries, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KtSetMapEntry>());
        let block_len: usize = vector_as_bytes(&self.preserve_entries, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KtSetMapEntry>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetKeyType from byte buffer");
        let (mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_levels, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_map_entries, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (preserve, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (entries, block_len): (Vec<KtSetMapEntry>, usize) =
            vector_from_bytes(&bytes[index..], (n_map_entries as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KtSetMapEntry>());
        let (preserve_entries, block_len): (Vec<KtSetMapEntry>, usize) = vector_from_bytes(
            &bytes[index..],
            ((preserve as usize) * (n_map_entries as usize)) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KtSetMapEntry>());
        Some((
            SetKeyType {
                mask: mask,
                real_mods: real_mods,
                virtual_mods: virtual_mods,
                num_levels: num_levels,
                n_map_entries: n_map_entries,
                preserve: preserve,
                entries: entries,
                preserve_entries: preserve_entries,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.mask.size()
            + self.real_mods.size()
            + self.virtual_mods.size()
            + self.num_levels.size()
            + self.n_map_entries.size()
            + self.preserve.size()
            + 1
            + {
                let block_len: usize = self.entries.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KtSetMapEntry>());
                block_len + pad
            }
            + {
                let block_len: usize = self.preserve_entries.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KtSetMapEntry>());
                block_len + pad
            }
    }
}
pub type String8 = Char;
#[derive(Clone, Debug, Default)]
pub struct Outline {
    pub corner_radius: Card8,
    pub points: Vec<Point>,
}
impl Outline {}
impl AsByteSequence for Outline {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += (self.points.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.corner_radius.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.points, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Outline from byte buffer");
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (corner_radius, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (points, block_len): (Vec<Point>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Point>());
        Some((
            Outline {
                corner_radius: corner_radius,
                points: points,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<Card8>() + self.corner_radius.size() + 2 + {
            let block_len: usize = self.points.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Point>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Shape {
    pub name: Atom,
    pub primary_ndx: Card8,
    pub approx_ndx: Card8,
    pub outlines: Vec<Outline>,
}
impl Shape {}
impl AsByteSequence for Shape {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.name.as_bytes(&mut bytes[index..]);
        index += (self.outlines.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.primary_ndx.as_bytes(&mut bytes[index..]);
        index += self.approx_ndx.as_bytes(&mut bytes[index..]);
        index += 1;
        let block_len: usize = vector_as_bytes(&self.outlines, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Outline>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Shape from byte buffer");
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (primary_ndx, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (approx_ndx, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (outlines, block_len): (Vec<Outline>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Outline>());
        Some((
            Shape {
                name: name,
                primary_ndx: primary_ndx,
                approx_ndx: approx_ndx,
                outlines: outlines,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.name.size()
            + ::core::mem::size_of::<Card8>()
            + self.primary_ndx.size()
            + self.approx_ndx.size()
            + 1
            + {
                let block_len: usize = self.outlines.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Outline>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Key {
    pub name: [String8; 4],
    pub gap: Int16,
    pub shape_ndx: Card8,
    pub color_ndx: Card8,
}
impl Key {}
impl AsByteSequence for Key {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.name.as_bytes(&mut bytes[index..]);
        index += self.gap.as_bytes(&mut bytes[index..]);
        index += self.shape_ndx.as_bytes(&mut bytes[index..]);
        index += self.color_ndx.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Key from byte buffer");
        let (name, sz): ([String8; 4], usize) = <[String8; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (gap, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (shape_ndx, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (color_ndx, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            Key {
                name: name,
                gap: gap,
                shape_ndx: shape_ndx,
                color_ndx: color_ndx,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.name.size() + self.gap.size() + self.shape_ndx.size() + self.color_ndx.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct OverlayKey {
    pub over: [String8; 4],
    pub under: [String8; 4],
}
impl OverlayKey {}
impl AsByteSequence for OverlayKey {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.over.as_bytes(&mut bytes[index..]);
        index += self.under.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OverlayKey from byte buffer");
        let (over, sz): ([String8; 4], usize) = <[String8; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        let (under, sz): ([String8; 4], usize) = <[String8; 4]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            OverlayKey {
                over: over,
                under: under,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.over.size() + self.under.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct OverlayRow {
    pub row_under: Card8,
    pub keys: Vec<OverlayKey>,
}
impl OverlayRow {}
impl AsByteSequence for OverlayRow {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.row_under.as_bytes(&mut bytes[index..]);
        index += (self.keys.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.keys, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<OverlayKey>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing OverlayRow from byte buffer");
        let (row_under, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (keys, block_len): (Vec<OverlayKey>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<OverlayKey>());
        Some((
            OverlayRow {
                row_under: row_under,
                keys: keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.row_under.size() + ::core::mem::size_of::<Card8>() + 2 + {
            let block_len: usize = self.keys.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<OverlayKey>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Overlay {
    pub name: Atom,
    pub rows: Vec<OverlayRow>,
}
impl Overlay {}
impl AsByteSequence for Overlay {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.name.as_bytes(&mut bytes[index..]);
        index += (self.rows.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 3;
        let block_len: usize = vector_as_bytes(&self.rows, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<OverlayRow>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Overlay from byte buffer");
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (rows, block_len): (Vec<OverlayRow>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<OverlayRow>());
        Some((
            Overlay {
                name: name,
                rows: rows,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.name.size() + ::core::mem::size_of::<Card8>() + 3 + {
            let block_len: usize = self.rows.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<OverlayRow>());
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Row {
    pub top: Int16,
    pub left: Int16,
    pub vertical: bool,
    pub keys: Vec<Key>,
}
impl Row {}
impl AsByteSequence for Row {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.top.as_bytes(&mut bytes[index..]);
        index += self.left.as_bytes(&mut bytes[index..]);
        index += (self.keys.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.vertical.as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.keys, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Key>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Row from byte buffer");
        let (top, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (left, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vertical, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (keys, block_len): (Vec<Key>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Key>());
        Some((
            Row {
                top: top,
                left: left,
                vertical: vertical,
                keys: keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.top.size()
            + self.left.size()
            + ::core::mem::size_of::<Card8>()
            + self.vertical.size()
            + 2
            + {
                let block_len: usize = self.keys.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Key>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct Listing {
    pub flags: Card16,
    pub length: Card16,
    pub string: Vec<String8>,
}
impl Listing {}
impl AsByteSequence for Listing {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.string, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 2);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing Listing from byte buffer");
        let (flags, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (string, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], (length as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 2);
        Some((
            Listing {
                flags: flags,
                length: length,
                string: string,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.flags.size() + self.length.size() + {
            let block_len: usize = self.string.iter().map(|i| i.size()).sum();
            let pad: usize = buffer_pad(block_len, 2);
            block_len + pad
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct DeviceLedInfo {
    pub led_class: LedClass,
    pub led_id: IdSpec,
    pub names_present: Card32,
    pub maps_present: Card32,
    pub phys_indicators: Card32,
    pub state: Card32,
    pub names: Vec<Atom>,
    pub maps: Vec<IndicatorMap>,
}
impl DeviceLedInfo {}
impl AsByteSequence for DeviceLedInfo {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.led_class.as_bytes(&mut bytes[index..]);
        index += self.led_id.as_bytes(&mut bytes[index..]);
        index += self.names_present.as_bytes(&mut bytes[index..]);
        index += self.maps_present.as_bytes(&mut bytes[index..]);
        index += self.phys_indicators.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.maps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing DeviceLedInfo from byte buffer");
        let (led_class, sz): (LedClass, usize) = <LedClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_id, sz): (IdSpec, usize) = <IdSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (names_present, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (maps_present, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (phys_indicators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((names_present).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (maps, block_len): (Vec<IndicatorMap>, usize) =
            vector_from_bytes(&bytes[index..], ((maps_present).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        Some((
            DeviceLedInfo {
                led_class: led_class,
                led_id: led_id,
                names_present: names_present,
                maps_present: maps_present,
                phys_indicators: phys_indicators,
                state: state,
                names: names,
                maps: maps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.led_class.size()
            + self.led_id.size()
            + self.names_present.size()
            + self.maps_present.size()
            + self.phys_indicators.size()
            + self.state.size()
            + {
                let block_len: usize = self.names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.maps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
                block_len + pad
            }
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedClass {
    KbdFeedbackClass = 0,
    LedFeedbackClass = 4,
    DfltXiClass = 768,
    AllXiClasses = 1280,
}
impl AsByteSequence for LedClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::KbdFeedbackClass, sz)),
            4 => Some((Self::LedFeedbackClass, sz)),
            768 => Some((Self::DfltXiClass, sz)),
            1280 => Some((Self::AllXiClasses, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for LedClass {
    #[inline]
    fn default() -> LedClass {
        LedClass::KbdFeedbackClass
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Id {
    UseCoreKbd = 256,
    UseCorePtr = 512,
    DfltXiClass = 768,
    DfltXiId = 1024,
    AllXiClass = 1280,
    AllXiId = 1536,
    XiNone = 65280,
}
impl AsByteSequence for Id {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            256 => Some((Self::UseCoreKbd, sz)),
            512 => Some((Self::UseCorePtr, sz)),
            768 => Some((Self::DfltXiClass, sz)),
            1024 => Some((Self::DfltXiId, sz)),
            1280 => Some((Self::AllXiClass, sz)),
            1536 => Some((Self::AllXiId, sz)),
            65280 => Some((Self::XiNone, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for Id {
    #[inline]
    fn default() -> Id {
        Id::UseCoreKbd
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaNoAction {
    pub ty: SaType,
}
impl SaNoAction {}
impl AsByteSequence for SaNoAction {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += 7;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaNoAction from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 7;
        Some((SaNoAction { ty: ty }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + 7
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SaType {
    NoAction = 0,
    SetMods = 1,
    LatchMods = 2,
    LockMods = 3,
    SetGroup = 4,
    LatchGroup = 5,
    LockGroup = 6,
    MovePtr = 7,
    PtrBtn = 8,
    LockPtrBtn = 9,
    SetPtrDflt = 10,
    IsoLock = 11,
    Terminate = 12,
    SwitchScreen = 13,
    SetControls = 14,
    LockControls = 15,
    ActionMessage = 16,
    RedirectKey = 17,
    DeviceBtn = 18,
    LockDeviceBtn = 19,
    DeviceValuator = 20,
}
impl AsByteSequence for SaType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NoAction, sz)),
            1 => Some((Self::SetMods, sz)),
            2 => Some((Self::LatchMods, sz)),
            3 => Some((Self::LockMods, sz)),
            4 => Some((Self::SetGroup, sz)),
            5 => Some((Self::LatchGroup, sz)),
            6 => Some((Self::LockGroup, sz)),
            7 => Some((Self::MovePtr, sz)),
            8 => Some((Self::PtrBtn, sz)),
            9 => Some((Self::LockPtrBtn, sz)),
            10 => Some((Self::SetPtrDflt, sz)),
            11 => Some((Self::IsoLock, sz)),
            12 => Some((Self::Terminate, sz)),
            13 => Some((Self::SwitchScreen, sz)),
            14 => Some((Self::SetControls, sz)),
            15 => Some((Self::LockControls, sz)),
            16 => Some((Self::ActionMessage, sz)),
            17 => Some((Self::RedirectKey, sz)),
            18 => Some((Self::DeviceBtn, sz)),
            19 => Some((Self::LockDeviceBtn, sz)),
            20 => Some((Self::DeviceValuator, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SaType {
    #[inline]
    fn default() -> SaType {
        SaType::NoAction
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaSetMods {
    pub ty: SaType,
    pub flags: Sa,
    pub mask: ModMask,
    pub real_mods: ModMask,
    pub vmods_high: VModsHigh,
    pub vmods_low: VModsLow,
}
impl SaSetMods {}
impl AsByteSequence for SaSetMods {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.real_mods.as_bytes(&mut bytes[index..]);
        index += self.vmods_high.as_bytes(&mut bytes[index..]);
        index += self.vmods_low.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaSetMods from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Sa, usize) = <Sa>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_high, sz): (VModsHigh, usize) = <VModsHigh>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_low, sz): (VModsLow, usize) = <VModsLow>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            SaSetMods {
                ty: ty,
                flags: flags,
                mask: mask,
                real_mods: real_mods,
                vmods_high: vmods_high,
                vmods_low: vmods_low,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.flags.size()
            + self.mask.size()
            + self.real_mods.size()
            + self.vmods_high.size()
            + self.vmods_low.size()
            + 2
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Sa {
    pub inner: u8,
}
impl Sa {
    #[inline]
    pub fn clear_locks(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_clear_locks(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn latch_to_lock(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_latch_to_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn use_mod_map_mods(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_use_mod_map_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(clear_locks: bool, latch_to_lock: bool, use_mod_map_mods: bool) -> Self {
        let mut inner: u8 = 0;
        if clear_locks {
            inner |= 1 << 0;
        }
        if latch_to_lock {
            inner |= 1 << 1;
        }
        if use_mod_map_mods {
            inner |= 1 << 2;
        }
        Sa { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const CLEAR_LOCKS: Self = Self { inner: 1 };
    pub const LATCH_TO_LOCK: Self = Self { inner: 2 };
    pub const USE_MOD_MAP_MODS: Self = Self { inner: 4 };
    pub const COMPLETE: Self = Self { inner: 7 };
}
impl AsByteSequence for Sa {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((Sa { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Sa {
    type Output = Sa;
    #[inline]
    fn not(self) -> Sa {
        Sa { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Sa {
    type Output = Sa;
    #[inline]
    fn bitand(self, rhs: Sa) -> Sa {
        Sa {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Sa {
    type Output = Sa;
    #[inline]
    fn bitor(self, rhs: Sa) -> Sa {
        Sa {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Sa {
    type Output = Sa;
    #[inline]
    fn bitxor(self, rhs: Sa) -> Sa {
        Sa {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VModsHigh {
    pub inner: u8,
}
impl VModsHigh {
    #[inline]
    pub fn Eight(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_Eight(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn Nine(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_Nine(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn Ten(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_Ten(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn Eleven(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_Eleven(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn Twelve(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_Twelve(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn Thirteen(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_Thirteen(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn Fourteen(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_Fourteen(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn Fifteen(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_Fifteen(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        Eight: bool,
        Nine: bool,
        Ten: bool,
        Eleven: bool,
        Twelve: bool,
        Thirteen: bool,
        Fourteen: bool,
        Fifteen: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if Eight {
            inner |= 1 << 0;
        }
        if Nine {
            inner |= 1 << 1;
        }
        if Ten {
            inner |= 1 << 2;
        }
        if Eleven {
            inner |= 1 << 3;
        }
        if Twelve {
            inner |= 1 << 4;
        }
        if Thirteen {
            inner |= 1 << 5;
        }
        if Fourteen {
            inner |= 1 << 6;
        }
        if Fifteen {
            inner |= 1 << 7;
        }
        VModsHigh { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const EIGHT: Self = Self { inner: 1 };
    pub const NINE: Self = Self { inner: 2 };
    pub const TEN: Self = Self { inner: 4 };
    pub const ELEVEN: Self = Self { inner: 8 };
    pub const TWELVE: Self = Self { inner: 16 };
    pub const THIRTEEN: Self = Self { inner: 32 };
    pub const FOURTEEN: Self = Self { inner: 64 };
    pub const FIFTEEN: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for VModsHigh {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((VModsHigh { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for VModsHigh {
    type Output = VModsHigh;
    #[inline]
    fn not(self) -> VModsHigh {
        VModsHigh { inner: !self.inner }
    }
}
impl core::ops::BitAnd for VModsHigh {
    type Output = VModsHigh;
    #[inline]
    fn bitand(self, rhs: VModsHigh) -> VModsHigh {
        VModsHigh {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for VModsHigh {
    type Output = VModsHigh;
    #[inline]
    fn bitor(self, rhs: VModsHigh) -> VModsHigh {
        VModsHigh {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for VModsHigh {
    type Output = VModsHigh;
    #[inline]
    fn bitxor(self, rhs: VModsHigh) -> VModsHigh {
        VModsHigh {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct VModsLow {
    pub inner: u8,
}
impl VModsLow {
    #[inline]
    pub fn Zero(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_Zero(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn One(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_One(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn Two(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_Two(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn Three(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_Three(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn Four(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_Four(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn Five(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_Five(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn Six(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_Six(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn Seven(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_Seven(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        Zero: bool,
        One: bool,
        Two: bool,
        Three: bool,
        Four: bool,
        Five: bool,
        Six: bool,
        Seven: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if Zero {
            inner |= 1 << 0;
        }
        if One {
            inner |= 1 << 1;
        }
        if Two {
            inner |= 1 << 2;
        }
        if Three {
            inner |= 1 << 3;
        }
        if Four {
            inner |= 1 << 4;
        }
        if Five {
            inner |= 1 << 5;
        }
        if Six {
            inner |= 1 << 6;
        }
        if Seven {
            inner |= 1 << 7;
        }
        VModsLow { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const ZERO: Self = Self { inner: 1 };
    pub const ONE: Self = Self { inner: 2 };
    pub const TWO: Self = Self { inner: 4 };
    pub const THREE: Self = Self { inner: 8 };
    pub const FOUR: Self = Self { inner: 16 };
    pub const FIVE: Self = Self { inner: 32 };
    pub const SIX: Self = Self { inner: 64 };
    pub const SEVEN: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for VModsLow {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((VModsLow { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for VModsLow {
    type Output = VModsLow;
    #[inline]
    fn not(self) -> VModsLow {
        VModsLow { inner: !self.inner }
    }
}
impl core::ops::BitAnd for VModsLow {
    type Output = VModsLow;
    #[inline]
    fn bitand(self, rhs: VModsLow) -> VModsLow {
        VModsLow {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for VModsLow {
    type Output = VModsLow;
    #[inline]
    fn bitor(self, rhs: VModsLow) -> VModsLow {
        VModsLow {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for VModsLow {
    type Output = VModsLow;
    #[inline]
    fn bitxor(self, rhs: VModsLow) -> VModsLow {
        VModsLow {
            inner: self.inner ^ rhs.inner,
        }
    }
}
pub type SaLatchMods = SaSetMods;
pub type SaLockMods = SaSetMods;
#[derive(Clone, Debug, Default)]
pub struct SaSetGroup {
    pub ty: SaType,
    pub flags: Sa,
    pub group: Int8,
}
impl SaSetGroup {}
impl AsByteSequence for SaSetGroup {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        index += 5;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaSetGroup from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Sa, usize) = <Sa>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 5;
        Some((
            SaSetGroup {
                ty: ty,
                flags: flags,
                group: group,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.flags.size() + self.group.size() + 5
    }
}
pub type SaLatchGroup = SaSetGroup;
pub type SaLockGroup = SaSetGroup;
#[derive(Clone, Debug, Default)]
pub struct SaMovePtr {
    pub ty: SaType,
    pub flags: SaMovePtrFlag,
    pub x_high: Int8,
    pub x_low: Card8,
    pub y_high: Int8,
    pub y_low: Card8,
}
impl SaMovePtr {}
impl AsByteSequence for SaMovePtr {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.x_high.as_bytes(&mut bytes[index..]);
        index += self.x_low.as_bytes(&mut bytes[index..]);
        index += self.y_high.as_bytes(&mut bytes[index..]);
        index += self.y_low.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaMovePtr from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (SaMovePtrFlag, usize) = <SaMovePtrFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_high, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (x_low, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_high, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (y_low, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            SaMovePtr {
                ty: ty,
                flags: flags,
                x_high: x_high,
                x_low: x_low,
                y_high: y_high,
                y_low: y_low,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.flags.size()
            + self.x_high.size()
            + self.x_low.size()
            + self.y_high.size()
            + self.y_low.size()
            + 2
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaMovePtrFlag {
    pub inner: u8,
}
impl SaMovePtrFlag {
    #[inline]
    pub fn no_acceleration(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_no_acceleration(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn move_absolute_x(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_move_absolute_x(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn move_absolute_y(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_move_absolute_y(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(no_acceleration: bool, move_absolute_x: bool, move_absolute_y: bool) -> Self {
        let mut inner: u8 = 0;
        if no_acceleration {
            inner |= 1 << 0;
        }
        if move_absolute_x {
            inner |= 1 << 1;
        }
        if move_absolute_y {
            inner |= 1 << 2;
        }
        SaMovePtrFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const NO_ACCELERATION: Self = Self { inner: 1 };
    pub const MOVE_ABSOLUTE_X: Self = Self { inner: 2 };
    pub const MOVE_ABSOLUTE_Y: Self = Self { inner: 4 };
    pub const COMPLETE: Self = Self { inner: 7 };
}
impl AsByteSequence for SaMovePtrFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((SaMovePtrFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SaMovePtrFlag {
    type Output = SaMovePtrFlag;
    #[inline]
    fn not(self) -> SaMovePtrFlag {
        SaMovePtrFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SaMovePtrFlag {
    type Output = SaMovePtrFlag;
    #[inline]
    fn bitand(self, rhs: SaMovePtrFlag) -> SaMovePtrFlag {
        SaMovePtrFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SaMovePtrFlag {
    type Output = SaMovePtrFlag;
    #[inline]
    fn bitor(self, rhs: SaMovePtrFlag) -> SaMovePtrFlag {
        SaMovePtrFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SaMovePtrFlag {
    type Output = SaMovePtrFlag;
    #[inline]
    fn bitxor(self, rhs: SaMovePtrFlag) -> SaMovePtrFlag {
        SaMovePtrFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaPtrBtn {
    pub ty: SaType,
    pub flags: Card8,
    pub count: Card8,
    pub button: Card8,
}
impl SaPtrBtn {}
impl AsByteSequence for SaPtrBtn {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += self.button.as_bytes(&mut bytes[index..]);
        index += 4;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaPtrBtn from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        Some((
            SaPtrBtn {
                ty: ty,
                flags: flags,
                count: count,
                button: button,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.flags.size() + self.count.size() + self.button.size() + 4
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaLockPtrBtn {
    pub ty: SaType,
    pub flags: Card8,
    pub button: Card8,
}
impl SaLockPtrBtn {}
impl AsByteSequence for SaLockPtrBtn {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.button.as_bytes(&mut bytes[index..]);
        index += 4;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaLockPtrBtn from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        Some((
            SaLockPtrBtn {
                ty: ty,
                flags: flags,
                button: button,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.flags.size() + 1 + self.button.size() + 4
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaSetPtrDflt {
    pub ty: SaType,
    pub flags: SaSetPtrDfltFlag,
    pub affect: SaSetPtrDfltFlag,
    pub value: Int8,
}
impl SaSetPtrDflt {}
impl AsByteSequence for SaSetPtrDflt {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.affect.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index += 4;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaSetPtrDflt from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (SaSetPtrDfltFlag, usize) =
            <SaSetPtrDfltFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect, sz): (SaSetPtrDfltFlag, usize) =
            <SaSetPtrDfltFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        Some((
            SaSetPtrDflt {
                ty: ty,
                flags: flags,
                affect: affect,
                value: value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.flags.size() + self.affect.size() + self.value.size() + 4
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaSetPtrDfltFlag {
    pub inner: u8,
}
impl SaSetPtrDfltFlag {
    #[inline]
    pub fn affect_dflt_button(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_affect_dflt_button(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn dflt_btn_absolute(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_dflt_btn_absolute(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(affect_dflt_button: bool, dflt_btn_absolute: bool) -> Self {
        let mut inner: u8 = 0;
        if affect_dflt_button {
            inner |= 1 << 0;
        }
        if dflt_btn_absolute {
            inner |= 1 << 2;
        }
        SaSetPtrDfltFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const AFFECT_DFLT_BUTTON: Self = Self { inner: 1 };
    pub const DFLT_BTN_ABSOLUTE: Self = Self { inner: 4 };
    pub const COMPLETE: Self = Self { inner: 5 };
}
impl AsByteSequence for SaSetPtrDfltFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((SaSetPtrDfltFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SaSetPtrDfltFlag {
    type Output = SaSetPtrDfltFlag;
    #[inline]
    fn not(self) -> SaSetPtrDfltFlag {
        SaSetPtrDfltFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SaSetPtrDfltFlag {
    type Output = SaSetPtrDfltFlag;
    #[inline]
    fn bitand(self, rhs: SaSetPtrDfltFlag) -> SaSetPtrDfltFlag {
        SaSetPtrDfltFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SaSetPtrDfltFlag {
    type Output = SaSetPtrDfltFlag;
    #[inline]
    fn bitor(self, rhs: SaSetPtrDfltFlag) -> SaSetPtrDfltFlag {
        SaSetPtrDfltFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SaSetPtrDfltFlag {
    type Output = SaSetPtrDfltFlag;
    #[inline]
    fn bitxor(self, rhs: SaSetPtrDfltFlag) -> SaSetPtrDfltFlag {
        SaSetPtrDfltFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaIsoLock {
    pub ty: SaType,
    pub flags: SaIsoLockFlag,
    pub mask: ModMask,
    pub real_mods: ModMask,
    pub group: Int8,
    pub affect: SaIsoLockNoAffect,
    pub vmods_high: VModsHigh,
    pub vmods_low: VModsLow,
}
impl SaIsoLock {}
impl AsByteSequence for SaIsoLock {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.real_mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        index += self.affect.as_bytes(&mut bytes[index..]);
        index += self.vmods_high.as_bytes(&mut bytes[index..]);
        index += self.vmods_low.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaIsoLock from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (SaIsoLockFlag, usize) = <SaIsoLockFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect, sz): (SaIsoLockNoAffect, usize) =
            <SaIsoLockNoAffect>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_high, sz): (VModsHigh, usize) = <VModsHigh>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_low, sz): (VModsLow, usize) = <VModsLow>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SaIsoLock {
                ty: ty,
                flags: flags,
                mask: mask,
                real_mods: real_mods,
                group: group,
                affect: affect,
                vmods_high: vmods_high,
                vmods_low: vmods_low,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.flags.size()
            + self.mask.size()
            + self.real_mods.size()
            + self.group.size()
            + self.affect.size()
            + self.vmods_high.size()
            + self.vmods_low.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaIsoLockFlag {
    pub inner: u8,
}
impl SaIsoLockFlag {
    #[inline]
    pub fn no_lock(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_no_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn no_unlock(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_no_unlock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn use_mod_map_mods(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_use_mod_map_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn iso_dflt_is_group(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_iso_dflt_is_group(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn new(
        no_lock: bool,
        no_unlock: bool,
        use_mod_map_mods: bool,
        iso_dflt_is_group: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if no_lock {
            inner |= 1 << 0;
        }
        if no_unlock {
            inner |= 1 << 1;
        }
        if use_mod_map_mods {
            inner |= 1 << 2;
        }
        if iso_dflt_is_group {
            inner |= 1 << 3;
        }
        SaIsoLockFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const NO_LOCK: Self = Self { inner: 1 };
    pub const NO_UNLOCK: Self = Self { inner: 2 };
    pub const USE_MOD_MAP_MODS: Self = Self { inner: 4 };
    pub const ISO_DFLT_IS_GROUP: Self = Self { inner: 8 };
    pub const COMPLETE: Self = Self { inner: 15 };
}
impl AsByteSequence for SaIsoLockFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((SaIsoLockFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SaIsoLockFlag {
    type Output = SaIsoLockFlag;
    #[inline]
    fn not(self) -> SaIsoLockFlag {
        SaIsoLockFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SaIsoLockFlag {
    type Output = SaIsoLockFlag;
    #[inline]
    fn bitand(self, rhs: SaIsoLockFlag) -> SaIsoLockFlag {
        SaIsoLockFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SaIsoLockFlag {
    type Output = SaIsoLockFlag;
    #[inline]
    fn bitor(self, rhs: SaIsoLockFlag) -> SaIsoLockFlag {
        SaIsoLockFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SaIsoLockFlag {
    type Output = SaIsoLockFlag;
    #[inline]
    fn bitxor(self, rhs: SaIsoLockFlag) -> SaIsoLockFlag {
        SaIsoLockFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SaIsoLockNoAffect {
    pub inner: u8,
}
impl SaIsoLockNoAffect {
    #[inline]
    pub fn ctrls(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_ctrls(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn ptr(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_ptr(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn group(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_group(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn mods(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn new(ctrls: bool, ptr: bool, group: bool, mods: bool) -> Self {
        let mut inner: u8 = 0;
        if ctrls {
            inner |= 1 << 3;
        }
        if ptr {
            inner |= 1 << 4;
        }
        if group {
            inner |= 1 << 5;
        }
        if mods {
            inner |= 1 << 6;
        }
        SaIsoLockNoAffect { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const CTRLS: Self = Self { inner: 8 };
    pub const PTR: Self = Self { inner: 16 };
    pub const GROUP: Self = Self { inner: 32 };
    pub const MODS: Self = Self { inner: 64 };
    pub const COMPLETE: Self = Self { inner: 120 };
}
impl AsByteSequence for SaIsoLockNoAffect {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((SaIsoLockNoAffect { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SaIsoLockNoAffect {
    type Output = SaIsoLockNoAffect;
    #[inline]
    fn not(self) -> SaIsoLockNoAffect {
        SaIsoLockNoAffect { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SaIsoLockNoAffect {
    type Output = SaIsoLockNoAffect;
    #[inline]
    fn bitand(self, rhs: SaIsoLockNoAffect) -> SaIsoLockNoAffect {
        SaIsoLockNoAffect {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SaIsoLockNoAffect {
    type Output = SaIsoLockNoAffect;
    #[inline]
    fn bitor(self, rhs: SaIsoLockNoAffect) -> SaIsoLockNoAffect {
        SaIsoLockNoAffect {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SaIsoLockNoAffect {
    type Output = SaIsoLockNoAffect;
    #[inline]
    fn bitxor(self, rhs: SaIsoLockNoAffect) -> SaIsoLockNoAffect {
        SaIsoLockNoAffect {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaTerminate {
    pub ty: SaType,
}
impl SaTerminate {}
impl AsByteSequence for SaTerminate {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += 7;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaTerminate from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 7;
        Some((SaTerminate { ty: ty }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + 7
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaSwitchScreen {
    pub ty: SaType,
    pub flags: Card8,
    pub new_screen: Int8,
}
impl SaSwitchScreen {}
impl AsByteSequence for SaSwitchScreen {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.new_screen.as_bytes(&mut bytes[index..]);
        index += 5;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaSwitchScreen from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (new_screen, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 5;
        Some((
            SaSwitchScreen {
                ty: ty,
                flags: flags,
                new_screen: new_screen,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.flags.size() + self.new_screen.size() + 5
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaSetControls {
    pub ty: SaType,
    pub bool_ctrls_high: BoolCtrlsHigh,
    pub bool_ctrls_low: BoolCtrlsLow,
}
impl SaSetControls {}
impl AsByteSequence for SaSetControls {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.bool_ctrls_high.as_bytes(&mut bytes[index..]);
        index += self.bool_ctrls_low.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaSetControls from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (bool_ctrls_high, sz): (BoolCtrlsHigh, usize) =
            <BoolCtrlsHigh>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bool_ctrls_low, sz): (BoolCtrlsLow, usize) =
            <BoolCtrlsLow>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            SaSetControls {
                ty: ty,
                bool_ctrls_high: bool_ctrls_high,
                bool_ctrls_low: bool_ctrls_low,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + 3 + self.bool_ctrls_high.size() + self.bool_ctrls_low.size() + 2
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoolCtrlsHigh {
    pub inner: u8,
}
impl BoolCtrlsHigh {
    #[inline]
    pub fn access_x_feedback(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_access_x_feedback(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn audible_bell(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_audible_bell(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn overlay1(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_overlay1(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn overlay2(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_overlay2(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn ignore_group_lock(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_ignore_group_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn new(
        access_x_feedback: bool,
        audible_bell: bool,
        overlay1: bool,
        overlay2: bool,
        ignore_group_lock: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if access_x_feedback {
            inner |= 1 << 0;
        }
        if audible_bell {
            inner |= 1 << 1;
        }
        if overlay1 {
            inner |= 1 << 2;
        }
        if overlay2 {
            inner |= 1 << 3;
        }
        if ignore_group_lock {
            inner |= 1 << 4;
        }
        BoolCtrlsHigh { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const ACCESS_X_FEEDBACK: Self = Self { inner: 1 };
    pub const AUDIBLE_BELL: Self = Self { inner: 2 };
    pub const OVERLAY1: Self = Self { inner: 4 };
    pub const OVERLAY2: Self = Self { inner: 8 };
    pub const IGNORE_GROUP_LOCK: Self = Self { inner: 16 };
    pub const COMPLETE: Self = Self { inner: 31 };
}
impl AsByteSequence for BoolCtrlsHigh {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((BoolCtrlsHigh { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for BoolCtrlsHigh {
    type Output = BoolCtrlsHigh;
    #[inline]
    fn not(self) -> BoolCtrlsHigh {
        BoolCtrlsHigh { inner: !self.inner }
    }
}
impl core::ops::BitAnd for BoolCtrlsHigh {
    type Output = BoolCtrlsHigh;
    #[inline]
    fn bitand(self, rhs: BoolCtrlsHigh) -> BoolCtrlsHigh {
        BoolCtrlsHigh {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for BoolCtrlsHigh {
    type Output = BoolCtrlsHigh;
    #[inline]
    fn bitor(self, rhs: BoolCtrlsHigh) -> BoolCtrlsHigh {
        BoolCtrlsHigh {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for BoolCtrlsHigh {
    type Output = BoolCtrlsHigh;
    #[inline]
    fn bitxor(self, rhs: BoolCtrlsHigh) -> BoolCtrlsHigh {
        BoolCtrlsHigh {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct BoolCtrlsLow {
    pub inner: u8,
}
impl BoolCtrlsLow {
    #[inline]
    pub fn repeat_keys(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_repeat_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn slow_keys(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_slow_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn bounce_keys(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_bounce_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn sticky_keys(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_sticky_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn mouse_keys(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_mouse_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn mouse_keys_accel(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_mouse_keys_accel(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn access_x_keys(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_access_x_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn access_x_timeout(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_access_x_timeout(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        repeat_keys: bool,
        slow_keys: bool,
        bounce_keys: bool,
        sticky_keys: bool,
        mouse_keys: bool,
        mouse_keys_accel: bool,
        access_x_keys: bool,
        access_x_timeout: bool,
    ) -> Self {
        let mut inner: u8 = 0;
        if repeat_keys {
            inner |= 1 << 0;
        }
        if slow_keys {
            inner |= 1 << 1;
        }
        if bounce_keys {
            inner |= 1 << 2;
        }
        if sticky_keys {
            inner |= 1 << 3;
        }
        if mouse_keys {
            inner |= 1 << 4;
        }
        if mouse_keys_accel {
            inner |= 1 << 5;
        }
        if access_x_keys {
            inner |= 1 << 6;
        }
        if access_x_timeout {
            inner |= 1 << 7;
        }
        BoolCtrlsLow { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const REPEAT_KEYS: Self = Self { inner: 1 };
    pub const SLOW_KEYS: Self = Self { inner: 2 };
    pub const BOUNCE_KEYS: Self = Self { inner: 4 };
    pub const STICKY_KEYS: Self = Self { inner: 8 };
    pub const MOUSE_KEYS: Self = Self { inner: 16 };
    pub const MOUSE_KEYS_ACCEL: Self = Self { inner: 32 };
    pub const ACCESS_X_KEYS: Self = Self { inner: 64 };
    pub const ACCESS_X_TIMEOUT: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for BoolCtrlsLow {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((BoolCtrlsLow { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for BoolCtrlsLow {
    type Output = BoolCtrlsLow;
    #[inline]
    fn not(self) -> BoolCtrlsLow {
        BoolCtrlsLow { inner: !self.inner }
    }
}
impl core::ops::BitAnd for BoolCtrlsLow {
    type Output = BoolCtrlsLow;
    #[inline]
    fn bitand(self, rhs: BoolCtrlsLow) -> BoolCtrlsLow {
        BoolCtrlsLow {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for BoolCtrlsLow {
    type Output = BoolCtrlsLow;
    #[inline]
    fn bitor(self, rhs: BoolCtrlsLow) -> BoolCtrlsLow {
        BoolCtrlsLow {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for BoolCtrlsLow {
    type Output = BoolCtrlsLow;
    #[inline]
    fn bitxor(self, rhs: BoolCtrlsLow) -> BoolCtrlsLow {
        BoolCtrlsLow {
            inner: self.inner ^ rhs.inner,
        }
    }
}
pub type SaLockControls = SaSetControls;
#[derive(Clone, Debug, Default)]
pub struct SaActionMessage {
    pub ty: SaType,
    pub flags: ActionMessageFlag,
    pub message: [Card8; 6],
}
impl SaActionMessage {}
impl AsByteSequence for SaActionMessage {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.message.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaActionMessage from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (ActionMessageFlag, usize) =
            <ActionMessageFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (message, sz): ([Card8; 6], usize) = <[Card8; 6]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SaActionMessage {
                ty: ty,
                flags: flags,
                message: message,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.flags.size() + self.message.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct ActionMessageFlag {
    pub inner: u8,
}
impl ActionMessageFlag {
    #[inline]
    pub fn on_press(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_on_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn on_release(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_on_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn gen_key_event(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_gen_key_event(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(on_press: bool, on_release: bool, gen_key_event: bool) -> Self {
        let mut inner: u8 = 0;
        if on_press {
            inner |= 1 << 0;
        }
        if on_release {
            inner |= 1 << 1;
        }
        if gen_key_event {
            inner |= 1 << 2;
        }
        ActionMessageFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const ON_PRESS: Self = Self { inner: 1 };
    pub const ON_RELEASE: Self = Self { inner: 2 };
    pub const GEN_KEY_EVENT: Self = Self { inner: 4 };
    pub const COMPLETE: Self = Self { inner: 7 };
}
impl AsByteSequence for ActionMessageFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((ActionMessageFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for ActionMessageFlag {
    type Output = ActionMessageFlag;
    #[inline]
    fn not(self) -> ActionMessageFlag {
        ActionMessageFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for ActionMessageFlag {
    type Output = ActionMessageFlag;
    #[inline]
    fn bitand(self, rhs: ActionMessageFlag) -> ActionMessageFlag {
        ActionMessageFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for ActionMessageFlag {
    type Output = ActionMessageFlag;
    #[inline]
    fn bitor(self, rhs: ActionMessageFlag) -> ActionMessageFlag {
        ActionMessageFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for ActionMessageFlag {
    type Output = ActionMessageFlag;
    #[inline]
    fn bitxor(self, rhs: ActionMessageFlag) -> ActionMessageFlag {
        ActionMessageFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaRedirectKey {
    pub ty: SaType,
    pub newkey: Keycode,
    pub mask: ModMask,
    pub real_modifiers: ModMask,
    pub vmods_mask_high: VModsHigh,
    pub vmods_mask_low: VModsLow,
    pub vmods_high: VModsHigh,
    pub vmods_low: VModsLow,
}
impl SaRedirectKey {}
impl AsByteSequence for SaRedirectKey {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.newkey.as_bytes(&mut bytes[index..]);
        index += self.mask.as_bytes(&mut bytes[index..]);
        index += self.real_modifiers.as_bytes(&mut bytes[index..]);
        index += self.vmods_mask_high.as_bytes(&mut bytes[index..]);
        index += self.vmods_mask_low.as_bytes(&mut bytes[index..]);
        index += self.vmods_high.as_bytes(&mut bytes[index..]);
        index += self.vmods_low.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaRedirectKey from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (newkey, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_modifiers, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_mask_high, sz): (VModsHigh, usize) = <VModsHigh>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_mask_low, sz): (VModsLow, usize) = <VModsLow>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_high, sz): (VModsHigh, usize) = <VModsHigh>::from_bytes(&bytes[index..])?;
        index += sz;
        let (vmods_low, sz): (VModsLow, usize) = <VModsLow>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SaRedirectKey {
                ty: ty,
                newkey: newkey,
                mask: mask,
                real_modifiers: real_modifiers,
                vmods_mask_high: vmods_mask_high,
                vmods_mask_low: vmods_mask_low,
                vmods_high: vmods_high,
                vmods_low: vmods_low,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.newkey.size()
            + self.mask.size()
            + self.real_modifiers.size()
            + self.vmods_mask_high.size()
            + self.vmods_mask_low.size()
            + self.vmods_high.size()
            + self.vmods_low.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaDeviceBtn {
    pub ty: SaType,
    pub flags: Card8,
    pub count: Card8,
    pub button: Card8,
    pub device: Card8,
}
impl SaDeviceBtn {}
impl AsByteSequence for SaDeviceBtn {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.count.as_bytes(&mut bytes[index..]);
        index += self.button.as_bytes(&mut bytes[index..]);
        index += self.device.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaDeviceBtn from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (count, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            SaDeviceBtn {
                ty: ty,
                flags: flags,
                count: count,
                button: button,
                device: device,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.flags.size()
            + self.count.size()
            + self.button.size()
            + self.device.size()
            + 3
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaLockDeviceBtn {
    pub ty: SaType,
    pub flags: LockDeviceFlags,
    pub button: Card8,
    pub device: Card8,
}
impl SaLockDeviceBtn {}
impl AsByteSequence for SaLockDeviceBtn {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.button.as_bytes(&mut bytes[index..]);
        index += self.device.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaLockDeviceBtn from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (LockDeviceFlags, usize) = <LockDeviceFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            SaLockDeviceBtn {
                ty: ty,
                flags: flags,
                button: button,
                device: device,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.flags.size() + 1 + self.button.size() + self.device.size() + 3
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct LockDeviceFlags {
    pub inner: u8,
}
impl LockDeviceFlags {
    #[inline]
    pub fn no_lock(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_no_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn no_unlock(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_no_unlock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(no_lock: bool, no_unlock: bool) -> Self {
        let mut inner: u8 = 0;
        if no_lock {
            inner |= 1 << 0;
        }
        if no_unlock {
            inner |= 1 << 1;
        }
        LockDeviceFlags { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const NO_LOCK: Self = Self { inner: 1 };
    pub const NO_UNLOCK: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for LockDeviceFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((LockDeviceFlags { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for LockDeviceFlags {
    type Output = LockDeviceFlags;
    #[inline]
    fn not(self) -> LockDeviceFlags {
        LockDeviceFlags { inner: !self.inner }
    }
}
impl core::ops::BitAnd for LockDeviceFlags {
    type Output = LockDeviceFlags;
    #[inline]
    fn bitand(self, rhs: LockDeviceFlags) -> LockDeviceFlags {
        LockDeviceFlags {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for LockDeviceFlags {
    type Output = LockDeviceFlags;
    #[inline]
    fn bitor(self, rhs: LockDeviceFlags) -> LockDeviceFlags {
        LockDeviceFlags {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for LockDeviceFlags {
    type Output = LockDeviceFlags;
    #[inline]
    fn bitxor(self, rhs: LockDeviceFlags) -> LockDeviceFlags {
        LockDeviceFlags {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SaDeviceValuator {
    pub ty: SaType,
    pub device: Card8,
    pub val1what: SaValWhat,
    pub val1index: Card8,
    pub val1value: Card8,
    pub val2what: SaValWhat,
    pub val2index: Card8,
    pub val2value: Card8,
}
impl SaDeviceValuator {}
impl AsByteSequence for SaDeviceValuator {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.device.as_bytes(&mut bytes[index..]);
        index += self.val1what.as_bytes(&mut bytes[index..]);
        index += self.val1index.as_bytes(&mut bytes[index..]);
        index += self.val1value.as_bytes(&mut bytes[index..]);
        index += self.val2what.as_bytes(&mut bytes[index..]);
        index += self.val2index.as_bytes(&mut bytes[index..]);
        index += self.val2value.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SaDeviceValuator from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (val1what, sz): (SaValWhat, usize) = <SaValWhat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (val1index, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (val1value, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (val2what, sz): (SaValWhat, usize) = <SaValWhat>::from_bytes(&bytes[index..])?;
        index += sz;
        let (val2index, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (val2value, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SaDeviceValuator {
                ty: ty,
                device: device,
                val1what: val1what,
                val1index: val1index,
                val1value: val1value,
                val2what: val2what,
                val2index: val2index,
                val2value: val2value,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size()
            + self.device.size()
            + self.val1what.size()
            + self.val1index.size()
            + self.val1value.size()
            + self.val2what.size()
            + self.val2index.size()
            + self.val2value.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SaValWhat {
    IgnoreVal = 0,
    SetValMin = 1,
    SetValCenter = 2,
    SetValMax = 3,
    SetValRelative = 4,
    SetValAbsolute = 5,
}
impl AsByteSequence for SaValWhat {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::IgnoreVal, sz)),
            1 => Some((Self::SetValMin, sz)),
            2 => Some((Self::SetValCenter, sz)),
            3 => Some((Self::SetValMax, sz)),
            4 => Some((Self::SetValRelative, sz)),
            5 => Some((Self::SetValAbsolute, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SaValWhat {
    #[inline]
    fn default() -> SaValWhat {
        SaValWhat::IgnoreVal
    }
}
#[derive(Clone, Debug, Default)]
pub struct SiAction {
    pub ty: SaType,
    pub data: [Card8; 7],
}
impl SiAction {}
impl AsByteSequence for SiAction {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.ty.as_bytes(&mut bytes[index..]);
        index += self.data.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SiAction from byte buffer");
        let (ty, sz): (SaType, usize) = <SaType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (data, sz): ([Card8; 7], usize) = <[Card8; 7]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((SiAction { ty: ty, data: data }, index))
    }
    #[inline]
    fn size(&self) -> usize {
        self.ty.size() + self.data.size()
    }
}
#[derive(Clone, Debug, Default)]
pub struct SymInterpret {
    pub sym: Keysym,
    pub mods: ModMask,
    pub match_: Card8,
    pub virtual_mod: VModsLow,
    pub flags: Card8,
    pub action: SiAction,
}
impl SymInterpret {}
impl AsByteSequence for SymInterpret {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.sym.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.match_.as_bytes(&mut bytes[index..]);
        index += self.virtual_mod.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.action.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SymInterpret from byte buffer");
        let (sym, sz): (Keysym, usize) = <Keysym>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (match_, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mod, sz): (VModsLow, usize) = <VModsLow>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (action, sz): (SiAction, usize) = <SiAction>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SymInterpret {
                sym: sym,
                mods: mods,
                match_: match_,
                virtual_mod: virtual_mod,
                flags: flags,
                action: action,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.sym.size()
            + self.mods.size()
            + self.match_.size()
            + self.virtual_mod.size()
            + self.flags.size()
            + self.action.size()
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum SymInterpretMatch {
    NoneOf = 0,
    AnyOfOrNone = 1,
    AnyOf = 2,
    AllOf = 3,
    Exactly = 4,
}
impl AsByteSequence for SymInterpretMatch {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::NoneOf, sz)),
            1 => Some((Self::AnyOfOrNone, sz)),
            2 => Some((Self::AnyOf, sz)),
            3 => Some((Self::AllOf, sz)),
            4 => Some((Self::Exactly, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for SymInterpretMatch {
    #[inline]
    fn default() -> SymInterpretMatch {
        SymInterpretMatch::NoneOf
    }
}
#[derive(Clone, Debug, Default)]
pub struct UseExtensionRequest {
    pub req_type: u8,
    pub wanted_major: Card16,
    pub length: u16,
    pub wanted_minor: Card16,
}
impl UseExtensionRequest {}
impl AsByteSequence for UseExtensionRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.wanted_major.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.wanted_minor.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UseExtensionRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wanted_major, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wanted_minor, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            UseExtensionRequest {
                req_type: req_type,
                wanted_major: wanted_major,
                length: length,
                wanted_minor: wanted_minor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.wanted_major.size()
            + self.length.size()
            + self.wanted_minor.size()
    }
}
impl Request for UseExtensionRequest {
    const OPCODE: u8 = 0;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = UseExtensionReply;
}
#[derive(Clone, Debug, Default)]
pub struct UseExtensionReply {
    pub reply_type: u8,
    pub supported: bool,
    pub sequence: u16,
    pub length: u32,
    pub server_major: Card16,
    pub server_minor: Card16,
}
impl UseExtensionReply {}
impl AsByteSequence for UseExtensionReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.supported.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.server_major.as_bytes(&mut bytes[index..]);
        index += self.server_minor.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing UseExtensionReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (supported, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_major, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (server_minor, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            UseExtensionReply {
                reply_type: reply_type,
                supported: supported,
                sequence: sequence,
                length: length,
                server_major: server_major,
                server_minor: server_minor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.supported.size()
            + self.sequence.size()
            + self.length.size()
            + self.server_major.size()
            + self.server_minor.size()
            + 20
    }
}
#[derive(Clone, Debug, Default)]
pub struct SelectEventsRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub affect_which: EventType,
    pub clear: EventType,
    pub select_all: EventType,
    pub affect_map: MapPart,
    pub map: MapPart,
    pub affect_new_keyboard: NknDetail,
    pub new_keyboard_details: NknDetail,
    pub affect_state: StatePart,
    pub state_details: StatePart,
    pub affect_ctrls: Control,
    pub ctrl_details: Control,
    pub affect_indicator_state: Card32,
    pub indicator_state_details: Card32,
    pub affect_indicator_map: Card32,
    pub indicator_map_details: Card32,
    pub affect_names: NameDetail,
    pub names_details: NameDetail,
    pub affect_compat: CmDetail,
    pub compat_details: CmDetail,
    pub affect_bell: Card8,
    pub bell_details: Card8,
    pub affect_msg_details: Card8,
    pub msg_details: Card8,
    pub affect_access_x: AxnDetail,
    pub access_x_details: AxnDetail,
    pub affect_ext_dev: XiFeature,
    pub extdev_details: XiFeature,
}
impl SelectEventsRequest {}
impl AsByteSequence for SelectEventsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.affect_which.as_bytes(&mut bytes[index..]);
        index += self.clear.as_bytes(&mut bytes[index..]);
        index += self.select_all.as_bytes(&mut bytes[index..]);
        index += self.affect_map.as_bytes(&mut bytes[index..]);
        index += self.map.as_bytes(&mut bytes[index..]);
        let cond0 = ((self.affect_which) & ((!(self.clear)) & (!(self.select_all))));
        if cond0.new_keyboard_notify() {
            index += self.affect_new_keyboard.as_bytes(&mut bytes[index..]);
        }
        if cond0.new_keyboard_notify() {
            index += self.new_keyboard_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.state_notify() {
            index += self.affect_state.as_bytes(&mut bytes[index..]);
        }
        if cond0.state_notify() {
            index += self.state_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.controls_notify() {
            index += self.affect_ctrls.as_bytes(&mut bytes[index..]);
        }
        if cond0.controls_notify() {
            index += self.ctrl_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_state_notify() {
            index += self.affect_indicator_state.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_state_notify() {
            index += self.indicator_state_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_map_notify() {
            index += self.affect_indicator_map.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_map_notify() {
            index += self.indicator_map_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.names_notify() {
            index += self.affect_names.as_bytes(&mut bytes[index..]);
        }
        if cond0.names_notify() {
            index += self.names_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat_map_notify() {
            index += self.affect_compat.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat_map_notify() {
            index += self.compat_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.bell_notify() {
            index += self.affect_bell.as_bytes(&mut bytes[index..]);
        }
        if cond0.bell_notify() {
            index += self.bell_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.action_message() {
            index += self.affect_msg_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.action_message() {
            index += self.msg_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.access_x_notify() {
            index += self.affect_access_x.as_bytes(&mut bytes[index..]);
        }
        if cond0.access_x_notify() {
            index += self.access_x_details.as_bytes(&mut bytes[index..]);
        }
        if cond0.extension_device_notify() {
            index += self.affect_ext_dev.as_bytes(&mut bytes[index..]);
        }
        if cond0.extension_device_notify() {
            index += self.extdev_details.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SelectEventsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_which, sz): (EventType, usize) = <EventType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (clear, sz): (EventType, usize) = <EventType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (select_all, sz): (EventType, usize) = <EventType>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_map, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = ((affect_which) & ((!(clear)) & (!(select_all))));
        let affect_new_keyboard: NknDetail = if cond0.new_keyboard_notify() {
            let (affect_new_keyboard, sz): (NknDetail, usize) =
                <NknDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_new_keyboard
        } else {
            Default::default()
        };
        let new_keyboard_details: NknDetail = if cond0.new_keyboard_notify() {
            let (new_keyboard_details, sz): (NknDetail, usize) =
                <NknDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            new_keyboard_details
        } else {
            Default::default()
        };
        let affect_state: StatePart = if cond0.state_notify() {
            let (affect_state, sz): (StatePart, usize) = <StatePart>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_state
        } else {
            Default::default()
        };
        let state_details: StatePart = if cond0.state_notify() {
            let (state_details, sz): (StatePart, usize) = <StatePart>::from_bytes(&bytes[index..])?;
            index += sz;
            state_details
        } else {
            Default::default()
        };
        let affect_ctrls: Control = if cond0.controls_notify() {
            let (affect_ctrls, sz): (Control, usize) = <Control>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_ctrls
        } else {
            Default::default()
        };
        let ctrl_details: Control = if cond0.controls_notify() {
            let (ctrl_details, sz): (Control, usize) = <Control>::from_bytes(&bytes[index..])?;
            index += sz;
            ctrl_details
        } else {
            Default::default()
        };
        let affect_indicator_state: Card32 = if cond0.indicator_state_notify() {
            let (affect_indicator_state, sz): (Card32, usize) =
                <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_indicator_state
        } else {
            Default::default()
        };
        let indicator_state_details: Card32 = if cond0.indicator_state_notify() {
            let (indicator_state_details, sz): (Card32, usize) =
                <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            indicator_state_details
        } else {
            Default::default()
        };
        let affect_indicator_map: Card32 = if cond0.indicator_map_notify() {
            let (affect_indicator_map, sz): (Card32, usize) =
                <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_indicator_map
        } else {
            Default::default()
        };
        let indicator_map_details: Card32 = if cond0.indicator_map_notify() {
            let (indicator_map_details, sz): (Card32, usize) =
                <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            indicator_map_details
        } else {
            Default::default()
        };
        let affect_names: NameDetail = if cond0.names_notify() {
            let (affect_names, sz): (NameDetail, usize) =
                <NameDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_names
        } else {
            Default::default()
        };
        let names_details: NameDetail = if cond0.names_notify() {
            let (names_details, sz): (NameDetail, usize) =
                <NameDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            names_details
        } else {
            Default::default()
        };
        let affect_compat: CmDetail = if cond0.compat_map_notify() {
            let (affect_compat, sz): (CmDetail, usize) = <CmDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_compat
        } else {
            Default::default()
        };
        let compat_details: CmDetail = if cond0.compat_map_notify() {
            let (compat_details, sz): (CmDetail, usize) = <CmDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            compat_details
        } else {
            Default::default()
        };
        let affect_bell: Card8 = if cond0.bell_notify() {
            let (affect_bell, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_bell
        } else {
            Default::default()
        };
        let bell_details: Card8 = if cond0.bell_notify() {
            let (bell_details, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            bell_details
        } else {
            Default::default()
        };
        let affect_msg_details: Card8 = if cond0.action_message() {
            let (affect_msg_details, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_msg_details
        } else {
            Default::default()
        };
        let msg_details: Card8 = if cond0.action_message() {
            let (msg_details, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            msg_details
        } else {
            Default::default()
        };
        let affect_access_x: AxnDetail = if cond0.access_x_notify() {
            let (affect_access_x, sz): (AxnDetail, usize) =
                <AxnDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_access_x
        } else {
            Default::default()
        };
        let access_x_details: AxnDetail = if cond0.access_x_notify() {
            let (access_x_details, sz): (AxnDetail, usize) =
                <AxnDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            access_x_details
        } else {
            Default::default()
        };
        let affect_ext_dev: XiFeature = if cond0.extension_device_notify() {
            let (affect_ext_dev, sz): (XiFeature, usize) =
                <XiFeature>::from_bytes(&bytes[index..])?;
            index += sz;
            affect_ext_dev
        } else {
            Default::default()
        };
        let extdev_details: XiFeature = if cond0.extension_device_notify() {
            let (extdev_details, sz): (XiFeature, usize) =
                <XiFeature>::from_bytes(&bytes[index..])?;
            index += sz;
            extdev_details
        } else {
            Default::default()
        };
        Some((
            SelectEventsRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                affect_which: affect_which,
                clear: clear,
                select_all: select_all,
                affect_map: affect_map,
                map: map,
                affect_new_keyboard: affect_new_keyboard,
                new_keyboard_details: new_keyboard_details,
                affect_state: affect_state,
                state_details: state_details,
                affect_ctrls: affect_ctrls,
                ctrl_details: ctrl_details,
                affect_indicator_state: affect_indicator_state,
                indicator_state_details: indicator_state_details,
                affect_indicator_map: affect_indicator_map,
                indicator_map_details: indicator_map_details,
                affect_names: affect_names,
                names_details: names_details,
                affect_compat: affect_compat,
                compat_details: compat_details,
                affect_bell: affect_bell,
                bell_details: bell_details,
                affect_msg_details: affect_msg_details,
                msg_details: msg_details,
                affect_access_x: affect_access_x,
                access_x_details: access_x_details,
                affect_ext_dev: affect_ext_dev,
                extdev_details: extdev_details,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.affect_which.size()
            + self.clear.size()
            + self.select_all.size()
            + self.affect_map.size()
            + self.map.size()
            + self.affect_new_keyboard.size()
            + self.new_keyboard_details.size()
            + self.affect_state.size()
            + self.state_details.size()
            + self.affect_ctrls.size()
            + self.ctrl_details.size()
            + self.affect_indicator_state.size()
            + self.indicator_state_details.size()
            + self.affect_indicator_map.size()
            + self.indicator_map_details.size()
            + self.affect_names.size()
            + self.names_details.size()
            + self.affect_compat.size()
            + self.compat_details.size()
            + self.affect_bell.size()
            + self.bell_details.size()
            + self.affect_msg_details.size()
            + self.msg_details.size()
            + self.affect_access_x.size()
            + self.access_x_details.size()
            + self.affect_ext_dev.size()
            + self.extdev_details.size()
    }
}
impl Request for SelectEventsRequest {
    const OPCODE: u8 = 1;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct EventType {
    pub inner: u16,
}
impl EventType {
    #[inline]
    pub fn new_keyboard_notify(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_new_keyboard_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn map_notify(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_map_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn state_notify(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_state_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn controls_notify(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_controls_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn indicator_state_notify(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_indicator_state_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn indicator_map_notify(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_indicator_map_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn names_notify(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_names_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn compat_map_notify(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_compat_map_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn bell_notify(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_bell_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn action_message(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_action_message(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn access_x_notify(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_access_x_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn extension_device_notify(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_extension_device_notify(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn new(
        new_keyboard_notify: bool,
        map_notify: bool,
        state_notify: bool,
        controls_notify: bool,
        indicator_state_notify: bool,
        indicator_map_notify: bool,
        names_notify: bool,
        compat_map_notify: bool,
        bell_notify: bool,
        action_message: bool,
        access_x_notify: bool,
        extension_device_notify: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if new_keyboard_notify {
            inner |= 1 << 0;
        }
        if map_notify {
            inner |= 1 << 1;
        }
        if state_notify {
            inner |= 1 << 2;
        }
        if controls_notify {
            inner |= 1 << 3;
        }
        if indicator_state_notify {
            inner |= 1 << 4;
        }
        if indicator_map_notify {
            inner |= 1 << 5;
        }
        if names_notify {
            inner |= 1 << 6;
        }
        if compat_map_notify {
            inner |= 1 << 7;
        }
        if bell_notify {
            inner |= 1 << 8;
        }
        if action_message {
            inner |= 1 << 9;
        }
        if access_x_notify {
            inner |= 1 << 10;
        }
        if extension_device_notify {
            inner |= 1 << 11;
        }
        EventType { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const NEW_KEYBOARD_NOTIFY: Self = Self { inner: 1 };
    pub const MAP_NOTIFY: Self = Self { inner: 2 };
    pub const STATE_NOTIFY: Self = Self { inner: 4 };
    pub const CONTROLS_NOTIFY: Self = Self { inner: 8 };
    pub const INDICATOR_STATE_NOTIFY: Self = Self { inner: 16 };
    pub const INDICATOR_MAP_NOTIFY: Self = Self { inner: 32 };
    pub const NAMES_NOTIFY: Self = Self { inner: 64 };
    pub const COMPAT_MAP_NOTIFY: Self = Self { inner: 128 };
    pub const BELL_NOTIFY: Self = Self { inner: 256 };
    pub const ACTION_MESSAGE: Self = Self { inner: 512 };
    pub const ACCESS_X_NOTIFY: Self = Self { inner: 1024 };
    pub const EXTENSION_DEVICE_NOTIFY: Self = Self { inner: 2048 };
    pub const COMPLETE: Self = Self { inner: 4095 };
}
impl AsByteSequence for EventType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((EventType { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for EventType {
    type Output = EventType;
    #[inline]
    fn not(self) -> EventType {
        EventType { inner: !self.inner }
    }
}
impl core::ops::BitAnd for EventType {
    type Output = EventType;
    #[inline]
    fn bitand(self, rhs: EventType) -> EventType {
        EventType {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for EventType {
    type Output = EventType;
    #[inline]
    fn bitor(self, rhs: EventType) -> EventType {
        EventType {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for EventType {
    type Output = EventType;
    #[inline]
    fn bitxor(self, rhs: EventType) -> EventType {
        EventType {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct MapPart {
    pub inner: u16,
}
impl MapPart {
    #[inline]
    pub fn key_types(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_key_types(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn key_syms(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_key_syms(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn modifier_map(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_modifier_map(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn explicit_components(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_explicit_components(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn key_actions(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_key_actions(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn key_behaviors(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_key_behaviors(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn virtual_mods(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_virtual_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn virtual_mod_map(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_virtual_mod_map(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        key_types: bool,
        key_syms: bool,
        modifier_map: bool,
        explicit_components: bool,
        key_actions: bool,
        key_behaviors: bool,
        virtual_mods: bool,
        virtual_mod_map: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if key_types {
            inner |= 1 << 0;
        }
        if key_syms {
            inner |= 1 << 1;
        }
        if modifier_map {
            inner |= 1 << 2;
        }
        if explicit_components {
            inner |= 1 << 3;
        }
        if key_actions {
            inner |= 1 << 4;
        }
        if key_behaviors {
            inner |= 1 << 5;
        }
        if virtual_mods {
            inner |= 1 << 6;
        }
        if virtual_mod_map {
            inner |= 1 << 7;
        }
        MapPart { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const KEY_TYPES: Self = Self { inner: 1 };
    pub const KEY_SYMS: Self = Self { inner: 2 };
    pub const MODIFIER_MAP: Self = Self { inner: 4 };
    pub const EXPLICIT_COMPONENTS: Self = Self { inner: 8 };
    pub const KEY_ACTIONS: Self = Self { inner: 16 };
    pub const KEY_BEHAVIORS: Self = Self { inner: 32 };
    pub const VIRTUAL_MODS: Self = Self { inner: 64 };
    pub const VIRTUAL_MOD_MAP: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for MapPart {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((MapPart { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for MapPart {
    type Output = MapPart;
    #[inline]
    fn not(self) -> MapPart {
        MapPart { inner: !self.inner }
    }
}
impl core::ops::BitAnd for MapPart {
    type Output = MapPart;
    #[inline]
    fn bitand(self, rhs: MapPart) -> MapPart {
        MapPart {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for MapPart {
    type Output = MapPart;
    #[inline]
    fn bitor(self, rhs: MapPart) -> MapPart {
        MapPart {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for MapPart {
    type Output = MapPart;
    #[inline]
    fn bitxor(self, rhs: MapPart) -> MapPart {
        MapPart {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct BellRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub bell_class: BellClassSpec,
    pub bell_id: IdSpec,
    pub percent: Int8,
    pub force_sound: bool,
    pub event_only: bool,
    pub pitch: Int16,
    pub duration: Int16,
    pub name: Atom,
    pub window: Window,
}
impl BellRequest {}
impl AsByteSequence for BellRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.bell_class.as_bytes(&mut bytes[index..]);
        index += self.bell_id.as_bytes(&mut bytes[index..]);
        index += self.percent.as_bytes(&mut bytes[index..]);
        index += self.force_sound.as_bytes(&mut bytes[index..]);
        index += self.event_only.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.pitch.as_bytes(&mut bytes[index..]);
        index += self.duration.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.name.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BellRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_class, sz): (BellClassSpec, usize) =
            <BellClassSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_id, sz): (IdSpec, usize) = <IdSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (percent, sz): (Int8, usize) = <Int8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (force_sound, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_only, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (pitch, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (duration, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            BellRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                bell_class: bell_class,
                bell_id: bell_id,
                percent: percent,
                force_sound: force_sound,
                event_only: event_only,
                pitch: pitch,
                duration: duration,
                name: name,
                window: window,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.bell_class.size()
            + self.bell_id.size()
            + self.percent.size()
            + self.force_sound.size()
            + self.event_only.size()
            + 1
            + self.pitch.size()
            + self.duration.size()
            + 2
            + self.name.size()
            + self.window.size()
    }
}
impl Request for BellRequest {
    const OPCODE: u8 = 3;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetStateRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
}
impl GetStateRequest {}
impl AsByteSequence for GetStateRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetStateRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetStateRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.device_spec.size() + self.length.size() + 2
    }
}
impl Request for GetStateRequest {
    const OPCODE: u8 = 4;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetStateReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetStateReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub mods: ModMask,
    pub base_mods: ModMask,
    pub latched_mods: ModMask,
    pub locked_mods: ModMask,
    pub group: Group,
    pub locked_group: Group,
    pub base_group: Int16,
    pub latched_group: Int16,
    pub compat_state: ModMask,
    pub grab_mods: ModMask,
    pub compat_grab_mods: ModMask,
    pub lookup_mods: ModMask,
    pub compat_lookup_mods: ModMask,
    pub ptr_btn_state: KeyButMask,
}
impl GetStateReply {}
impl AsByteSequence for GetStateReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.base_mods.as_bytes(&mut bytes[index..]);
        index += self.latched_mods.as_bytes(&mut bytes[index..]);
        index += self.locked_mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        index += self.locked_group.as_bytes(&mut bytes[index..]);
        index += self.base_group.as_bytes(&mut bytes[index..]);
        index += self.latched_group.as_bytes(&mut bytes[index..]);
        index += self.compat_state.as_bytes(&mut bytes[index..]);
        index += self.grab_mods.as_bytes(&mut bytes[index..]);
        index += self.compat_grab_mods.as_bytes(&mut bytes[index..]);
        index += self.lookup_mods.as_bytes(&mut bytes[index..]);
        index += self.compat_lookup_mods.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.ptr_btn_state.as_bytes(&mut bytes[index..]);
        index += 6;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetStateReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (base_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (latched_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (locked_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (Group, usize) = <Group>::from_bytes(&bytes[index..])?;
        index += sz;
        let (locked_group, sz): (Group, usize) = <Group>::from_bytes(&bytes[index..])?;
        index += sz;
        let (base_group, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (latched_group, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (compat_state, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (compat_grab_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lookup_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (compat_lookup_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (ptr_btn_state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 6;
        Some((
            GetStateReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                mods: mods,
                base_mods: base_mods,
                latched_mods: latched_mods,
                locked_mods: locked_mods,
                group: group,
                locked_group: locked_group,
                base_group: base_group,
                latched_group: latched_group,
                compat_state: compat_state,
                grab_mods: grab_mods,
                compat_grab_mods: compat_grab_mods,
                lookup_mods: lookup_mods,
                compat_lookup_mods: compat_lookup_mods,
                ptr_btn_state: ptr_btn_state,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.mods.size()
            + self.base_mods.size()
            + self.latched_mods.size()
            + self.locked_mods.size()
            + self.group.size()
            + self.locked_group.size()
            + self.base_group.size()
            + self.latched_group.size()
            + self.compat_state.size()
            + self.grab_mods.size()
            + self.compat_grab_mods.size()
            + self.lookup_mods.size()
            + self.compat_lookup_mods.size()
            + 1
            + self.ptr_btn_state.size()
            + 6
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Group {
    One = 0,
    Two = 1,
    Three = 2,
    Four = 3,
}
impl AsByteSequence for Group {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::One, sz)),
            1 => Some((Self::Two, sz)),
            2 => Some((Self::Three, sz)),
            3 => Some((Self::Four, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for Group {
    #[inline]
    fn default() -> Group {
        Group::One
    }
}
#[derive(Clone, Debug, Default)]
pub struct LatchLockStateRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub affect_mod_locks: ModMask,
    pub mod_locks: ModMask,
    pub lock_group: bool,
    pub group_lock: Group,
    pub affect_mod_latches: ModMask,
    pub latch_group: bool,
    pub group_latch: Card16,
}
impl LatchLockStateRequest {}
impl AsByteSequence for LatchLockStateRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.affect_mod_locks.as_bytes(&mut bytes[index..]);
        index += self.mod_locks.as_bytes(&mut bytes[index..]);
        index += self.lock_group.as_bytes(&mut bytes[index..]);
        index += self.group_lock.as_bytes(&mut bytes[index..]);
        index += self.affect_mod_latches.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.latch_group.as_bytes(&mut bytes[index..]);
        index += self.group_latch.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing LatchLockStateRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_mod_locks, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mod_locks, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lock_group, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group_lock, sz): (Group, usize) = <Group>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_mod_latches, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (latch_group, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group_latch, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            LatchLockStateRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                affect_mod_locks: affect_mod_locks,
                mod_locks: mod_locks,
                lock_group: lock_group,
                group_lock: group_lock,
                affect_mod_latches: affect_mod_latches,
                latch_group: latch_group,
                group_latch: group_latch,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.affect_mod_locks.size()
            + self.mod_locks.size()
            + self.lock_group.size()
            + self.group_lock.size()
            + self.affect_mod_latches.size()
            + 1
            + self.latch_group.size()
            + self.group_latch.size()
    }
}
impl Request for LatchLockStateRequest {
    const OPCODE: u8 = 5;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetControlsRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
}
impl GetControlsRequest {}
impl AsByteSequence for GetControlsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetControlsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetControlsRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.device_spec.size() + self.length.size() + 2
    }
}
impl Request for GetControlsRequest {
    const OPCODE: u8 = 6;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetControlsReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetControlsReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub mouse_keys_dflt_btn: Card8,
    pub num_groups: Card8,
    pub groups_wrap: Card8,
    pub internal_mods_mask: ModMask,
    pub ignore_lock_mods_mask: ModMask,
    pub internal_mods_real_mods: ModMask,
    pub ignore_lock_mods_real_mods: ModMask,
    pub internal_mods_vmods: VMod,
    pub ignore_lock_mods_vmods: VMod,
    pub repeat_delay: Card16,
    pub repeat_interval: Card16,
    pub slow_keys_delay: Card16,
    pub debounce_delay: Card16,
    pub mouse_keys_delay: Card16,
    pub mouse_keys_interval: Card16,
    pub mouse_keys_time_to_max: Card16,
    pub mouse_keys_max_speed: Card16,
    pub mouse_keys_curve: Int16,
    pub access_x_option: AxOption,
    pub access_x_timeout: Card16,
    pub access_x_timeout_options_mask: AxOption,
    pub access_x_timeout_options_values: AxOption,
    pub access_x_timeout_mask: BoolCtrl,
    pub access_x_timeout_values: BoolCtrl,
    pub enabled_controls: BoolCtrl,
    pub per_key_repeat: [Card8; 32],
}
impl GetControlsReply {}
impl AsByteSequence for GetControlsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_dflt_btn.as_bytes(&mut bytes[index..]);
        index += self.num_groups.as_bytes(&mut bytes[index..]);
        index += self.groups_wrap.as_bytes(&mut bytes[index..]);
        index += self.internal_mods_mask.as_bytes(&mut bytes[index..]);
        index += self.ignore_lock_mods_mask.as_bytes(&mut bytes[index..]);
        index += self.internal_mods_real_mods.as_bytes(&mut bytes[index..]);
        index += self
            .ignore_lock_mods_real_mods
            .as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.internal_mods_vmods.as_bytes(&mut bytes[index..]);
        index += self.ignore_lock_mods_vmods.as_bytes(&mut bytes[index..]);
        index += self.repeat_delay.as_bytes(&mut bytes[index..]);
        index += self.repeat_interval.as_bytes(&mut bytes[index..]);
        index += self.slow_keys_delay.as_bytes(&mut bytes[index..]);
        index += self.debounce_delay.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_delay.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_interval.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_time_to_max.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_max_speed.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_curve.as_bytes(&mut bytes[index..]);
        index += self.access_x_option.as_bytes(&mut bytes[index..]);
        index += self.access_x_timeout.as_bytes(&mut bytes[index..]);
        index += self
            .access_x_timeout_options_mask
            .as_bytes(&mut bytes[index..]);
        index += self
            .access_x_timeout_options_values
            .as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.access_x_timeout_mask.as_bytes(&mut bytes[index..]);
        index += self.access_x_timeout_values.as_bytes(&mut bytes[index..]);
        index += self.enabled_controls.as_bytes(&mut bytes[index..]);
        index += self.per_key_repeat.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetControlsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_dflt_btn, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_groups, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (groups_wrap, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (internal_mods_mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ignore_lock_mods_mask, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (internal_mods_real_mods, sz): (ModMask, usize) =
            <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ignore_lock_mods_real_mods, sz): (ModMask, usize) =
            <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (internal_mods_vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ignore_lock_mods_vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (repeat_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (repeat_interval, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (slow_keys_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (debounce_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_interval, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_time_to_max, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_max_speed, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_curve, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_option, sz): (AxOption, usize) = <AxOption>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout_options_mask, sz): (AxOption, usize) =
            <AxOption>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout_options_values, sz): (AxOption, usize) =
            <AxOption>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (access_x_timeout_mask, sz): (BoolCtrl, usize) =
            <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout_values, sz): (BoolCtrl, usize) =
            <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled_controls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (per_key_repeat, sz): ([Card8; 32], usize) =
            <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetControlsReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                mouse_keys_dflt_btn: mouse_keys_dflt_btn,
                num_groups: num_groups,
                groups_wrap: groups_wrap,
                internal_mods_mask: internal_mods_mask,
                ignore_lock_mods_mask: ignore_lock_mods_mask,
                internal_mods_real_mods: internal_mods_real_mods,
                ignore_lock_mods_real_mods: ignore_lock_mods_real_mods,
                internal_mods_vmods: internal_mods_vmods,
                ignore_lock_mods_vmods: ignore_lock_mods_vmods,
                repeat_delay: repeat_delay,
                repeat_interval: repeat_interval,
                slow_keys_delay: slow_keys_delay,
                debounce_delay: debounce_delay,
                mouse_keys_delay: mouse_keys_delay,
                mouse_keys_interval: mouse_keys_interval,
                mouse_keys_time_to_max: mouse_keys_time_to_max,
                mouse_keys_max_speed: mouse_keys_max_speed,
                mouse_keys_curve: mouse_keys_curve,
                access_x_option: access_x_option,
                access_x_timeout: access_x_timeout,
                access_x_timeout_options_mask: access_x_timeout_options_mask,
                access_x_timeout_options_values: access_x_timeout_options_values,
                access_x_timeout_mask: access_x_timeout_mask,
                access_x_timeout_values: access_x_timeout_values,
                enabled_controls: enabled_controls,
                per_key_repeat: per_key_repeat,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.mouse_keys_dflt_btn.size()
            + self.num_groups.size()
            + self.groups_wrap.size()
            + self.internal_mods_mask.size()
            + self.ignore_lock_mods_mask.size()
            + self.internal_mods_real_mods.size()
            + self.ignore_lock_mods_real_mods.size()
            + 1
            + self.internal_mods_vmods.size()
            + self.ignore_lock_mods_vmods.size()
            + self.repeat_delay.size()
            + self.repeat_interval.size()
            + self.slow_keys_delay.size()
            + self.debounce_delay.size()
            + self.mouse_keys_delay.size()
            + self.mouse_keys_interval.size()
            + self.mouse_keys_time_to_max.size()
            + self.mouse_keys_max_speed.size()
            + self.mouse_keys_curve.size()
            + self.access_x_option.size()
            + self.access_x_timeout.size()
            + self.access_x_timeout_options_mask.size()
            + self.access_x_timeout_options_values.size()
            + 2
            + self.access_x_timeout_mask.size()
            + self.access_x_timeout_values.size()
            + self.enabled_controls.size()
            + self.per_key_repeat.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AxOption {
    pub inner: u16,
}
impl AxOption {
    #[inline]
    pub fn sk_press_fb(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_sk_press_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn sk_accept_fb(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_sk_accept_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn feature_fb(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_feature_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn slow_warn_fb(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_slow_warn_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn indicator_fb(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_indicator_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn sticky_keys_fb(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_sticky_keys_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn two_keys(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_two_keys(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn latch_to_lock(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_latch_to_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn sk_release_fb(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_sk_release_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn sk_reject_fb(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_sk_reject_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn bk_reject_fb(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_bk_reject_fb(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn dumb_bell(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_dumb_bell(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn new(
        sk_press_fb: bool,
        sk_accept_fb: bool,
        feature_fb: bool,
        slow_warn_fb: bool,
        indicator_fb: bool,
        sticky_keys_fb: bool,
        two_keys: bool,
        latch_to_lock: bool,
        sk_release_fb: bool,
        sk_reject_fb: bool,
        bk_reject_fb: bool,
        dumb_bell: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if sk_press_fb {
            inner |= 1 << 0;
        }
        if sk_accept_fb {
            inner |= 1 << 1;
        }
        if feature_fb {
            inner |= 1 << 2;
        }
        if slow_warn_fb {
            inner |= 1 << 3;
        }
        if indicator_fb {
            inner |= 1 << 4;
        }
        if sticky_keys_fb {
            inner |= 1 << 5;
        }
        if two_keys {
            inner |= 1 << 6;
        }
        if latch_to_lock {
            inner |= 1 << 7;
        }
        if sk_release_fb {
            inner |= 1 << 8;
        }
        if sk_reject_fb {
            inner |= 1 << 9;
        }
        if bk_reject_fb {
            inner |= 1 << 10;
        }
        if dumb_bell {
            inner |= 1 << 11;
        }
        AxOption { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const SK_PRESS_FB: Self = Self { inner: 1 };
    pub const SK_ACCEPT_FB: Self = Self { inner: 2 };
    pub const FEATURE_FB: Self = Self { inner: 4 };
    pub const SLOW_WARN_FB: Self = Self { inner: 8 };
    pub const INDICATOR_FB: Self = Self { inner: 16 };
    pub const STICKY_KEYS_FB: Self = Self { inner: 32 };
    pub const TWO_KEYS: Self = Self { inner: 64 };
    pub const LATCH_TO_LOCK: Self = Self { inner: 128 };
    pub const SK_RELEASE_FB: Self = Self { inner: 256 };
    pub const SK_REJECT_FB: Self = Self { inner: 512 };
    pub const BK_REJECT_FB: Self = Self { inner: 1024 };
    pub const DUMB_BELL: Self = Self { inner: 2048 };
    pub const COMPLETE: Self = Self { inner: 4095 };
}
impl AsByteSequence for AxOption {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((AxOption { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for AxOption {
    type Output = AxOption;
    #[inline]
    fn not(self) -> AxOption {
        AxOption { inner: !self.inner }
    }
}
impl core::ops::BitAnd for AxOption {
    type Output = AxOption;
    #[inline]
    fn bitand(self, rhs: AxOption) -> AxOption {
        AxOption {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for AxOption {
    type Output = AxOption;
    #[inline]
    fn bitor(self, rhs: AxOption) -> AxOption {
        AxOption {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for AxOption {
    type Output = AxOption;
    #[inline]
    fn bitxor(self, rhs: AxOption) -> AxOption {
        AxOption {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetControlsRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub affect_internal_real_mods: ModMask,
    pub internal_real_mods: ModMask,
    pub affect_ignore_lock_real_mods: ModMask,
    pub ignore_lock_real_mods: ModMask,
    pub affect_internal_virtual_mods: VMod,
    pub internal_virtual_mods: VMod,
    pub affect_ignore_lock_virtual_mods: VMod,
    pub ignore_lock_virtual_mods: VMod,
    pub mouse_keys_dflt_btn: Card8,
    pub groups_wrap: Card8,
    pub access_x_options: AxOption,
    pub affect_enabled_controls: BoolCtrl,
    pub enabled_controls: BoolCtrl,
    pub change_controls: Control,
    pub repeat_delay: Card16,
    pub repeat_interval: Card16,
    pub slow_keys_delay: Card16,
    pub debounce_delay: Card16,
    pub mouse_keys_delay: Card16,
    pub mouse_keys_interval: Card16,
    pub mouse_keys_time_to_max: Card16,
    pub mouse_keys_max_speed: Card16,
    pub mouse_keys_curve: Int16,
    pub access_x_timeout: Card16,
    pub access_x_timeout_mask: BoolCtrl,
    pub access_x_timeout_values: BoolCtrl,
    pub access_x_timeout_options_mask: AxOption,
    pub access_x_timeout_options_values: AxOption,
    pub per_key_repeat: [Card8; 32],
}
impl SetControlsRequest {}
impl AsByteSequence for SetControlsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.affect_internal_real_mods.as_bytes(&mut bytes[index..]);
        index += self.internal_real_mods.as_bytes(&mut bytes[index..]);
        index += self
            .affect_ignore_lock_real_mods
            .as_bytes(&mut bytes[index..]);
        index += self.ignore_lock_real_mods.as_bytes(&mut bytes[index..]);
        index += self
            .affect_internal_virtual_mods
            .as_bytes(&mut bytes[index..]);
        index += self.internal_virtual_mods.as_bytes(&mut bytes[index..]);
        index += self
            .affect_ignore_lock_virtual_mods
            .as_bytes(&mut bytes[index..]);
        index += self.ignore_lock_virtual_mods.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_dflt_btn.as_bytes(&mut bytes[index..]);
        index += self.groups_wrap.as_bytes(&mut bytes[index..]);
        index += self.access_x_options.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.affect_enabled_controls.as_bytes(&mut bytes[index..]);
        index += self.enabled_controls.as_bytes(&mut bytes[index..]);
        index += self.change_controls.as_bytes(&mut bytes[index..]);
        index += self.repeat_delay.as_bytes(&mut bytes[index..]);
        index += self.repeat_interval.as_bytes(&mut bytes[index..]);
        index += self.slow_keys_delay.as_bytes(&mut bytes[index..]);
        index += self.debounce_delay.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_delay.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_interval.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_time_to_max.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_max_speed.as_bytes(&mut bytes[index..]);
        index += self.mouse_keys_curve.as_bytes(&mut bytes[index..]);
        index += self.access_x_timeout.as_bytes(&mut bytes[index..]);
        index += self.access_x_timeout_mask.as_bytes(&mut bytes[index..]);
        index += self.access_x_timeout_values.as_bytes(&mut bytes[index..]);
        index += self
            .access_x_timeout_options_mask
            .as_bytes(&mut bytes[index..]);
        index += self
            .access_x_timeout_options_values
            .as_bytes(&mut bytes[index..]);
        index += self.per_key_repeat.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetControlsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_internal_real_mods, sz): (ModMask, usize) =
            <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (internal_real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_ignore_lock_real_mods, sz): (ModMask, usize) =
            <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ignore_lock_real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_internal_virtual_mods, sz): (VMod, usize) =
            <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (internal_virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_ignore_lock_virtual_mods, sz): (VMod, usize) =
            <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ignore_lock_virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_dflt_btn, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (groups_wrap, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_options, sz): (AxOption, usize) = <AxOption>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (affect_enabled_controls, sz): (BoolCtrl, usize) =
            <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled_controls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (change_controls, sz): (Control, usize) = <Control>::from_bytes(&bytes[index..])?;
        index += sz;
        let (repeat_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (repeat_interval, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (slow_keys_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (debounce_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_interval, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_time_to_max, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_max_speed, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mouse_keys_curve, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout_mask, sz): (BoolCtrl, usize) =
            <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout_values, sz): (BoolCtrl, usize) =
            <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout_options_mask, sz): (AxOption, usize) =
            <AxOption>::from_bytes(&bytes[index..])?;
        index += sz;
        let (access_x_timeout_options_values, sz): (AxOption, usize) =
            <AxOption>::from_bytes(&bytes[index..])?;
        index += sz;
        let (per_key_repeat, sz): ([Card8; 32], usize) =
            <[Card8; 32]>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetControlsRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                affect_internal_real_mods: affect_internal_real_mods,
                internal_real_mods: internal_real_mods,
                affect_ignore_lock_real_mods: affect_ignore_lock_real_mods,
                ignore_lock_real_mods: ignore_lock_real_mods,
                affect_internal_virtual_mods: affect_internal_virtual_mods,
                internal_virtual_mods: internal_virtual_mods,
                affect_ignore_lock_virtual_mods: affect_ignore_lock_virtual_mods,
                ignore_lock_virtual_mods: ignore_lock_virtual_mods,
                mouse_keys_dflt_btn: mouse_keys_dflt_btn,
                groups_wrap: groups_wrap,
                access_x_options: access_x_options,
                affect_enabled_controls: affect_enabled_controls,
                enabled_controls: enabled_controls,
                change_controls: change_controls,
                repeat_delay: repeat_delay,
                repeat_interval: repeat_interval,
                slow_keys_delay: slow_keys_delay,
                debounce_delay: debounce_delay,
                mouse_keys_delay: mouse_keys_delay,
                mouse_keys_interval: mouse_keys_interval,
                mouse_keys_time_to_max: mouse_keys_time_to_max,
                mouse_keys_max_speed: mouse_keys_max_speed,
                mouse_keys_curve: mouse_keys_curve,
                access_x_timeout: access_x_timeout,
                access_x_timeout_mask: access_x_timeout_mask,
                access_x_timeout_values: access_x_timeout_values,
                access_x_timeout_options_mask: access_x_timeout_options_mask,
                access_x_timeout_options_values: access_x_timeout_options_values,
                per_key_repeat: per_key_repeat,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.affect_internal_real_mods.size()
            + self.internal_real_mods.size()
            + self.affect_ignore_lock_real_mods.size()
            + self.ignore_lock_real_mods.size()
            + self.affect_internal_virtual_mods.size()
            + self.internal_virtual_mods.size()
            + self.affect_ignore_lock_virtual_mods.size()
            + self.ignore_lock_virtual_mods.size()
            + self.mouse_keys_dflt_btn.size()
            + self.groups_wrap.size()
            + self.access_x_options.size()
            + 2
            + self.affect_enabled_controls.size()
            + self.enabled_controls.size()
            + self.change_controls.size()
            + self.repeat_delay.size()
            + self.repeat_interval.size()
            + self.slow_keys_delay.size()
            + self.debounce_delay.size()
            + self.mouse_keys_delay.size()
            + self.mouse_keys_interval.size()
            + self.mouse_keys_time_to_max.size()
            + self.mouse_keys_max_speed.size()
            + self.mouse_keys_curve.size()
            + self.access_x_timeout.size()
            + self.access_x_timeout_mask.size()
            + self.access_x_timeout_values.size()
            + self.access_x_timeout_options_mask.size()
            + self.access_x_timeout_options_values.size()
            + self.per_key_repeat.size()
    }
}
impl Request for SetControlsRequest {
    const OPCODE: u8 = 7;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct Control {
    pub inner: u32,
}
impl Control {
    #[inline]
    pub fn groups_wrap(&self) -> bool {
        self.inner & (1 << 27) != 0
    }
    #[inline]
    pub fn set_groups_wrap(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 27;
        } else {
            self.inner &= !(1 << 27);
        }
        self
    }
    #[inline]
    pub fn internal_mods(&self) -> bool {
        self.inner & (1 << 28) != 0
    }
    #[inline]
    pub fn set_internal_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 28;
        } else {
            self.inner &= !(1 << 28);
        }
        self
    }
    #[inline]
    pub fn ignore_lock_mods(&self) -> bool {
        self.inner & (1 << 29) != 0
    }
    #[inline]
    pub fn set_ignore_lock_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 29;
        } else {
            self.inner &= !(1 << 29);
        }
        self
    }
    #[inline]
    pub fn per_key_repeat(&self) -> bool {
        self.inner & (1 << 30) != 0
    }
    #[inline]
    pub fn set_per_key_repeat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 30;
        } else {
            self.inner &= !(1 << 30);
        }
        self
    }
    #[inline]
    pub fn controls_enabled(&self) -> bool {
        self.inner & (1 << 31) != 0
    }
    #[inline]
    pub fn set_controls_enabled(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 31;
        } else {
            self.inner &= !(1 << 31);
        }
        self
    }
    #[inline]
    pub fn new(
        groups_wrap: bool,
        internal_mods: bool,
        ignore_lock_mods: bool,
        per_key_repeat: bool,
        controls_enabled: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if groups_wrap {
            inner |= 1 << 27;
        }
        if internal_mods {
            inner |= 1 << 28;
        }
        if ignore_lock_mods {
            inner |= 1 << 29;
        }
        if per_key_repeat {
            inner |= 1 << 30;
        }
        if controls_enabled {
            inner |= 1 << 31;
        }
        Control { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const GROUPS_WRAP: Self = Self { inner: 134217728 };
    pub const INTERNAL_MODS: Self = Self { inner: 268435456 };
    pub const IGNORE_LOCK_MODS: Self = Self { inner: 536870912 };
    pub const PER_KEY_REPEAT: Self = Self { inner: 1073741824 };
    pub const CONTROLS_ENABLED: Self = Self { inner: 2147483648 };
    pub const COMPLETE: Self = Self { inner: 4160749568 };
}
impl AsByteSequence for Control {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((Control { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for Control {
    type Output = Control;
    #[inline]
    fn not(self) -> Control {
        Control { inner: !self.inner }
    }
}
impl core::ops::BitAnd for Control {
    type Output = Control;
    #[inline]
    fn bitand(self, rhs: Control) -> Control {
        Control {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for Control {
    type Output = Control;
    #[inline]
    fn bitor(self, rhs: Control) -> Control {
        Control {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for Control {
    type Output = Control;
    #[inline]
    fn bitxor(self, rhs: Control) -> Control {
        Control {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetMapRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub full: MapPart,
    pub partial: MapPart,
    pub first_type: Card8,
    pub n_types: Card8,
    pub first_key_sym: Keycode,
    pub n_key_syms: Card8,
    pub first_key_action: Keycode,
    pub n_key_actions: Card8,
    pub first_key_behavior: Keycode,
    pub n_key_behaviors: Card8,
    pub virtual_mods: VMod,
    pub first_key_explicit: Keycode,
    pub n_key_explicit: Card8,
    pub first_mod_map_key: Keycode,
    pub n_mod_map_keys: Card8,
    pub first_v_mod_map_key: Keycode,
    pub n_v_mod_map_keys: Card8,
}
impl GetMapRequest {}
impl AsByteSequence for GetMapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.full.as_bytes(&mut bytes[index..]);
        index += self.partial.as_bytes(&mut bytes[index..]);
        index += self.first_type.as_bytes(&mut bytes[index..]);
        index += self.n_types.as_bytes(&mut bytes[index..]);
        index += self.first_key_sym.as_bytes(&mut bytes[index..]);
        index += self.n_key_syms.as_bytes(&mut bytes[index..]);
        index += self.first_key_action.as_bytes(&mut bytes[index..]);
        index += self.n_key_actions.as_bytes(&mut bytes[index..]);
        index += self.first_key_behavior.as_bytes(&mut bytes[index..]);
        index += self.n_key_behaviors.as_bytes(&mut bytes[index..]);
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        index += self.first_key_explicit.as_bytes(&mut bytes[index..]);
        index += self.n_key_explicit.as_bytes(&mut bytes[index..]);
        index += self.first_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += self.first_v_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_v_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (full, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
        index += sz;
        let (partial, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_sym, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_syms, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_action, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_actions, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_behavior, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_behaviors, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_explicit, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_explicit, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_v_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_v_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetMapRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                full: full,
                partial: partial,
                first_type: first_type,
                n_types: n_types,
                first_key_sym: first_key_sym,
                n_key_syms: n_key_syms,
                first_key_action: first_key_action,
                n_key_actions: n_key_actions,
                first_key_behavior: first_key_behavior,
                n_key_behaviors: n_key_behaviors,
                virtual_mods: virtual_mods,
                first_key_explicit: first_key_explicit,
                n_key_explicit: n_key_explicit,
                first_mod_map_key: first_mod_map_key,
                n_mod_map_keys: n_mod_map_keys,
                first_v_mod_map_key: first_v_mod_map_key,
                n_v_mod_map_keys: n_v_mod_map_keys,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.full.size()
            + self.partial.size()
            + self.first_type.size()
            + self.n_types.size()
            + self.first_key_sym.size()
            + self.n_key_syms.size()
            + self.first_key_action.size()
            + self.n_key_actions.size()
            + self.first_key_behavior.size()
            + self.n_key_behaviors.size()
            + self.virtual_mods.size()
            + self.first_key_explicit.size()
            + self.n_key_explicit.size()
            + self.first_mod_map_key.size()
            + self.n_mod_map_keys.size()
            + self.first_v_mod_map_key.size()
            + self.n_v_mod_map_keys.size()
            + 2
    }
}
impl Request for GetMapRequest {
    const OPCODE: u8 = 8;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetMapReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetMapReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub present: MapPart,
    pub first_type: Card8,
    pub total_types: Card8,
    pub first_key_sym: Keycode,
    pub total_syms: Card16,
    pub first_key_action: Keycode,
    pub first_key_behavior: Keycode,
    pub n_key_behaviors: Card8,
    pub first_key_explicit: Keycode,
    pub n_key_explicit: Card8,
    pub first_mod_map_key: Keycode,
    pub n_mod_map_keys: Card8,
    pub first_v_mod_map_key: Keycode,
    pub n_v_mod_map_keys: Card8,
    pub virtual_mods: VMod,
    pub types_rtrn: Vec<KeyType>,
    pub syms_rtrn: Vec<KeySymMap>,
    pub acts_rtrn_count: Vec<Card8>,
    pub acts_rtrn_acts: Vec<Action>,
    pub behaviors_rtrn: Vec<SetBehavior>,
    pub vmods_rtrn: Vec<Card8>,
    pub explicit_rtrn: Vec<SetExplicit>,
    pub modmap_rtrn: Vec<KeyModMap>,
    pub vmodmap_rtrn: Vec<KeyVModMap>,
}
impl GetMapReply {}
impl AsByteSequence for GetMapReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.min_key_code.as_bytes(&mut bytes[index..]);
        index += self.max_key_code.as_bytes(&mut bytes[index..]);
        index += self.present.as_bytes(&mut bytes[index..]);
        index += self.first_type.as_bytes(&mut bytes[index..]);
        index += (self.types_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.total_types.as_bytes(&mut bytes[index..]);
        index += self.first_key_sym.as_bytes(&mut bytes[index..]);
        index += self.total_syms.as_bytes(&mut bytes[index..]);
        index += (self.syms_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_key_action.as_bytes(&mut bytes[index..]);
        index += (self.acts_rtrn_acts.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.acts_rtrn_count.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_key_behavior.as_bytes(&mut bytes[index..]);
        index += self.n_key_behaviors.as_bytes(&mut bytes[index..]);
        index += (self.behaviors_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_key_explicit.as_bytes(&mut bytes[index..]);
        index += self.n_key_explicit.as_bytes(&mut bytes[index..]);
        index += (self.explicit_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += (self.modmap_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_v_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_v_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += (self.vmodmap_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.types_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyType>());
        let block_len: usize = vector_as_bytes(&self.syms_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
        let block_len: usize = vector_as_bytes(&self.acts_rtrn_count, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.acts_rtrn_acts, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let block_len: usize = vector_as_bytes(&self.behaviors_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
        let block_len: usize = vector_as_bytes(&self.vmods_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let block_len: usize = vector_as_bytes(&self.explicit_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
        let block_len: usize = vector_as_bytes(&self.modmap_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
        let block_len: usize = vector_as_bytes(&self.vmodmap_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetMapReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (present, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (total_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_sym, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (total_syms, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_action, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len3, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_behavior, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_behaviors, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len4, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_explicit, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_explicit, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len5, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len6, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_v_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_v_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len7, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (types_rtrn, block_len): (Vec<KeyType>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyType>());
        let (syms_rtrn, block_len): (Vec<KeySymMap>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
        let (acts_rtrn_count, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (acts_rtrn_acts, block_len): (Vec<Action>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let (behaviors_rtrn, block_len): (Vec<SetBehavior>, usize) =
            vector_from_bytes(&bytes[index..], len4 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
        let (vmods_rtrn, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], ((virtual_mods).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let (explicit_rtrn, block_len): (Vec<SetExplicit>, usize) =
            vector_from_bytes(&bytes[index..], len5 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
        let (modmap_rtrn, block_len): (Vec<KeyModMap>, usize) =
            vector_from_bytes(&bytes[index..], len6 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
        let (vmodmap_rtrn, block_len): (Vec<KeyVModMap>, usize) =
            vector_from_bytes(&bytes[index..], len7 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
        Some((
            GetMapReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                min_key_code: min_key_code,
                max_key_code: max_key_code,
                present: present,
                first_type: first_type,
                total_types: total_types,
                first_key_sym: first_key_sym,
                total_syms: total_syms,
                first_key_action: first_key_action,
                first_key_behavior: first_key_behavior,
                n_key_behaviors: n_key_behaviors,
                first_key_explicit: first_key_explicit,
                n_key_explicit: n_key_explicit,
                first_mod_map_key: first_mod_map_key,
                n_mod_map_keys: n_mod_map_keys,
                first_v_mod_map_key: first_v_mod_map_key,
                n_v_mod_map_keys: n_v_mod_map_keys,
                virtual_mods: virtual_mods,
                types_rtrn: types_rtrn,
                syms_rtrn: syms_rtrn,
                acts_rtrn_count: acts_rtrn_count,
                acts_rtrn_acts: acts_rtrn_acts,
                behaviors_rtrn: behaviors_rtrn,
                vmods_rtrn: vmods_rtrn,
                explicit_rtrn: explicit_rtrn,
                modmap_rtrn: modmap_rtrn,
                vmodmap_rtrn: vmodmap_rtrn,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + 2
            + self.min_key_code.size()
            + self.max_key_code.size()
            + self.present.size()
            + self.first_type.size()
            + ::core::mem::size_of::<Card8>()
            + self.total_types.size()
            + self.first_key_sym.size()
            + self.total_syms.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_action.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_behavior.size()
            + self.n_key_behaviors.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_explicit.size()
            + self.n_key_explicit.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_mod_map_key.size()
            + self.n_mod_map_keys.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_v_mod_map_key.size()
            + self.n_v_mod_map_keys.size()
            + ::core::mem::size_of::<Card8>()
            + 1
            + self.virtual_mods.size()
            + {
                let block_len: usize = self.types_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyType>());
                block_len + pad
            }
            + {
                let block_len: usize = self.syms_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
                block_len + pad
            }
            + {
                let block_len: usize = self.acts_rtrn_count.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.acts_rtrn_acts.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Action>());
                block_len + pad
            }
            + {
                let block_len: usize = self.behaviors_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vmods_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.explicit_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
                block_len + pad
            }
            + {
                let block_len: usize = self.modmap_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vmodmap_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetMapRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub present: MapPart,
    pub flags: SetMapFlags,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub first_type: Card8,
    pub first_key_sym: Keycode,
    pub total_syms: Card16,
    pub first_key_action: Keycode,
    pub first_key_behavior: Keycode,
    pub n_key_behaviors: Card8,
    pub first_key_explicit: Keycode,
    pub n_key_explicit: Card8,
    pub first_mod_map_key: Keycode,
    pub n_mod_map_keys: Card8,
    pub first_v_mod_map_key: Keycode,
    pub n_v_mod_map_keys: Card8,
    pub virtual_mods: VMod,
    pub types: Vec<SetKeyType>,
    pub syms: Vec<KeySymMap>,
    pub actions_count: Vec<Card8>,
    pub actions: Vec<Action>,
    pub behaviors: Vec<SetBehavior>,
    pub vmods: Vec<Card8>,
    pub explicit: Vec<SetExplicit>,
    pub modmap: Vec<KeyModMap>,
    pub vmodmap: Vec<KeyVModMap>,
}
impl SetMapRequest {}
impl AsByteSequence for SetMapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.present.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.min_key_code.as_bytes(&mut bytes[index..]);
        index += self.max_key_code.as_bytes(&mut bytes[index..]);
        index += self.first_type.as_bytes(&mut bytes[index..]);
        index += (self.types.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_key_sym.as_bytes(&mut bytes[index..]);
        index += (self.syms.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.total_syms.as_bytes(&mut bytes[index..]);
        index += self.first_key_action.as_bytes(&mut bytes[index..]);
        index += (self.actions_count.len() as Card8).as_bytes(&mut bytes[index..]);
        index += (self.actions.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.first_key_behavior.as_bytes(&mut bytes[index..]);
        index += self.n_key_behaviors.as_bytes(&mut bytes[index..]);
        index += (self.behaviors.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_key_explicit.as_bytes(&mut bytes[index..]);
        index += self.n_key_explicit.as_bytes(&mut bytes[index..]);
        index += (self.explicit.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += (self.modmap.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_v_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_v_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += (self.vmodmap.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.types, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetKeyType>());
        let block_len: usize = vector_as_bytes(&self.syms, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
        let block_len: usize = vector_as_bytes(&self.actions_count, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.actions, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let block_len: usize = vector_as_bytes(&self.behaviors, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
        let block_len: usize = vector_as_bytes(&self.vmods, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let block_len: usize = vector_as_bytes(&self.explicit, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
        let block_len: usize = vector_as_bytes(&self.modmap, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
        let block_len: usize = vector_as_bytes(&self.vmodmap, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetMapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (present, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (SetMapFlags, usize) = <SetMapFlags>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_sym, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (total_syms, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_action, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len3, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_behavior, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_behaviors, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len4, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_explicit, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_explicit, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len5, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len6, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_v_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_v_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len7, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (types, block_len): (Vec<SetKeyType>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetKeyType>());
        let (syms, block_len): (Vec<KeySymMap>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
        let (actions_count, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (actions, block_len): (Vec<Action>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let (behaviors, block_len): (Vec<SetBehavior>, usize) =
            vector_from_bytes(&bytes[index..], len4 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
        let (vmods, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], ((virtual_mods).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let (explicit, block_len): (Vec<SetExplicit>, usize) =
            vector_from_bytes(&bytes[index..], len5 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
        let (modmap, block_len): (Vec<KeyModMap>, usize) =
            vector_from_bytes(&bytes[index..], len6 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
        let (vmodmap, block_len): (Vec<KeyVModMap>, usize) =
            vector_from_bytes(&bytes[index..], len7 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
        Some((
            SetMapRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                present: present,
                flags: flags,
                min_key_code: min_key_code,
                max_key_code: max_key_code,
                first_type: first_type,
                first_key_sym: first_key_sym,
                total_syms: total_syms,
                first_key_action: first_key_action,
                first_key_behavior: first_key_behavior,
                n_key_behaviors: n_key_behaviors,
                first_key_explicit: first_key_explicit,
                n_key_explicit: n_key_explicit,
                first_mod_map_key: first_mod_map_key,
                n_mod_map_keys: n_mod_map_keys,
                first_v_mod_map_key: first_v_mod_map_key,
                n_v_mod_map_keys: n_v_mod_map_keys,
                virtual_mods: virtual_mods,
                types: types,
                syms: syms,
                actions_count: actions_count,
                actions: actions,
                behaviors: behaviors,
                vmods: vmods,
                explicit: explicit,
                modmap: modmap,
                vmodmap: vmodmap,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.present.size()
            + self.flags.size()
            + self.min_key_code.size()
            + self.max_key_code.size()
            + self.first_type.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_sym.size()
            + ::core::mem::size_of::<Card8>()
            + self.total_syms.size()
            + self.first_key_action.size()
            + ::core::mem::size_of::<Card8>()
            + ::core::mem::size_of::<Card16>()
            + self.first_key_behavior.size()
            + self.n_key_behaviors.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_explicit.size()
            + self.n_key_explicit.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_mod_map_key.size()
            + self.n_mod_map_keys.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_v_mod_map_key.size()
            + self.n_v_mod_map_keys.size()
            + ::core::mem::size_of::<Card8>()
            + self.virtual_mods.size()
            + {
                let block_len: usize = self.types.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SetKeyType>());
                block_len + pad
            }
            + {
                let block_len: usize = self.syms.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
                block_len + pad
            }
            + {
                let block_len: usize = self.actions_count.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.actions.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Action>());
                block_len + pad
            }
            + {
                let block_len: usize = self.behaviors.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vmods.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.explicit.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
                block_len + pad
            }
            + {
                let block_len: usize = self.modmap.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vmodmap.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
                block_len + pad
            }
    }
}
impl Request for SetMapRequest {
    const OPCODE: u8 = 9;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetMapFlags {
    pub inner: u16,
}
impl SetMapFlags {
    #[inline]
    pub fn resize_types(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_resize_types(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn recompute_actions(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_recompute_actions(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(resize_types: bool, recompute_actions: bool) -> Self {
        let mut inner: u16 = 0;
        if resize_types {
            inner |= 1 << 0;
        }
        if recompute_actions {
            inner |= 1 << 1;
        }
        SetMapFlags { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const RESIZE_TYPES: Self = Self { inner: 1 };
    pub const RECOMPUTE_ACTIONS: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for SetMapFlags {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((SetMapFlags { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SetMapFlags {
    type Output = SetMapFlags;
    #[inline]
    fn not(self) -> SetMapFlags {
        SetMapFlags { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SetMapFlags {
    type Output = SetMapFlags;
    #[inline]
    fn bitand(self, rhs: SetMapFlags) -> SetMapFlags {
        SetMapFlags {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SetMapFlags {
    type Output = SetMapFlags;
    #[inline]
    fn bitor(self, rhs: SetMapFlags) -> SetMapFlags {
        SetMapFlags {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SetMapFlags {
    type Output = SetMapFlags;
    #[inline]
    fn bitxor(self, rhs: SetMapFlags) -> SetMapFlags {
        SetMapFlags {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetCompatMapRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub groups: SetOfGroup,
    pub get_all_si: bool,
    pub first_si: Card16,
    pub n_si: Card16,
}
impl GetCompatMapRequest {}
impl AsByteSequence for GetCompatMapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.groups.as_bytes(&mut bytes[index..]);
        index += self.get_all_si.as_bytes(&mut bytes[index..]);
        index += self.first_si.as_bytes(&mut bytes[index..]);
        index += self.n_si.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCompatMapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (groups, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        let (get_all_si, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetCompatMapRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                groups: groups,
                get_all_si: get_all_si,
                first_si: first_si,
                n_si: n_si,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.groups.size()
            + self.get_all_si.size()
            + self.first_si.size()
            + self.n_si.size()
    }
}
impl Request for GetCompatMapRequest {
    const OPCODE: u8 = 10;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetCompatMapReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetCompatMapReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub groups_rtrn: SetOfGroup,
    pub first_si_rtrn: Card16,
    pub n_total_si: Card16,
    pub si_rtrn: Vec<SymInterpret>,
    pub group_rtrn: Vec<ModDef>,
}
impl GetCompatMapReply {}
impl AsByteSequence for GetCompatMapReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.groups_rtrn.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.first_si_rtrn.as_bytes(&mut bytes[index..]);
        index += (self.si_rtrn.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.n_total_si.as_bytes(&mut bytes[index..]);
        index += 16;
        let block_len: usize = vector_as_bytes(&self.si_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
        let block_len: usize = vector_as_bytes(&self.group_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetCompatMapReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (groups_rtrn, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (first_si_rtrn, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_total_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let (si_rtrn, block_len): (Vec<SymInterpret>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
        let (group_rtrn, block_len): (Vec<ModDef>, usize) =
            vector_from_bytes(&bytes[index..], ((groups_rtrn).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        Some((
            GetCompatMapReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                groups_rtrn: groups_rtrn,
                first_si_rtrn: first_si_rtrn,
                n_total_si: n_total_si,
                si_rtrn: si_rtrn,
                group_rtrn: group_rtrn,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.groups_rtrn.size()
            + 1
            + self.first_si_rtrn.size()
            + ::core::mem::size_of::<Card16>()
            + self.n_total_si.size()
            + 16
            + {
                let block_len: usize = self.si_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
                block_len + pad
            }
            + {
                let block_len: usize = self.group_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetCompatMapRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub recompute_actions: bool,
    pub truncate_si: bool,
    pub groups: SetOfGroup,
    pub first_si: Card16,
    pub si: Vec<SymInterpret>,
    pub group_maps: Vec<ModDef>,
}
impl SetCompatMapRequest {}
impl AsByteSequence for SetCompatMapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.recompute_actions.as_bytes(&mut bytes[index..]);
        index += self.truncate_si.as_bytes(&mut bytes[index..]);
        index += self.groups.as_bytes(&mut bytes[index..]);
        index += self.first_si.as_bytes(&mut bytes[index..]);
        index += (self.si.len() as Card16).as_bytes(&mut bytes[index..]);
        index += 2;
        let block_len: usize = vector_as_bytes(&self.si, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
        let block_len: usize = vector_as_bytes(&self.group_maps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetCompatMapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (recompute_actions, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (truncate_si, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (groups, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (si, block_len): (Vec<SymInterpret>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
        let (group_maps, block_len): (Vec<ModDef>, usize) =
            vector_from_bytes(&bytes[index..], ((groups).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        Some((
            SetCompatMapRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                recompute_actions: recompute_actions,
                truncate_si: truncate_si,
                groups: groups,
                first_si: first_si,
                si: si,
                group_maps: group_maps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + 1
            + self.recompute_actions.size()
            + self.truncate_si.size()
            + self.groups.size()
            + self.first_si.size()
            + ::core::mem::size_of::<Card16>()
            + 2
            + {
                let block_len: usize = self.si.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
                block_len + pad
            }
            + {
                let block_len: usize = self.group_maps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
                block_len + pad
            }
    }
}
impl Request for SetCompatMapRequest {
    const OPCODE: u8 = 11;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetIndicatorStateRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
}
impl GetIndicatorStateRequest {}
impl AsByteSequence for GetIndicatorStateRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetIndicatorStateRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            GetIndicatorStateRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.device_spec.size() + self.length.size() + 2
    }
}
impl Request for GetIndicatorStateRequest {
    const OPCODE: u8 = 12;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetIndicatorStateReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetIndicatorStateReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub state: Card32,
}
impl GetIndicatorStateReply {}
impl AsByteSequence for GetIndicatorStateReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.state.as_bytes(&mut bytes[index..]);
        index += 20;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetIndicatorStateReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 20;
        Some((
            GetIndicatorStateReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                state: state,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.state.size()
            + 20
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetIndicatorMapRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub which: Card32,
}
impl GetIndicatorMapRequest {}
impl AsByteSequence for GetIndicatorMapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.which.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetIndicatorMapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (which, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetIndicatorMapRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                which: which,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.device_spec.size() + self.length.size() + 2 + self.which.size()
    }
}
impl Request for GetIndicatorMapRequest {
    const OPCODE: u8 = 13;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetIndicatorMapReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetIndicatorMapReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub which: Card32,
    pub real_indicators: Card32,
    pub n_indicators: Card8,
    pub maps: Vec<IndicatorMap>,
}
impl GetIndicatorMapReply {}
impl AsByteSequence for GetIndicatorMapReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.which.as_bytes(&mut bytes[index..]);
        index += self.real_indicators.as_bytes(&mut bytes[index..]);
        index += self.n_indicators.as_bytes(&mut bytes[index..]);
        index += 15;
        let block_len: usize = vector_as_bytes(&self.maps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetIndicatorMapReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (which, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_indicators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_indicators, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 15;
        let (maps, block_len): (Vec<IndicatorMap>, usize) =
            vector_from_bytes(&bytes[index..], ((which).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        Some((
            GetIndicatorMapReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                which: which,
                real_indicators: real_indicators,
                n_indicators: n_indicators,
                maps: maps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.which.size()
            + self.real_indicators.size()
            + self.n_indicators.size()
            + 15
            + {
                let block_len: usize = self.maps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetIndicatorMapRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub which: Card32,
    pub maps: Vec<IndicatorMap>,
}
impl SetIndicatorMapRequest {}
impl AsByteSequence for SetIndicatorMapRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.which.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.maps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetIndicatorMapRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (which, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (maps, block_len): (Vec<IndicatorMap>, usize) =
            vector_from_bytes(&bytes[index..], ((which).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        Some((
            SetIndicatorMapRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                which: which,
                maps: maps,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + 2
            + self.which.size()
            + {
                let block_len: usize = self.maps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
                block_len + pad
            }
    }
}
impl Request for SetIndicatorMapRequest {
    const OPCODE: u8 = 14;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetNamedIndicatorRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub led_class: LedClass,
    pub led_id: IdSpec,
    pub indicator: Atom,
}
impl GetNamedIndicatorRequest {}
impl AsByteSequence for GetNamedIndicatorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.led_class.as_bytes(&mut bytes[index..]);
        index += self.led_id.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.indicator.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetNamedIndicatorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_class, sz): (LedClass, usize) = <LedClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_id, sz): (IdSpec, usize) = <IdSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (indicator, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetNamedIndicatorRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                led_class: led_class,
                led_id: led_id,
                indicator: indicator,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.led_class.size()
            + self.led_id.size()
            + 2
            + self.indicator.size()
    }
}
impl Request for GetNamedIndicatorRequest {
    const OPCODE: u8 = 15;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetNamedIndicatorReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetNamedIndicatorReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub indicator: Atom,
    pub found: bool,
    pub on: bool,
    pub real_indicator: bool,
    pub ndx: Card8,
    pub map_flags: ImFlag,
    pub map_which_groups: ImGroupsWhich,
    pub map_groups: SetOfGroups,
    pub map_which_mods: ImModsWhich,
    pub map_mods: ModMask,
    pub map_real_mods: ModMask,
    pub map_vmod: VMod,
    pub map_ctrls: BoolCtrl,
    pub supported: bool,
}
impl GetNamedIndicatorReply {}
impl AsByteSequence for GetNamedIndicatorReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.indicator.as_bytes(&mut bytes[index..]);
        index += self.found.as_bytes(&mut bytes[index..]);
        index += self.on.as_bytes(&mut bytes[index..]);
        index += self.real_indicator.as_bytes(&mut bytes[index..]);
        index += self.ndx.as_bytes(&mut bytes[index..]);
        index += self.map_flags.as_bytes(&mut bytes[index..]);
        index += self.map_which_groups.as_bytes(&mut bytes[index..]);
        index += self.map_groups.as_bytes(&mut bytes[index..]);
        index += self.map_which_mods.as_bytes(&mut bytes[index..]);
        index += self.map_mods.as_bytes(&mut bytes[index..]);
        index += self.map_real_mods.as_bytes(&mut bytes[index..]);
        index += self.map_vmod.as_bytes(&mut bytes[index..]);
        index += self.map_ctrls.as_bytes(&mut bytes[index..]);
        index += self.supported.as_bytes(&mut bytes[index..]);
        index += 3;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetNamedIndicatorReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (indicator, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (found, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (on, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (real_indicator, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ndx, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_flags, sz): (ImFlag, usize) = <ImFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_which_groups, sz): (ImGroupsWhich, usize) =
            <ImGroupsWhich>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_groups, sz): (SetOfGroups, usize) = <SetOfGroups>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_which_mods, sz): (ImModsWhich, usize) =
            <ImModsWhich>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_vmod, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_ctrls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (supported, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        Some((
            GetNamedIndicatorReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                indicator: indicator,
                found: found,
                on: on,
                real_indicator: real_indicator,
                ndx: ndx,
                map_flags: map_flags,
                map_which_groups: map_which_groups,
                map_groups: map_groups,
                map_which_mods: map_which_mods,
                map_mods: map_mods,
                map_real_mods: map_real_mods,
                map_vmod: map_vmod,
                map_ctrls: map_ctrls,
                supported: supported,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.indicator.size()
            + self.found.size()
            + self.on.size()
            + self.real_indicator.size()
            + self.ndx.size()
            + self.map_flags.size()
            + self.map_which_groups.size()
            + self.map_groups.size()
            + self.map_which_mods.size()
            + self.map_mods.size()
            + self.map_real_mods.size()
            + self.map_vmod.size()
            + self.map_ctrls.size()
            + self.supported.size()
            + 3
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SetOfGroups {
    pub inner: u8,
}
impl SetOfGroups {
    #[inline]
    pub fn any(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_any(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(any: bool) -> Self {
        let mut inner: u8 = 0;
        if any {
            inner |= 1 << 7;
        }
        SetOfGroups { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const ANY: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 128 };
}
impl AsByteSequence for SetOfGroups {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        Some((SetOfGroups { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SetOfGroups {
    type Output = SetOfGroups;
    #[inline]
    fn not(self) -> SetOfGroups {
        SetOfGroups { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SetOfGroups {
    type Output = SetOfGroups;
    #[inline]
    fn bitand(self, rhs: SetOfGroups) -> SetOfGroups {
        SetOfGroups {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SetOfGroups {
    type Output = SetOfGroups;
    #[inline]
    fn bitor(self, rhs: SetOfGroups) -> SetOfGroups {
        SetOfGroups {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SetOfGroups {
    type Output = SetOfGroups;
    #[inline]
    fn bitxor(self, rhs: SetOfGroups) -> SetOfGroups {
        SetOfGroups {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetNamedIndicatorRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub led_class: LedClass,
    pub led_id: IdSpec,
    pub indicator: Atom,
    pub set_state: bool,
    pub on: bool,
    pub set_map: bool,
    pub create_map: bool,
    pub map_flags: ImFlag,
    pub map_which_groups: ImGroupsWhich,
    pub map_groups: SetOfGroups,
    pub map_which_mods: ImModsWhich,
    pub map_real_mods: ModMask,
    pub map_vmods: VMod,
    pub map_ctrls: BoolCtrl,
}
impl SetNamedIndicatorRequest {}
impl AsByteSequence for SetNamedIndicatorRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.led_class.as_bytes(&mut bytes[index..]);
        index += self.led_id.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.indicator.as_bytes(&mut bytes[index..]);
        index += self.set_state.as_bytes(&mut bytes[index..]);
        index += self.on.as_bytes(&mut bytes[index..]);
        index += self.set_map.as_bytes(&mut bytes[index..]);
        index += self.create_map.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.map_flags.as_bytes(&mut bytes[index..]);
        index += self.map_which_groups.as_bytes(&mut bytes[index..]);
        index += self.map_groups.as_bytes(&mut bytes[index..]);
        index += self.map_which_mods.as_bytes(&mut bytes[index..]);
        index += self.map_real_mods.as_bytes(&mut bytes[index..]);
        index += self.map_vmods.as_bytes(&mut bytes[index..]);
        index += self.map_ctrls.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetNamedIndicatorRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_class, sz): (LedClass, usize) = <LedClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_id, sz): (IdSpec, usize) = <IdSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (indicator, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (set_state, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (on, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (set_map, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (create_map, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (map_flags, sz): (ImFlag, usize) = <ImFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_which_groups, sz): (ImGroupsWhich, usize) =
            <ImGroupsWhich>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_groups, sz): (SetOfGroups, usize) = <SetOfGroups>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_which_mods, sz): (ImModsWhich, usize) =
            <ImModsWhich>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_real_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_vmods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_ctrls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            SetNamedIndicatorRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                led_class: led_class,
                led_id: led_id,
                indicator: indicator,
                set_state: set_state,
                on: on,
                set_map: set_map,
                create_map: create_map,
                map_flags: map_flags,
                map_which_groups: map_which_groups,
                map_groups: map_groups,
                map_which_mods: map_which_mods,
                map_real_mods: map_real_mods,
                map_vmods: map_vmods,
                map_ctrls: map_ctrls,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.led_class.size()
            + self.led_id.size()
            + 2
            + self.indicator.size()
            + self.set_state.size()
            + self.on.size()
            + self.set_map.size()
            + self.create_map.size()
            + 1
            + self.map_flags.size()
            + self.map_which_groups.size()
            + self.map_groups.size()
            + self.map_which_mods.size()
            + self.map_real_mods.size()
            + self.map_vmods.size()
            + self.map_ctrls.size()
    }
}
impl Request for SetNamedIndicatorRequest {
    const OPCODE: u8 = 16;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct GetNamesRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub which: NameDetail,
}
impl GetNamesRequest {}
impl AsByteSequence for GetNamesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.which.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetNamesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (which, sz): (NameDetail, usize) = <NameDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetNamesRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                which: which,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.device_spec.size() + self.length.size() + 2 + self.which.size()
    }
}
impl Request for GetNamesRequest {
    const OPCODE: u8 = 17;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetNamesReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetNamesReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub which: NameDetail,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub n_types: Card8,
    pub group_names: SetOfGroup,
    pub virtual_mods: VMod,
    pub first_key: Keycode,
    pub indicators: Card32,
    pub n_kt_levels: Card16,
    pub keycodes_name: Atom,
    pub geometry_name: Atom,
    pub symbols_name: Atom,
    pub phys_symbols_name: Atom,
    pub types_name: Atom,
    pub compat_name: Atom,
    pub type_names: Vec<Atom>,
    pub n_levels_per_type: Vec<Card8>,
    pub kt_level_names: Vec<Atom>,
    pub indicator_names: Vec<Atom>,
    pub virtual_mod_names: Vec<Atom>,
    pub groups: Vec<Atom>,
    pub key_names: Vec<KeyName>,
    pub key_aliases: Vec<KeyAlias>,
    pub radio_group_names: Vec<Atom>,
}
impl GetNamesReply {}
impl AsByteSequence for GetNamesReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.which.as_bytes(&mut bytes[index..]);
        index += self.min_key_code.as_bytes(&mut bytes[index..]);
        index += self.max_key_code.as_bytes(&mut bytes[index..]);
        index += self.n_types.as_bytes(&mut bytes[index..]);
        index += self.group_names.as_bytes(&mut bytes[index..]);
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        index += self.first_key.as_bytes(&mut bytes[index..]);
        index += (self.key_names.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.indicators.as_bytes(&mut bytes[index..]);
        index += (self.radio_group_names.len() as Card8).as_bytes(&mut bytes[index..]);
        index += (self.key_aliases.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.n_kt_levels.as_bytes(&mut bytes[index..]);
        index += 4;
        let cond0 = (self.which);
        if cond0.keycodes() {
            index += self.keycodes_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.geometry_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.symbols() {
            index += self.symbols_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.phys_symbols() {
            index += self.phys_symbols_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.types() {
            index += self.types_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat() {
            index += self.compat_name.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.type_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.n_levels_per_type, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let block_len: usize = vector_as_bytes(&self.kt_level_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.indicator_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.virtual_mod_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.groups, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.key_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
        let block_len: usize = vector_as_bytes(&self.key_aliases, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
        let block_len: usize = vector_as_bytes(&self.radio_group_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetNamesReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (which, sz): (NameDetail, usize) = <NameDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group_names, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (indicators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_kt_levels, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        let cond0 = (which);
        let keycodes_name: Atom = if cond0.keycodes() {
            let (keycodes_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            keycodes_name
        } else {
            Default::default()
        };
        let geometry_name: Atom = if cond0.geometry() {
            let (geometry_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_name
        } else {
            Default::default()
        };
        let symbols_name: Atom = if cond0.symbols() {
            let (symbols_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            symbols_name
        } else {
            Default::default()
        };
        let phys_symbols_name: Atom = if cond0.phys_symbols() {
            let (phys_symbols_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            phys_symbols_name
        } else {
            Default::default()
        };
        let types_name: Atom = if cond0.types() {
            let (types_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            types_name
        } else {
            Default::default()
        };
        let compat_name: Atom = if cond0.compat() {
            let (compat_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            compat_name
        } else {
            Default::default()
        };
        let (type_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], (n_types as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (n_levels_per_type, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], (n_types as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let (kt_level_names, block_len): (Vec<Atom>, usize) = vector_from_bytes(
            &bytes[index..],
            (n_levels_per_type
                .iter()
                .map(|a| {
                    (TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize")) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (indicator_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((indicators).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (virtual_mod_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((virtual_mods).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (groups, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((group_names).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (key_names, block_len): (Vec<KeyName>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
        let (key_aliases, block_len): (Vec<KeyAlias>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
        let (radio_group_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            GetNamesReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                which: which,
                min_key_code: min_key_code,
                max_key_code: max_key_code,
                n_types: n_types,
                group_names: group_names,
                virtual_mods: virtual_mods,
                first_key: first_key,
                indicators: indicators,
                n_kt_levels: n_kt_levels,
                keycodes_name: keycodes_name,
                geometry_name: geometry_name,
                symbols_name: symbols_name,
                phys_symbols_name: phys_symbols_name,
                types_name: types_name,
                compat_name: compat_name,
                type_names: type_names,
                n_levels_per_type: n_levels_per_type,
                kt_level_names: kt_level_names,
                indicator_names: indicator_names,
                virtual_mod_names: virtual_mod_names,
                groups: groups,
                key_names: key_names,
                key_aliases: key_aliases,
                radio_group_names: radio_group_names,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.which.size()
            + self.min_key_code.size()
            + self.max_key_code.size()
            + self.n_types.size()
            + self.group_names.size()
            + self.virtual_mods.size()
            + self.first_key.size()
            + ::core::mem::size_of::<Card8>()
            + self.indicators.size()
            + ::core::mem::size_of::<Card8>()
            + ::core::mem::size_of::<Card8>()
            + self.n_kt_levels.size()
            + 4
            + self.keycodes_name.size()
            + self.geometry_name.size()
            + self.symbols_name.size()
            + self.phys_symbols_name.size()
            + self.types_name.size()
            + self.compat_name.size()
            + {
                let block_len: usize = self.type_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.n_levels_per_type.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.kt_level_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.indicator_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.virtual_mod_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.groups.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.key_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
                block_len + pad
            }
            + {
                let block_len: usize = self.key_aliases.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
                block_len + pad
            }
            + {
                let block_len: usize = self.radio_group_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct NameDetail {
    pub inner: u32,
}
impl NameDetail {
    #[inline]
    pub fn keycodes(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_keycodes(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn geometry(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_geometry(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn symbols(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_symbols(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn phys_symbols(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_phys_symbols(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn types(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_types(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn compat(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_compat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn key_type_names(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_key_type_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn kt_level_names(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_kt_level_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn indicator_names(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_indicator_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn key_names(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_key_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn key_aliases(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_key_aliases(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn virtual_mod_names(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_virtual_mod_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn group_names(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_group_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn rg_names(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_rg_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn new(
        keycodes: bool,
        geometry: bool,
        symbols: bool,
        phys_symbols: bool,
        types: bool,
        compat: bool,
        key_type_names: bool,
        kt_level_names: bool,
        indicator_names: bool,
        key_names: bool,
        key_aliases: bool,
        virtual_mod_names: bool,
        group_names: bool,
        rg_names: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if keycodes {
            inner |= 1 << 0;
        }
        if geometry {
            inner |= 1 << 1;
        }
        if symbols {
            inner |= 1 << 2;
        }
        if phys_symbols {
            inner |= 1 << 3;
        }
        if types {
            inner |= 1 << 4;
        }
        if compat {
            inner |= 1 << 5;
        }
        if key_type_names {
            inner |= 1 << 6;
        }
        if kt_level_names {
            inner |= 1 << 7;
        }
        if indicator_names {
            inner |= 1 << 8;
        }
        if key_names {
            inner |= 1 << 9;
        }
        if key_aliases {
            inner |= 1 << 10;
        }
        if virtual_mod_names {
            inner |= 1 << 11;
        }
        if group_names {
            inner |= 1 << 12;
        }
        if rg_names {
            inner |= 1 << 13;
        }
        NameDetail { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const KEYCODES: Self = Self { inner: 1 };
    pub const GEOMETRY: Self = Self { inner: 2 };
    pub const SYMBOLS: Self = Self { inner: 4 };
    pub const PHYS_SYMBOLS: Self = Self { inner: 8 };
    pub const TYPES: Self = Self { inner: 16 };
    pub const COMPAT: Self = Self { inner: 32 };
    pub const KEY_TYPE_NAMES: Self = Self { inner: 64 };
    pub const KT_LEVEL_NAMES: Self = Self { inner: 128 };
    pub const INDICATOR_NAMES: Self = Self { inner: 256 };
    pub const KEY_NAMES: Self = Self { inner: 512 };
    pub const KEY_ALIASES: Self = Self { inner: 1024 };
    pub const VIRTUAL_MOD_NAMES: Self = Self { inner: 2048 };
    pub const GROUP_NAMES: Self = Self { inner: 4096 };
    pub const RG_NAMES: Self = Self { inner: 8192 };
    pub const COMPLETE: Self = Self { inner: 16383 };
}
impl AsByteSequence for NameDetail {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((NameDetail { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for NameDetail {
    type Output = NameDetail;
    #[inline]
    fn not(self) -> NameDetail {
        NameDetail { inner: !self.inner }
    }
}
impl core::ops::BitAnd for NameDetail {
    type Output = NameDetail;
    #[inline]
    fn bitand(self, rhs: NameDetail) -> NameDetail {
        NameDetail {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for NameDetail {
    type Output = NameDetail;
    #[inline]
    fn bitor(self, rhs: NameDetail) -> NameDetail {
        NameDetail {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for NameDetail {
    type Output = NameDetail;
    #[inline]
    fn bitxor(self, rhs: NameDetail) -> NameDetail {
        NameDetail {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetNamesRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub virtual_mods: VMod,
    pub which: NameDetail,
    pub first_type: Card8,
    pub n_types: Card8,
    pub first_kt_levelt: Card8,
    pub n_kt_levels: Card8,
    pub indicators: Card32,
    pub group_names: SetOfGroup,
    pub first_key: Keycode,
    pub total_kt_level_names: Card16,
    pub keycodes_name: Atom,
    pub geometry_name: Atom,
    pub symbols_name: Atom,
    pub phys_symbols_name: Atom,
    pub types_name: Atom,
    pub compat_name: Atom,
    pub type_names: Vec<Atom>,
    pub n_levels_per_type: Vec<Card8>,
    pub kt_level_names: Vec<Atom>,
    pub indicator_names: Vec<Atom>,
    pub virtual_mod_names: Vec<Atom>,
    pub groups: Vec<Atom>,
    pub key_names: Vec<KeyName>,
    pub key_aliases: Vec<KeyAlias>,
    pub radio_group_names: Vec<Atom>,
}
impl SetNamesRequest {}
impl AsByteSequence for SetNamesRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        index += self.which.as_bytes(&mut bytes[index..]);
        index += self.first_type.as_bytes(&mut bytes[index..]);
        index += self.n_types.as_bytes(&mut bytes[index..]);
        index += self.first_kt_levelt.as_bytes(&mut bytes[index..]);
        index += self.n_kt_levels.as_bytes(&mut bytes[index..]);
        index += self.indicators.as_bytes(&mut bytes[index..]);
        index += self.group_names.as_bytes(&mut bytes[index..]);
        index += (self.radio_group_names.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.first_key.as_bytes(&mut bytes[index..]);
        index += (self.key_names.len() as Card8).as_bytes(&mut bytes[index..]);
        index += (self.key_aliases.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.total_kt_level_names.as_bytes(&mut bytes[index..]);
        let cond0 = (self.which);
        if cond0.keycodes() {
            index += self.keycodes_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.geometry_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.symbols() {
            index += self.symbols_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.phys_symbols() {
            index += self.phys_symbols_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.types() {
            index += self.types_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat() {
            index += self.compat_name.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.type_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.n_levels_per_type, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.kt_level_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.indicator_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.virtual_mod_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.groups, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.key_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
        let block_len: usize = vector_as_bytes(&self.key_aliases, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
        let block_len: usize = vector_as_bytes(&self.radio_group_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetNamesRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (which, sz): (NameDetail, usize) = <NameDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_kt_levelt, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_kt_levels, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (indicators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group_names, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (total_kt_level_names, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let cond0 = (which);
        let keycodes_name: Atom = if cond0.keycodes() {
            let (keycodes_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            keycodes_name
        } else {
            Default::default()
        };
        let geometry_name: Atom = if cond0.geometry() {
            let (geometry_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_name
        } else {
            Default::default()
        };
        let symbols_name: Atom = if cond0.symbols() {
            let (symbols_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            symbols_name
        } else {
            Default::default()
        };
        let phys_symbols_name: Atom = if cond0.phys_symbols() {
            let (phys_symbols_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            phys_symbols_name
        } else {
            Default::default()
        };
        let types_name: Atom = if cond0.types() {
            let (types_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            types_name
        } else {
            Default::default()
        };
        let compat_name: Atom = if cond0.compat() {
            let (compat_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            compat_name
        } else {
            Default::default()
        };
        let (type_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], (n_types as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (n_levels_per_type, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], (n_types as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (kt_level_names, block_len): (Vec<Atom>, usize) = vector_from_bytes(
            &bytes[index..],
            (n_levels_per_type
                .iter()
                .map(|a| {
                    (TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize")) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (indicator_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((indicators).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (virtual_mod_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((virtual_mods).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (groups, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((group_names).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (key_names, block_len): (Vec<KeyName>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
        let (key_aliases, block_len): (Vec<KeyAlias>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
        let (radio_group_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        Some((
            SetNamesRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                virtual_mods: virtual_mods,
                which: which,
                first_type: first_type,
                n_types: n_types,
                first_kt_levelt: first_kt_levelt,
                n_kt_levels: n_kt_levels,
                indicators: indicators,
                group_names: group_names,
                first_key: first_key,
                total_kt_level_names: total_kt_level_names,
                keycodes_name: keycodes_name,
                geometry_name: geometry_name,
                symbols_name: symbols_name,
                phys_symbols_name: phys_symbols_name,
                types_name: types_name,
                compat_name: compat_name,
                type_names: type_names,
                n_levels_per_type: n_levels_per_type,
                kt_level_names: kt_level_names,
                indicator_names: indicator_names,
                virtual_mod_names: virtual_mod_names,
                groups: groups,
                key_names: key_names,
                key_aliases: key_aliases,
                radio_group_names: radio_group_names,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.virtual_mods.size()
            + self.which.size()
            + self.first_type.size()
            + self.n_types.size()
            + self.first_kt_levelt.size()
            + self.n_kt_levels.size()
            + self.indicators.size()
            + self.group_names.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_key.size()
            + ::core::mem::size_of::<Card8>()
            + ::core::mem::size_of::<Card8>()
            + 1
            + self.total_kt_level_names.size()
            + self.keycodes_name.size()
            + self.geometry_name.size()
            + self.symbols_name.size()
            + self.phys_symbols_name.size()
            + self.types_name.size()
            + self.compat_name.size()
            + {
                let block_len: usize = self.type_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.n_levels_per_type.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.kt_level_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.indicator_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.virtual_mod_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.groups.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.key_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
                block_len + pad
            }
            + {
                let block_len: usize = self.key_aliases.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
                block_len + pad
            }
            + {
                let block_len: usize = self.radio_group_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
    }
}
impl Request for SetNamesRequest {
    const OPCODE: u8 = 18;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct PerClientFlagsRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub change: PerClientFlag,
    pub value: PerClientFlag,
    pub ctrls_to_change: BoolCtrl,
    pub auto_ctrls: BoolCtrl,
    pub auto_ctrls_values: BoolCtrl,
}
impl PerClientFlagsRequest {}
impl AsByteSequence for PerClientFlagsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.change.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index += self.ctrls_to_change.as_bytes(&mut bytes[index..]);
        index += self.auto_ctrls.as_bytes(&mut bytes[index..]);
        index += self.auto_ctrls_values.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PerClientFlagsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (change, sz): (PerClientFlag, usize) = <PerClientFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (PerClientFlag, usize) = <PerClientFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ctrls_to_change, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (auto_ctrls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (auto_ctrls_values, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            PerClientFlagsRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                change: change,
                value: value,
                ctrls_to_change: ctrls_to_change,
                auto_ctrls: auto_ctrls,
                auto_ctrls_values: auto_ctrls_values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + 2
            + self.change.size()
            + self.value.size()
            + self.ctrls_to_change.size()
            + self.auto_ctrls.size()
            + self.auto_ctrls_values.size()
    }
}
impl Request for PerClientFlagsRequest {
    const OPCODE: u8 = 21;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = PerClientFlagsReply;
}
#[derive(Clone, Debug, Default)]
pub struct PerClientFlagsReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub supported: PerClientFlag,
    pub value: PerClientFlag,
    pub auto_ctrls: BoolCtrl,
    pub auto_ctrls_values: BoolCtrl,
}
impl PerClientFlagsReply {}
impl AsByteSequence for PerClientFlagsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.supported.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index += self.auto_ctrls.as_bytes(&mut bytes[index..]);
        index += self.auto_ctrls_values.as_bytes(&mut bytes[index..]);
        index += 8;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing PerClientFlagsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (supported, sz): (PerClientFlag, usize) = <PerClientFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (value, sz): (PerClientFlag, usize) = <PerClientFlag>::from_bytes(&bytes[index..])?;
        index += sz;
        let (auto_ctrls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (auto_ctrls_values, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        Some((
            PerClientFlagsReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                supported: supported,
                value: value,
                auto_ctrls: auto_ctrls,
                auto_ctrls_values: auto_ctrls_values,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.supported.size()
            + self.value.size()
            + self.auto_ctrls.size()
            + self.auto_ctrls_values.size()
            + 8
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct PerClientFlag {
    pub inner: u32,
}
impl PerClientFlag {
    #[inline]
    pub fn detectable_auto_repeat(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_detectable_auto_repeat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn grabs_use_xkb_state(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_grabs_use_xkb_state(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn auto_reset_controls(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_auto_reset_controls(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn lookup_state_when_grabbed(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_lookup_state_when_grabbed(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn send_event_uses_xkb_state(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_send_event_uses_xkb_state(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn new(
        detectable_auto_repeat: bool,
        grabs_use_xkb_state: bool,
        auto_reset_controls: bool,
        lookup_state_when_grabbed: bool,
        send_event_uses_xkb_state: bool,
    ) -> Self {
        let mut inner: u32 = 0;
        if detectable_auto_repeat {
            inner |= 1 << 0;
        }
        if grabs_use_xkb_state {
            inner |= 1 << 1;
        }
        if auto_reset_controls {
            inner |= 1 << 2;
        }
        if lookup_state_when_grabbed {
            inner |= 1 << 3;
        }
        if send_event_uses_xkb_state {
            inner |= 1 << 4;
        }
        PerClientFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const DETECTABLE_AUTO_REPEAT: Self = Self { inner: 1 };
    pub const GRABS_USE_XKB_STATE: Self = Self { inner: 2 };
    pub const AUTO_RESET_CONTROLS: Self = Self { inner: 4 };
    pub const LOOKUP_STATE_WHEN_GRABBED: Self = Self { inner: 8 };
    pub const SEND_EVENT_USES_XKB_STATE: Self = Self { inner: 16 };
    pub const COMPLETE: Self = Self { inner: 31 };
}
impl AsByteSequence for PerClientFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u32, usize) = <u32>::from_bytes(bytes)?;
        Some((PerClientFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for PerClientFlag {
    type Output = PerClientFlag;
    #[inline]
    fn not(self) -> PerClientFlag {
        PerClientFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for PerClientFlag {
    type Output = PerClientFlag;
    #[inline]
    fn bitand(self, rhs: PerClientFlag) -> PerClientFlag {
        PerClientFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for PerClientFlag {
    type Output = PerClientFlag;
    #[inline]
    fn bitor(self, rhs: PerClientFlag) -> PerClientFlag {
        PerClientFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for PerClientFlag {
    type Output = PerClientFlag;
    #[inline]
    fn bitxor(self, rhs: PerClientFlag) -> PerClientFlag {
        PerClientFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct ListComponentsRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub max_names: Card16,
}
impl ListComponentsRequest {}
impl AsByteSequence for ListComponentsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.max_names.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListComponentsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_names, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            ListComponentsRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                max_names: max_names,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size() + self.device_spec.size() + self.length.size() + self.max_names.size()
    }
}
impl Request for ListComponentsRequest {
    const OPCODE: u8 = 22;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ListComponentsReply;
}
#[derive(Clone, Debug, Default)]
pub struct ListComponentsReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub extra: Card16,
    pub keymaps: Vec<Listing>,
    pub keycodes: Vec<Listing>,
    pub types: Vec<Listing>,
    pub compat_maps: Vec<Listing>,
    pub symbols: Vec<Listing>,
    pub geometries: Vec<Listing>,
}
impl ListComponentsReply {}
impl AsByteSequence for ListComponentsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += (self.keymaps.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.keycodes.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.types.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.compat_maps.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.symbols.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.geometries.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.extra.as_bytes(&mut bytes[index..]);
        index += 10;
        let block_len: usize = vector_as_bytes(&self.keymaps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let block_len: usize = vector_as_bytes(&self.keycodes, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let block_len: usize = vector_as_bytes(&self.types, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let block_len: usize = vector_as_bytes(&self.compat_maps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let block_len: usize = vector_as_bytes(&self.symbols, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let block_len: usize = vector_as_bytes(&self.geometries, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ListComponentsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len3, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len4, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len5, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (extra, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 10;
        let (keymaps, block_len): (Vec<Listing>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let (keycodes, block_len): (Vec<Listing>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let (types, block_len): (Vec<Listing>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let (compat_maps, block_len): (Vec<Listing>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let (symbols, block_len): (Vec<Listing>, usize) =
            vector_from_bytes(&bytes[index..], len4 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        let (geometries, block_len): (Vec<Listing>, usize) =
            vector_from_bytes(&bytes[index..], len5 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Listing>());
        Some((
            ListComponentsReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                extra: extra,
                keymaps: keymaps,
                keycodes: keycodes,
                types: types,
                compat_maps: compat_maps,
                symbols: symbols,
                geometries: geometries,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card16>()
            + self.extra.size()
            + 10
            + {
                let block_len: usize = self.keymaps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Listing>());
                block_len + pad
            }
            + {
                let block_len: usize = self.keycodes.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Listing>());
                block_len + pad
            }
            + {
                let block_len: usize = self.types.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Listing>());
                block_len + pad
            }
            + {
                let block_len: usize = self.compat_maps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Listing>());
                block_len + pad
            }
            + {
                let block_len: usize = self.symbols.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Listing>());
                block_len + pad
            }
            + {
                let block_len: usize = self.geometries.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Listing>());
                block_len + pad
            }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetKbdByNameRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub need: GbnDetail,
    pub want: GbnDetail,
    pub load: bool,
}
impl GetKbdByNameRequest {}
impl AsByteSequence for GetKbdByNameRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.need.as_bytes(&mut bytes[index..]);
        index += self.want.as_bytes(&mut bytes[index..]);
        index += self.load.as_bytes(&mut bytes[index..]);
        index += 1;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetKbdByNameRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (need, sz): (GbnDetail, usize) = <GbnDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (want, sz): (GbnDetail, usize) = <GbnDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (load, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        Some((
            GetKbdByNameRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                need: need,
                want: want,
                load: load,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.need.size()
            + self.want.size()
            + self.load.size()
            + 1
    }
}
impl Request for GetKbdByNameRequest {
    const OPCODE: u8 = 23;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetKbdByNameReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetKbdByNameReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub loaded: bool,
    pub new_keyboard: bool,
    pub found: GbnDetail,
    pub reported: GbnDetail,
    pub getmap_type: Card8,
    pub type_device_id: Card8,
    pub getmap_sequence: Card16,
    pub getmap_length: Card32,
    pub type_min_key_code: Keycode,
    pub type_max_key_code: Keycode,
    pub present: MapPart,
    pub first_type: Card8,
    pub n_types: Card8,
    pub total_types: Card8,
    pub first_key_sym: Keycode,
    pub total_syms: Card16,
    pub first_key_action: Keycode,
    pub first_key_behavior: Keycode,
    pub n_key_behaviors: Card8,
    pub first_key_explicit: Keycode,
    pub n_key_explicit: Card8,
    pub first_mod_map_key: Keycode,
    pub n_mod_map_keys: Card8,
    pub first_v_mod_map_key: Keycode,
    pub n_v_mod_map_keys: Card8,
    pub virtual_mods: VMod,
    pub types_rtrn: Vec<KeyType>,
    pub syms_rtrn: Vec<KeySymMap>,
    pub acts_rtrn_count: Vec<Card8>,
    pub acts_rtrn_acts: Vec<Action>,
    pub behaviors_rtrn: Vec<SetBehavior>,
    pub vmods_rtrn: Vec<Card8>,
    pub explicit_rtrn: Vec<SetExplicit>,
    pub modmap_rtrn: Vec<KeyModMap>,
    pub vmodmap_rtrn: Vec<KeyVModMap>,
    pub compatmap_type: Card8,
    pub compat_device_id: Card8,
    pub compatmap_sequence: Card16,
    pub compatmap_length: Card32,
    pub groups_rtrn: SetOfGroup,
    pub first_si_rtrn: Card16,
    pub n_total_si: Card16,
    pub si_rtrn: Vec<SymInterpret>,
    pub group_rtrn: Vec<ModDef>,
    pub indicatormap_type: Card8,
    pub indicator_device_id: Card8,
    pub indicatormap_sequence: Card16,
    pub indicatormap_length: Card32,
    pub which: Card32,
    pub real_indicators: Card32,
    pub maps: Vec<IndicatorMap>,
    pub keyname_type: Card8,
    pub key_device_id: Card8,
    pub keyname_sequence: Card16,
    pub keyname_length: Card32,
    pub which_: NameDetail,
    pub key_min_key_code: Keycode,
    pub key_max_key_code: Keycode,
    pub n_types_: Card8,
    pub group_names: SetOfGroup,
    pub virtual_mods_: VMod,
    pub first_key: Keycode,
    pub indicators: Card32,
    pub n_kt_levels: Card16,
    pub keycodes_name: Atom,
    pub geometry_name: Atom,
    pub symbols_name: Atom,
    pub phys_symbols_name: Atom,
    pub types_name: Atom,
    pub compat_name: Atom,
    pub type_names: Vec<Atom>,
    pub n_levels_per_type: Vec<Card8>,
    pub kt_level_names: Vec<Atom>,
    pub indicator_names: Vec<Atom>,
    pub virtual_mod_names: Vec<Atom>,
    pub groups: Vec<Atom>,
    pub key_names: Vec<KeyName>,
    pub key_aliases: Vec<KeyAlias>,
    pub radio_group_names: Vec<Atom>,
    pub geometry_type: Card8,
    pub geometry_device_id: Card8,
    pub geometry_sequence: Card16,
    pub geometry_length: Card32,
    pub name: Atom,
    pub geometry_found: bool,
    pub width_mm: Card16,
    pub height_mm: Card16,
    pub n_properties: Card16,
    pub n_colors: Card16,
    pub n_shapes: Card16,
    pub n_sections: Card16,
    pub n_doodads: Card16,
    pub n_key_aliases: Card16,
    pub base_color_ndx: Card8,
    pub label_color_ndx: Card8,
    pub label_font: CountedString16,
}
impl GetKbdByNameReply {}
impl AsByteSequence for GetKbdByNameReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.min_key_code.as_bytes(&mut bytes[index..]);
        index += self.max_key_code.as_bytes(&mut bytes[index..]);
        index += self.loaded.as_bytes(&mut bytes[index..]);
        index += self.new_keyboard.as_bytes(&mut bytes[index..]);
        index += self.found.as_bytes(&mut bytes[index..]);
        index += self.reported.as_bytes(&mut bytes[index..]);
        index += 16;
        let cond0 = (self.reported);
        if cond0.server_symbols() {
            index += self.getmap_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.type_device_id.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.getmap_sequence.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.getmap_length.as_bytes(&mut bytes[index..]);
        }
        index += 2;
        if cond0.server_symbols() {
            index += self.type_min_key_code.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.type_max_key_code.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.present.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.first_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.n_types.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.total_types.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.first_key_sym.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.total_syms.as_bytes(&mut bytes[index..]);
        }
        index += (self.syms_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0.server_symbols() {
            index += self.first_key_action.as_bytes(&mut bytes[index..]);
        }
        index += (self.acts_rtrn_acts.len() as Card16).as_bytes(&mut bytes[index..]);
        index += (self.acts_rtrn_count.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0.server_symbols() {
            index += self.first_key_behavior.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.n_key_behaviors.as_bytes(&mut bytes[index..]);
        }
        index += (self.behaviors_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0.server_symbols() {
            index += self.first_key_explicit.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.n_key_explicit.as_bytes(&mut bytes[index..]);
        }
        index += (self.explicit_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0.server_symbols() {
            index += self.first_mod_map_key.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.n_mod_map_keys.as_bytes(&mut bytes[index..]);
        }
        index += (self.modmap_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0.server_symbols() {
            index += self.first_v_mod_map_key.as_bytes(&mut bytes[index..]);
        }
        if cond0.server_symbols() {
            index += self.n_v_mod_map_keys.as_bytes(&mut bytes[index..]);
        }
        index += (self.vmodmap_rtrn.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 1;
        if cond0.server_symbols() {
            index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.types_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyType>());
        let block_len: usize = vector_as_bytes(&self.syms_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
        let block_len: usize = vector_as_bytes(&self.acts_rtrn_count, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let block_len: usize = vector_as_bytes(&self.acts_rtrn_acts, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let block_len: usize = vector_as_bytes(&self.behaviors_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
        let block_len: usize = vector_as_bytes(&self.vmods_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let block_len: usize = vector_as_bytes(&self.explicit_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
        let block_len: usize = vector_as_bytes(&self.modmap_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
        let block_len: usize = vector_as_bytes(&self.vmodmap_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
        if cond0.compat_map() {
            index += self.compatmap_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat_map() {
            index += self.compat_device_id.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat_map() {
            index += self.compatmap_sequence.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat_map() {
            index += self.compatmap_length.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat_map() {
            index += self.groups_rtrn.as_bytes(&mut bytes[index..]);
        }
        if cond0.compat_map() {
            index += self.first_si_rtrn.as_bytes(&mut bytes[index..]);
        }
        index += (self.si_rtrn.len() as Card16).as_bytes(&mut bytes[index..]);
        if cond0.compat_map() {
            index += self.n_total_si.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.si_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
        let block_len: usize = vector_as_bytes(&self.group_rtrn, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        if cond0.indicator_maps() {
            index += self.indicatormap_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_maps() {
            index += self.indicator_device_id.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_maps() {
            index += self.indicatormap_sequence.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_maps() {
            index += self.indicatormap_length.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_maps() {
            index += self.which.as_bytes(&mut bytes[index..]);
        }
        if cond0.indicator_maps() {
            index += self.real_indicators.as_bytes(&mut bytes[index..]);
        }
        index += (self.maps.len() as Card8).as_bytes(&mut bytes[index..]);
        index += 15;
        let block_len: usize = vector_as_bytes(&self.maps, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        if cond0.other_names() {
            index += self.keyname_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.key_device_id.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.keyname_sequence.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.keyname_length.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.which_.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.key_min_key_code.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.key_max_key_code.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.n_types_.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.group_names.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.virtual_mods_.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.first_key.as_bytes(&mut bytes[index..]);
        }
        index += (self.key_names.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0.other_names() {
            index += self.indicators.as_bytes(&mut bytes[index..]);
        }
        index += (self.radio_group_names.len() as Card8).as_bytes(&mut bytes[index..]);
        index += (self.key_aliases.len() as Card8).as_bytes(&mut bytes[index..]);
        if cond0.other_names() {
            index += self.n_kt_levels.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.keycodes_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.geometry_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.symbols_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.phys_symbols_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.types_name.as_bytes(&mut bytes[index..]);
        }
        if cond0.other_names() {
            index += self.compat_name.as_bytes(&mut bytes[index..]);
        }
        let block_len: usize = vector_as_bytes(&self.type_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.n_levels_per_type, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let block_len: usize = vector_as_bytes(&self.kt_level_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.indicator_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.virtual_mod_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.groups, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let block_len: usize = vector_as_bytes(&self.key_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
        let block_len: usize = vector_as_bytes(&self.key_aliases, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
        let block_len: usize = vector_as_bytes(&self.radio_group_names, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        if cond0.geometry() {
            index += self.geometry_type.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.geometry_device_id.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.geometry_sequence.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.geometry_length.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.name.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.geometry_found.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.width_mm.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.height_mm.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.n_properties.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.n_colors.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.n_shapes.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.n_sections.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.n_doodads.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.n_key_aliases.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.base_color_ndx.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.label_color_ndx.as_bytes(&mut bytes[index..]);
        }
        if cond0.geometry() {
            index += self.label_font.as_bytes(&mut bytes[index..]);
        }
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetKbdByNameReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (loaded, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (new_keyboard, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (found, sz): (GbnDetail, usize) = <GbnDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (reported, sz): (GbnDetail, usize) = <GbnDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        let cond0 = (reported);
        let getmap_type: Card8 = if cond0.server_symbols() {
            let (getmap_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            getmap_type
        } else {
            Default::default()
        };
        let type_device_id: Card8 = if cond0.server_symbols() {
            let (type_device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            type_device_id
        } else {
            Default::default()
        };
        let getmap_sequence: Card16 = if cond0.server_symbols() {
            let (getmap_sequence, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            getmap_sequence
        } else {
            Default::default()
        };
        let getmap_length: Card32 = if cond0.server_symbols() {
            let (getmap_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            getmap_length
        } else {
            Default::default()
        };
        index += 2;
        let type_min_key_code: Keycode = if cond0.server_symbols() {
            let (type_min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            type_min_key_code
        } else {
            Default::default()
        };
        let type_max_key_code: Keycode = if cond0.server_symbols() {
            let (type_max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            type_max_key_code
        } else {
            Default::default()
        };
        let present: MapPart = if cond0.server_symbols() {
            let (present, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
            index += sz;
            present
        } else {
            Default::default()
        };
        let first_type: Card8 = if cond0.server_symbols() {
            let (first_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            first_type
        } else {
            Default::default()
        };
        let n_types: Card8 = if cond0.server_symbols() {
            let (n_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            n_types
        } else {
            Default::default()
        };
        let total_types: Card8 = if cond0.server_symbols() {
            let (total_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            total_types
        } else {
            Default::default()
        };
        let first_key_sym: Keycode = if cond0.server_symbols() {
            let (first_key_sym, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            first_key_sym
        } else {
            Default::default()
        };
        let total_syms: Card16 = if cond0.server_symbols() {
            let (total_syms, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            total_syms
        } else {
            Default::default()
        };
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let first_key_action: Keycode = if cond0.server_symbols() {
            let (first_key_action, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            first_key_action
        } else {
            Default::default()
        };
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let first_key_behavior: Keycode = if cond0.server_symbols() {
            let (first_key_behavior, sz): (Keycode, usize) =
                <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            first_key_behavior
        } else {
            Default::default()
        };
        let n_key_behaviors: Card8 = if cond0.server_symbols() {
            let (n_key_behaviors, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            n_key_behaviors
        } else {
            Default::default()
        };
        let (len3, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let first_key_explicit: Keycode = if cond0.server_symbols() {
            let (first_key_explicit, sz): (Keycode, usize) =
                <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            first_key_explicit
        } else {
            Default::default()
        };
        let n_key_explicit: Card8 = if cond0.server_symbols() {
            let (n_key_explicit, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            n_key_explicit
        } else {
            Default::default()
        };
        let (len4, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let first_mod_map_key: Keycode = if cond0.server_symbols() {
            let (first_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            first_mod_map_key
        } else {
            Default::default()
        };
        let n_mod_map_keys: Card8 = if cond0.server_symbols() {
            let (n_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            n_mod_map_keys
        } else {
            Default::default()
        };
        let (len5, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let first_v_mod_map_key: Keycode = if cond0.server_symbols() {
            let (first_v_mod_map_key, sz): (Keycode, usize) =
                <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            first_v_mod_map_key
        } else {
            Default::default()
        };
        let n_v_mod_map_keys: Card8 = if cond0.server_symbols() {
            let (n_v_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            n_v_mod_map_keys
        } else {
            Default::default()
        };
        let (len6, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let virtual_mods: VMod = if cond0.server_symbols() {
            let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
            index += sz;
            virtual_mods
        } else {
            Default::default()
        };
        let (types_rtrn, block_len): (Vec<KeyType>, usize) =
            vector_from_bytes(&bytes[index..], (n_types as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyType>());
        let (syms_rtrn, block_len): (Vec<KeySymMap>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
        let (acts_rtrn_count, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        index += 4;
        let (acts_rtrn_acts, block_len): (Vec<Action>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let (behaviors_rtrn, block_len): (Vec<SetBehavior>, usize) =
            vector_from_bytes(&bytes[index..], len3 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
        let (vmods_rtrn, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], ((virtual_mods).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let (explicit_rtrn, block_len): (Vec<SetExplicit>, usize) =
            vector_from_bytes(&bytes[index..], len4 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
        let (modmap_rtrn, block_len): (Vec<KeyModMap>, usize) =
            vector_from_bytes(&bytes[index..], len5 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
        let (vmodmap_rtrn, block_len): (Vec<KeyVModMap>, usize) =
            vector_from_bytes(&bytes[index..], len6 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
        let compatmap_type: Card8 = if cond0.compat_map() {
            let (compatmap_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            compatmap_type
        } else {
            Default::default()
        };
        let compat_device_id: Card8 = if cond0.compat_map() {
            let (compat_device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            compat_device_id
        } else {
            Default::default()
        };
        let compatmap_sequence: Card16 = if cond0.compat_map() {
            let (compatmap_sequence, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            compatmap_sequence
        } else {
            Default::default()
        };
        let compatmap_length: Card32 = if cond0.compat_map() {
            let (compatmap_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            compatmap_length
        } else {
            Default::default()
        };
        let groups_rtrn: SetOfGroup = if cond0.compat_map() {
            let (groups_rtrn, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
            index += sz;
            groups_rtrn
        } else {
            Default::default()
        };
        let first_si_rtrn: Card16 = if cond0.compat_map() {
            let (first_si_rtrn, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            first_si_rtrn
        } else {
            Default::default()
        };
        let (len7, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let n_total_si: Card16 = if cond0.compat_map() {
            let (n_total_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_total_si
        } else {
            Default::default()
        };
        let (si_rtrn, block_len): (Vec<SymInterpret>, usize) =
            vector_from_bytes(&bytes[index..], len7 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
        let (group_rtrn, block_len): (Vec<ModDef>, usize) =
            vector_from_bytes(&bytes[index..], ((groups_rtrn).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
        let indicatormap_type: Card8 = if cond0.indicator_maps() {
            let (indicatormap_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            indicatormap_type
        } else {
            Default::default()
        };
        let indicator_device_id: Card8 = if cond0.indicator_maps() {
            let (indicator_device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            indicator_device_id
        } else {
            Default::default()
        };
        let indicatormap_sequence: Card16 = if cond0.indicator_maps() {
            let (indicatormap_sequence, sz): (Card16, usize) =
                <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            indicatormap_sequence
        } else {
            Default::default()
        };
        let indicatormap_length: Card32 = if cond0.indicator_maps() {
            let (indicatormap_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            indicatormap_length
        } else {
            Default::default()
        };
        let which: Card32 = if cond0.indicator_maps() {
            let (which, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            which
        } else {
            Default::default()
        };
        let real_indicators: Card32 = if cond0.indicator_maps() {
            let (real_indicators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            real_indicators
        } else {
            Default::default()
        };
        let (len8, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 15;
        let (maps, block_len): (Vec<IndicatorMap>, usize) =
            vector_from_bytes(&bytes[index..], len8 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
        let keyname_type: Card8 = if cond0.other_names() {
            let (keyname_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            keyname_type
        } else {
            Default::default()
        };
        let key_device_id: Card8 = if cond0.other_names() {
            let (key_device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            key_device_id
        } else {
            Default::default()
        };
        let keyname_sequence: Card16 = if cond0.other_names() {
            let (keyname_sequence, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            keyname_sequence
        } else {
            Default::default()
        };
        let keyname_length: Card32 = if cond0.other_names() {
            let (keyname_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            keyname_length
        } else {
            Default::default()
        };
        let which_: NameDetail = if cond0.other_names() {
            let (which_, sz): (NameDetail, usize) = <NameDetail>::from_bytes(&bytes[index..])?;
            index += sz;
            which_
        } else {
            Default::default()
        };
        let key_min_key_code: Keycode = if cond0.other_names() {
            let (key_min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            key_min_key_code
        } else {
            Default::default()
        };
        let key_max_key_code: Keycode = if cond0.other_names() {
            let (key_max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            key_max_key_code
        } else {
            Default::default()
        };
        let n_types_: Card8 = if cond0.other_names() {
            let (n_types_, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            n_types_
        } else {
            Default::default()
        };
        let group_names: SetOfGroup = if cond0.other_names() {
            let (group_names, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
            index += sz;
            group_names
        } else {
            Default::default()
        };
        let virtual_mods_: VMod = if cond0.other_names() {
            let (virtual_mods_, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
            index += sz;
            virtual_mods_
        } else {
            Default::default()
        };
        let first_key: Keycode = if cond0.other_names() {
            let (first_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
            index += sz;
            first_key
        } else {
            Default::default()
        };
        let (len9, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let indicators: Card32 = if cond0.other_names() {
            let (indicators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            indicators
        } else {
            Default::default()
        };
        let (len10, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len11, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let n_kt_levels: Card16 = if cond0.other_names() {
            let (n_kt_levels, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_kt_levels
        } else {
            Default::default()
        };
        let keycodes_name: Atom = if cond0.other_names() {
            let (keycodes_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            keycodes_name
        } else {
            Default::default()
        };
        let geometry_name: Atom = if cond0.other_names() {
            let (geometry_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_name
        } else {
            Default::default()
        };
        let symbols_name: Atom = if cond0.other_names() {
            let (symbols_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            symbols_name
        } else {
            Default::default()
        };
        let phys_symbols_name: Atom = if cond0.other_names() {
            let (phys_symbols_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            phys_symbols_name
        } else {
            Default::default()
        };
        let types_name: Atom = if cond0.other_names() {
            let (types_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            types_name
        } else {
            Default::default()
        };
        let compat_name: Atom = if cond0.other_names() {
            let (compat_name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            compat_name
        } else {
            Default::default()
        };
        let (type_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], (n_types as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (n_levels_per_type, block_len): (Vec<Card8>, usize) =
            vector_from_bytes(&bytes[index..], (n_types as usize) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Card8>());
        let (kt_level_names, block_len): (Vec<Atom>, usize) = vector_from_bytes(
            &bytes[index..],
            (n_levels_per_type
                .iter()
                .map(|a| {
                    (TryInto::<usize>::try_into(*a).expect("Unable to cast type to usize")) as usize
                })
                .sum::<usize>()) as usize,
        )?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (indicator_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((indicators).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (virtual_mod_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((virtual_mods).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (groups, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], ((group_names).count_ones()) as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let (key_names, block_len): (Vec<KeyName>, usize) =
            vector_from_bytes(&bytes[index..], len9 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
        let (key_aliases, block_len): (Vec<KeyAlias>, usize) =
            vector_from_bytes(&bytes[index..], len11 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
        let (radio_group_names, block_len): (Vec<Atom>, usize) =
            vector_from_bytes(&bytes[index..], len10 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Atom>());
        let geometry_type: Card8 = if cond0.geometry() {
            let (geometry_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_type
        } else {
            Default::default()
        };
        let geometry_device_id: Card8 = if cond0.geometry() {
            let (geometry_device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_device_id
        } else {
            Default::default()
        };
        let geometry_sequence: Card16 = if cond0.geometry() {
            let (geometry_sequence, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_sequence
        } else {
            Default::default()
        };
        let geometry_length: Card32 = if cond0.geometry() {
            let (geometry_length, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_length
        } else {
            Default::default()
        };
        let name: Atom = if cond0.geometry() {
            let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
            index += sz;
            name
        } else {
            Default::default()
        };
        let geometry_found: bool = if cond0.geometry() {
            let (geometry_found, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
            index += sz;
            geometry_found
        } else {
            Default::default()
        };
        let width_mm: Card16 = if cond0.geometry() {
            let (width_mm, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            width_mm
        } else {
            Default::default()
        };
        let height_mm: Card16 = if cond0.geometry() {
            let (height_mm, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            height_mm
        } else {
            Default::default()
        };
        let n_properties: Card16 = if cond0.geometry() {
            let (n_properties, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_properties
        } else {
            Default::default()
        };
        let n_colors: Card16 = if cond0.geometry() {
            let (n_colors, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_colors
        } else {
            Default::default()
        };
        let n_shapes: Card16 = if cond0.geometry() {
            let (n_shapes, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_shapes
        } else {
            Default::default()
        };
        let n_sections: Card16 = if cond0.geometry() {
            let (n_sections, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_sections
        } else {
            Default::default()
        };
        let n_doodads: Card16 = if cond0.geometry() {
            let (n_doodads, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_doodads
        } else {
            Default::default()
        };
        let n_key_aliases: Card16 = if cond0.geometry() {
            let (n_key_aliases, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
            index += sz;
            n_key_aliases
        } else {
            Default::default()
        };
        let base_color_ndx: Card8 = if cond0.geometry() {
            let (base_color_ndx, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            base_color_ndx
        } else {
            Default::default()
        };
        let label_color_ndx: Card8 = if cond0.geometry() {
            let (label_color_ndx, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
            index += sz;
            label_color_ndx
        } else {
            Default::default()
        };
        let label_font: CountedString16 = if cond0.geometry() {
            let (label_font, sz): (CountedString16, usize) =
                <CountedString16>::from_bytes(&bytes[index..])?;
            index += sz;
            label_font
        } else {
            Default::default()
        };
        Some((
            GetKbdByNameReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                min_key_code: min_key_code,
                max_key_code: max_key_code,
                loaded: loaded,
                new_keyboard: new_keyboard,
                found: found,
                reported: reported,
                getmap_type: getmap_type,
                type_device_id: type_device_id,
                getmap_sequence: getmap_sequence,
                getmap_length: getmap_length,
                type_min_key_code: type_min_key_code,
                type_max_key_code: type_max_key_code,
                present: present,
                first_type: first_type,
                n_types: n_types,
                total_types: total_types,
                first_key_sym: first_key_sym,
                total_syms: total_syms,
                first_key_action: first_key_action,
                first_key_behavior: first_key_behavior,
                n_key_behaviors: n_key_behaviors,
                first_key_explicit: first_key_explicit,
                n_key_explicit: n_key_explicit,
                first_mod_map_key: first_mod_map_key,
                n_mod_map_keys: n_mod_map_keys,
                first_v_mod_map_key: first_v_mod_map_key,
                n_v_mod_map_keys: n_v_mod_map_keys,
                virtual_mods: virtual_mods,
                types_rtrn: types_rtrn,
                syms_rtrn: syms_rtrn,
                acts_rtrn_count: acts_rtrn_count,
                acts_rtrn_acts: acts_rtrn_acts,
                behaviors_rtrn: behaviors_rtrn,
                vmods_rtrn: vmods_rtrn,
                explicit_rtrn: explicit_rtrn,
                modmap_rtrn: modmap_rtrn,
                vmodmap_rtrn: vmodmap_rtrn,
                compatmap_type: compatmap_type,
                compat_device_id: compat_device_id,
                compatmap_sequence: compatmap_sequence,
                compatmap_length: compatmap_length,
                groups_rtrn: groups_rtrn,
                first_si_rtrn: first_si_rtrn,
                n_total_si: n_total_si,
                si_rtrn: si_rtrn,
                group_rtrn: group_rtrn,
                indicatormap_type: indicatormap_type,
                indicator_device_id: indicator_device_id,
                indicatormap_sequence: indicatormap_sequence,
                indicatormap_length: indicatormap_length,
                which: which,
                real_indicators: real_indicators,
                maps: maps,
                keyname_type: keyname_type,
                key_device_id: key_device_id,
                keyname_sequence: keyname_sequence,
                keyname_length: keyname_length,
                which_: which_,
                key_min_key_code: key_min_key_code,
                key_max_key_code: key_max_key_code,
                n_types_: n_types_,
                group_names: group_names,
                virtual_mods_: virtual_mods_,
                first_key: first_key,
                indicators: indicators,
                n_kt_levels: n_kt_levels,
                keycodes_name: keycodes_name,
                geometry_name: geometry_name,
                symbols_name: symbols_name,
                phys_symbols_name: phys_symbols_name,
                types_name: types_name,
                compat_name: compat_name,
                type_names: type_names,
                n_levels_per_type: n_levels_per_type,
                kt_level_names: kt_level_names,
                indicator_names: indicator_names,
                virtual_mod_names: virtual_mod_names,
                groups: groups,
                key_names: key_names,
                key_aliases: key_aliases,
                radio_group_names: radio_group_names,
                geometry_type: geometry_type,
                geometry_device_id: geometry_device_id,
                geometry_sequence: geometry_sequence,
                geometry_length: geometry_length,
                name: name,
                geometry_found: geometry_found,
                width_mm: width_mm,
                height_mm: height_mm,
                n_properties: n_properties,
                n_colors: n_colors,
                n_shapes: n_shapes,
                n_sections: n_sections,
                n_doodads: n_doodads,
                n_key_aliases: n_key_aliases,
                base_color_ndx: base_color_ndx,
                label_color_ndx: label_color_ndx,
                label_font: label_font,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.min_key_code.size()
            + self.max_key_code.size()
            + self.loaded.size()
            + self.new_keyboard.size()
            + self.found.size()
            + self.reported.size()
            + 16
            + self.getmap_type.size()
            + self.type_device_id.size()
            + self.getmap_sequence.size()
            + self.getmap_length.size()
            + 2
            + self.type_min_key_code.size()
            + self.type_max_key_code.size()
            + self.present.size()
            + self.first_type.size()
            + self.n_types.size()
            + self.total_types.size()
            + self.first_key_sym.size()
            + self.total_syms.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_action.size()
            + ::core::mem::size_of::<Card16>()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_behavior.size()
            + self.n_key_behaviors.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_key_explicit.size()
            + self.n_key_explicit.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_mod_map_key.size()
            + self.n_mod_map_keys.size()
            + ::core::mem::size_of::<Card8>()
            + self.first_v_mod_map_key.size()
            + self.n_v_mod_map_keys.size()
            + ::core::mem::size_of::<Card8>()
            + 1
            + self.virtual_mods.size()
            + {
                let block_len: usize = self.types_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyType>());
                block_len + pad
            }
            + {
                let block_len: usize = self.syms_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeySymMap>());
                block_len + pad
            }
            + {
                let block_len: usize = self.acts_rtrn_count.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + 4
            + {
                let block_len: usize = self.acts_rtrn_acts.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Action>());
                block_len + pad
            }
            + {
                let block_len: usize = self.behaviors_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SetBehavior>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vmods_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.explicit_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SetExplicit>());
                block_len + pad
            }
            + {
                let block_len: usize = self.modmap_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyModMap>());
                block_len + pad
            }
            + {
                let block_len: usize = self.vmodmap_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyVModMap>());
                block_len + pad
            }
            + self.compatmap_type.size()
            + self.compat_device_id.size()
            + self.compatmap_sequence.size()
            + self.compatmap_length.size()
            + self.groups_rtrn.size()
            + self.first_si_rtrn.size()
            + ::core::mem::size_of::<Card16>()
            + self.n_total_si.size()
            + {
                let block_len: usize = self.si_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<SymInterpret>());
                block_len + pad
            }
            + {
                let block_len: usize = self.group_rtrn.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<ModDef>());
                block_len + pad
            }
            + self.indicatormap_type.size()
            + self.indicator_device_id.size()
            + self.indicatormap_sequence.size()
            + self.indicatormap_length.size()
            + self.which.size()
            + self.real_indicators.size()
            + ::core::mem::size_of::<Card8>()
            + 15
            + {
                let block_len: usize = self.maps.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<IndicatorMap>());
                block_len + pad
            }
            + self.keyname_type.size()
            + self.key_device_id.size()
            + self.keyname_sequence.size()
            + self.keyname_length.size()
            + self.which_.size()
            + self.key_min_key_code.size()
            + self.key_max_key_code.size()
            + self.n_types_.size()
            + self.group_names.size()
            + self.virtual_mods_.size()
            + self.first_key.size()
            + ::core::mem::size_of::<Card8>()
            + self.indicators.size()
            + ::core::mem::size_of::<Card8>()
            + ::core::mem::size_of::<Card8>()
            + self.n_kt_levels.size()
            + self.keycodes_name.size()
            + self.geometry_name.size()
            + self.symbols_name.size()
            + self.phys_symbols_name.size()
            + self.types_name.size()
            + self.compat_name.size()
            + {
                let block_len: usize = self.type_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.n_levels_per_type.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Card8>());
                block_len + pad
            }
            + {
                let block_len: usize = self.kt_level_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.indicator_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.virtual_mod_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.groups.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + {
                let block_len: usize = self.key_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyName>());
                block_len + pad
            }
            + {
                let block_len: usize = self.key_aliases.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<KeyAlias>());
                block_len + pad
            }
            + {
                let block_len: usize = self.radio_group_names.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Atom>());
                block_len + pad
            }
            + self.geometry_type.size()
            + self.geometry_device_id.size()
            + self.geometry_sequence.size()
            + self.geometry_length.size()
            + self.name.size()
            + self.geometry_found.size()
            + self.width_mm.size()
            + self.height_mm.size()
            + self.n_properties.size()
            + self.n_colors.size()
            + self.n_shapes.size()
            + self.n_sections.size()
            + self.n_doodads.size()
            + self.n_key_aliases.size()
            + self.base_color_ndx.size()
            + self.label_color_ndx.size()
            + self.label_font.size()
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GbnDetail {
    pub inner: u16,
}
impl GbnDetail {
    #[inline]
    pub fn types(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_types(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn compat_map(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_compat_map(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn client_symbols(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_client_symbols(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn server_symbols(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_server_symbols(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn indicator_maps(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_indicator_maps(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn key_names(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_key_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn geometry(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_geometry(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn other_names(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_other_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(
        types: bool,
        compat_map: bool,
        client_symbols: bool,
        server_symbols: bool,
        indicator_maps: bool,
        key_names: bool,
        geometry: bool,
        other_names: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if types {
            inner |= 1 << 0;
        }
        if compat_map {
            inner |= 1 << 1;
        }
        if client_symbols {
            inner |= 1 << 2;
        }
        if server_symbols {
            inner |= 1 << 3;
        }
        if indicator_maps {
            inner |= 1 << 4;
        }
        if key_names {
            inner |= 1 << 5;
        }
        if geometry {
            inner |= 1 << 6;
        }
        if other_names {
            inner |= 1 << 7;
        }
        GbnDetail { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const TYPES: Self = Self { inner: 1 };
    pub const COMPAT_MAP: Self = Self { inner: 2 };
    pub const CLIENT_SYMBOLS: Self = Self { inner: 4 };
    pub const SERVER_SYMBOLS: Self = Self { inner: 8 };
    pub const INDICATOR_MAPS: Self = Self { inner: 16 };
    pub const KEY_NAMES: Self = Self { inner: 32 };
    pub const GEOMETRY: Self = Self { inner: 64 };
    pub const OTHER_NAMES: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 255 };
}
impl AsByteSequence for GbnDetail {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((GbnDetail { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for GbnDetail {
    type Output = GbnDetail;
    #[inline]
    fn not(self) -> GbnDetail {
        GbnDetail { inner: !self.inner }
    }
}
impl core::ops::BitAnd for GbnDetail {
    type Output = GbnDetail;
    #[inline]
    fn bitand(self, rhs: GbnDetail) -> GbnDetail {
        GbnDetail {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for GbnDetail {
    type Output = GbnDetail;
    #[inline]
    fn bitor(self, rhs: GbnDetail) -> GbnDetail {
        GbnDetail {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for GbnDetail {
    type Output = GbnDetail;
    #[inline]
    fn bitxor(self, rhs: GbnDetail) -> GbnDetail {
        GbnDetail {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct GetDeviceInfoRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub wanted: XiFeature,
    pub all_buttons: bool,
    pub first_button: Card8,
    pub n_buttons: Card8,
    pub led_class: LedClass,
    pub led_id: IdSpec,
}
impl GetDeviceInfoRequest {}
impl AsByteSequence for GetDeviceInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.wanted.as_bytes(&mut bytes[index..]);
        index += self.all_buttons.as_bytes(&mut bytes[index..]);
        index += self.first_button.as_bytes(&mut bytes[index..]);
        index += self.n_buttons.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.led_class.as_bytes(&mut bytes[index..]);
        index += self.led_id.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (wanted, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        let (all_buttons, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_buttons, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (led_class, sz): (LedClass, usize) = <LedClass>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_id, sz): (IdSpec, usize) = <IdSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            GetDeviceInfoRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                wanted: wanted,
                all_buttons: all_buttons,
                first_button: first_button,
                n_buttons: n_buttons,
                led_class: led_class,
                led_id: led_id,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.wanted.size()
            + self.all_buttons.size()
            + self.first_button.size()
            + self.n_buttons.size()
            + 1
            + self.led_class.size()
            + self.led_id.size()
    }
}
impl Request for GetDeviceInfoRequest {
    const OPCODE: u8 = 24;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = GetDeviceInfoReply;
}
#[derive(Clone, Debug, Default)]
pub struct GetDeviceInfoReply {
    pub reply_type: u8,
    pub device_id: Card8,
    pub sequence: u16,
    pub length: u32,
    pub present: XiFeature,
    pub supported: XiFeature,
    pub unsupported: XiFeature,
    pub first_btn_wanted: Card8,
    pub n_btns_wanted: Card8,
    pub first_btn_rtrn: Card8,
    pub total_btns: Card8,
    pub has_own_state: bool,
    pub dflt_kbd_fb: Card16,
    pub dflt_led_fb: Card16,
    pub dev_type: Atom,
    pub name: Vec<String8>,
    pub btn_actions: Vec<Action>,
    pub leds: Vec<DeviceLedInfo>,
}
impl GetDeviceInfoReply {}
impl AsByteSequence for GetDeviceInfoReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.present.as_bytes(&mut bytes[index..]);
        index += self.supported.as_bytes(&mut bytes[index..]);
        index += self.unsupported.as_bytes(&mut bytes[index..]);
        index += (self.leds.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.first_btn_wanted.as_bytes(&mut bytes[index..]);
        index += self.n_btns_wanted.as_bytes(&mut bytes[index..]);
        index += self.first_btn_rtrn.as_bytes(&mut bytes[index..]);
        index += (self.btn_actions.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.total_btns.as_bytes(&mut bytes[index..]);
        index += self.has_own_state.as_bytes(&mut bytes[index..]);
        index += self.dflt_kbd_fb.as_bytes(&mut bytes[index..]);
        index += self.dflt_led_fb.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.dev_type.as_bytes(&mut bytes[index..]);
        index += (self.name.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.name, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, 4);
        let block_len: usize = vector_as_bytes(&self.btn_actions, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let block_len: usize = vector_as_bytes(&self.leds, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DeviceLedInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing GetDeviceInfoReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (present, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        let (supported, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        let (unsupported, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_btn_wanted, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_btns_wanted, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_btn_rtrn, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (total_btns, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (has_own_state, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dflt_kbd_fb, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (dflt_led_fb, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (dev_type, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len2, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len2 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, 4);
        let (btn_actions, block_len): (Vec<Action>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let (leds, block_len): (Vec<DeviceLedInfo>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DeviceLedInfo>());
        Some((
            GetDeviceInfoReply {
                reply_type: reply_type,
                device_id: device_id,
                sequence: sequence,
                length: length,
                present: present,
                supported: supported,
                unsupported: unsupported,
                first_btn_wanted: first_btn_wanted,
                n_btns_wanted: n_btns_wanted,
                first_btn_rtrn: first_btn_rtrn,
                total_btns: total_btns,
                has_own_state: has_own_state,
                dflt_kbd_fb: dflt_kbd_fb,
                dflt_led_fb: dflt_led_fb,
                dev_type: dev_type,
                name: name,
                btn_actions: btn_actions,
                leds: leds,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.reply_type.size()
            + self.device_id.size()
            + self.sequence.size()
            + self.length.size()
            + self.present.size()
            + self.supported.size()
            + self.unsupported.size()
            + ::core::mem::size_of::<Card16>()
            + self.first_btn_wanted.size()
            + self.n_btns_wanted.size()
            + self.first_btn_rtrn.size()
            + ::core::mem::size_of::<Card8>()
            + self.total_btns.size()
            + self.has_own_state.size()
            + self.dflt_kbd_fb.size()
            + self.dflt_led_fb.size()
            + 2
            + self.dev_type.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.name.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, 4);
                block_len + pad
            }
            + {
                let block_len: usize = self.btn_actions.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Action>());
                block_len + pad
            }
            + {
                let block_len: usize = self.leds.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<DeviceLedInfo>());
                block_len + pad
            }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct XiFeature {
    pub inner: u16,
}
impl XiFeature {
    #[inline]
    pub fn keyboards(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_keyboards(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn button_actions(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_button_actions(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn indicator_names(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_indicator_names(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn indicator_maps(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_indicator_maps(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn indicator_state(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_indicator_state(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn new(
        keyboards: bool,
        button_actions: bool,
        indicator_names: bool,
        indicator_maps: bool,
        indicator_state: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if keyboards {
            inner |= 1 << 0;
        }
        if button_actions {
            inner |= 1 << 1;
        }
        if indicator_names {
            inner |= 1 << 2;
        }
        if indicator_maps {
            inner |= 1 << 3;
        }
        if indicator_state {
            inner |= 1 << 4;
        }
        XiFeature { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const KEYBOARDS: Self = Self { inner: 1 };
    pub const BUTTON_ACTIONS: Self = Self { inner: 2 };
    pub const INDICATOR_NAMES: Self = Self { inner: 4 };
    pub const INDICATOR_MAPS: Self = Self { inner: 8 };
    pub const INDICATOR_STATE: Self = Self { inner: 16 };
    pub const COMPLETE: Self = Self { inner: 31 };
}
impl AsByteSequence for XiFeature {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((XiFeature { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for XiFeature {
    type Output = XiFeature;
    #[inline]
    fn not(self) -> XiFeature {
        XiFeature { inner: !self.inner }
    }
}
impl core::ops::BitAnd for XiFeature {
    type Output = XiFeature;
    #[inline]
    fn bitand(self, rhs: XiFeature) -> XiFeature {
        XiFeature {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for XiFeature {
    type Output = XiFeature;
    #[inline]
    fn bitor(self, rhs: XiFeature) -> XiFeature {
        XiFeature {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for XiFeature {
    type Output = XiFeature;
    #[inline]
    fn bitxor(self, rhs: XiFeature) -> XiFeature {
        XiFeature {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct SetDeviceInfoRequest {
    pub req_type: u8,
    pub device_spec: DeviceSpec,
    pub length: u16,
    pub first_btn: Card8,
    pub change: XiFeature,
    pub btn_actions: Vec<Action>,
    pub leds: Vec<DeviceLedInfo>,
}
impl SetDeviceInfoRequest {}
impl AsByteSequence for SetDeviceInfoRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += self.device_spec.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.first_btn.as_bytes(&mut bytes[index..]);
        index += (self.btn_actions.len() as Card8).as_bytes(&mut bytes[index..]);
        index += self.change.as_bytes(&mut bytes[index..]);
        index += (self.leds.len() as Card16).as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.btn_actions, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let block_len: usize = vector_as_bytes(&self.leds, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DeviceLedInfo>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDeviceInfoRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_spec, sz): (DeviceSpec, usize) = <DeviceSpec>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_btn, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (change, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len1, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (btn_actions, block_len): (Vec<Action>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<Action>());
        let (leds, block_len): (Vec<DeviceLedInfo>, usize) =
            vector_from_bytes(&bytes[index..], len1 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<DeviceLedInfo>());
        Some((
            SetDeviceInfoRequest {
                req_type: req_type,
                device_spec: device_spec,
                length: length,
                first_btn: first_btn,
                change: change,
                btn_actions: btn_actions,
                leds: leds,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + self.device_spec.size()
            + self.length.size()
            + self.first_btn.size()
            + ::core::mem::size_of::<Card8>()
            + self.change.size()
            + ::core::mem::size_of::<Card16>()
            + {
                let block_len: usize = self.btn_actions.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<Action>());
                block_len + pad
            }
            + {
                let block_len: usize = self.leds.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<DeviceLedInfo>());
                block_len + pad
            }
    }
}
impl Request for SetDeviceInfoRequest {
    const OPCODE: u8 = 25;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = ();
}
#[derive(Clone, Debug, Default)]
pub struct SetDebuggingFlagsRequest {
    pub req_type: u8,
    pub length: u16,
    pub affect_flags: Card32,
    pub flags: Card32,
    pub affect_ctrls: Card32,
    pub ctrls: Card32,
    pub message: Vec<String8>,
}
impl SetDebuggingFlagsRequest {}
impl AsByteSequence for SetDebuggingFlagsRequest {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.req_type.as_bytes(&mut bytes[index..]);
        index += (self.message.len() as Card16).as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.affect_flags.as_bytes(&mut bytes[index..]);
        index += self.flags.as_bytes(&mut bytes[index..]);
        index += self.affect_ctrls.as_bytes(&mut bytes[index..]);
        index += self.ctrls.as_bytes(&mut bytes[index..]);
        let block_len: usize = vector_as_bytes(&self.message, &mut bytes[index..]);
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDebuggingFlagsRequest from byte buffer");
        let (req_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (len0, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (affect_flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (affect_ctrls, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ctrls, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (message, block_len): (Vec<String8>, usize) =
            vector_from_bytes(&bytes[index..], len0 as usize)?;
        index += block_len;
        index += buffer_pad(block_len, ::core::mem::align_of::<String8>());
        Some((
            SetDebuggingFlagsRequest {
                req_type: req_type,
                length: length,
                affect_flags: affect_flags,
                flags: flags,
                affect_ctrls: affect_ctrls,
                ctrls: ctrls,
                message: message,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.req_type.size()
            + ::core::mem::size_of::<Card16>()
            + self.length.size()
            + 2
            + self.affect_flags.size()
            + self.flags.size()
            + self.affect_ctrls.size()
            + self.ctrls.size()
            + {
                let block_len: usize = self.message.iter().map(|i| i.size()).sum();
                let pad: usize = buffer_pad(block_len, ::core::mem::align_of::<String8>());
                block_len + pad
            }
    }
}
impl Request for SetDebuggingFlagsRequest {
    const OPCODE: u8 = 101;
    const EXTENSION: Option<&'static str> = Some("XKEYBOARD");
    const REPLY_EXPECTS_FDS: bool = false;
    type Reply = SetDebuggingFlagsReply;
}
#[derive(Clone, Debug, Default)]
pub struct SetDebuggingFlagsReply {
    pub reply_type: u8,
    pub sequence: u16,
    pub length: u32,
    pub current_flags: Card32,
    pub current_ctrls: Card32,
    pub supported_flags: Card32,
    pub supported_ctrls: Card32,
}
impl SetDebuggingFlagsReply {}
impl AsByteSequence for SetDebuggingFlagsReply {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.reply_type.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.length.as_bytes(&mut bytes[index..]);
        index += self.current_flags.as_bytes(&mut bytes[index..]);
        index += self.current_ctrls.as_bytes(&mut bytes[index..]);
        index += self.supported_flags.as_bytes(&mut bytes[index..]);
        index += self.supported_ctrls.as_bytes(&mut bytes[index..]);
        index += 8;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing SetDebuggingFlagsReply from byte buffer");
        let (reply_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (length, sz): (u32, usize) = <u32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (current_flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (current_ctrls, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (supported_flags, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (supported_ctrls, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 8;
        Some((
            SetDebuggingFlagsReply {
                reply_type: reply_type,
                sequence: sequence,
                length: length,
                current_flags: current_flags,
                current_ctrls: current_ctrls,
                supported_flags: supported_flags,
                supported_ctrls: supported_ctrls,
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
            + self.current_flags.size()
            + self.current_ctrls.size()
            + self.supported_flags.size()
            + self.supported_ctrls.size()
            + 8
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct NknDetail {
    pub inner: u16,
}
impl NknDetail {
    #[inline]
    pub fn keycodes(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_keycodes(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn geometry(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_geometry(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn device_id(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_device_id(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(keycodes: bool, geometry: bool, device_id: bool) -> Self {
        let mut inner: u16 = 0;
        if keycodes {
            inner |= 1 << 0;
        }
        if geometry {
            inner |= 1 << 1;
        }
        if device_id {
            inner |= 1 << 2;
        }
        NknDetail { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const KEYCODES: Self = Self { inner: 1 };
    pub const GEOMETRY: Self = Self { inner: 2 };
    pub const DEVICE_ID: Self = Self { inner: 4 };
    pub const COMPLETE: Self = Self { inner: 7 };
}
impl AsByteSequence for NknDetail {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((NknDetail { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for NknDetail {
    type Output = NknDetail;
    #[inline]
    fn not(self) -> NknDetail {
        NknDetail { inner: !self.inner }
    }
}
impl core::ops::BitAnd for NknDetail {
    type Output = NknDetail;
    #[inline]
    fn bitand(self, rhs: NknDetail) -> NknDetail {
        NknDetail {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for NknDetail {
    type Output = NknDetail;
    #[inline]
    fn bitor(self, rhs: NknDetail) -> NknDetail {
        NknDetail {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for NknDetail {
    type Output = NknDetail;
    #[inline]
    fn bitxor(self, rhs: NknDetail) -> NknDetail {
        NknDetail {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct StatePart {
    pub inner: u16,
}
impl StatePart {
    #[inline]
    pub fn modifier_state(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_modifier_state(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn modifier_base(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_modifier_base(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn modifier_latch(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_modifier_latch(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn modifier_lock(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_modifier_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn group_state(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_group_state(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn group_base(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_group_base(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn group_latch(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_group_latch(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn group_lock(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_group_lock(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn compat_state(&self) -> bool {
        self.inner & (1 << 8) != 0
    }
    #[inline]
    pub fn set_compat_state(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 8;
        } else {
            self.inner &= !(1 << 8);
        }
        self
    }
    #[inline]
    pub fn grab_mods(&self) -> bool {
        self.inner & (1 << 9) != 0
    }
    #[inline]
    pub fn set_grab_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 9;
        } else {
            self.inner &= !(1 << 9);
        }
        self
    }
    #[inline]
    pub fn compat_grab_mods(&self) -> bool {
        self.inner & (1 << 10) != 0
    }
    #[inline]
    pub fn set_compat_grab_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 10;
        } else {
            self.inner &= !(1 << 10);
        }
        self
    }
    #[inline]
    pub fn lookup_mods(&self) -> bool {
        self.inner & (1 << 11) != 0
    }
    #[inline]
    pub fn set_lookup_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 11;
        } else {
            self.inner &= !(1 << 11);
        }
        self
    }
    #[inline]
    pub fn compat_lookup_mods(&self) -> bool {
        self.inner & (1 << 12) != 0
    }
    #[inline]
    pub fn set_compat_lookup_mods(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 12;
        } else {
            self.inner &= !(1 << 12);
        }
        self
    }
    #[inline]
    pub fn pointer_buttons(&self) -> bool {
        self.inner & (1 << 13) != 0
    }
    #[inline]
    pub fn set_pointer_buttons(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 13;
        } else {
            self.inner &= !(1 << 13);
        }
        self
    }
    #[inline]
    pub fn new(
        modifier_state: bool,
        modifier_base: bool,
        modifier_latch: bool,
        modifier_lock: bool,
        group_state: bool,
        group_base: bool,
        group_latch: bool,
        group_lock: bool,
        compat_state: bool,
        grab_mods: bool,
        compat_grab_mods: bool,
        lookup_mods: bool,
        compat_lookup_mods: bool,
        pointer_buttons: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if modifier_state {
            inner |= 1 << 0;
        }
        if modifier_base {
            inner |= 1 << 1;
        }
        if modifier_latch {
            inner |= 1 << 2;
        }
        if modifier_lock {
            inner |= 1 << 3;
        }
        if group_state {
            inner |= 1 << 4;
        }
        if group_base {
            inner |= 1 << 5;
        }
        if group_latch {
            inner |= 1 << 6;
        }
        if group_lock {
            inner |= 1 << 7;
        }
        if compat_state {
            inner |= 1 << 8;
        }
        if grab_mods {
            inner |= 1 << 9;
        }
        if compat_grab_mods {
            inner |= 1 << 10;
        }
        if lookup_mods {
            inner |= 1 << 11;
        }
        if compat_lookup_mods {
            inner |= 1 << 12;
        }
        if pointer_buttons {
            inner |= 1 << 13;
        }
        StatePart { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const MODIFIER_STATE: Self = Self { inner: 1 };
    pub const MODIFIER_BASE: Self = Self { inner: 2 };
    pub const MODIFIER_LATCH: Self = Self { inner: 4 };
    pub const MODIFIER_LOCK: Self = Self { inner: 8 };
    pub const GROUP_STATE: Self = Self { inner: 16 };
    pub const GROUP_BASE: Self = Self { inner: 32 };
    pub const GROUP_LATCH: Self = Self { inner: 64 };
    pub const GROUP_LOCK: Self = Self { inner: 128 };
    pub const COMPAT_STATE: Self = Self { inner: 256 };
    pub const GRAB_MODS: Self = Self { inner: 512 };
    pub const COMPAT_GRAB_MODS: Self = Self { inner: 1024 };
    pub const LOOKUP_MODS: Self = Self { inner: 2048 };
    pub const COMPAT_LOOKUP_MODS: Self = Self { inner: 4096 };
    pub const POINTER_BUTTONS: Self = Self { inner: 8192 };
    pub const COMPLETE: Self = Self { inner: 16383 };
}
impl AsByteSequence for StatePart {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((StatePart { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for StatePart {
    type Output = StatePart;
    #[inline]
    fn not(self) -> StatePart {
        StatePart { inner: !self.inner }
    }
}
impl core::ops::BitAnd for StatePart {
    type Output = StatePart;
    #[inline]
    fn bitand(self, rhs: StatePart) -> StatePart {
        StatePart {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for StatePart {
    type Output = StatePart;
    #[inline]
    fn bitor(self, rhs: StatePart) -> StatePart {
        StatePart {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for StatePart {
    type Output = StatePart;
    #[inline]
    fn bitxor(self, rhs: StatePart) -> StatePart {
        StatePart {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BellClassResult {
    KbdFeedbackClass = 0,
    BellFeedbackClass = 5,
}
impl AsByteSequence for BellClassResult {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u8).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u8, usize) = <u8>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::KbdFeedbackClass, sz)),
            5 => Some((Self::BellFeedbackClass, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u8>()
    }
}
impl Default for BellClassResult {
    #[inline]
    fn default() -> BellClassResult {
        BellClassResult::KbdFeedbackClass
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct AxnDetail {
    pub inner: u16,
}
impl AxnDetail {
    #[inline]
    pub fn sk_press(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_sk_press(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn sk_accept(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_sk_accept(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn sk_reject(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_sk_reject(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn sk_release(&self) -> bool {
        self.inner & (1 << 3) != 0
    }
    #[inline]
    pub fn set_sk_release(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 3;
        } else {
            self.inner &= !(1 << 3);
        }
        self
    }
    #[inline]
    pub fn bk_accept(&self) -> bool {
        self.inner & (1 << 4) != 0
    }
    #[inline]
    pub fn set_bk_accept(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 4;
        } else {
            self.inner &= !(1 << 4);
        }
        self
    }
    #[inline]
    pub fn bk_reject(&self) -> bool {
        self.inner & (1 << 5) != 0
    }
    #[inline]
    pub fn set_bk_reject(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 5;
        } else {
            self.inner &= !(1 << 5);
        }
        self
    }
    #[inline]
    pub fn axk_warning(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_axk_warning(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn new(
        sk_press: bool,
        sk_accept: bool,
        sk_reject: bool,
        sk_release: bool,
        bk_accept: bool,
        bk_reject: bool,
        axk_warning: bool,
    ) -> Self {
        let mut inner: u16 = 0;
        if sk_press {
            inner |= 1 << 0;
        }
        if sk_accept {
            inner |= 1 << 1;
        }
        if sk_reject {
            inner |= 1 << 2;
        }
        if sk_release {
            inner |= 1 << 3;
        }
        if bk_accept {
            inner |= 1 << 4;
        }
        if bk_reject {
            inner |= 1 << 5;
        }
        if axk_warning {
            inner |= 1 << 6;
        }
        AxnDetail { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const SK_PRESS: Self = Self { inner: 1 };
    pub const SK_ACCEPT: Self = Self { inner: 2 };
    pub const SK_REJECT: Self = Self { inner: 4 };
    pub const SK_RELEASE: Self = Self { inner: 8 };
    pub const BK_ACCEPT: Self = Self { inner: 16 };
    pub const BK_REJECT: Self = Self { inner: 32 };
    pub const AXK_WARNING: Self = Self { inner: 64 };
    pub const COMPLETE: Self = Self { inner: 127 };
}
impl AsByteSequence for AxnDetail {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        Some((AxnDetail { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for AxnDetail {
    type Output = AxnDetail;
    #[inline]
    fn not(self) -> AxnDetail {
        AxnDetail { inner: !self.inner }
    }
}
impl core::ops::BitAnd for AxnDetail {
    type Output = AxnDetail;
    #[inline]
    fn bitand(self, rhs: AxnDetail) -> AxnDetail {
        AxnDetail {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for AxnDetail {
    type Output = AxnDetail;
    #[inline]
    fn bitor(self, rhs: AxnDetail) -> AxnDetail {
        AxnDetail {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for AxnDetail {
    type Output = AxnDetail;
    #[inline]
    fn bitxor(self, rhs: AxnDetail) -> AxnDetail {
        AxnDetail {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(u16)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum LedClassResult {
    KbdFeedbackClass = 0,
    LedFeedbackClass = 4,
}
impl AsByteSequence for LedClassResult {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as u16).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (u16, usize) = <u16>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::KbdFeedbackClass, sz)),
            4 => Some((Self::LedFeedbackClass, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<u16>()
    }
}
impl Default for LedClassResult {
    #[inline]
    fn default() -> LedClassResult {
        LedClassResult::KbdFeedbackClass
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SymInterpMatch {
    pub inner: i32,
}
impl SymInterpMatch {
    #[inline]
    pub fn level_one_only(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_level_one_only(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(level_one_only: bool) -> Self {
        let mut inner: i32 = 0;
        if level_one_only {
            inner |= 1 << 7;
        }
        SymInterpMatch { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const LEVEL_ONE_ONLY: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 128 };
}
impl AsByteSequence for SymInterpMatch {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((SymInterpMatch { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SymInterpMatch {
    type Output = SymInterpMatch;
    #[inline]
    fn not(self) -> SymInterpMatch {
        SymInterpMatch { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SymInterpMatch {
    type Output = SymInterpMatch;
    #[inline]
    fn bitand(self, rhs: SymInterpMatch) -> SymInterpMatch {
        SymInterpMatch {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SymInterpMatch {
    type Output = SymInterpMatch;
    #[inline]
    fn bitor(self, rhs: SymInterpMatch) -> SymInterpMatch {
        SymInterpMatch {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SymInterpMatch {
    type Output = SymInterpMatch;
    #[inline]
    fn bitxor(self, rhs: SymInterpMatch) -> SymInterpMatch {
        SymInterpMatch {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct GroupsWrap {
    pub inner: i32,
}
impl GroupsWrap {
    #[inline]
    pub fn clamp_into_range(&self) -> bool {
        self.inner & (1 << 6) != 0
    }
    #[inline]
    pub fn set_clamp_into_range(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 6;
        } else {
            self.inner &= !(1 << 6);
        }
        self
    }
    #[inline]
    pub fn redirect_into_range(&self) -> bool {
        self.inner & (1 << 7) != 0
    }
    #[inline]
    pub fn set_redirect_into_range(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 7;
        } else {
            self.inner &= !(1 << 7);
        }
        self
    }
    #[inline]
    pub fn new(clamp_into_range: bool, redirect_into_range: bool) -> Self {
        let mut inner: i32 = 0;
        if clamp_into_range {
            inner |= 1 << 6;
        }
        if redirect_into_range {
            inner |= 1 << 7;
        }
        GroupsWrap { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const CLAMP_INTO_RANGE: Self = Self { inner: 64 };
    pub const REDIRECT_INTO_RANGE: Self = Self { inner: 128 };
    pub const COMPLETE: Self = Self { inner: 192 };
}
impl AsByteSequence for GroupsWrap {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((GroupsWrap { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for GroupsWrap {
    type Output = GroupsWrap;
    #[inline]
    fn not(self) -> GroupsWrap {
        GroupsWrap { inner: !self.inner }
    }
}
impl core::ops::BitAnd for GroupsWrap {
    type Output = GroupsWrap;
    #[inline]
    fn bitand(self, rhs: GroupsWrap) -> GroupsWrap {
        GroupsWrap {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for GroupsWrap {
    type Output = GroupsWrap;
    #[inline]
    fn bitor(self, rhs: GroupsWrap) -> GroupsWrap {
        GroupsWrap {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for GroupsWrap {
    type Output = GroupsWrap;
    #[inline]
    fn bitxor(self, rhs: GroupsWrap) -> GroupsWrap {
        GroupsWrap {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BellClass {
    KbdFeedbackClass = 0,
    BellFeedbackClass = 5,
    DfltXiClass = 768,
}
impl AsByteSequence for BellClass {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::KbdFeedbackClass, sz)),
            5 => Some((Self::BellFeedbackClass, sz)),
            768 => Some((Self::DfltXiClass, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for BellClass {
    #[inline]
    fn default() -> BellClass {
        BellClass::KbdFeedbackClass
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Groups {
    Any = 254,
    All = 255,
}
impl AsByteSequence for Groups {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            254 => Some((Self::Any, sz)),
            255 => Some((Self::All, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Groups {
    #[inline]
    fn default() -> Groups {
        Groups::Any
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum DoodadType {
    Outline = 1,
    Solid = 2,
    Text = 3,
    Indicator = 4,
    Logo = 5,
}
impl AsByteSequence for DoodadType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            1 => Some((Self::Outline, sz)),
            2 => Some((Self::Solid, sz)),
            3 => Some((Self::Text, sz)),
            4 => Some((Self::Indicator, sz)),
            5 => Some((Self::Logo, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for DoodadType {
    #[inline]
    fn default() -> DoodadType {
        DoodadType::Outline
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct CmDetail {
    pub inner: i32,
}
impl CmDetail {
    #[inline]
    pub fn sym_interp(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_sym_interp(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn group_compat(&self) -> bool {
        self.inner & (1 << 1) != 0
    }
    #[inline]
    pub fn set_group_compat(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 1;
        } else {
            self.inner &= !(1 << 1);
        }
        self
    }
    #[inline]
    pub fn new(sym_interp: bool, group_compat: bool) -> Self {
        let mut inner: i32 = 0;
        if sym_interp {
            inner |= 1 << 0;
        }
        if group_compat {
            inner |= 1 << 1;
        }
        CmDetail { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const SYM_INTERP: Self = Self { inner: 1 };
    pub const GROUP_COMPAT: Self = Self { inner: 2 };
    pub const COMPLETE: Self = Self { inner: 3 };
}
impl AsByteSequence for CmDetail {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((CmDetail { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for CmDetail {
    type Output = CmDetail;
    #[inline]
    fn not(self) -> CmDetail {
        CmDetail { inner: !self.inner }
    }
}
impl core::ops::BitAnd for CmDetail {
    type Output = CmDetail;
    #[inline]
    fn bitand(self, rhs: CmDetail) -> CmDetail {
        CmDetail {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for CmDetail {
    type Output = CmDetail;
    #[inline]
    fn bitor(self, rhs: CmDetail) -> CmDetail {
        CmDetail {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for CmDetail {
    type Output = CmDetail;
    #[inline]
    fn bitxor(self, rhs: CmDetail) -> CmDetail {
        CmDetail {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    BadId = 253,
    BadClass = 254,
    BadDevice = 255,
}
impl AsByteSequence for Error {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            253 => Some((Self::BadId, sz)),
            254 => Some((Self::BadClass, sz)),
            255 => Some((Self::BadDevice, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Error {
    #[inline]
    fn default() -> Error {
        Error::BadId
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Const {
    KeyNameLength = 4,
    PerKeyBitArraySize = 32,
    MaxLegalKeyCode = 255,
}
impl AsByteSequence for Const {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            4 => Some((Self::KeyNameLength, sz)),
            32 => Some((Self::PerKeyBitArraySize, sz)),
            255 => Some((Self::MaxLegalKeyCode, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for Const {
    #[inline]
    fn default() -> Const {
        Const::KeyNameLength
    }
}
#[repr(i32)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum BehaviorType {
    Default = 0,
    Lock = 1,
    RadioGroup = 2,
    Overlay1 = 3,
    Overlay2 = 4,
    PermamentLock = 129,
    PermamentRadioGroup = 130,
    PermamentOverlay1 = 131,
    PermamentOverlay2 = 132,
}
impl AsByteSequence for BehaviorType {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        (*self as i32).as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (underlying, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        match underlying {
            0 => Some((Self::Default, sz)),
            1 => Some((Self::Lock, sz)),
            2 => Some((Self::RadioGroup, sz)),
            3 => Some((Self::Overlay1, sz)),
            4 => Some((Self::Overlay2, sz)),
            129 => Some((Self::PermamentLock, sz)),
            130 => Some((Self::PermamentRadioGroup, sz)),
            131 => Some((Self::PermamentOverlay1, sz)),
            132 => Some((Self::PermamentOverlay2, sz)),
            _ => None,
        }
    }
    #[inline]
    fn size(&self) -> usize {
        ::core::mem::size_of::<i32>()
    }
}
impl Default for BehaviorType {
    #[inline]
    fn default() -> BehaviorType {
        BehaviorType::Default
    }
}
#[repr(transparent)]
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct SwitchScreenFlag {
    pub inner: i32,
}
impl SwitchScreenFlag {
    #[inline]
    pub fn application(&self) -> bool {
        self.inner & (1 << 0) != 0
    }
    #[inline]
    pub fn set_application(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 0;
        } else {
            self.inner &= !(1 << 0);
        }
        self
    }
    #[inline]
    pub fn absolute(&self) -> bool {
        self.inner & (1 << 2) != 0
    }
    #[inline]
    pub fn set_absolute(&mut self, val: bool) -> &mut Self {
        if val {
            self.inner |= 1 << 2;
        } else {
            self.inner &= !(1 << 2);
        }
        self
    }
    #[inline]
    pub fn new(application: bool, absolute: bool) -> Self {
        let mut inner: i32 = 0;
        if application {
            inner |= 1 << 0;
        }
        if absolute {
            inner |= 1 << 2;
        }
        SwitchScreenFlag { inner: inner }
    }
    #[inline]
    pub fn count_ones(&self) -> usize {
        self.inner.count_ones() as usize
    }
    pub const APPLICATION: Self = Self { inner: 1 };
    pub const ABSOLUTE: Self = Self { inner: 4 };
    pub const COMPLETE: Self = Self { inner: 5 };
}
impl AsByteSequence for SwitchScreenFlag {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        self.inner.as_bytes(bytes)
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let (inner, sz): (i32, usize) = <i32>::from_bytes(bytes)?;
        Some((SwitchScreenFlag { inner: inner }, sz))
    }
    #[inline]
    fn size(&self) -> usize {
        self.inner.size()
    }
}
impl core::ops::Not for SwitchScreenFlag {
    type Output = SwitchScreenFlag;
    #[inline]
    fn not(self) -> SwitchScreenFlag {
        SwitchScreenFlag { inner: !self.inner }
    }
}
impl core::ops::BitAnd for SwitchScreenFlag {
    type Output = SwitchScreenFlag;
    #[inline]
    fn bitand(self, rhs: SwitchScreenFlag) -> SwitchScreenFlag {
        SwitchScreenFlag {
            inner: self.inner & rhs.inner,
        }
    }
}
impl core::ops::BitOr for SwitchScreenFlag {
    type Output = SwitchScreenFlag;
    #[inline]
    fn bitor(self, rhs: SwitchScreenFlag) -> SwitchScreenFlag {
        SwitchScreenFlag {
            inner: self.inner | rhs.inner,
        }
    }
}
impl core::ops::BitXor for SwitchScreenFlag {
    type Output = SwitchScreenFlag;
    #[inline]
    fn bitxor(self, rhs: SwitchScreenFlag) -> SwitchScreenFlag {
        SwitchScreenFlag {
            inner: self.inner ^ rhs.inner,
        }
    }
}
#[derive(Clone, Debug, Default)]
pub struct KeyboardError {
    pub _error_type: u8,
    pub error_code: u8,
    pub major_code: u8,
    pub minor_code: u8,
    pub sequence: u16,
    pub value: Card32,
    pub minor_opcode: Card16,
    pub major_opcode: Card8,
}
impl KeyboardError {}
impl AsByteSequence for KeyboardError {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self._error_type.as_bytes(&mut bytes[index..]);
        index += self.error_code.as_bytes(&mut bytes[index..]);
        index += self.major_code.as_bytes(&mut bytes[index..]);
        index += self.minor_code.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.value.as_bytes(&mut bytes[index..]);
        index += self.minor_opcode.as_bytes(&mut bytes[index..]);
        index += self.major_opcode.as_bytes(&mut bytes[index..]);
        index += 21;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing KeyboardError from byte buffer");
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
        let (value, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (minor_opcode, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (major_opcode, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 21;
        Some((
            KeyboardError {
                _error_type: _error_type,
                error_code: error_code,
                major_code: major_code,
                minor_code: minor_code,
                sequence: sequence,
                value: value,
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
            + self.value.size()
            + self.minor_opcode.size()
            + self.major_opcode.size()
            + 21
    }
}
impl crate::auto::Error for KeyboardError {
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default)]
pub struct AccessXNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub keycode: Keycode,
    pub detailt: AxnDetail,
    pub slow_keys_delay: Card16,
    pub debounce_delay: Card16,
}
impl AccessXNotifyEvent {}
impl AsByteSequence for AccessXNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += self.detailt.as_bytes(&mut bytes[index..]);
        index += self.slow_keys_delay.as_bytes(&mut bytes[index..]);
        index += self.debounce_delay.as_bytes(&mut bytes[index..]);
        index += 16;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing AccessXNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (detailt, sz): (AxnDetail, usize) = <AxnDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (slow_keys_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (debounce_delay, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        Some((
            AccessXNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                keycode: keycode,
                detailt: detailt,
                slow_keys_delay: slow_keys_delay,
                debounce_delay: debounce_delay,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.keycode.size()
            + self.detailt.size()
            + self.slow_keys_delay.size()
            + self.debounce_delay.size()
            + 16
    }
}
impl crate::auto::Event for AccessXNotifyEvent {
    const OPCODE: u8 = 10;
}
#[derive(Clone, Debug, Default)]
pub struct ActionMessageEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub keycode: Keycode,
    pub press: bool,
    pub key_event_follows: bool,
    pub mods: ModMask,
    pub group: Group,
    pub message: [String8; 8],
}
impl ActionMessageEvent {}
impl AsByteSequence for ActionMessageEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += self.press.as_bytes(&mut bytes[index..]);
        index += self.key_event_follows.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        index += self.message.as_bytes(&mut bytes[index..]);
        index += 10;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ActionMessageEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (press, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (key_event_follows, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (Group, usize) = <Group>::from_bytes(&bytes[index..])?;
        index += sz;
        let (message, sz): ([String8; 8], usize) = <[String8; 8]>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 10;
        Some((
            ActionMessageEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                keycode: keycode,
                press: press,
                key_event_follows: key_event_follows,
                mods: mods,
                group: group,
                message: message,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.keycode.size()
            + self.press.size()
            + self.key_event_follows.size()
            + self.mods.size()
            + self.group.size()
            + self.message.size()
            + 10
    }
}
impl crate::auto::Event for ActionMessageEvent {
    const OPCODE: u8 = 9;
}
#[derive(Clone, Debug, Default)]
pub struct CompatMapNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub changed_groups: SetOfGroup,
    pub first_si: Card16,
    pub n_si: Card16,
    pub n_total_si: Card16,
}
impl CompatMapNotifyEvent {}
impl AsByteSequence for CompatMapNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.changed_groups.as_bytes(&mut bytes[index..]);
        index += self.first_si.as_bytes(&mut bytes[index..]);
        index += self.n_si.as_bytes(&mut bytes[index..]);
        index += self.n_total_si.as_bytes(&mut bytes[index..]);
        index += 16;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing CompatMapNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (changed_groups, sz): (SetOfGroup, usize) = <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_total_si, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 16;
        Some((
            CompatMapNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                changed_groups: changed_groups,
                first_si: first_si,
                n_si: n_si,
                n_total_si: n_total_si,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.changed_groups.size()
            + self.first_si.size()
            + self.n_si.size()
            + self.n_total_si.size()
            + 16
    }
}
impl crate::auto::Event for CompatMapNotifyEvent {
    const OPCODE: u8 = 7;
}
#[derive(Clone, Debug, Default)]
pub struct IndicatorStateNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub state: Card32,
    pub state_changed: Card32,
}
impl IndicatorStateNotifyEvent {}
impl AsByteSequence for IndicatorStateNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.state_changed.as_bytes(&mut bytes[index..]);
        index += 12;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IndicatorStateNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (state, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (state_changed, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        Some((
            IndicatorStateNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                state: state,
                state_changed: state_changed,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + 3
            + self.state.size()
            + self.state_changed.size()
            + 12
    }
}
impl crate::auto::Event for IndicatorStateNotifyEvent {
    const OPCODE: u8 = 4;
}
#[derive(Clone, Debug, Default)]
pub struct NamesNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub changed: NameDetail,
    pub first_type: Card8,
    pub n_types: Card8,
    pub first_level_name: Card8,
    pub n_level_names: Card8,
    pub n_radio_groups: Card8,
    pub n_key_aliases: Card8,
    pub changed_group_names: SetOfGroup,
    pub changed_virtual_mods: VMod,
    pub first_key: Keycode,
    pub n_keys: Card8,
    pub changed_indicators: Card32,
}
impl NamesNotifyEvent {}
impl AsByteSequence for NamesNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.changed.as_bytes(&mut bytes[index..]);
        index += self.first_type.as_bytes(&mut bytes[index..]);
        index += self.n_types.as_bytes(&mut bytes[index..]);
        index += self.first_level_name.as_bytes(&mut bytes[index..]);
        index += self.n_level_names.as_bytes(&mut bytes[index..]);
        index += self.n_radio_groups.as_bytes(&mut bytes[index..]);
        index += self.n_key_aliases.as_bytes(&mut bytes[index..]);
        index += self.changed_group_names.as_bytes(&mut bytes[index..]);
        index += self.changed_virtual_mods.as_bytes(&mut bytes[index..]);
        index += self.first_key.as_bytes(&mut bytes[index..]);
        index += self.n_keys.as_bytes(&mut bytes[index..]);
        index += self.changed_indicators.as_bytes(&mut bytes[index..]);
        index += 4;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NamesNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (changed, sz): (NameDetail, usize) = <NameDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_level_name, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_level_names, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_radio_groups, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_aliases, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (changed_group_names, sz): (SetOfGroup, usize) =
            <SetOfGroup>::from_bytes(&bytes[index..])?;
        index += sz;
        let (changed_virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (changed_indicators, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        Some((
            NamesNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                changed: changed,
                first_type: first_type,
                n_types: n_types,
                first_level_name: first_level_name,
                n_level_names: n_level_names,
                n_radio_groups: n_radio_groups,
                n_key_aliases: n_key_aliases,
                changed_group_names: changed_group_names,
                changed_virtual_mods: changed_virtual_mods,
                first_key: first_key,
                n_keys: n_keys,
                changed_indicators: changed_indicators,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + 1
            + self.changed.size()
            + self.first_type.size()
            + self.n_types.size()
            + self.first_level_name.size()
            + self.n_level_names.size()
            + self.n_radio_groups.size()
            + self.n_key_aliases.size()
            + self.changed_group_names.size()
            + self.changed_virtual_mods.size()
            + self.first_key.size()
            + self.n_keys.size()
            + self.changed_indicators.size()
            + 4
    }
}
impl crate::auto::Event for NamesNotifyEvent {
    const OPCODE: u8 = 6;
}
#[derive(Clone, Debug, Default)]
pub struct StateNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub mods: ModMask,
    pub base_mods: ModMask,
    pub latched_mods: ModMask,
    pub locked_mods: ModMask,
    pub group: Group,
    pub base_group: Int16,
    pub latched_group: Int16,
    pub locked_group: Group,
    pub compat_state: ModMask,
    pub grab_mods: ModMask,
    pub compat_grab_mods: ModMask,
    pub lookup_mods: ModMask,
    pub compat_loockup_mods: ModMask,
    pub ptr_btn_state: KeyButMask,
    pub changed: StatePart,
    pub keycode: Keycode,
    pub event_type_: Card8,
    pub request_major: Card8,
    pub request_minor: Card8,
}
impl StateNotifyEvent {}
impl AsByteSequence for StateNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.mods.as_bytes(&mut bytes[index..]);
        index += self.base_mods.as_bytes(&mut bytes[index..]);
        index += self.latched_mods.as_bytes(&mut bytes[index..]);
        index += self.locked_mods.as_bytes(&mut bytes[index..]);
        index += self.group.as_bytes(&mut bytes[index..]);
        index += self.base_group.as_bytes(&mut bytes[index..]);
        index += self.latched_group.as_bytes(&mut bytes[index..]);
        index += self.locked_group.as_bytes(&mut bytes[index..]);
        index += self.compat_state.as_bytes(&mut bytes[index..]);
        index += self.grab_mods.as_bytes(&mut bytes[index..]);
        index += self.compat_grab_mods.as_bytes(&mut bytes[index..]);
        index += self.lookup_mods.as_bytes(&mut bytes[index..]);
        index += self.compat_loockup_mods.as_bytes(&mut bytes[index..]);
        index += self.ptr_btn_state.as_bytes(&mut bytes[index..]);
        index += self.changed.as_bytes(&mut bytes[index..]);
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += self.event_type_.as_bytes(&mut bytes[index..]);
        index += self.request_major.as_bytes(&mut bytes[index..]);
        index += self.request_minor.as_bytes(&mut bytes[index..]);
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing StateNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (base_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (latched_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (locked_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (group, sz): (Group, usize) = <Group>::from_bytes(&bytes[index..])?;
        index += sz;
        let (base_group, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (latched_group, sz): (Int16, usize) = <Int16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (locked_group, sz): (Group, usize) = <Group>::from_bytes(&bytes[index..])?;
        index += sz;
        let (compat_state, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (grab_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (compat_grab_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (lookup_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (compat_loockup_mods, sz): (ModMask, usize) = <ModMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ptr_btn_state, sz): (KeyButMask, usize) = <KeyButMask>::from_bytes(&bytes[index..])?;
        index += sz;
        let (changed, sz): (StatePart, usize) = <StatePart>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_type_, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_major, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_minor, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        Some((
            StateNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                mods: mods,
                base_mods: base_mods,
                latched_mods: latched_mods,
                locked_mods: locked_mods,
                group: group,
                base_group: base_group,
                latched_group: latched_group,
                locked_group: locked_group,
                compat_state: compat_state,
                grab_mods: grab_mods,
                compat_grab_mods: compat_grab_mods,
                lookup_mods: lookup_mods,
                compat_loockup_mods: compat_loockup_mods,
                ptr_btn_state: ptr_btn_state,
                changed: changed,
                keycode: keycode,
                event_type_: event_type_,
                request_major: request_major,
                request_minor: request_minor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.mods.size()
            + self.base_mods.size()
            + self.latched_mods.size()
            + self.locked_mods.size()
            + self.group.size()
            + self.base_group.size()
            + self.latched_group.size()
            + self.locked_group.size()
            + self.compat_state.size()
            + self.grab_mods.size()
            + self.compat_grab_mods.size()
            + self.lookup_mods.size()
            + self.compat_loockup_mods.size()
            + self.ptr_btn_state.size()
            + self.changed.size()
            + self.keycode.size()
            + self.event_type_.size()
            + self.request_major.size()
            + self.request_minor.size()
    }
}
impl crate::auto::Event for StateNotifyEvent {
    const OPCODE: u8 = 2;
}
#[derive(Clone, Debug, Default)]
pub struct ControlsNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub num_groups: Card8,
    pub changed_controls: Control,
    pub enabled_controls: BoolCtrl,
    pub enabled_control_changes: BoolCtrl,
    pub keycode: Keycode,
    pub event_type_: Card8,
    pub request_major: Card8,
    pub request_minor: Card8,
}
impl ControlsNotifyEvent {}
impl AsByteSequence for ControlsNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.num_groups.as_bytes(&mut bytes[index..]);
        index += 2;
        index += self.changed_controls.as_bytes(&mut bytes[index..]);
        index += self.enabled_controls.as_bytes(&mut bytes[index..]);
        index += self.enabled_control_changes.as_bytes(&mut bytes[index..]);
        index += self.keycode.as_bytes(&mut bytes[index..]);
        index += self.event_type_.as_bytes(&mut bytes[index..]);
        index += self.request_major.as_bytes(&mut bytes[index..]);
        index += self.request_minor.as_bytes(&mut bytes[index..]);
        index += 4;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ControlsNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (num_groups, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        let (changed_controls, sz): (Control, usize) = <Control>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled_controls, sz): (BoolCtrl, usize) = <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (enabled_control_changes, sz): (BoolCtrl, usize) =
            <BoolCtrl>::from_bytes(&bytes[index..])?;
        index += sz;
        let (keycode, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_type_, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_major, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_minor, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 4;
        Some((
            ControlsNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                num_groups: num_groups,
                changed_controls: changed_controls,
                enabled_controls: enabled_controls,
                enabled_control_changes: enabled_control_changes,
                keycode: keycode,
                event_type_: event_type_,
                request_major: request_major,
                request_minor: request_minor,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.num_groups.size()
            + 2
            + self.changed_controls.size()
            + self.enabled_controls.size()
            + self.enabled_control_changes.size()
            + self.keycode.size()
            + self.event_type_.size()
            + self.request_major.size()
            + self.request_minor.size()
            + 4
    }
}
impl crate::auto::Event for ControlsNotifyEvent {
    const OPCODE: u8 = 3;
}
#[derive(Clone, Debug, Default)]
pub struct IndicatorMapNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub state: Card32,
    pub map_changed: Card32,
}
impl IndicatorMapNotifyEvent {}
impl AsByteSequence for IndicatorMapNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 3;
        index += self.state.as_bytes(&mut bytes[index..]);
        index += self.map_changed.as_bytes(&mut bytes[index..]);
        index += 12;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing IndicatorMapNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 3;
        let (state, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (map_changed, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 12;
        Some((
            IndicatorMapNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                state: state,
                map_changed: map_changed,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + 3
            + self.state.size()
            + self.map_changed.size()
            + 12
    }
}
impl crate::auto::Event for IndicatorMapNotifyEvent {
    const OPCODE: u8 = 5;
}
#[derive(Clone, Debug, Default)]
pub struct MapNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub ptr_btn_actions: Card8,
    pub changed: MapPart,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub first_type: Card8,
    pub n_types: Card8,
    pub first_key_sym: Keycode,
    pub n_key_syms: Card8,
    pub first_key_act: Keycode,
    pub n_key_acts: Card8,
    pub first_key_behavior: Keycode,
    pub n_key_behavior: Card8,
    pub first_key_explicit: Keycode,
    pub n_key_explicit: Card8,
    pub first_mod_map_key: Keycode,
    pub n_mod_map_keys: Card8,
    pub first_v_mod_map_key: Keycode,
    pub n_v_mod_map_keys: Card8,
    pub virtual_mods: VMod,
}
impl MapNotifyEvent {}
impl AsByteSequence for MapNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.ptr_btn_actions.as_bytes(&mut bytes[index..]);
        index += self.changed.as_bytes(&mut bytes[index..]);
        index += self.min_key_code.as_bytes(&mut bytes[index..]);
        index += self.max_key_code.as_bytes(&mut bytes[index..]);
        index += self.first_type.as_bytes(&mut bytes[index..]);
        index += self.n_types.as_bytes(&mut bytes[index..]);
        index += self.first_key_sym.as_bytes(&mut bytes[index..]);
        index += self.n_key_syms.as_bytes(&mut bytes[index..]);
        index += self.first_key_act.as_bytes(&mut bytes[index..]);
        index += self.n_key_acts.as_bytes(&mut bytes[index..]);
        index += self.first_key_behavior.as_bytes(&mut bytes[index..]);
        index += self.n_key_behavior.as_bytes(&mut bytes[index..]);
        index += self.first_key_explicit.as_bytes(&mut bytes[index..]);
        index += self.n_key_explicit.as_bytes(&mut bytes[index..]);
        index += self.first_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += self.first_v_mod_map_key.as_bytes(&mut bytes[index..]);
        index += self.n_v_mod_map_keys.as_bytes(&mut bytes[index..]);
        index += self.virtual_mods.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing MapNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (ptr_btn_actions, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (changed, sz): (MapPart, usize) = <MapPart>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_types, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_sym, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_syms, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_act, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_acts, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_behavior, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_behavior, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_key_explicit, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_key_explicit, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_v_mod_map_key, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_v_mod_map_keys, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (virtual_mods, sz): (VMod, usize) = <VMod>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            MapNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                ptr_btn_actions: ptr_btn_actions,
                changed: changed,
                min_key_code: min_key_code,
                max_key_code: max_key_code,
                first_type: first_type,
                n_types: n_types,
                first_key_sym: first_key_sym,
                n_key_syms: n_key_syms,
                first_key_act: first_key_act,
                n_key_acts: n_key_acts,
                first_key_behavior: first_key_behavior,
                n_key_behavior: n_key_behavior,
                first_key_explicit: first_key_explicit,
                n_key_explicit: n_key_explicit,
                first_mod_map_key: first_mod_map_key,
                n_mod_map_keys: n_mod_map_keys,
                first_v_mod_map_key: first_v_mod_map_key,
                n_v_mod_map_keys: n_v_mod_map_keys,
                virtual_mods: virtual_mods,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.ptr_btn_actions.size()
            + self.changed.size()
            + self.min_key_code.size()
            + self.max_key_code.size()
            + self.first_type.size()
            + self.n_types.size()
            + self.first_key_sym.size()
            + self.n_key_syms.size()
            + self.first_key_act.size()
            + self.n_key_acts.size()
            + self.first_key_behavior.size()
            + self.n_key_behavior.size()
            + self.first_key_explicit.size()
            + self.n_key_explicit.size()
            + self.first_mod_map_key.size()
            + self.n_mod_map_keys.size()
            + self.first_v_mod_map_key.size()
            + self.n_v_mod_map_keys.size()
            + self.virtual_mods.size()
            + 2
    }
}
impl crate::auto::Event for MapNotifyEvent {
    const OPCODE: u8 = 1;
}
#[derive(Clone, Debug, Default)]
pub struct ExtensionDeviceNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub reason: XiFeature,
    pub led_class: LedClassResult,
    pub led_id: Card16,
    pub leds_defined: Card32,
    pub led_state: Card32,
    pub first_button: Card8,
    pub n_buttons: Card8,
    pub supported: XiFeature,
    pub unsupported: XiFeature,
}
impl ExtensionDeviceNotifyEvent {}
impl AsByteSequence for ExtensionDeviceNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += 1;
        index += self.reason.as_bytes(&mut bytes[index..]);
        index += self.led_class.as_bytes(&mut bytes[index..]);
        index += self.led_id.as_bytes(&mut bytes[index..]);
        index += self.leds_defined.as_bytes(&mut bytes[index..]);
        index += self.led_state.as_bytes(&mut bytes[index..]);
        index += self.first_button.as_bytes(&mut bytes[index..]);
        index += self.n_buttons.as_bytes(&mut bytes[index..]);
        index += self.supported.as_bytes(&mut bytes[index..]);
        index += self.unsupported.as_bytes(&mut bytes[index..]);
        index += 2;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing ExtensionDeviceNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 1;
        let (reason, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_class, sz): (LedClassResult, usize) =
            <LedClassResult>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_id, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (leds_defined, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (led_state, sz): (Card32, usize) = <Card32>::from_bytes(&bytes[index..])?;
        index += sz;
        let (first_button, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (n_buttons, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (supported, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        let (unsupported, sz): (XiFeature, usize) = <XiFeature>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 2;
        Some((
            ExtensionDeviceNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                reason: reason,
                led_class: led_class,
                led_id: led_id,
                leds_defined: leds_defined,
                led_state: led_state,
                first_button: first_button,
                n_buttons: n_buttons,
                supported: supported,
                unsupported: unsupported,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + 1
            + self.reason.size()
            + self.led_class.size()
            + self.led_id.size()
            + self.leds_defined.size()
            + self.led_state.size()
            + self.first_button.size()
            + self.n_buttons.size()
            + self.supported.size()
            + self.unsupported.size()
            + 2
    }
}
impl crate::auto::Event for ExtensionDeviceNotifyEvent {
    const OPCODE: u8 = 11;
}
#[derive(Clone, Debug, Default)]
pub struct NewKeyboardNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub old_device_id: Card8,
    pub min_key_code: Keycode,
    pub max_key_code: Keycode,
    pub old_min_key_code: Keycode,
    pub old_max_key_code: Keycode,
    pub request_major: Card8,
    pub request_minor: Card8,
    pub changed: NknDetail,
}
impl NewKeyboardNotifyEvent {}
impl AsByteSequence for NewKeyboardNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.old_device_id.as_bytes(&mut bytes[index..]);
        index += self.min_key_code.as_bytes(&mut bytes[index..]);
        index += self.max_key_code.as_bytes(&mut bytes[index..]);
        index += self.old_min_key_code.as_bytes(&mut bytes[index..]);
        index += self.old_max_key_code.as_bytes(&mut bytes[index..]);
        index += self.request_major.as_bytes(&mut bytes[index..]);
        index += self.request_minor.as_bytes(&mut bytes[index..]);
        index += self.changed.as_bytes(&mut bytes[index..]);
        index += 14;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing NewKeyboardNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (old_device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (old_min_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (old_max_key_code, sz): (Keycode, usize) = <Keycode>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_major, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (request_minor, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (changed, sz): (NknDetail, usize) = <NknDetail>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 14;
        Some((
            NewKeyboardNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                old_device_id: old_device_id,
                min_key_code: min_key_code,
                max_key_code: max_key_code,
                old_min_key_code: old_min_key_code,
                old_max_key_code: old_max_key_code,
                request_major: request_major,
                request_minor: request_minor,
                changed: changed,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.old_device_id.size()
            + self.min_key_code.size()
            + self.max_key_code.size()
            + self.old_min_key_code.size()
            + self.old_max_key_code.size()
            + self.request_major.size()
            + self.request_minor.size()
            + self.changed.size()
            + 14
    }
}
impl crate::auto::Event for NewKeyboardNotifyEvent {
    const OPCODE: u8 = 0;
}
#[derive(Clone, Debug, Default)]
pub struct BellNotifyEvent {
    pub event_type: u8,
    pub xkb_type: Card8,
    pub sequence: u16,
    pub time: Timestamp,
    pub device_id: Card8,
    pub bell_class: BellClassResult,
    pub bell_id: Card8,
    pub percent: Card8,
    pub pitch: Card16,
    pub duration: Card16,
    pub name: Atom,
    pub window: Window,
    pub event_only: bool,
}
impl BellNotifyEvent {}
impl AsByteSequence for BellNotifyEvent {
    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        let mut index: usize = 0;
        index += self.event_type.as_bytes(&mut bytes[index..]);
        index += self.xkb_type.as_bytes(&mut bytes[index..]);
        index += self.sequence.as_bytes(&mut bytes[index..]);
        index += self.time.as_bytes(&mut bytes[index..]);
        index += self.device_id.as_bytes(&mut bytes[index..]);
        index += self.bell_class.as_bytes(&mut bytes[index..]);
        index += self.bell_id.as_bytes(&mut bytes[index..]);
        index += self.percent.as_bytes(&mut bytes[index..]);
        index += self.pitch.as_bytes(&mut bytes[index..]);
        index += self.duration.as_bytes(&mut bytes[index..]);
        index += self.name.as_bytes(&mut bytes[index..]);
        index += self.window.as_bytes(&mut bytes[index..]);
        index += self.event_only.as_bytes(&mut bytes[index..]);
        index += 7;
        index
    }
    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let mut index: usize = 0;
        log::trace!("Deserializing BellNotifyEvent from byte buffer");
        let (event_type, sz): (u8, usize) = <u8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (xkb_type, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (sequence, sz): (u16, usize) = <u16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (time, sz): (Timestamp, usize) = <Timestamp>::from_bytes(&bytes[index..])?;
        index += sz;
        let (device_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_class, sz): (BellClassResult, usize) =
            <BellClassResult>::from_bytes(&bytes[index..])?;
        index += sz;
        let (bell_id, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (percent, sz): (Card8, usize) = <Card8>::from_bytes(&bytes[index..])?;
        index += sz;
        let (pitch, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (duration, sz): (Card16, usize) = <Card16>::from_bytes(&bytes[index..])?;
        index += sz;
        let (name, sz): (Atom, usize) = <Atom>::from_bytes(&bytes[index..])?;
        index += sz;
        let (window, sz): (Window, usize) = <Window>::from_bytes(&bytes[index..])?;
        index += sz;
        let (event_only, sz): (bool, usize) = <bool>::from_bytes(&bytes[index..])?;
        index += sz;
        index += 7;
        Some((
            BellNotifyEvent {
                event_type: event_type,
                xkb_type: xkb_type,
                sequence: sequence,
                time: time,
                device_id: device_id,
                bell_class: bell_class,
                bell_id: bell_id,
                percent: percent,
                pitch: pitch,
                duration: duration,
                name: name,
                window: window,
                event_only: event_only,
            },
            index,
        ))
    }
    #[inline]
    fn size(&self) -> usize {
        self.event_type.size()
            + self.xkb_type.size()
            + self.sequence.size()
            + self.time.size()
            + self.device_id.size()
            + self.bell_class.size()
            + self.bell_id.size()
            + self.percent.size()
            + self.pitch.size()
            + self.duration.size()
            + self.name.size()
            + self.window.size()
            + self.event_only.size()
            + 7
    }
}
impl crate::auto::Event for BellNotifyEvent {
    const OPCODE: u8 = 8;
}
