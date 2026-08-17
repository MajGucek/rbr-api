use std::ptr::{addr_of_mut};

use crate::raw::globals::{RBR_CAR_INFO, RBR_CAR_MOVEMENT, RBR_MAP_SETTINGS};
use crate::{
    PluginResult,
};
use crate::PluginError::WriteError;
use crate::rbr::{CameraType, Matrix, Vector3};

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


    pub fn set_car_absolute_position(
        &self,
        current_car_position: Vector3,
        current_matrix: Matrix,
        target: Vector3,
    ) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_INFO.is_null() {
                return Err(WriteError(
                    "RBRCarInfo is null".to_owned(),
                ));
            }

            if RBR_CAR_MOVEMENT.is_null() {
                return Err(WriteError(
                    "RBRCarMovement is null".to_owned(),
                ));
            }

            let current_local = Vector3 {
                x: current_matrix.0[3][0],
                y: current_matrix.0[3][1],
                z: current_matrix.0[3][2],
            };


            let displacement = Vector3 {
                x: target.x - current_car_position.x,
                y: target.y - current_car_position.y,
                z: target.z - current_car_position.z,
            };

            let mut mat = current_matrix.clone();

            mat.0[3][0] = current_local.x + displacement.x;
            mat.0[3][1] = current_local.y + displacement.y;
            mat.0[3][2] = current_local.z + displacement.z;


            self.set_car_map_location(mat)?;
        }

        Ok(())
    }


    pub fn set_car_map_location(&self, matrix: Matrix) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_MOVEMENT.is_null() {
                return Err(WriteError("RBRCarMovement is null".to_owned()));
            }
            addr_of_mut!((*RBR_CAR_MOVEMENT).car_map_location)
                .write_unaligned(matrix.into());
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
