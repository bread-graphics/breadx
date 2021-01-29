// MIT/Apache2 License

/*

There are two ways to handle keyboard input in the context of X11:

1). Using xproto's natural GetKeyboardMapping functions.
2). Using the xkb extension, which we don't support yet.

We keep stuff in the given KeyboardState, which acts as a unifying interface between
both of the above.

*/

use crate::{
    auto::xproto::{Keycode, Keysym, ModMask},
    display::{Connection, Display},
};
use alloc::{borrow::Cow, boxed::Box};
use core::{future::Future, pin::Pin};
use gluten_keyboard::Key;

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

mod convert;
pub use convert::*;

mod xproto;
pub use xproto::*;

/// Default keymap.
pub type DefaultKeymap = XprotoKeymap;

/// Keep track of keys and currently tracked modifiers.
#[derive(Debug, Clone)]
pub struct KeyboardState<Km: ?Sized = DefaultKeymap> {
    set_mods: ModMask,
    clear_mods: ModMask,
    keymap: Km,
}

/// Enum representing a type of modifier.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Modifier {
    Lock = 1,
    Shift = 0,
    Control = 2,
    Mod1 = 3,
    Mod2 = 4,
    Mod3 = 5,
    Mod4 = 6,
    Mod5 = 7,
}

pub const MODIFIERS: &[Modifier] = &[
    Modifier::Lock,
    Modifier::Shift,
    Modifier::Control,
    Modifier::Mod1,
    Modifier::Mod2,
    Modifier::Mod3,
    Modifier::Mod4,
    Modifier::Mod5,
];

impl<Km> KeyboardState<Km> {
    #[inline]
    pub fn from_keymap(keymap: Km) -> Self {
        Self {
            keymap,
            set_mods: Default::default(),
            clear_mods: Default::default(),
        }
    }
}

impl KeyboardState<XprotoKeymap> {
    #[inline]
    pub fn new<Conn: Connection>(display: &mut Display<Conn>) -> crate::Result<Self> {
        Ok(Self::from_keymap(XprotoKeymap::init_from(display)?))
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn new_async<Conn: AsyncConnection>(
        display: &mut Display<Conn>,
    ) -> crate::Result<Self> {
        Ok(Self::from_keymap(
            XprotoKeymap::init_from_async(display).await?,
        ))
    }
}

impl<Km: Keymap + ?Sized> KeyboardState<Km> {
    #[inline]
    pub fn lookup_keysyms(&self, keycode: Keycode) -> &[Keysym] {
        self.keymap.lookup_keysyms(keycode)
    }

    #[inline]
    fn set_modifier(&mut self, modifier: Modifier, val: bool) {
        match modifier {
            Modifier::Lock => {
                if self.clear_mods.lock() == val {
                    self.set_mods.set_lock(val);
                } else {
                    self.clear_mods.set_lock(val);
                }
            }
            Modifier::Shift => {
                self.set_mods.set_shift(val);
            }
            Modifier::Control => {
                self.set_mods.set_control(true);
            }
            _ => (),
        }
    }

    #[inline]
    pub fn process_keycode_press(&mut self, keycode: Keycode) -> Option<Key> {
        // if the keycode is a modifier, update the appropriate modifier in our state
        if let Some(modifier) = self.keymap.lookup_modifier(keycode) {
            log::info!("Modifier: {:?}", modifier);
            self.set_modifier(modifier, true);
        }

        // get the index we need
        let index = if self.set_mods.shift() {
            1
        } else {
            0
        };

        let syms = self.lookup_keysyms(keycode);
        if syms.is_empty() {
            None
        } else if syms.len() == 1 {
            keysym_to_key(syms[0])
        } else {
            keysym_to_key(syms[index])
        }
    }

    #[inline]
    pub fn process_keycode_release(&mut self, keycode: Keycode) {
        if let Some(modifier) = self.keymap.lookup_modifier(keycode) {
            log::info!("Modifier released: {:?}", modifier);
            self.set_modifier(modifier, false);
        }
    }
}

pub trait Keymap {
    fn lookup_keysyms(&self, keycode: Keycode) -> &[Keysym];
    fn lookup_modifier(&self, keycode: Keycode) -> Option<Modifier>;
}
