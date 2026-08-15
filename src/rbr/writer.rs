use std::ptr::addr_of_mut;

use crate::raw::globals::{
    RBR_CAR_INFO,
    RBR_MAP_SETTINGS,
    RBR_MAP_SETTINGS_EXT,
};
use crate::rbr::game::{
    DamageType,
    SkyCloudType,
    SkyType,
    SurfaceAge,
    SurfaceWetness,
    TimeOfDay,
    TransmissionType,
    TyreType,
    WeatherType,
};
use crate::{
    PluginError,
    PluginResult,
};
use crate::rbr::Rbr;

pub struct RbrWriter<'a> {
    _rbr: &'a Rbr,
}

impl<'a> RbrWriter<'a> {
    pub(crate) fn new(rbr: &'a Rbr) -> Self {
        Self { _rbr: rbr }
    }

    /*
     * ----RACE----
     */

    pub fn set_race_paused(
        &self,
        paused: bool,
    ) -> PluginResult<()> {
        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            addr_of_mut!((*settings).race_paused)
                .write_unaligned(i32::from(paused));
        }

        Ok(())
    }

    /// RBR normally changes this value continuously.
    ///
    /// To freeze the countdown, call this repeatedly from
    /// UpdateEvent rather than only once.
    pub fn set_stage_start_countdown(
        &self,
        countdown: f32,
    ) -> PluginResult<()> {
        if !countdown.is_finite() {
            return Err(PluginError::WriteError(
                "Stage countdown must be finite".to_owned(),
            ));
        }

        unsafe {
            let car_info = require_pointer(
                RBR_CAR_INFO,
                "RBR car info",
            )?;

            addr_of_mut!((*car_info).stage_start_countdown)
                .write_unaligned(countdown);
        }

        Ok(())
    }

    /*
     * ----NEXT STAGE----
     */

    pub fn set_track_id(
        &self,
        track_id: i32,
    ) -> PluginResult<()> {
        if track_id < 0 {
            return Err(PluginError::WriteError(
                format!("Invalid track ID: {track_id}"),
            ));
        }

        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            let extra = require_pointer(
                RBR_MAP_SETTINGS_EXT,
                "RBR extended map settings",
            )?;

            addr_of_mut!((*settings).track_id)
                .write_unaligned(track_id);

            // Both structures contain the selected track ID.
            addr_of_mut!((*extra).track_id)
                .write_unaligned(track_id);
        }

        Ok(())
    }

    pub fn set_car_id(
        &self,
        car_id: i32,
    ) -> PluginResult<()> {
        if !(0..=7).contains(&car_id) {
            return Err(PluginError::WriteError(
                format!("Car ID must be 0..=7, got {car_id}"),
            ));
        }

        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            addr_of_mut!((*settings).car_id)
                .write_unaligned(car_id);
        }

        Ok(())
    }

    pub fn set_transmission_type(
        &self,
        transmission: TransmissionType,
    ) -> PluginResult<()> {
        let value = checked_value(
            "transmission type",
            transmission.raw(),
            0,
            1,
        )?;

        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            addr_of_mut!((*settings).transmission_type)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_tyre_type(
        &self,
        tyre: TyreType,
    ) -> PluginResult<()> {
        let value = checked_value(
            "tyre type",
            tyre.raw(),
            0,
            6,
        )?;

        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            addr_of_mut!((*settings).tyre_type)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_weather_type(
        &self,
        weather: WeatherType,
    ) -> PluginResult<()> {
        let value = checked_value(
            "weather type",
            weather.raw(),
            0,
            2,
        )?;

        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            addr_of_mut!((*settings).weather_type)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_damage_type(
        &self,
        damage: DamageType,
    ) -> PluginResult<()> {
        let value = checked_value(
            "damage type",
            damage.raw(),
            0,
            3,
        )?;

        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            addr_of_mut!((*settings).damage_type)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_pacecar_enabled(
        &self,
        enabled: bool,
    ) -> PluginResult<()> {
        unsafe {
            let settings = require_pointer(
                RBR_MAP_SETTINGS,
                "RBR map settings",
            )?;

            addr_of_mut!((*settings).pacecar_enabled)
                .write_unaligned(i32::from(enabled));
        }

        Ok(())
    }

    /*
     * ----NEXT STAGE ENVIRONMENT----
     */

    pub fn set_sky_cloud_type(
        &self,
        cloud: SkyCloudType,
    ) -> PluginResult<()> {
        let value = checked_value(
            "sky cloud type",
            cloud.raw(),
            0,
            3,
        )?;

        unsafe {
            let extra = require_pointer(
                RBR_MAP_SETTINGS_EXT,
                "RBR extended map settings",
            )?;

            addr_of_mut!((*extra).sky_cloud_type)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_surface_wetness(
        &self,
        wetness: SurfaceWetness,
    ) -> PluginResult<()> {
        let value = checked_value(
            "surface wetness",
            wetness.raw(),
            0,
            2,
        )?;

        unsafe {
            let extra = require_pointer(
                RBR_MAP_SETTINGS_EXT,
                "RBR extended map settings",
            )?;

            addr_of_mut!((*extra).surface_wetness)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_surface_age(
        &self,
        age: SurfaceAge,
    ) -> PluginResult<()> {
        let value = checked_value(
            "surface age",
            age.raw(),
            0,
            2,
        )?;

        unsafe {
            let extra = require_pointer(
                RBR_MAP_SETTINGS_EXT,
                "RBR extended map settings",
            )?;

            addr_of_mut!((*extra).surface_age)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_time_of_day(
        &self,
        time: TimeOfDay,
    ) -> PluginResult<()> {
        let value = checked_value(
            "time of day",
            time.raw(),
            0,
            2,
        )?;

        unsafe {
            let extra = require_pointer(
                RBR_MAP_SETTINGS_EXT,
                "RBR extended map settings",
            )?;

            addr_of_mut!((*extra).time_of_day)
                .write_unaligned(value);
        }

        Ok(())
    }

    pub fn set_sky_type(
        &self,
        sky: SkyType,
    ) -> PluginResult<()> {
        let value = checked_value(
            "sky type",
            sky.raw(),
            0,
            9,
        )?;

        unsafe {
            let extra = require_pointer(
                RBR_MAP_SETTINGS_EXT,
                "RBR extended map settings",
            )?;

            addr_of_mut!((*extra).sky_type)
                .write_unaligned(value);
        }

        Ok(())
    }
}

fn require_pointer<T>(
    pointer: *mut T,
    name: &'static str,
) -> PluginResult<*mut T> {
    if pointer.is_null() {
        Err(PluginError::WriteError(
            format!("{name} is unavailable"),
        ))
    } else {
        Ok(pointer)
    }
}

fn checked_value(
    name: &str,
    value: i32,
    minimum: i32,
    maximum: i32,
) -> PluginResult<i32> {
    if (minimum..=maximum).contains(&value) {
        Ok(value)
    } else {
        Err(PluginError::WriteError(
            format!(
                "Invalid {name}: {value}; expected \
                 {minimum}..={maximum}"
            ),
        ))
    }
}