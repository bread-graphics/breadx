// MIT/Apache2 License

use super::{Keymap, Modifier, MODIFIERS};
use crate::{
    auto::xproto::{Keycode, Keysym},
    display::{Connection, Display, KeyboardMapping, ModifierMapping},
};
use alloc::boxed::Box;

#[cfg(feature = "async")]
use crate::display::AsyncConnection;

#[derive(Debug, Clone)]
pub struct XprotoKeymap {
    min_keycode: Keycode,
    max_keycode: Keycode,
    keysyms_per_keycode: u8,
    keysyms: Box<[Keysym]>,
    keycodes_per_modifier: u8,
    modifiers: Box<[Keycode]>,
}

impl XprotoKeymap {
    #[inline]
    pub(crate) fn init_from<Conn: Connection>(display: &mut Display<Conn>) -> crate::Result<Self> {
        let keyboard_tok = display.get_keyboard_mapping()?;
        let modifier_tok = display.get_modifier_mapping()?;
        let keyboard_map: KeyboardMapping = display.resolve_request(keyboard_tok)?.into();
        let modifier_map: ModifierMapping = display.resolve_request(modifier_tok)?.into();

        Ok(Self {
            min_keycode: display.setup().min_keycode,
            max_keycode: display.setup().max_keycode,
            keysyms_per_keycode: keyboard_map.keysyms_per_keycode,
            keysyms: keyboard_map.keysyms,
            keycodes_per_modifier: modifier_map.keycodes_per_modifier,
            modifiers: modifier_map.keycodes,
        })
    }

    #[cfg(feature = "async")]
    #[inline]
    pub(crate) async fn init_from_async<Conn: AsyncConnection>(
        display: &mut Display<Conn>,
    ) -> crate::Result<Self> {
        let keyboard_tok = display.get_keyboard_mapping_async().await?;
        let modifier_tok = display.get_modifier_mapping_async().await?;
        let keyboard_map: KeyboardMapping =
            display.resolve_request_async(keyboard_tok).await?.into();
        let modifier_map: ModifierMapping =
            display.resolve_request_async(modifier_tok).await?.into();

        Ok(Self {
            min_keycode: display.setup().min_keycode,
            max_keycode: display.setup().max_keycode,
            keysyms_per_keycode: keyboard_map.keysyms_per_keycode,
            keysyms: keyboard_map.keysyms,
            keycodes_per_modifier: modifier_map.keycodes_per_modifier,
            modifiers: modifier_map.modifiers,
        })
    }
}

impl Keymap for XprotoKeymap {
    #[inline]
    fn lookup_keysyms(&self, keycode: Keycode) -> &[Keysym] {
        let start = (keycode - self.min_keycode) as usize * self.keysyms_per_keycode as usize;
        &self.keysyms[start..start + self.keysyms_per_keycode as usize]
    }

    #[inline]
    fn lookup_modifier(&self, keycode: Keycode) -> Option<Modifier> {
        MODIFIERS
            .iter()
            .find(|modifier| {
                let start = **modifier as usize * self.keycodes_per_modifier as usize;
                self.modifiers[start..start + self.keycodes_per_modifier as usize]
                    .contains(&keycode)
            })
            .cloned()
    }
}
