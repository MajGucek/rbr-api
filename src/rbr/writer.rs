use std::ptr::{addr_of, addr_of_mut};

use crate::raw::globals::{RBR_CAR_INFO, RBR_CAR_MOVEMENT, RBR_MAP_SETTINGS};
use crate::{
    PluginResult,
};
use crate::PluginError::WriteError;
use crate::rbr::{CameraType, Matrix, Quaternion, Rbr, Vector3};

pub struct RbrWriter<'a> {
    _rbr: &'a Rbr,
}

impl<'a> RbrWriter<'a> {
    pub(crate) fn new(rbr: &'a Rbr) -> Self {
        Self { _rbr: rbr }
    }

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

            let current_absolute: Vector3 =
                addr_of!((*RBR_CAR_INFO).car_position)
                    .read_unaligned()
                    .into();

            let matrix = addr_of_mut!(
            (*RBR_CAR_MOVEMENT).car_map_location
        )
                .cast::<f32>();

            let current_local = Vector3 {
                x: matrix.add(12).read_unaligned(),
                y: matrix.add(13).read_unaligned(),
                z: matrix.add(14).read_unaligned(),
            };

            let displacement = Vector3 {
                x: target.x - current_absolute.x,
                y: target.y - current_absolute.y,
                z: target.z - current_absolute.z,
            };

            matrix
                .add(12)
                .write_unaligned(
                    current_local.x + displacement.x,
                );

            matrix
                .add(13)
                .write_unaligned(
                    current_local.y + displacement.y,
                );

            matrix
                .add(14)
                .write_unaligned(
                    current_local.z + displacement.z,
                );
        }

        Ok(())
    }


    pub (crate) fn set_car_map_location(&self, matrix: Matrix) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_MOVEMENT.is_null() {
                return Err(WriteError("RBRCarMovement is null".to_owned()));
            }
            addr_of_mut!((*RBR_CAR_MOVEMENT).car_map_location)
                .write_unaligned(matrix.into());
        }
        Ok(())
    }
    #[deprecated]
    fn set_distance_travelled(&self, distance: f32) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_INFO.is_null() {
                return Err(WriteError("RBRCarInfo is null".to_owned()));
            }
            addr_of_mut!((*RBR_CAR_INFO).distance_travelled)
                .write_unaligned(distance);
        }
        Ok(())
    }

    #[deprecated]
    fn set_distance_from_start_control(&self, distance: f32) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_INFO.is_null() {
                return Err(WriteError("RBRCarInfo is null".to_owned()));
            }
            addr_of_mut!((*RBR_CAR_INFO).distance_from_start_control)
                .write_unaligned(distance);
        }
        Ok(())
    }

    #[deprecated]
    fn set_distance_to_finish(&self, distance: f32) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_INFO.is_null() {
                return Err(WriteError("RBRCarInfo is null".to_owned()));
            }
            addr_of_mut!((*RBR_CAR_INFO).distance_to_finish)
                .write_unaligned(distance);
        }
        Ok(())
    }

    #[deprecated]
    fn set_stage_progress(&self, progress: f32) -> PluginResult<()> {
        unsafe {
            if RBR_CAR_INFO.is_null() {
                return Err(WriteError("RBRCarInfo is null".to_owned()));
            }
            addr_of_mut!((*RBR_CAR_INFO).stage_progress)
                .write_unaligned(progress);
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
