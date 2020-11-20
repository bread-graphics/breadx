// MIT/Apache2 License

use crate::auto::{
    xproto::{ClientMessageEvent, ConfigureNotifyEvent},
    AsByteSequence, Event as AutoEvent,
};
use core::fmt;
use tinyvec::TinyVec;

const OPCODE_MASK: u8 = !0x80;

#[derive(Debug)]
pub enum Event {
    ConfigureNotify(ConfigureNotifyEvent),
    ClientMessage(ClientMessageEvent),
    NoneOfTheAbove {
        opcode: u8,
        bytes: TinyVec<[u8; 32]>,
    },
}

impl Event {
    #[inline]
    pub(crate) fn from_bytes(mut bytes: TinyVec<[u8; 32]>) -> crate::Result<Self> {
        let mut e = Event::NoneOfTheAbove {
            opcode: bytes[0] & OPCODE_MASK,
            bytes,
        };
        e.differentiate()?;
        Ok(e)
    }

    #[inline]
    pub(crate) fn differentiate(&mut self) -> crate::Result {
        if let Event::NoneOfTheAbove { opcode, ref bytes } = self {
            let opcode = *opcode;
            if opcode == ConfigureNotifyEvent::OPCODE {
                let cne = ConfigureNotifyEvent::from_bytes(bytes).ok_or_else(|| {
                    crate::BreadError::BadObjectRead(Some("ConfigureNotifyEvent"))
                })?;
                *self = Self::ConfigureNotify(cne.0);
            } else if opcode == ClientMessageEvent::OPCODE {
                let cme = ClientMessageEvent::from_bytes(bytes)
                    .ok_or_else(|| crate::BreadError::BadObjectRead(Some("ClientMessageEvent")))?;
                *self = Self::ClientMessage(cme.0);
            }
        }

        Ok(())
    }

    /// Get the opcode of this event.
    #[inline]
    pub fn opcode(&self) -> u8 {
        match self {
            Self::ConfigureNotify(_) => ConfigureNotifyEvent::OPCODE,
            Self::ClientMessage(_) => ClientMessageEvent::OPCODE,
            Self::NoneOfTheAbove { opcode, .. } => *opcode,
        }
    }
}
