pub fn range_remap(value: f32, low1: f32, high1: f32, low2: f32, high2: f32, ) -> f32 {
    low2 + (value - low1) * (high2 - low2) / (high1 - low1)
}

use std::ffi::c_void;
use std::mem::MaybeUninit;
use windows::core::{Interface, HRESULT};
use windows::Win32::Foundation::{POINT, RECT};
use windows::Win32::Graphics::Direct3D9::{IDirect3DDevice9, D3DDEVICE_CREATION_PARAMETERS};
use windows::core::Result;
use windows::Win32::UI::WindowsAndMessaging::{GetClientRect, GetWindowRect};
use windows::Win32::Graphics::Gdi::MapWindowPoints;

use crate::raw::globals::*;
use crate::raw::types::*;


pub(crate) unsafe fn initialize_race_time_object_references(
) -> Result<()> {
    unsafe {
        RBR_MAP_INFO = (0x0165_9184 as *const *mut RBRMapInfo).read_unaligned();

        RBR_CAR_MOVEMENT = (0x008E_F660 as *const *mut RBRCarMovement).read_unaligned();

        let base = (0x007E_ABA8 as *const usize).read_unaligned();

        RBR_PACENOTES = ((base + 0x10) as *const *mut RBRPacenotes).read_unaligned();

        if RBR_CAR_MOVEMENT.is_null() {
            return Err(windows::core::Error::from_hresult(
                HRESULT(0x8000_4003u32 as i32),
            ));
        }

        Ok(())
    }
}

pub(crate) unsafe fn refresh_window_rect() -> Result<()> {
    unsafe {
        let window = RBR_WINDOW;

        let mut window_rect = RECT::default();
        let mut client_rect = RECT::default();

        GetWindowRect(window, &mut window_rect)?;
        GetClientRect(window, &mut client_rect)?;

        let mut points = [
            POINT {
                x: client_rect.left,
                y: client_rect.top,
            },
            POINT {
                x: client_rect.right,
                y: client_rect.bottom,
            },
        ];

        MapWindowPoints(
            Some(window),
            None,
            &mut points,
        );

        RBR_WINDOW_RECT = window_rect;
        RBR_WINDOW_CLIENT_RECT = client_rect;
        RBR_WINDOW_MAPPED_RECT = RECT {
            left: points[0].x,
            top: points[0].y,
            right: points[1].x,
            bottom: points[1].y,
        };

        Ok(())
    }
}


pub (crate) unsafe fn initialize_object_references() -> Result<()> {
    unsafe {
        if RBR_GAME_CONFIG.is_null() {
            RBR_GAME_CONFIG = (0x007E_AC48 as *const *mut RBRGameConfig).read_unaligned();
        }

        if RBR_GAME_MODE.is_null() {
            RBR_GAME_MODE = (0x007E_AC48 as *const *mut RBRGameMode).read_unaligned();
        }

        if RBR_GAME_MODE_EXT.is_null() {
            RBR_GAME_MODE_EXT = (0x0089_3634 as *const *mut RBRGameModeExtra).read_unaligned();
        }

        if RBR_GAME_MODE_EXT2.is_null() {
            let base = (0x007E_A678 as *const usize).read_unaligned();

            RBR_GAME_MODE_EXT2 = ((base + 0x70) as *const *mut RBRGameModeExtra2).read_unaligned();
        }

        if RBR_CAR_INFO.is_null() {
            RBR_CAR_INFO = (0x0165_FC68 as *const *mut RBRCarInfo).read_unaligned();
        }

        if RBR_CAR_CONTROLS.is_null() {
            RBR_CAR_CONTROLS = (0x007E_AC48 as *const *mut RBRCarControls).read_unaligned();
        }

        if RBR_GHOST_CAR_MOVEMENT.is_null() {
            RBR_GHOST_CAR_MOVEMENT = 0x0089_3060 as *mut RBRGhostCarMovement;
        }

        if RBR_GHOST_CAR_REPLAY_MODE.is_null() {
            RBR_GHOST_CAR_REPLAY_MODE = 0x0089_2EEC as *mut i32;
        }

        if RBR_MENU_SYSTEM.is_null() {
            RBR_MENU_SYSTEM = (0x0165_FA48 as *const *mut RBRMenuSystem).read_unaligned();
        }

        RBR_MAP_SETTINGS = 0x0166_0800 as *mut RBRMapSettings;

        RBR_MAP_SETTINGS_EXT = 0x0089_38F8 as *mut RBRMapSettingsExtra;

        RBR_MAP_LOCATION_NAME = 0x007D_1D64 as *mut WCHAR;

        RBR_PROFILE = (0x007D_2554 as *const *mut RBRProfile).read_unaligned();

        RBR_COLOR_TABLE = 0x007C_3668 as *mut RBRColorTable;

        RBR_STATUS_TEXT = (0x007D_1D50 as *const *mut RBRStatusText).read_unaligned();

        if RBR_DIRECT3D_DEVICE.is_null() {
            let base1 = (0x007E_A990 as *const usize).read_unaligned();

            let base2 = ((base1 + 0x28) as *const usize).read_unaligned();

            RBR_DIRECT3D_DEVICE = ((base2 + 0xF4) as *const *mut c_void).read_unaligned();
        }

        let raw_device = RBR_DIRECT3D_DEVICE;

        let device =
            IDirect3DDevice9::from_raw_borrowed(&raw_device)
                .ok_or_else(|| { windows::core::Error::from_hresult(windows::core::HRESULT(0x8000_4003u32 as i32)) })?;

        let mut parameters = MaybeUninit::<D3DDEVICE_CREATION_PARAMETERS>::uninit();

        device.GetCreationParameters(parameters.as_mut_ptr(), )?;

        let parameters = parameters.assume_init();

        RBR_WINDOW = parameters.hFocusWindow;

        refresh_window_rect()?;

        Ok(())
    }
}

