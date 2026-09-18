use std::ffi::c_void;
use std::iter;
use std::slice;

use windows::Win32::System::Diagnostics::Debug::FlushInstructionCache;
use windows::Win32::System::Memory::{PAGE_EXECUTE_READWRITE, PAGE_PROTECTION_FLAGS, VirtualProtect};
use windows::Win32::System::Threading::GetCurrentProcess;

use crate::PluginError::WriteError;
use crate::PluginResult;

const NOP: u8 = 0x90;

/// An instruction in RBR's executable that can be swapped for NOPs and back.
pub(crate) struct NopPatch {
    pub(crate) address: usize,
    pub(crate) original: &'static [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum PatchState {
    Original,
    Applied,
}

impl NopPatch {
    pub(crate) fn verify(&self) -> PluginResult<()> {
        self.state().map(|_| ())
    }

    pub(crate) fn apply(&self) -> PluginResult<()> {
        match self.state()? {
            PatchState::Original => {
                let nops: Vec<u8> = iter::repeat_n(NOP, self.original.len()).collect();
                unsafe { write_code(self.address, &nops) }
            }
            PatchState::Applied => Ok(()),
        }
    }

    pub(crate) fn restore(&self) -> PluginResult<()> {
        match self.state()? {
            PatchState::Applied => unsafe { write_code(self.address, self.original) },
            PatchState::Original => Ok(()),
        }
    }

    fn state(&self) -> PluginResult<PatchState> {
        let current = unsafe { slice::from_raw_parts(self.address as *const u8, self.original.len()) };

        if current == self.original {
            Ok(PatchState::Original)
        } else if current.iter().all(|&byte| byte == NOP) {
            Ok(PatchState::Applied)
        } else {
            Err(WriteError(format!(
                "unexpected code at {:#010x}: {current:02X?}, expected {:02X?}",
                self.address, self.original
            )))
        }
    }
}

unsafe fn write_code(address: usize, bytes: &[u8]) -> PluginResult<()> {
    let target = address as *mut u8;
    let region = target.cast::<c_void>().cast_const();
    let mut previous = PAGE_PROTECTION_FLAGS::default();

    unsafe {
        VirtualProtect(region, bytes.len(), PAGE_EXECUTE_READWRITE, &raw mut previous)
            .map_err(|err| WriteError(format!("VirtualProtect at {address:#010x}: {err}")))?;

        std::ptr::copy_nonoverlapping(bytes.as_ptr(), target, bytes.len());

        VirtualProtect(region, bytes.len(), previous, &raw mut previous)
            .map_err(|err| WriteError(format!("VirtualProtect at {address:#010x}: {err}")))?;

        FlushInstructionCache(GetCurrentProcess(), Some(region), bytes.len())
            .map_err(|err| WriteError(format!("FlushInstructionCache at {address:#010x}: {err}")))?;
    }

    Ok(())
}

/// RBR's car reset, which moves the car back to its last tracked position and rotation after a large jump.
pub(crate) const CAR_RESET_WRITES: [NopPatch; 5] = [
    NopPatch { address: 0x0052_D00E, original: &[0x0F, 0x11, 0x00] },
    NopPatch { address: 0x0052_D055, original: &[0x0F, 0x11, 0x08] },
    NopPatch { address: 0x0052_D087, original: &[0x0F, 0x11, 0x00] },
    NopPatch { address: 0x0052_D0C2, original: &[0x0F, 0x11, 0x08] },
    NopPatch { address: 0x0052_D233, original: &[0x0F, 0x11, 0x08] },
];
