use std::ptr::addr_of_mut;
use crate::{PluginError, PluginResult};
use crate::raw::globals::{RBR_MAP_SETTINGS};
use crate::rbr::Rbr;

pub struct RbrWriter<'a> {
    rbr: &'a Rbr,
}

impl<'a> RbrWriter<'a> {
    pub (crate) fn new(rbr: &'a Rbr) -> Self {
        Self { rbr }
    }

    pub fn set_race_paused(&self, value: bool) -> PluginResult<()> {
        unsafe {
            if RBR_MAP_SETTINGS.is_null() {
                return Err(PluginError::WriteError(
                    "RBR map settings are null".to_owned()
                ));
            }
            
            addr_of_mut!((*RBR_MAP_SETTINGS).race_paused)
                .write_unaligned(i32::from(value));
            
            Ok(())
        }
    }
}