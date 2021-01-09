// MIT/Apache2 License

//! This module provides the events used to drive the `breadx` event system.

#[cfg(feature = "input")]
pub mod input;

use crate::auto::{
    xproto::{
        ButtonPressEvent, ButtonReleaseEvent, CirculateNotifyEvent, CirculateRequestEvent,
        ClientMessageEvent, ConfigureNotifyEvent, ConfigureRequestEvent, CreateNotifyEvent,
        DestroyNotifyEvent, EnterNotifyEvent, ExposeEvent, FocusInEvent, FocusOutEvent,
        GraphicsExposureEvent, GravityNotifyEvent, KeyPressEvent, KeyReleaseEvent,
        KeymapNotifyEvent, LeaveNotifyEvent, MapNotifyEvent, MapRequestEvent, MappingNotifyEvent,
        NoExposureEvent, PropertyNotifyEvent, ReparentNotifyEvent, ResizeRequestEvent,
        SelectionClearEvent, SelectionNotifyEvent, SelectionRequestEvent, UnmapNotifyEvent,
        VisibilityNotifyEvent,
    },
    AsByteSequence, Event as AutoEvent,
};
use tinyvec::TinyVec;

const OPCODE_MASK: u8 = !0x80;

#[derive(Debug)]
pub enum Event {
    ConfigureNotify(ConfigureNotifyEvent),
    ClientMessage(ClientMessageEvent),
    Expose(ExposeEvent),
    ButtonPress(ButtonPressEvent),
    ButtonRelease(ButtonReleaseEvent),
    CirculateNotify(CirculateNotifyEvent),
    CirculateRequest(CirculateRequestEvent),
    ConfigureRequest(ConfigureRequestEvent),
    CreateNotify(CreateNotifyEvent),
    DestroyNotify(DestroyNotifyEvent),
    EnterNotify(EnterNotifyEvent),
    FocusIn(FocusInEvent),
    FocusOut(FocusOutEvent),
    GraphicsExposure(GraphicsExposureEvent),
    GravityNotify(GravityNotifyEvent),
    KeyPress(KeyPressEvent),
    KeyRelease(KeyReleaseEvent),
    KeymapNotify(KeymapNotifyEvent),
    LeaveNotify(LeaveNotifyEvent),
    MapNotify(MapNotifyEvent),
    MapRequest(MapRequestEvent),
    MappingNotify(MappingNotifyEvent),
    NoExposure(NoExposureEvent),
    PropertyNotify(PropertyNotifyEvent),
    ReparentNotify(ReparentNotifyEvent),
    ResizeRequest(ResizeRequestEvent),
    SelectionClear(SelectionClearEvent),
    SelectionNotify(SelectionNotifyEvent),
    SelectionRequest(SelectionRequestEvent),
    UnmapNotify(UnmapNotifyEvent),
    VisibilityNotify(VisibilityNotifyEvent),
    NoneOfTheAbove {
        opcode: u8,
        bytes: TinyVec<[u8; 32]>,
    },
}

impl Event {
    #[inline]
    pub(crate) fn from_bytes(bytes: TinyVec<[u8; 32]>) -> crate::Result<Self> {
        let mut e = Event::NoneOfTheAbove {
            opcode: bytes[0] & OPCODE_MASK,
            bytes,
        };
        e.differentiate()?;
        Ok(e)
    }

    #[inline]
    pub(crate) fn as_byte_slice(&self) -> Option<&[u8]> {
        match self {
            Self::NoneOfTheAbove { bytes, .. } => Some(&*bytes),
            _ => None,
        }
    }

    #[inline]
    pub(crate) fn as_bytes(&self, bytes: &mut [u8]) {
        match self {
            Self::ConfigureNotify(cne) => cne.as_bytes(bytes),
            Self::ClientMessage(cne) => cne.as_bytes(bytes),
            Self::Expose(ee) => ee.as_bytes(bytes),
            Self::ButtonPress(bpe) => bpe.as_bytes(bytes),
            Self::ButtonRelease(bre) => bre.as_bytes(bytes),
            Self::CirculateNotify(cne) => cne.as_bytes(bytes),
            Self::CirculateRequest(cre) => cre.as_bytes(bytes),
            Self::ConfigureRequest(cre) => cre.as_bytes(bytes),
            Self::CreateNotify(cne) => cne.as_bytes(bytes),
            Self::DestroyNotify(dne) => dne.as_bytes(bytes),
            Self::EnterNotify(ene) => ene.as_bytes(bytes),
            Self::FocusIn(fie) => fie.as_bytes(bytes),
            Self::FocusOut(foe) => foe.as_bytes(bytes),
            Self::GraphicsExposure(gee) => gee.as_bytes(bytes),
            Self::GravityNotify(gne) => gne.as_bytes(bytes),
            Self::KeyPress(kpe) => kpe.as_bytes(bytes),
            Self::KeyRelease(kre) => kre.as_bytes(bytes),
            Self::KeymapNotify(kne) => kne.as_bytes(bytes),
            Self::LeaveNotify(lne) => lne.as_bytes(bytes),
            Self::MapNotify(mne) => mne.as_bytes(bytes),
            Self::MapRequest(mre) => mre.as_bytes(bytes),
            Self::MappingNotify(mne) => mne.as_bytes(bytes),
            Self::NoExposure(nee) => nee.as_bytes(bytes),
            Self::PropertyNotify(pne) => pne.as_bytes(bytes),
            Self::ReparentNotify(rne) => rne.as_bytes(bytes),
            Self::ResizeRequest(rre) => rre.as_bytes(bytes),
            Self::SelectionClear(sce) => sce.as_bytes(bytes),
            Self::SelectionNotify(sne) => sne.as_bytes(bytes),
            Self::SelectionRequest(sre) => sre.as_bytes(bytes),
            Self::UnmapNotify(une) => une.as_bytes(bytes),
            Self::VisibilityNotify(vne) => vne.as_bytes(bytes),
            Self::NoneOfTheAbove { bytes: b, .. } => {
                (&mut bytes[0..b.len()]).copy_from_slice(b);
                0
            }
        };
    }

    #[allow(clippy::too_many_lines)]
    #[inline]
    pub(crate) fn differentiate(&mut self) -> crate::Result {
        if let Event::NoneOfTheAbove { opcode, ref bytes } = self {
            log::debug!("Differentiating event bytes: {}", bytes);

            let opcode = *opcode;
            if opcode == ConfigureNotifyEvent::OPCODE {
                let cne = ConfigureNotifyEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("ConfigureNotifyEvent")),
                )?;
                *self = Self::ConfigureNotify(cne.0);
            } else if opcode == ClientMessageEvent::OPCODE {
                let cme = ClientMessageEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("ClientMessageEvent")))?;
                *self = Self::ClientMessage(cme.0);
            } else if opcode == ExposeEvent::OPCODE {
                let ee = ExposeEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("ExposeEvent")))?;
                *self = Self::Expose(ee.0);
            } else if opcode == ButtonPressEvent::OPCODE {
                let e = ButtonPressEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("ButtonPressEvent")))?;
                *self = Self::ButtonPress(e.0);
            } else if opcode == ButtonReleaseEvent::OPCODE {
                let e = ButtonReleaseEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("ButtonReleaseEvent")))?;
                *self = Self::ButtonRelease(e.0);
            } else if opcode == CirculateNotifyEvent::OPCODE {
                let e = CirculateNotifyEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("CirculateNotifyEvent")),
                )?;
                *self = Self::CirculateNotify(e.0);
            } else if opcode == CirculateRequestEvent::OPCODE {
                let e = CirculateRequestEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("CirculateRequestEvent")),
                )?;
                *self = Self::CirculateRequest(e.0);
            } else if opcode == ConfigureRequestEvent::OPCODE {
                let e = ConfigureRequestEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("ConfigureRequestEvent")),
                )?;
                *self = Self::ConfigureRequest(e.0);
            } else if opcode == CreateNotifyEvent::OPCODE {
                let e = CreateNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("CreateNotifyEvent")))?;
                *self = Self::CreateNotify(e.0);
            } else if opcode == DestroyNotifyEvent::OPCODE {
                let e = DestroyNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("DestroyNotifyEvent")))?;
                *self = Self::DestroyNotify(e.0);
            } else if opcode == EnterNotifyEvent::OPCODE {
                let e = EnterNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("EnterNotifyEvent")))?;
                *self = Self::EnterNotify(e.0);
            } else if opcode == FocusInEvent::OPCODE {
                let e = FocusInEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("FocusInEvent")))?;
                *self = Self::FocusIn(e.0);
            } else if opcode == FocusOutEvent::OPCODE {
                let e = FocusOutEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("FocusOutEvent")))?;
                *self = Self::FocusOut(e.0);
            } else if opcode == GraphicsExposureEvent::OPCODE {
                let e = GraphicsExposureEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("GraphicsExposureEvent")),
                )?;
                *self = Self::GraphicsExposure(e.0);
            } else if opcode == GravityNotifyEvent::OPCODE {
                let e = GravityNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("GravityNotifyEvent")))?;
                *self = Self::GravityNotify(e.0);
            } else if opcode == KeyPressEvent::OPCODE {
                let e = KeyPressEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("KeyPressEvent")))?;
                *self = Self::KeyPress(e.0);
            } else if opcode == KeyReleaseEvent::OPCODE {
                let e = KeyReleaseEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("KeyReleaseEvent")))?;
                *self = Self::KeyRelease(e.0);
            } else if opcode == KeymapNotifyEvent::OPCODE {
                let e = KeymapNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("KeymapNotifyEvent")))?;
                *self = Self::KeymapNotify(e.0);
            } else if opcode == LeaveNotifyEvent::OPCODE {
                let e = LeaveNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("LeaveNotifyEvent")))?;
                *self = Self::LeaveNotify(e.0);
            } else if opcode == MapNotifyEvent::OPCODE {
                let e = MapNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("MapNotifyEvent")))?;
                *self = Self::MapNotify(e.0);
            } else if opcode == MapRequestEvent::OPCODE {
                let e = MapRequestEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("MapRequestEvent")))?;
                *self = Self::MapRequest(e.0);
            } else if opcode == MappingNotifyEvent::OPCODE {
                let e = MappingNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("MappingNotifyEvent")))?;
                *self = Self::MappingNotify(e.0);
            } else if opcode == NoExposureEvent::OPCODE {
                let e = NoExposureEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("NoExposureEvent")))?;
                *self = Self::NoExposure(e.0);
            } else if opcode == PropertyNotifyEvent::OPCODE {
                let e = PropertyNotifyEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("PropertyNotifyEvent")),
                )?;
                *self = Self::PropertyNotify(e.0);
            } else if opcode == ReparentNotifyEvent::OPCODE {
                let e = ReparentNotifyEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("ReparentNotifyEvent")),
                )?;
                *self = Self::ReparentNotify(e.0);
            } else if opcode == ResizeRequestEvent::OPCODE {
                let e = ResizeRequestEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("ResizeRequestEvent")))?;
                *self = Self::ResizeRequest(e.0);
            } else if opcode == SelectionClearEvent::OPCODE {
                let e = SelectionClearEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("SelectionClearEvent")),
                )?;
                *self = Self::SelectionClear(e.0);
            } else if opcode == SelectionNotifyEvent::OPCODE {
                let e = SelectionNotifyEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("SelectionNotifyEvent")),
                )?;
                *self = Self::SelectionNotify(e.0);
            } else if opcode == SelectionRequestEvent::OPCODE {
                let e = SelectionRequestEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("SelectionRequestEvent")),
                )?;
                *self = Self::SelectionRequest(e.0);
            } else if opcode == UnmapNotifyEvent::OPCODE {
                let e = UnmapNotifyEvent::from_bytes(bytes)
                    .ok_or(crate::BreadError::BadObjectRead(Some("UnmapNotifyEvent")))?;
                *self = Self::UnmapNotify(e.0);
            } else if opcode == VisibilityNotifyEvent::OPCODE {
                let e = VisibilityNotifyEvent::from_bytes(bytes).ok_or(
                    crate::BreadError::BadObjectRead(Some("VisibilityNotifyEvent")),
                )?;
                *self = Self::VisibilityNotify(e.0);
            }
        }

        Ok(())
    }

    /// Get the opcode of this event.
    #[inline]
    #[must_use]
    pub fn opcode(&self) -> u8 {
        match self {
            Self::ConfigureNotify(_) => ConfigureNotifyEvent::OPCODE,
            Self::ClientMessage(_) => ClientMessageEvent::OPCODE,
            Self::Expose(_) => ExposeEvent::OPCODE,
            Self::ButtonPress(_) => ButtonPressEvent::OPCODE,
            Self::ButtonRelease(_) => ButtonReleaseEvent::OPCODE,
            Self::CirculateNotify(_) => CirculateNotifyEvent::OPCODE,
            Self::CirculateRequest(_) => CirculateRequestEvent::OPCODE,
            Self::ConfigureRequest(_) => ConfigureRequestEvent::OPCODE,
            Self::CreateNotify(_) => CreateNotifyEvent::OPCODE,
            Self::DestroyNotify(_) => DestroyNotifyEvent::OPCODE,
            Self::EnterNotify(_) => EnterNotifyEvent::OPCODE,
            Self::FocusIn(_) => FocusInEvent::OPCODE,
            Self::FocusOut(_) => FocusOutEvent::OPCODE,
            Self::GraphicsExposure(_) => GraphicsExposureEvent::OPCODE,
            Self::GravityNotify(_) => GravityNotifyEvent::OPCODE,
            Self::KeyPress(_) => KeyPressEvent::OPCODE,
            Self::KeyRelease(_) => KeyReleaseEvent::OPCODE,
            Self::KeymapNotify(_) => KeymapNotifyEvent::OPCODE,
            Self::LeaveNotify(_) => LeaveNotifyEvent::OPCODE,
            Self::MapNotify(_) => MapNotifyEvent::OPCODE,
            Self::MapRequest(_) => MapRequestEvent::OPCODE,
            Self::MappingNotify(_) => MappingNotifyEvent::OPCODE,
            Self::NoExposure(_) => NoExposureEvent::OPCODE,
            Self::PropertyNotify(_) => PropertyNotifyEvent::OPCODE,
            Self::ReparentNotify(_) => ReparentNotifyEvent::OPCODE,
            Self::ResizeRequest(_) => ResizeRequestEvent::OPCODE,
            Self::SelectionClear(_) => SelectionClearEvent::OPCODE,
            Self::SelectionNotify(_) => SelectionNotifyEvent::OPCODE,
            Self::SelectionRequest(_) => SelectionRequestEvent::OPCODE,
            Self::UnmapNotify(_) => UnmapNotifyEvent::OPCODE,
            Self::VisibilityNotify(_) => VisibilityNotifyEvent::OPCODE,
            Self::NoneOfTheAbove { opcode, .. } => *opcode,
        }
    }
}
