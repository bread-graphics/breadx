// MIT/Apache2 License

use super::Keymap;
use crate::{
    auto::xproto::{Keycode, Keysym},
    display::{prelude::*, Connection, Display, KeyboardMapping},
};
use alloc::boxed::Box;

#[cfg(feature = "async")]
use crate::display::AsyncDisplay;

#[derive(Debug, Clone)]
pub struct XprotoKeymap {
    min_keycode: Keycode,
    max_keycode: Keycode,
    keysyms_per_keycode: u8,
    keysyms: Box<[Keysym]>,
}

impl XprotoKeymap {
    #[inline]
    pub(crate) fn init_from<Dpy: Display + ?Sized>(display: &mut Dpy) -> crate::Result<Self> {
        let keyboard_tok = display.get_keyboard_mapping()?;
        let keyboard_map: KeyboardMapping = display.resolve_request(keyboard_tok)?.into();

        Ok(Self {
            min_keycode: display.setup().min_keycode,
            max_keycode: display.setup().max_keycode,
            keysyms_per_keycode: keyboard_map.keysyms_per_keycode,
            keysyms: keyboard_map.keysyms,
        })
    }

    #[cfg(feature = "async")]
    #[inline]
    pub(crate) async fn init_from_async<Dpy: AsyncDisplay + ?Sized>(
        display: &mut Dpy,
    ) -> crate::Result<Self> {
        let keyboard_tok = display.get_keyboard_mapping_async().await?;
        let keyboard_map: KeyboardMapping =
            display.resolve_request_async(keyboard_tok).await?.into();

        Ok(Self {
            min_keycode: display.setup().min_keycode,
            max_keycode: display.setup().max_keycode,
            keysyms_per_keycode: keyboard_map.keysyms_per_keycode,
            keysyms: keyboard_map.keysyms,
        })
    }
}

impl Keymap for XprotoKeymap {
    #[inline]
    fn lookup_keysyms(&self, keycode: Keycode) -> &[Keysym] {
        let start = (keycode - self.min_keycode) as usize * self.keysyms_per_keycode as usize;
        &self.keysyms[start..start + self.keysyms_per_keycode as usize]
    }
}
