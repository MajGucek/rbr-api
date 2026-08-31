use std::ptr::{addr_of_mut};

use crate::raw::globals::{RBR_CAR_INFO, RBR_CAR_MOVEMENT, RBR_MAP_SETTINGS};
use crate::{
    PluginResult,
};
use crate::PluginError::WriteError;
use crate::raw::types::D3DXQuaternion;
use crate::rbr::{CameraType, Quaternion, Vector3};

pub struct RbrWriter {}

impl RbrWriter {
    pub fn set_stage_start_countdown(&self, value: f32) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_INFO.is_null() {
                return Err(WriteError("RBRCarInfo is null".to_owned()));
            }
            addr_of_mut!(
                (*RBR_CAR_INFO).stage_start_countdown
            ).write_unaligned(value)
        }
        Ok(())
    }


    pub fn set_car_quaternion(&self, quat: Quaternion) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_MOVEMENT.is_null() {
                return Err(WriteError(
                    "RBRCarMovement is null".to_owned(),
                ));
            }

            addr_of_mut!(
                (*RBR_CAR_MOVEMENT).car_quaternion
            ).write_unaligned(D3DXQuaternion::from(quat));
        }
        Ok(())
    }


    /// You can write to this field, but some other auth-state also overwrites you.
    /// TODO: check in Cheat Engine which instruction overwrites and write nop instead.
    pub fn set_car_absolute_position(
        &self,
        target: Vector3,
    ) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_MOVEMENT.is_null() {
                return Err(WriteError(
                    "RBRCarMovement is null".to_owned(),
                ));
            }

            let matrix = addr_of_mut!(
                (*RBR_CAR_MOVEMENT).car_map_location
            )
                .cast::<f32>();

            matrix.add(12).write_unaligned(target.x);
            matrix.add(13).write_unaligned(target.y);
            matrix.add(14).write_unaligned(target.z);
        }

        Ok(())
    }


    pub fn set_camera_type(&self, camera_type: CameraType) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_INFO.is_null() {
                return Err(WriteError("RBRCarInfo is null".to_owned()));
            }
            if (*RBR_CAR_INFO).camera.is_null() {
                return Err(WriteError("Camera is null in RBRCarInfo".to_owned()));
            }
            if (*(*RBR_CAR_INFO).camera).camera_info.is_null() {
                return Err(WriteError("CameraInfo is null in RBRCarInfo->camera".to_owned()));
            }

            addr_of_mut!(
                (*(*(*RBR_CAR_INFO).camera).camera_info).camera_type
            ).write_unaligned(i32::from(camera_type));
        }

        Ok(())
    }

    pub fn set_race_paused(&self, paused: bool) -> PluginResult<()> {
        unsafe {
            if RBR_MAP_SETTINGS.is_null() {
                return Err(WriteError("RBRMapSettings are null".to_owned()));
            }

            addr_of_mut!((*RBR_MAP_SETTINGS).race_paused)
                .write_unaligned(i32::from(paused));
        }

        Ok(())
    }
}
