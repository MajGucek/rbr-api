use std::ffi::c_char;

use crate::raw::globals::*;
use crate::raw::types::{
    D3DMatrix,
    D3DXQuaternion,
    D3DXVector3,
    RBRControllerAxis,
    RBRControllerAxisData,
    RBRControllerObject,
    RBRPacenote,
};
use crate::rbr::game::*;
use crate::rbr::math::{Matrix, Quaternion, Vector3};
use super::Rbr;

macro_rules! read_rbr_field {
    ($global:ident, $field:ident) => {{
        unsafe {
            let base = $global;

            if base.is_null() {
                None
            } else {
                Some(
                    std::ptr::addr_of!(
                        (*base).$field
                    )
                    .read_unaligned()
                )
            }
        }
    }};
}

fn vector3_from_raw(value: D3DXVector3) -> Vector3 {
    Vector3 {
        x: value.x,
        y: value.y,
        z: value.z,
    }
}

fn quaternion_from_raw(value: D3DXQuaternion) -> Quaternion {
    Quaternion {
        x: value.x,
        y: value.y,
        z: value.z,
        w: value.w,
    }
}

fn matrix_from_raw(value: D3DMatrix) -> Matrix {
    let values = unsafe {
        std::ptr::addr_of!(value)
            .cast::<[[f32; 4]; 4]>()
            .read_unaligned()
    };

    Matrix::from_raw(values)
}

const MAX_RBR_STRING_LENGTH: usize = 4096;

unsafe fn string_from_c_pointer(pointer: *const c_char) -> Option<String> {
    unsafe {
        if pointer.is_null() {
            return None;
        }

        let pointer = pointer.cast::<u8>();
        let mut bytes = Vec::new();

        for index in 0..MAX_RBR_STRING_LENGTH {
            let value = pointer
                .add(index)
                .read_unaligned();

            if value == 0 {
                return Some(
                    String::from_utf8_lossy(&bytes)
                        .into_owned()
                );
            }

            bytes.push(value);
        }

        None
    }
}

unsafe fn string_from_wide_pointer(pointer: *const u16) -> Option<String> {
    unsafe {
        if pointer.is_null() {
            return None;
        }

        let mut values = Vec::new();

        for index in 0..MAX_RBR_STRING_LENGTH {
            let value = pointer
                .add(index)
                .read_unaligned();

            if value == 0 {
                return Some(
                    String::from_utf16_lossy(&values)
                );
            }

            values.push(value);
        }

        None
    }
}

fn string_from_c_array<const LENGTH: usize>(values: [c_char; LENGTH]) -> String {
    let bytes: Vec<u8> = values
        .into_iter()
        .take_while(|value| *value != 0)
        .map(|value| value as u8)
        .collect();

    String::from_utf8_lossy(&bytes)
        .into_owned()
}

fn with_controller_object<T>(read: impl FnOnce(*const RBRControllerObject) -> Option<T>) -> Option<T> {
    let controller_base_object = read_rbr_field!(
        RBR_GAME_CONFIG,
        controller_base_object
    )?;

    let controller_object = read_rbr_field!(
        controller_base_object,
        controller_object
    )?;

    if controller_object.is_null() {
        None
    } else {
        read(controller_object)
    }
}

fn with_controller_axis<T>(axis: ControllerAxis, read: impl FnOnce(*const RBRControllerAxis) -> Option<T>) -> Option<T> {
    let axis_index = axis as usize;

    if axis_index >= 21 {
        return None;
    }

    with_controller_object(|controller_object| {
        let controller_axis = unsafe {
            std::ptr::addr_of!(
                (*controller_object).controller_axis
            )
                .cast::<RBRControllerAxis>()
                .add(axis_index)
        };

        read(controller_axis)
    })
}

fn with_controller_axis_data<T>(axis: ControllerAxis, read: impl FnOnce(*const RBRControllerAxisData) -> Option<T>) -> Option<T> {
    with_controller_axis(axis, |controller_axis| {
        let axis_data = read_rbr_field!(
            controller_axis,
            controller_axis_data
        )?;

        if axis_data.is_null() {
            None
        } else {
            read(axis_data)
        }
    })
}

fn with_pacenote<T>(index: usize, read: impl FnOnce(*const RBRPacenote) -> Option<T>) -> Option<T> {
    let number_of_pacenotes = read_rbr_field!(
        RBR_PACENOTES,
        number_pacenotes
    )?;

    if number_of_pacenotes < 0 || index >= number_of_pacenotes as usize {
        return None;
    }

    let pacenotes = read_rbr_field!(
        RBR_PACENOTES,
        pacenotes
    )?;

    if pacenotes.is_null() {
        return None;
    }

    let pacenote = unsafe {
        pacenotes.add(index)
    };

    read(pacenote)
}

pub struct RbrReader<'a> {
    rbr: &'a Rbr,
}

impl<'a> RbrReader<'a> {
    pub(crate) fn new(rbr: &'a Rbr) -> Self {
        Self { rbr }
    }

    /*
     * ----CAMERA----
     */
    pub fn get_camera_type(&self) -> Option<CameraType> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            camera_type
        ).map(CameraType::from_raw)
    }

    pub fn get_camera_fov(&self) -> Option<f32> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            camera_fov
        )
    }

    pub fn get_camera_near(&self) -> Option<f32> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            camera_near
        )
    }

    pub fn get_current_camera_map_location(&self) -> Option<Matrix> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            current_camera_map_location
        ).map(matrix_from_raw)
    }

    pub fn get_camera_orientation(&self) -> Option<Vector3> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            camera_orientation
        ).map(vector3_from_raw)
    }

    pub fn get_camera_pov1(&self) -> Option<Vector3> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            camera_pov1
        ).map(vector3_from_raw)
    }

    pub fn get_camera_pov2(&self) -> Option<Vector3> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            camera_pov2
        ).map(vector3_from_raw)
    }

    pub fn get_camera_position(&self) -> Option<Vector3> {
        let camera = read_rbr_field!(
            RBR_CAR_INFO,
            camera
        )?;

        let camera_info = read_rbr_field!(
            camera,
            camera_info
        )?;

        read_rbr_field!(
            camera_info,
            camera_position
        ).map(vector3_from_raw)
    }


    /*
     * ----GAME----
     */
    pub fn get_game_mode(&self) -> Option<GameMode> {
        read_rbr_field!(
            RBR_GAME_MODE,
            game_mode
        ).map(GameMode::from_raw)
    }

    pub fn get_game_mode_extra(&self) -> Option<GameModeExtra> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT,
            game_mode_extra
        ).map(GameModeExtra::from_raw)
    }

    pub fn get_resolution_x(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_CONFIG,
            resolution_x
        )
    }

    pub fn get_resolution_y(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_CONFIG,
            resolution_y
        )
    }

    pub fn get_current_track_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT,
            track_id
        )
    }

    pub fn get_current_car_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT,
            car_id
        )
    }

    pub fn get_loading_mode(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT2,
            loading_mode
        )
    }

    pub fn get_racing_paused(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT2,
            racing_paused
        ).map(|value| value != 0)
    }

    pub fn get_loading_track_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT2,
            track_id
        )
    }

    pub fn get_loading_car_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT2,
            car_id
        )
    }


    /*
     * ----CAR----
     */
    pub fn get_hud_position_x(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            hud_position_x
        )
    }

    pub fn get_hud_position_y(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            hud_position_y
        )
    }

    pub fn get_speed(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            speed
        )
    }

    pub fn get_rpm(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            rpm
        )
    }

    pub fn get_temperature(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            temperature
        )
    }

    pub fn get_turbo(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            turbo
        )
    }

    pub fn get_gear(&self) -> Option<Gear> {
        read_rbr_field!(
            RBR_CAR_INFO,
            gear
        ).map(Gear::from_raw)
    }

    pub fn get_driving_wrong_way(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_CAR_INFO,
            driving_direction
        ).map(|value| value != 0)
    }

    pub fn get_wrong_way_message_fade(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            fade_wrong_way_message
        )
    }

    pub fn get_car_position(&self) -> Option<Vector3> {
        read_rbr_field!(
            RBR_CAR_INFO,
            car_position
        ).map(vector3_from_raw)
    }


    /*
     * ----CAR MOVEMENT----
     */
    pub fn get_car_quaternion(&self) -> Option<Quaternion> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            car_quaternion
        ).map(quaternion_from_raw)
    }

    pub fn get_car_map_location(&self) -> Option<Matrix> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            car_map_location
        ).map(matrix_from_raw)
    }

    pub fn get_car_spin(&self) -> Option<Vector3> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            spin
        ).map(vector3_from_raw)
    }

    pub fn get_car_movement_speed(&self) -> Option<Vector3> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            speed
        ).map(vector3_from_raw)
    }


    /*
     * ----CAR CONTROLS----
     */
    pub fn get_steering(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_CONTROLS,
            steering
        )
    }

    pub fn get_throttle(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_CONTROLS,
            throttle
        )
    }

    pub fn get_brake(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_CONTROLS,
            brake
        )
    }

    pub fn get_handbrake(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_CONTROLS,
            handbrake
        )
    }

    pub fn get_clutch(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_CONTROLS,
            clutch
        )
    }

    pub fn get_drive_throttle(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            drive_throttle
        )
    }

    pub fn get_drive_brake(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            drive_brake
        )
    }

    pub fn get_drive_handbrake(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            drive_handbrake
        )
    }

    pub fn get_drive_steering(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            drive_steering
        )
    }

    pub fn get_drive_clutch(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_MOVEMENT,
            drive_clutch
        )
    }


    /*
     * ----CONTROLLER----
     */
    pub fn get_throttle_inverted(&self) -> Option<bool> {
        with_controller_object(|controller_object| {
            read_rbr_field!(
                controller_object,
                throttle_inverted
            )
        }).map(|value| value != 0)
    }

    pub fn get_brake_inverted(&self) -> Option<bool> {
        with_controller_object(|controller_object| {
            read_rbr_field!(
                controller_object,
                brake_inverted
            )
        }).map(|value| value != 0)
    }

    pub fn get_combined_throttle_brake_inverted(&self) -> Option<bool> {
        with_controller_object(|controller_object| {
            read_rbr_field!(
                controller_object,
                combined_throttle_brake_inverted
            )
        }).map(|value| value != 0)
    }

    pub fn get_handbrake_inverted(&self) -> Option<bool> {
        with_controller_object(|controller_object| {
            read_rbr_field!(
                controller_object,
                handbrake_inverted
            )
        }).map(|value| value != 0)
    }

    pub fn get_clutch_inverted(&self) -> Option<bool> {
        with_controller_object(|controller_object| {
            read_rbr_field!(
                controller_object,
                clutch_inverted
            )
        }).map(|value| value != 0)
    }

    pub fn get_controller_axis_status(
        &self,
        axis: ControllerAxis,
    ) -> Option<i32> {
        with_controller_axis_data(axis, |axis_data| {
            read_rbr_field!(
                axis_data,
                status
            )
        })
    }

    pub fn get_controller_axis_dinput_status(
        &self,
        axis: ControllerAxis,
    ) -> Option<i32> {
        with_controller_axis_data(axis, |axis_data| {
            read_rbr_field!(
                axis_data,
                dinput_status
            )
        })
    }

    pub fn get_controller_axis_value(
        &self,
        axis: ControllerAxis,
    ) -> Option<f32> {
        with_controller_axis_data(axis, |axis_data| {
            read_rbr_field!(
                axis_data,
                axis_value
            )
        })
    }

    pub fn get_controller_axis_raw_value(
        &self,
        axis: ControllerAxis,
    ) -> Option<u32> {
        with_controller_axis_data(axis, |axis_data| {
            read_rbr_field!(
                axis_data,
                axis_raw_value
            )
        })
    }

    pub fn get_controller_axis_value2(
        &self,
        axis: ControllerAxis,
    ) -> Option<f32> {
        with_controller_axis_data(axis, |axis_data| {
            read_rbr_field!(
                axis_data,
                axis_value2
            )
        })
    }

    pub fn get_controller_axis_raw_value2(
        &self,
        axis: ControllerAxis,
    ) -> Option<u32> {
        with_controller_axis_data(axis, |axis_data| {
            read_rbr_field!(
                axis_data,
                axis_raw_value2
            )
        })
    }

    pub fn get_controller_axis_name_id(
        &self,
        axis: ControllerAxis,
    ) -> Option<String> {
        with_controller_axis(axis, |controller_axis| {
            let pointer = read_rbr_field!(
                controller_axis,
                axis_name_id
            )?;

            unsafe {
                string_from_c_pointer(pointer)
            }
        })
    }

    pub fn get_controller_axis_name(
        &self,
        axis: ControllerAxis,
    ) -> Option<String> {
        with_controller_axis(axis, |controller_axis| {
            let pointer = read_rbr_field!(
                controller_axis,
                axis_name
            )?;

            unsafe {
                string_from_wide_pointer(pointer)
            }
        })
    }


    /*
     * ----RACE----
     */
    pub fn get_race_started(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_CAR_INFO,
            race_started
        ).map(|value| value != 0)
    }

    pub fn get_race_finished(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_CAR_INFO,
            race_finished
        ).map(|value| value != 0)
    }

    pub fn get_distance_from_start_control(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            distance_from_start_control
        )
    }

    pub fn get_distance_travelled(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            distance_travelled
        )
    }

    pub fn get_distance_to_finish(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            distance_to_finish
        )
    }

    pub fn get_stage_progress(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            stage_progress
        )
    }

    pub fn get_race_time(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            race_time
        )
    }

    pub fn get_stage_start_countdown(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            stage_start_countdown
        )
    }

    pub fn get_false_start(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_CAR_INFO,
            false_start
        ).map(|value| value != 0)
    }

    pub fn get_split_reached(&self) -> Option<SplitReached> {
        read_rbr_field!(
            RBR_CAR_INFO,
            split_reached_number
        ).map(SplitReached::from_raw)
    }

    pub fn get_split1_time(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            split1_time
        )
    }

    pub fn get_split2_time(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_CAR_INFO,
            split2_time
        )
    }

    pub fn get_finish_line_passed(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_CAR_INFO,
            finish_line_passed
        ).map(|value| value != 0)
    }


    /*
     * ----MAP----
     */
    pub fn get_stage_length(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_MAP_INFO,
            stage_length
        )
    }

    pub fn get_map_track_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            track_id
        )
    }

    pub fn get_map_car_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            car_id
        )
    }

    pub fn get_transmission_type(&self) -> Option<TransmissionType> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            transmission_type
        ).map(TransmissionType::from_raw)
    }

    pub fn get_race_paused(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            race_paused
        ).map(|value| value != 0)
    }

    pub fn get_tyre_type(&self) -> Option<TyreType> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            tyre_type
        ).map(TyreType::from_raw)
    }

    pub fn get_weather_type(&self) -> Option<WeatherType> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            weather_type
        ).map(WeatherType::from_raw)
    }

    pub fn get_damage_type(&self) -> Option<DamageType> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            damage_type
        ).map(DamageType::from_raw)
    }

    pub fn get_pacecar_enabled(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_MAP_SETTINGS,
            pacecar_enabled
        ).map(|value| value != 0)
    }

    pub fn get_map_extra_track_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_MAP_SETTINGS_EXT,
            track_id
        )
    }

    pub fn get_sky_cloud_type(&self) -> Option<SkyCloudType> {
        read_rbr_field!(
            RBR_MAP_SETTINGS_EXT,
            sky_cloud_type
        ).map(SkyCloudType::from_raw)
    }

    pub fn get_surface_wetness(&self) -> Option<SurfaceWetness> {
        read_rbr_field!(
            RBR_MAP_SETTINGS_EXT,
            surface_wetness
        ).map(SurfaceWetness::from_raw)
    }

    pub fn get_surface_age(&self) -> Option<SurfaceAge> {
        read_rbr_field!(
            RBR_MAP_SETTINGS_EXT,
            surface_age
        ).map(SurfaceAge::from_raw)
    }

    pub fn get_time_of_day(&self) -> Option<TimeOfDay> {
        read_rbr_field!(
            RBR_MAP_SETTINGS_EXT,
            time_of_day
        ).map(TimeOfDay::from_raw)
    }

    pub fn get_sky_type(&self) -> Option<SkyType> {
        read_rbr_field!(
            RBR_MAP_SETTINGS_EXT,
            sky_type
        ).map(SkyType::from_raw)
    }

    pub fn get_map_location_name(&self) -> Option<String> {
        unsafe {
            string_from_wide_pointer(
                RBR_MAP_LOCATION_NAME
            )
        }
    }


    /*
     * ----PROFILE----
     */
    pub fn get_profile_name(&self) -> Option<String> {
        read_rbr_field!(
            RBR_PROFILE,
            profile_name
        ).map(string_from_c_array)
    }


    /*
     * ----STATUS TEXT----
     */
    pub fn get_load_destination_title_id(&self) -> Option<String> {
        let pointer = read_rbr_field!(
            RBR_STATUS_TEXT,
            load_destination_title_id
        )?;

        unsafe {
            string_from_c_pointer(pointer)
        }
    }

    pub fn get_load_destination_title_name(&self) -> Option<String> {
        let pointer = read_rbr_field!(
            RBR_STATUS_TEXT,
            load_destination_title_name
        )?;

        unsafe {
            string_from_wide_pointer(pointer)
        }
    }

    pub fn get_load_replay_title_id(&self) -> Option<String> {
        let pointer = read_rbr_field!(
            RBR_STATUS_TEXT,
            load_replay_title_id
        )?;

        unsafe {
            string_from_c_pointer(pointer)
        }
    }

    pub fn get_load_replay_title_name(&self) -> Option<String> {
        let pointer = read_rbr_field!(
            RBR_STATUS_TEXT,
            load_replay_title_name
        )?;

        unsafe {
            string_from_wide_pointer(pointer)
        }
    }


    /*
     * ----GHOST CAR----
     */
    pub fn get_ghost_car_replay_mode(&self) -> Option<GhostCarReplayMode> {
        unsafe {
            let base = RBR_GHOST_CAR_REPLAY_MODE;

            if base.is_null() {
                None
            } else {
                Some(base.read_unaligned())
            }
        }.map(GhostCarReplayMode::from_raw)
    }

    pub fn get_ghost_car_id(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_GAME_MODE_EXT2,
            ghost_car_id
        )
    }

    pub fn get_ghost_car_map_location(&self) -> Option<Quaternion> {
        read_rbr_field!(
            RBR_GHOST_CAR_MOVEMENT,
            car_map_location
        ).map(quaternion_from_raw)
    }

    pub fn get_ghost_car_quaternion(&self) -> Option<Quaternion> {
        read_rbr_field!(
            RBR_GHOST_CAR_MOVEMENT,
            car_quaternion
        ).map(quaternion_from_raw)
    }


    /*
     * ----MENU----
     */
    pub fn get_menu_background_r(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_COLOR_TABLE,
            menu_background_r
        )
    }

    pub fn get_menu_background_g(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_COLOR_TABLE,
            menu_background_g
        )
    }

    pub fn get_menu_background_b(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_COLOR_TABLE,
            menu_background_b
        )
    }

    pub fn get_menu_background_a(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_COLOR_TABLE,
            menu_background_a
        )
    }

    pub fn get_menu_image_position_x(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_MENU_SYSTEM,
            menu_image_position_x
        )
    }

    pub fn get_menu_image_position_y(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_MENU_SYSTEM,
            menu_image_position_y
        )
    }

    pub fn get_menu_image_width(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_MENU_SYSTEM,
            menu_image_width
        )
    }

    pub fn get_menu_image_height(&self) -> Option<f32> {
        read_rbr_field!(
            RBR_MENU_SYSTEM,
            menu_image_height
        )
    }

    pub fn get_menu_visible(&self) -> Option<bool> {
        read_rbr_field!(
            RBR_MENU_SYSTEM,
            menu_visible
        ).map(|value| value != 0)
    }


    /*
     * ----PACENOTES----
     */
    pub fn get_number_of_pacenotes(&self) -> Option<i32> {
        read_rbr_field!(
            RBR_PACENOTES,
            number_pacenotes
        )
    }

    pub fn get_pacenote_type(&self, index: usize) -> Option<i32> {
        with_pacenote(index, |pacenote| {
            read_rbr_field!(
                pacenote,
                pacenote_type
            )
        })
    }

    pub fn get_pacenote_flags(&self, index: usize) -> Option<i32> {
        with_pacenote(index, |pacenote| {
            read_rbr_field!(
                pacenote,
                flags
            )
        })
    }

    pub fn get_pacenote_distance(&self, index: usize) -> Option<f32> {
        with_pacenote(index, |pacenote| {
            read_rbr_field!(
                pacenote,
                distance
            )
        })
    }
}