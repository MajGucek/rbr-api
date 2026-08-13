#[allow(non_camel_case_types)]
use std::ffi::{c_char, c_void};
use crate::raw::constants::RBRMENUSYSTEM_NUM_OF_MENUS;

#[repr(C)]
pub (crate) struct D3DMatrix {
    m: [[f32; 4]; 4],
}

#[repr(C, packed)]
pub (crate) struct D3DXVector3 {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
}

#[repr(C, packed)]
pub (crate) struct D3DXQuaternion {
    pub(crate) x: f32,
    pub(crate) y: f32,
    pub(crate) z: f32,
    pub(crate) w: f32,
}

pub (crate) type DWORD = u32; // unsigned long
pub (crate) type WCHAR = u16; // 16bit UNICODE character or wchar_t
pub (crate) type LPCWSTR = *const WCHAR;  // Long Pointer to Constant Wide String, apparently not related to long type?
pub (crate) type PCWSTR = *const WCHAR;
pub (crate) type LPCSTR = *const c_char;
pub (crate) type PCSTR = *const c_char;
pub (crate) type LPVOID = *mut c_void;



#[repr(C, packed)]
pub (crate) struct RBRCameraInfo {
    pub(crate) camera_type: i32,

    padding1: [u8; 0xCC - 0x00 - size_of::<i32>()],
    pub(crate) current_camera_map_location: D3DMatrix,

    padding2: [u8; 0x318 - 0xCC - size_of::<D3DMatrix>()],
    pub(crate) camera_orientation: D3DXVector3,
    pub(crate) camera_pov1: D3DXVector3,
    pub(crate) camera_pov2: D3DXVector3,
    pub(crate) camera_position: D3DXVector3,
    pub(crate) camera_fov: f32,
    pub(crate) camera_near: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRCamera1 {

    padding1: [u8; 0x10],
    pub(crate) camera_info: *mut RBRCameraInfo,
}

#[repr(C, packed)]
pub (crate) struct RBRCarInfo {
    pub(crate) hud_position_x: i32,
    pub(crate) hud_position_y: i32,
    pub(crate) race_started: i32,
    pub(crate) speed: f32,
    pub(crate) rpm: f32,
    pub(crate) temperature: f32,
    pub(crate) turbo: f32,
    unknown2: i32,
    pub(crate) distance_from_start_control: f32,
    pub(crate) distance_travelled: f32,
    pub(crate) distance_to_finish: f32,

    padding1: [u8; 0x13C - 0x28 - size_of::<f32>()],
    pub(crate) stage_progress: f32,
    pub(crate) race_time: f32,
    pub(crate) race_finished: i32,
    unknown4: i32,
    unknown5: i32,
    pub(crate) driving_direction: i32,
    pub(crate) fade_wrong_way_message: f32,

    padding3: [u8; 0x170 - 0x154 - size_of::<f32>()],
    pub(crate) gear: i32,

    padding4: [u8; 0x244 - 0x170 - size_of::<i32>()],
    pub(crate) stage_start_countdown: f32,
    pub(crate) false_start: i32,

    padding5: [u8; 0x254 - 0x248 - size_of::<i32>()],
    pub(crate) split_reached_number: i32,
    pub(crate) split1_time: f32,
    pub(crate) split2_time: f32,
    unknown6: f32,

    padding6: [u8; 0x2C4 - 0x260 - size_of::<f32>()],
    pub(crate) finish_line_passed: i32,

    padding7: [u8; 0x758 - 0x2C4 - size_of::<i32>()],
    pub(crate) camera: *mut RBRCamera1,

    padding8: [u8; 0xEF8 - 0x758 - size_of::<*mut RBRCamera1>()],
    pub(crate) car_position: D3DXVector3,
}

#[repr(C, packed)]
pub (crate) struct RBRCarMovement {

    padding1: [u8; 0x100],
    pub(crate) car_quaternion: D3DXQuaternion,
    pub(crate) car_map_location: D3DMatrix,

    padding2: [u8; 0x190 - 0x110 - size_of::<D3DMatrix>()],
    pub(crate) spin: D3DXVector3,

    padding3: [u8; 0x1C0 - 0x190 - size_of::<D3DXVector3>()],
    pub(crate) speed: D3DXVector3,

    padding4: [u8; 0x85C - 0x1C0 - size_of::<D3DXVector3>()],
    pub(crate) drive_throttle: f32,
    pub(crate) drive_brake: f32,
    pub(crate) drive_handbrake: f32,
    pub(crate) drive_steering: f32,
    pub(crate) drive_clutch: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerAxisData {

    padding1: [u8; 0x24],
    pub(crate) status: i32,
    unknown1: i32,
    pub(crate) dinput_status: i32,

    pub(crate) axis_value: f32,
    pub(crate) axis_raw_value: DWORD,

    pub(crate) axis_value2: f32,
    pub(crate) axis_raw_value2: DWORD,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerAxis {
    pub(crate) axis_name_id: *mut c_char,
    pub(crate) axis_name: *mut WCHAR,
    unknown1: i32,
    pub(crate) controller_axis_data: *mut RBRControllerAxisData,
    unknown2: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerObject {
    padding1: [u8; 0x24],
    pub(crate) controller_axis: [RBRControllerAxis; 21],

    padding2: [u8; 0x258 - 0x24 - (size_of::<RBRControllerAxis>() * 21)],
    pub(crate) throttle_inverted: i32,
    pub(crate) brake_inverted: i32,
    pub(crate) combined_throttle_brake_inverted: i32,
    pub(crate) handbrake_inverted: i32,
    pub(crate) clutch_inverted: i32,
    unknown1: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerBaseObject {
    unknown1: i32,
    pub(crate) controller_object: *mut RBRControllerObject,
}

#[repr(C, packed)]
pub (crate) struct RBRGameConfig {
    padding1: [u8; 0x54],
    pub(crate) resolution_x: i32,
    pub(crate) resolution_y: i32,

    padding3: [u8; 0x0CF8 - 0x58 - size_of::<i32>()],
    pub(crate) controller_base_object: *mut RBRControllerBaseObject,
}

#[repr(C, packed)]
pub (crate) struct RBRColorTable {
    pub(crate) menu_background_r: f32,
    pub(crate) menu_background_g: f32,
    pub(crate) menu_background_b: f32,
    pub(crate) menu_background_a: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRGameMode {
    padding1: [u8; 0x728],
    pub(crate) game_mode: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRGameModeExtra {
    padding1: [u8; 0x10],
    pub(crate) game_mode_extra: i32,
    pub(crate) track_id: i32,
    pub(crate) car_id: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRCarControls {
    padding1: [u8; 0x738 + 0x5C],
    pub(crate) steering: f32,
    pub(crate) throttle: f32,
    pub(crate) brake: f32,
    pub(crate) handbrake: f32,
    pub(crate) clutch: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRMapInfo {
    padding1: [u8; 0x75310],
    pub(crate) stage_length: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMapSettings {
    unknown1: i32,
    pub(crate) track_id: i32,
    pub(crate) car_id: i32,
    unknown2: i32,
    unknown3: i32,
    pub(crate) transmission_type: i32,

    padding1: [u8; 0x30 - 0x14 - size_of::<i32>()],
    pub(crate) race_paused: i32,

    padding2: [u8; 0x38 - 0x30 - size_of::<i32>()],
    pub(crate) tyre_type: i32,

    padding3: [u8; 0x48 - 0x38 - size_of::<i32>()],
    pub(crate) weather_type: i32,
    unknown4: i32,
    pub(crate) damage_type: i32,
    pub(crate) pacecar_enabled: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMapSettingsExtra {
    unknown1: i32,
    unknown2: i32,
    pub(crate) track_id: i32,
    unknown3: i32,
    pub(crate) sky_cloud_type: i32,
    pub(crate) surface_wetness: i32,
    pub(crate) surface_age: i32,

    padding1: [u8; 0x38 - 0x18 - size_of::<i32>()],
    pub(crate) time_of_day: i32,
    pub(crate) sky_type: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRGameModeExtra2 {
    padding1: [u8; 0x10],
    pub(crate) loading_mode: i32,
    pub(crate) racing_paused: i32,
    pub(crate) ghost_car_id: i32,
    pub(crate) car_id: i32,
    pub(crate) track_id: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRGhostCarMovement {
    pub(crate) car_map_location: D3DXQuaternion,
    pub(crate) car_quaternion: D3DXQuaternion,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuItemPosition {
    pub(crate) x: u16,
    pub(crate) y: u16,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuItemExtra {
    padding: [u8; 0x2C],
    pub(crate) menu_title_id: *mut c_char,
    pub(crate) menu_title_name: *mut WCHAR,
    unknown1: *mut c_void,
    unknown2: i32,
    unknown3: i32,
    unknown4: i32,
    pub(crate) selected_item_idx: i32,
    pub(crate) number_of_items: i32,
    pub(crate) selected_item_idx2: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuItemCarSelectionCarSpecTexts {
    padding1: [u8; 0x1C],
    pub(crate) tech_spec_value: *mut WCHAR,

    padding2: [u8; 0x28 - 0x1C - size_of::<*mut WCHAR>()],
    pub(crate) model_title: *mut WCHAR,

    padding3: [u8; 0x34 - 0x28 - size_of::<*mut WCHAR>()],
    pub(crate) horsepower_title: *mut WCHAR,

    padding4: [u8; 0x40 - 0x34 - size_of::<*mut WCHAR>()],
    pub(crate) horsepower_value: *mut WCHAR,

    padding5: [u8; 0x4C - 0x40 - size_of::<*mut WCHAR>()],
    pub(crate) torque_title: *mut WCHAR,

    padding6: [u8; 0x58 - 0x4C - size_of::<*mut WCHAR>()],
    pub(crate) torque_value: *mut WCHAR,

    padding7: [u8; 0x64 - 0x58 - size_of::<*mut WCHAR>()],
    pub(crate) engine_title: *mut WCHAR,

    padding8: [u8; 0x70 - 0x64 - size_of::<*mut WCHAR>()],
    pub(crate) engine_value: *mut WCHAR,

    padding9: [u8; 0x7C - 0x70 - size_of::<*mut WCHAR>()],
    pub(crate) tyres_title: *mut WCHAR,

    padding10: [u8; 0x88 - 0x7C - size_of::<*mut WCHAR>()],
    pub(crate) weight_title: *mut WCHAR,

    padding11: [u8; 0x94 - 0x88 - size_of::<*mut WCHAR>()],
    pub(crate) weight_value: *mut WCHAR,

    padding12: [u8; 0xA0 - 0x94 - size_of::<*mut WCHAR>()],
    pub(crate) transmission_title: *mut WCHAR,

    padding13: [u8; 0xAC - 0xA0 - size_of::<*mut WCHAR>()],
    pub(crate) transmission_value: *mut WCHAR,
}

#[repr(C)]
pub (crate) union UnionMenuTitleId {
    pub(crate) sz_menu_title_id: LPCSTR,
    pub(crate) wsz_menu_title_id: LPCWSTR,
}
#[repr(C)]
pub (crate) union UnionMenuTitleName {
    pub(crate) menu_title_name: LPCWSTR,
    pub(crate) title_attribute: DWORD,
}

#[repr(C, packed)]
pub (crate) struct RBRPluginMenuItemObject3 {
    padding1: [u8; 0x18],
    pub(crate) union_menu_title_id: UnionMenuTitleId,
    pub(crate) union_menu_title_name: UnionMenuTitleName,
    pub(crate) item_name: LPCSTR,
}

#[repr(C, packed)]
pub (crate) struct RBRPluginMenuItemObject2 {
    padding1: [u8; 0x24],
    pub(crate) menu_title_id: LPCSTR,
    pub(crate) menu_title_name: LPCWSTR,
}

#[repr(C)]
pub (crate) union UnionMenuItem {
    pub(crate) item_position: *mut RBRMenuItemPosition,
    pub(crate) extra_menu_object: *mut RBRMenuItemExtra,
}
#[repr(C, packed)]
pub (crate) struct RBRMenuObject {
    padding1: [u8; 0x04],
    pub(crate) root_menu_object: *mut RBRMenuObject,
    pub(crate) prev_menu_object: *mut RBRMenuObject,
    pub(crate) item_object: *mut LPVOID,
    pub(crate) union_menu_item: UnionMenuItem,
    pub(crate) number_of_items: i32,
    pub(crate) selected_item_idx: i32,
    pub(crate) first_selectable_item_idx: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuSystem {
    unknown1: LPVOID,
    pub(crate) root_menu_object: *mut RBRMenuObject,
    pub(crate) current_menu_object: *mut RBRMenuObject,
    pub(crate) current_menu_object2: *mut RBRMenuObject,

    padding1: [u8; 0x48 - 0x0C - size_of::<*mut RBRMenuObject>()],
    pub(crate) menu_image_position_x: f32,
    pub(crate) menu_image_position_y: f32,
    pub(crate) menu_image_width: f32,
    pub(crate) menu_image_height: f32,

    padding2: [u8; 0x70 - 0x54 - size_of::<i32>()],
    pub(crate) menu_visible: i32,
    unknown2: i32,
    pub(crate) menu_object: [*mut RBRMenuObject; RBRMENUSYSTEM_NUM_OF_MENUS],
}

#[repr(C, packed)]
pub (crate) struct RBRProfile {
    unknown1: LPVOID,
    unknown2: LPVOID,
    pub(crate) profile_name: [c_char; 16],
}

#[repr(C, packed)]
pub (crate) struct RBRStatusText {
    padding1: [u8; 0x8c],
    pub(crate) load_destination_title_id: *mut c_char,
    pub(crate) load_destination_title_name: *mut WCHAR,
    unknown1: i32,
    pub(crate) load_replay_title_id: *mut c_char,
    pub(crate) load_replay_title_name: *mut WCHAR,
    unknown2: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRPacenote {
    pub(crate) pacenote_type: i32,
    pub(crate) flags: i32,
    pub(crate) distance: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRPacenotes {
    padding1: [u8; 0x20],
    pub(crate) number_pacenotes: i32,
    pub(crate) pacenotes: *mut RBRPacenote,
}

#[repr(C)]
pub (crate) struct RBRPluginMenuSystem {
    pub(crate) plugins_menu_object: *mut RBRMenuObject,
    pub(crate) custom_plugin_menu_object: *mut RBRMenuObject,
    pub(crate) options_menu_object: *mut RBRMenuObject,
}