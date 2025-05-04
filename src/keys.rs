use bitflags::bitflags;
use macroquad::prelude::*;

use crate::wrap::{
    is_ctrl_down,
    is_meta_down,
    is_shift_down,
    is_super_down,
    is_key_active
};
use crate::edit::{Mode, Editor};



bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct Modifiers: u8 {
        const NoMod = 0;
        const Shift = 1;
        const Ctrl  = 1 << 1;
        const Meta  = 1 << 2;
        const Super = 1 << 3;
    }
}

impl Modifiers {

    /// Returns true if all of the required modifiers are down
    #[must_use]
    pub fn are_down(&self) -> bool {

        // TODO: refactor
        let nomod = self.is_empty() && !(
            is_shift_down() ||
            is_ctrl_down()  ||
            is_meta_down()  ||
            is_super_down()
        );
        let shift  = self.intersects(Self::Shift) && is_shift_down();
        let ctrl   = self.intersects(Self::Ctrl)  && is_ctrl_down();
        let meta   = self.intersects(Self::Meta)  && is_meta_down();
        let super_ = self.intersects(Self::Super) && is_super_down();
        nomod || shift || ctrl || meta || super_
    }

    #[must_use]
    pub fn count(&self) -> usize {
        self.iter().count()
    }

}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Key {
    pub key_code: KeyCode,
    pub mods: Modifiers,
}

impl Key {

    pub fn new(key_code: KeyCode, mods: Modifiers) -> Self {
        Self { key_code, mods }
    }

    // TODO: parse string into key

    #[must_use]
    pub fn is_active(&self) -> bool {
        is_key_active(self.key_code) && self.mods.are_down()
    }

}

#[macro_export]
macro_rules! keybind {
    ($mode:ident, $key:ident, $($mod_:ident),+ $(,)?) => {
        Keybind::new(
            Key::new(KeyCode::$key, $(Modifiers::$mod_) | +),
            Mode::$mode
        )
    };
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Keybind {
    // TODO: key-chord
    pub mode: Mode,
    pub key: Key,
}

impl Keybind {
    pub fn new(key: Key, mode: Mode) -> Self {
        Self { key, mode }
    }
}
