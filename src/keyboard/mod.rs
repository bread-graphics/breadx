// MIT/Apache2 License

/*

There are two ways to handle keyboard input in the context of X11:

1). Using xproto's natural GetKeyboardMapping functions.
2). Using the xkb extension, which we don't support yet.

We keep stuff in the given KeyboardState, which acts as a unifying interface between
both of the above.

*/

use crate::{
    auto::xproto::{KeyButMask, Keycode, Keysym},
    display::{Connection, Display},
};
use gluten_keyboard::Key;

#[cfg(feature = "async")]
use crate::display::AsyncDisplay;

mod convert;
pub use convert::*;

mod xproto;
pub use xproto::*;

pub type DefaultKeymap = XprotoKeymap;

/// Keep track of keys and currently tracked modifiers.
#[derive(Debug, Clone)]
pub struct KeyboardState<Km: ?Sized = DefaultKeymap> {
    keymap: Km,
}

impl<Km> KeyboardState<Km> {
    #[inline]
    pub fn from_keymap(keymap: Km) -> Self {
        Self { keymap }
    }
}

impl KeyboardState<XprotoKeymap> {
    #[inline]
    pub fn new<Dpy: Display + ?Sized>(display: &mut Dpy) -> crate::Result<Self> {
        Ok(Self::from_keymap(XprotoKeymap::init_from(display)?))
    }

    #[cfg(feature = "async")]
    #[inline]
    pub async fn new_async<Dpy: AsyncDisplay + ?Sized>(display: &mut Dpy) -> crate::Result<Self> {
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
    pub fn process_keycode(&mut self, keycode: Keycode, modifiers: KeyButMask) -> Option<Key> {
        // get the index we need
        let index = if modifiers.shift() { 1 } else { 0 };

        let syms = self.lookup_keysyms(keycode);
        if syms.is_empty() {
            None
        } else if syms.len() == 1 {
            keysym_to_key(syms[0])
        } else {
            keysym_to_key(syms[index])
        }
    }
}

pub trait Keymap {
    fn lookup_keysyms(&self, keycode: Keycode) -> &[Keysym];
}
