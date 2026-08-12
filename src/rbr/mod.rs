mod game;
mod reader;

pub use game::GameMode;
pub use reader::RbrReader;

use crate::PluginResult;

pub struct Rbr {}

impl Rbr {
    pub (crate) unsafe fn initialize() -> PluginResult<Self> {
        unsafe {
            crate::raw::functions::initialize_object_references()?;
        }
        Ok(Self {})
    }
    
    pub fn reader(&self) -> RbrReader<'_> {
        RbrReader::new(self)
    }
}