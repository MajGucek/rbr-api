use std::ffi::c_void;
use std::ptr::null_mut;
use windows::Win32::Foundation::{HWND, RECT};
use crate::raw::types::*;
use crate::raw::types::WCHAR;

pub (crate) static mut RBR_DIRECT3D_DEVICE: *mut c_void = null_mut(); // RBR D3D device

pub (crate) static mut RBR_WINDOW: HWND = HWND(null_mut());
pub (crate) static mut RBR_WINDOW_RECT: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0, };
pub (crate) static mut RBR_WINDOW_CLIENT_RECT: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0, };
pub (crate) static mut RBR_WINDOW_MAPPED_RECT: RECT = RECT { left: 0, top: 0, right: 0, bottom: 0, };

pub (crate) static mut RBR_GAME_CONFIG: *mut RBRGameConfig = null_mut();

pub (crate) static mut RBR_GAME_MODE: *mut RBRGameMode = null_mut();
pub (crate) static mut RBR_GAME_MODE_EXT: *mut RBRGameModeExtra = null_mut();
pub (crate) static mut RBR_GAME_MODE_EXT2: *mut RBRGameModeExtra2 = null_mut();

//pub (crate) static mut RBR_CAMERA_INFO: *mut RBRCameraInfo = null_mut();

pub (crate) static mut RBR_CAR_INFO: *mut RBRCarInfo = null_mut();
pub (crate) static mut RBR_CAR_CONTROLS: *mut RBRCarControls = null_mut();
pub (crate) static mut RBR_MAP_SETTINGS: *mut RBRMapSettings = null_mut();
pub (crate) static mut RBR_MAP_SETTINGS_EXT: *mut RBRMapSettingsExtra = null_mut();

pub (crate) static mut RBR_GHOST_CAR_REPLAY_MODE: *mut i32 = null_mut();
pub (crate) static mut RBR_GHOST_CAR_MOVEMENT: *mut RBRGhostCarMovement = null_mut();
pub (crate) static mut RBR_CAR_MOVEMENT: *mut RBRCarMovement = null_mut();
pub (crate) static mut RBR_MAP_INFO: *mut RBRMapInfo = null_mut();

pub (crate) static mut RBR_MENU_SYSTEM: *mut RBRMenuSystem = null_mut();

pub (crate) static mut RBR_PACENOTES: *mut RBRPacenotes = null_mut();

pub (crate) static mut RBR_MAP_LOCATION_NAME: *mut WCHAR = null_mut();
pub (crate) static mut RBR_PROFILE: *mut RBRProfile = null_mut();

pub (crate) static mut RBR_COLOR_TABLE: *mut RBRColorTable = null_mut();

pub (crate) static mut RBR_STATUS_TEXT: *mut RBRStatusText = null_mut();
