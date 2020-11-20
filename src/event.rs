// MIT/Apache2 License

use crate::auto::xproto;
use core::fmt;
use tinyvec::TinyVec;

#[derive(Debug)]
pub enum Event {
    ConfigureNotify(xproto::ConfigureNotifyEvent),
    NoneOfTheAbove {
        opcode: u8,
        bytes: TinyVec<[u8; 32]>,
    },
}

impl Event {
    #[inline]
    pub(crate) fn from_bytes(mut bytes: TinyVec<[u8; 32]>) -> crate::Result<Self> {
        Ok(Event::NoneOfTheAbove {
            opcode: bytes[0],
            bytes,
        })
    }
}

macro_rules! gen_differentiate {
    () => {};
}
