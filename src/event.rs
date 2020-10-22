// MIT/Apache2 License

use crate::auto::xproto;
use core::fmt;
use tinyvec::TinyVec;

pub enum Event {
    ConfigureNotify(xproto::ConfigureNotifyEvent),
}

impl fmt::Debug for Event {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::ConfigureNotify(cn) => fmt::Debug::fmt(cn, f),
        }
    }
}

impl Event {
    #[inline]
    pub(crate) fn from_bytes(bytes: TinyVec<[u8; 32]>) -> crate::Result<Self> {
        unimplemented!()
    }
}

pub trait EventType {
    fn tycode() -> u32;
}
