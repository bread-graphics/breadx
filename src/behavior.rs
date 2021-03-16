// MIT/Apache2 License

#![cfg(feature = "xkb")]

use super::auto::{
    xkb::{
        BehaviorType, CommonBehavior, DefaultBehavior, LockBehavior, OverlayBehavior,
        RadioGroupBehavior,
    },
    AsByteSequence,
};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Behavior {
    Common(CommonBehavior),
    Default(DefaultBehavior),
    Lock(LockBehavior),
    RadioGroup(RadioGroupBehavior),
    Overlay1(OverlayBehavior),
    Overlay2(OverlayBehavior),
    PermamentLock(LockBehavior),
    PermamentRadioGroup(RadioGroupBehavior),
    PermamentOverlay1(OverlayBehavior),
    PermamentOverlay2(OverlayBehavior),
}

impl Default for Behavior {
    #[inline]
    fn default() -> Self {
        Self::Default(Default::default())
    }
}

const BEHAVIOR_SIZE: usize = 2;

impl AsByteSequence for Behavior {
    #[inline]
    fn size(&self) -> usize {
        BEHAVIOR_SIZE
    }

    #[inline]
    fn as_bytes(&self, bytes: &mut [u8]) -> usize {
        match self {
            Self::Common(cb) => cb.as_bytes(bytes),
            Self::Default(d) => d.as_bytes(bytes),
            Self::Lock(lb) => lb.as_bytes(bytes),
            Self::RadioGroup(rg) => rg.as_bytes(bytes),
            Self::Overlay1(o) => o.as_bytes(bytes),
            Self::Overlay2(o) => o.as_bytes(bytes),
            Self::PermamentLock(l) => l.as_bytes(bytes),
            Self::PermamentRadioGroup(rg) => rg.as_bytes(bytes),
            Self::PermamentOverlay1(o) => o.as_bytes(bytes),
            Self::PermamentOverlay2(o) => o.as_bytes(bytes),
        }
    }

    #[inline]
    fn from_bytes(bytes: &[u8]) -> Option<(Self, usize)> {
        let ty = BehaviorType::from_bytes(bytes);

        let res = match ty {
            None => Self::Common(CommonBehavior::from_bytes(bytes)?.0),
            Some((ty, _)) => match ty {
                BehaviorType::Default => Self::Default(DefaultBehavior::from_bytes(bytes)?.0),
                BehaviorType::Lock => Self::Lock(LockBehavior::from_bytes(bytes)?.0),
                BehaviorType::RadioGroup => {
                    Self::RadioGroup(RadioGroupBehavior::from_bytes(bytes)?.0)
                }
                BehaviorType::Overlay1 => Self::Overlay1(OverlayBehavior::from_bytes(bytes)?.0),
                BehaviorType::Overlay2 => Self::Overlay2(OverlayBehavior::from_bytes(bytes)?.0),
                BehaviorType::PermamentLock => {
                    Self::PermamentLock(LockBehavior::from_bytes(bytes)?.0)
                }
                BehaviorType::PermamentRadioGroup => {
                    Self::PermamentRadioGroup(RadioGroupBehavior::from_bytes(bytes)?.0)
                }
                BehaviorType::PermamentOverlay1 => {
                    Self::PermamentOverlay1(OverlayBehavior::from_bytes(bytes)?.0)
                }
                BehaviorType::PermamentOverlay2 => {
                    Self::PermamentOverlay2(OverlayBehavior::from_bytes(bytes)?.0)
                }
            },
        };

        Some((res, BEHAVIOR_SIZE))
    }
}
