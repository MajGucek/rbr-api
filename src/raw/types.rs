#[allow(non_camel_case_types)]
use std::ffi::{c_char, c_void};
use crate::raw::constants::RBRMENUSYSTEM_NUM_OF_MENUS;

#[repr(C)]
pub (crate) struct D3DMatrix {
    m: [[f32; 4]; 4],
}

#[repr(C, packed)]
pub (crate) struct D3DXVector3 {
    x: f32,
    y: f32,
    z: f32,
}

#[repr(C, packed)]
pub (crate) struct D3DXQuaternion {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

pub (crate) type DWORD = u32; // unsigned long
pub (crate) type WCHAR = u16; // 16bit UNICODE character or wchar_t
pub (crate) type LPCWSTR = *const WCHAR;  // Long Pointer to Constant Wide String, apparently not related to long type?
pub (crate) type PCWSTR = *const WCHAR;
pub (crate) type LPCSTR = *const c_char;
pub (crate) type PCSTR = *const c_char;
pub (crate) type LPVOID = *mut c_void;



#[repr(C)]
pub (crate) union ByteBufferFloat {
    value: f32,
    byte_buffer: [u8; size_of::<f32>()],
    buffer: DWORD,
}

#[repr(C)]
pub (crate) union ByteBufferDword {
    value: DWORD,
    byte_buffer: [u8; size_of::<DWORD>()],
}

#[repr(C)]
pub (crate) union ByteBufferInt32 {
    value: i32,
    byte_buffer: [u8; size_of::<i32>()],
}

#[repr(C)]
pub (crate) union ByteBufferPtr {
    value: LPVOID,
    byte_buffer: [u8; size_of::<LPVOID>()],
}


#[repr(C, packed)]
pub (crate) struct RBRCameraInfo {
    camera_type: i32,

    padding1: [u8; 0xCC - 0x00 - size_of::<i32>()],
    current_camera_map_location: D3DMatrix,

    padding2: [u8; 0x318 - 0xCC - size_of::<D3DMatrix>()],
    camera_orientation: D3DXVector3,
    camera_pov1: D3DXVector3,
    camera_pov2: D3DXVector3,
    camera_position: D3DXVector3,
    camera_fov: f32,
    camera_near: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRCamera1 {

    padding1: [u8; 0x10],
    camera_info: *mut RBRCameraInfo,
}

#[repr(C, packed)]
pub (crate) struct RBRCarInfo {
    hud_position_x: i32,
    hud_position_y: i32,
    race_started: i32,
    speed: f32,
    rpm: f32,
    temperature: f32,
    turbo: f32,
    unknown2: i32,
    distance_from_start_control: f32,
    distance_travelled: f32,
    distance_to_finish: f32,

    padding1: [u8; 0x13C - 0x28 - size_of::<f32>()],
    stage_progress: f32,
    race_time: f32,
    race_finished: i32,
    unknown4: i32,
    unknown5: i32,
    driving_direction: i32,
    fade_wrong_way_message: f32,

    padding3: [u8; 0x170 - 0x154 - size_of::<f32>()],
    gear: i32,

    padding4: [u8; 0x244 - 0x170 - size_of::<i32>()],
    stage_start_countdown: f32,
    false_start: i32,

    padding5: [u8; 0x254 - 0x248 - size_of::<i32>()],
    split_reached_number: i32,
    split1_time: f32,
    split2_time: f32,
    unknown6: f32,

    padding6: [u8; 0x2C4 - 0x260 - size_of::<f32>()],
    finish_line_passed: i32,

    padding7: [u8; 0x758 - 0x2C4 - size_of::<i32>()],
    camera: *mut RBRCamera1,

    padding8: [u8; 0xEF8 - 0x758 - size_of::<*mut RBRCamera1>()],
    car_position: D3DXVector3,
}

#[repr(C, packed)]
pub (crate) struct RBRCarMovement {

    padding1: [u8; 0x100],
    car_quaternion: D3DXQuaternion,
    car_map_location: D3DMatrix,

    padding2: [u8; 0x190 - 0x110 - size_of::<D3DMatrix>()],
    spin: D3DXVector3,

    padding3: [u8; 0x1C0 - 0x190 - size_of::<D3DXVector3>()],
    speed: D3DXVector3,

    padding4: [u8; 0x85C - 0x1C0 - size_of::<D3DXVector3>()],
    drive_throttle: f32,
    drive_brake: f32,
    drive_handbrake: f32,
    drive_steering: f32,
    drive_clutch: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerAxisData {

    padding1: [u8; 0x24],
    status: i32,
    unknown1: i32,
    dinput_status: i32,

    axis_value: f32,
    axis_raw_value: DWORD,

    axis_value2: f32,
    axis_raw_value2: DWORD,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerAxis {
    axis_name_id: *mut c_char,
    axis_name: *mut WCHAR,
    unknown1: i32,
    controller_axis_data: *mut RBRControllerAxisData,
    unknown2: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerObject {
    padding1: [u8; 0x24],
    controller_axis: [RBRControllerAxis; 21],

    padding2: [u8; 0x258 - 0x24 - (size_of::<RBRControllerAxis>() * 21)],
    throttle_inverted: i32,
    brake_inverted: i32,
    combined_throttle_brake_inverted: i32,
    handbrake_inverted: i32,
    clutch_inverted: i32,
    unknown1: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRControllerBaseObject {
    unknown1: i32,
    controller_object: *mut RBRControllerObject,
}

#[repr(C, packed)]
pub (crate) struct RBRGameConfig {
    padding1: [u8; 0x54],
    resolution_x: i32,
    resolution_y: i32,

    padding3: [u8; 0x0CF8 - 0x58 - size_of::<i32>()],
    controller_base_object: *mut RBRControllerBaseObject,
}

#[repr(C, packed)]
pub (crate) struct RBRColorTable {
    menu_background_r: f32,
    menu_background_g: f32,
    menu_background_b: f32,
    menu_background_a: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRGameMode {
    padding1: [u8; 0x728],
    pub(crate) game_mode: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRGameModeExtra {
    padding1: [u8; 0x10],
    game_mode_extra: i32,
    track_id: i32,
    car_id: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRCarControls {
    padding1: [u8; 0x738 + 0x5C],
    steering: f32,
    throttle: f32,
    brake: f32,
    handbrake: f32,
    clutch: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRMapInfo {
    padding1: [u8; 0x75310],
    stage_length: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMapSettings {
    unknown1: i32,
    track_id: i32,
    car_id: i32,
    unknown2: i32,
    unknown3: i32,
    transmission_type: i32,

    padding1: [u8; 0x30 - 0x14 - size_of::<i32>()],
    race_paused: i32,

    padding2: [u8; 0x38 - 0x30 - size_of::<i32>()],
    tyre_type: i32,

    padding3: [u8; 0x48 - 0x38 - size_of::<i32>()],
    weather_type: i32,
    unknown4: i32,
    damage_type: i32,
    pacecar_enabled: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMapSettingsExtra {
    unknown1: i32,
    unknown2: i32,
    track_id: i32,
    unknown3: i32,
    sky_cloud_type: i32,
    surface_wetness: i32,
    surface_age: i32,

    padding1: [u8; 0x38 - 0x18 - size_of::<i32>()],
    time_of_day: i32,
    sky_type: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRGameModeExtra2 {
    padding1: [u8; 0x10],
    loading_mode: i32,
    racing_paused: i32,
    ghost_car_id: i32,
    car_id: i32,
    track_id: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRGhostCarMovement {
    car_map_location: D3DXQuaternion,
    car_quaternion: D3DXQuaternion,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuItemPosition {
    x: u16,
    y: u16,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuItemExtra {
    padding: [u8; 0x2C],
    menu_title_id: *mut c_char,
    menu_title_name: *mut WCHAR,
    unknown1: *mut c_void,
    unknown2: i32,
    unknown3: i32,
    unknown4: i32,
    selected_item_idx: i32,
    number_of_items: i32,
    selected_item_idx2: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuItemCarSelectionCarSpecTexts {
    padding1: [u8; 0x1C],
    tech_spec_value: *mut WCHAR,

    padding2: [u8; 0x28 - 0x1C - size_of::<*mut WCHAR>()],
    model_title: *mut WCHAR,

    padding3: [u8; 0x34 - 0x28 - size_of::<*mut WCHAR>()],
    horsepower_title: *mut WCHAR,

    padding4: [u8; 0x40 - 0x34 - size_of::<*mut WCHAR>()],
    horsepower_value: *mut WCHAR,

    padding5: [u8; 0x4C - 0x40 - size_of::<*mut WCHAR>()],
    torque_title: *mut WCHAR,

    padding6: [u8; 0x58 - 0x4C - size_of::<*mut WCHAR>()],
    torque_value: *mut WCHAR,

    padding7: [u8; 0x64 - 0x58 - size_of::<*mut WCHAR>()],
    engine_title: *mut WCHAR,

    padding8: [u8; 0x70 - 0x64 - size_of::<*mut WCHAR>()],
    engine_value: *mut WCHAR,

    padding9: [u8; 0x7C - 0x70 - size_of::<*mut WCHAR>()],
    tyres_title: *mut WCHAR,

    padding10: [u8; 0x88 - 0x7C - size_of::<*mut WCHAR>()],
    weight_title: *mut WCHAR,

    padding11: [u8; 0x94 - 0x88 - size_of::<*mut WCHAR>()],
    weight_value: *mut WCHAR,

    padding12: [u8; 0xA0 - 0x94 - size_of::<*mut WCHAR>()],
    transmission_title: *mut WCHAR,

    padding13: [u8; 0xAC - 0xA0 - size_of::<*mut WCHAR>()],
    transmission_value: *mut WCHAR,
}

#[repr(C)]
pub (crate) union UnionMenuTitleId {
    sz_menu_title_id: LPCSTR,
    wsz_menu_title_id: LPCWSTR,
}
#[repr(C)]
pub (crate) union UnionMenuTitleName {
    menu_title_name: LPCWSTR,
    title_attribute: DWORD,
}

#[repr(C, packed)]
pub (crate) struct RBRPluginMenuItemObject3 {
    padding1: [u8; 0x18],
    union_menu_title_id: UnionMenuTitleId,
    union_menu_title_name: UnionMenuTitleName,
    item_name: LPCSTR,
}

#[repr(C, packed)]
pub (crate) struct RBRPluginMenuItemObject2 {
    padding1: [u8; 0x24],
    menu_title_id: LPCSTR,
    menu_title_name: LPCWSTR,
}

#[repr(C)]
pub (crate) union UnionMenuItem {
    item_position: *mut RBRMenuItemPosition,
    extra_menu_object: *mut RBRMenuItemExtra,
}
#[repr(C, packed)]
pub (crate) struct RBRMenuObject {
    padding1: [u8; 0x04],
    root_menu_object: *mut RBRMenuObject,
    prev_menu_object: *mut RBRMenuObject,
    item_object: *mut LPVOID,
    union_menu_item: UnionMenuItem,
    number_of_items: i32,
    selected_item_idx: i32,
    first_selectable_item_idx: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRMenuSystem {
    unknown1: LPVOID,
    root_menu_object: *mut RBRMenuObject,
    current_menu_object: *mut RBRMenuObject,
    current_menu_object2: *mut RBRMenuObject,

    padding1: [u8; 0x48 - 0x0C - size_of::<*mut RBRMenuObject>()],
    menu_image_position_x: f32,
    menu_image_position_y: f32,
    menu_image_width: f32,
    menu_image_height: f32,

    padding2: [u8; 0x70 - 0x54 - size_of::<i32>()],
    menu_visible: i32,
    unknown2: i32,
    menu_object: [*mut RBRMenuObject; RBRMENUSYSTEM_NUM_OF_MENUS],
}

#[repr(C, packed)]
pub (crate) struct RBRProfile {
    unknown1: LPVOID,
    unknown2: LPVOID,
    profile_name: [c_char; 16],
}

#[repr(C, packed)]
pub (crate) struct RBRStatusText {
    padding1: [u8; 0x8c],
    load_destination_title_id: *mut c_char,
    load_destination_title_name: *mut WCHAR,
    unknown1: i32,
    load_replay_title_id: *mut c_char,
    load_replay_title_name: *mut WCHAR,
    unknown2: i32,
}

#[repr(C, packed)]
pub (crate) struct RBRPacenote {
    pacenote_type: i32,
    flags: i32,
    distance: f32,
}

#[repr(C, packed)]
pub (crate) struct RBRPacenotes {
    padding1: [u8; 0x20],
    number_pacenotes: i32,
    pacenotes: *mut RBRPacenote,
}

#[repr(C)]
pub (crate) struct RBRPluginMenuSystem {
    plugins_menu_object: *mut RBRMenuObject,
    custom_plugin_menu_object: *mut RBRMenuObject,
    options_menu_object: *mut RBRMenuObject,
}