use crate::raw::globals::RBR_GAME_MODE;
use super::{GameMode, Rbr};

pub struct RbrReader<'a> {
    rbr: &'a Rbr,
}

impl<'a> RbrReader<'a> {
    pub (crate) fn new(rbr: &'a Rbr) -> Self {
        Self { rbr }
    }

    pub fn game_mode(&self) -> Option<GameMode> {
        unsafe {
            if RBR_GAME_MODE.is_null() { return None }

            Some(
                GameMode::from(
                    std::ptr::addr_of!(
                        (*RBR_GAME_MODE).game_mode
                    ).read_unaligned()
                )
            )
        }
    }
}