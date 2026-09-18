use std::ptr::{addr_of_mut};

use crate::raw::globals::{RBR_CAR_INFO, RBR_CAR_MOVEMENT, RBR_MAP_SETTINGS};
use crate::{
    PluginResult,
};
use crate::PluginError::WriteError;
use crate::patch::{CAR_RESET_WRITES, NopPatch};
use crate::raw::types::{D3DMatrix, D3DXQuaternion};
use crate::rbr::{CameraType, Matrix, Quaternion};

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


    /// Plain write of the whole matrix. RBR recomputes this from its physics state every frame,
    /// so on its own the car snaps back; use it to find the overwriting instruction.
    pub fn set_car_map_location(&self, location: Matrix) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_MOVEMENT.is_null() {
                return Err(WriteError(
                    "RBRCarMovement is null".to_owned(),
                ));
            }

            addr_of_mut!(
                (*RBR_CAR_MOVEMENT).car_map_location
            ).write_unaligned(D3DMatrix::from(location));
        }
        Ok(())
    }


    /// Disables RBR's car reset, which otherwise moves the car back after a large position jump.
    /// Keep it disabled for some frames after writing a new pose, then call [`Self::enable_car_reset`].
    pub fn disable_car_reset(&self) -> PluginResult<()> {
        CAR_RESET_WRITES.iter().try_for_each(NopPatch::verify)?;
        CAR_RESET_WRITES.iter().try_for_each(NopPatch::apply)
    }

    pub fn enable_car_reset(&self) -> PluginResult<()> {
        CAR_RESET_WRITES.iter().try_for_each(NopPatch::restore)
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
