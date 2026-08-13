mod game;
mod reader;
mod writer;
mod math;

use windows::core::Interface;
use windows::Win32::Foundation::HWND;
use windows::Win32::Graphics::Direct3D9::IDirect3DDevice9;
pub use game::GameMode;
pub use reader::RbrReader;
pub use writer::RbrWriter;

use crate::PluginResult;

pub struct Rbr {}

impl Rbr {
    pub (crate) unsafe fn initialize() -> PluginResult<Self> {
        unsafe {
            crate::raw::functions::initialize_object_references()?;
        }
        Ok(Self {})
    }
    pub(crate) fn initialize_race_time_object_references(&self) -> PluginResult<()> {
        unsafe {
            crate::raw::functions::initialize_race_time_object_references()?;
        }

        Ok(())
    }
    
    pub fn reader(&self) -> RbrReader<'_> {
        RbrReader::new(self)
    }
    pub fn writer(&self) -> RbrWriter<'_> {
        RbrWriter::new(self)
    }
    pub fn raw_device(&self) -> Option<IDirect3DDevice9> {
        unsafe {
            let raw = crate::raw::globals::RBR_DIRECT3D_DEVICE;

            IDirect3DDevice9::from_raw_borrowed(&raw).cloned()
        }
    }
    pub (crate) fn window_handle(&self) -> HWND {
        unsafe {
            crate::raw::globals::RBR_WINDOW
        }
    }
}