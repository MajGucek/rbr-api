use crate::raw::globals::*;
use crate::rbr::game::*;

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
}