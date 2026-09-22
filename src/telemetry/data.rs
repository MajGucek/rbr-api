use crate::rbr::Vector3;

/*
 * Ported from NGP's rbr.telemetry.data.TelemetryData.h
 * Every field is 4 bytes, so the structs line up with the packet as is.
 * Temperatures are in Kelvin.
 */
pub const PACKET_SIZE: usize = 664;

const _: () = assert!(size_of::<Telemetry>() == PACKET_SIZE);

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Telemetry {
    pub total_steps: u32,
    pub stage: Stage,
    pub control: Control,
    pub car: Car,
}

impl Telemetry {
    pub fn from_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < PACKET_SIZE {
            return None;
        }

        // only f32/i32/u32 fields and no padding, so any bytes are a valid Telemetry
        let telemetry = unsafe {
            bytes.as_ptr().cast::<Self>().read_unaligned()
        };

        Some(telemetry)
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Stage {
    pub index: i32,
    pub progress: f32, // meters
    pub race_time: f32, // seconds
    pub drive_line_location: f32,
    pub distance_to_end: f32, // meters
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Control {
    pub steering: f32,
    pub throttle: f32, // 0..1, same for brake, handbrake and clutch
    pub brake: f32,
    pub handbrake: f32,
    pub clutch: f32,
    pub gear: i32, // 0 = reverse, 1 = neutral, 2 = first...
    pub footbrake_pressure: f32,
    pub handbrake_pressure: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Car {
    pub index: i32,
    pub speed: f32,
    pub position: Vector3,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocities: Motion,
    pub accelerations: Motion,
    pub engine: Engine,
    pub suspension_lf: Suspension,
    pub suspension_rf: Suspension,
    pub suspension_lb: Suspension,
    pub suspension_rb: Suspension,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Motion {
    pub surge: f32,
    pub sway: f32,
    pub heave: f32,
    pub roll: f32,
    pub pitch: f32,
    pub yaw: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Engine {
    pub rpm: f32,
    pub radiator_coolant_temperature: f32,
    pub engine_coolant_temperature: f32,
    pub engine_temperature: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Suspension {
    pub spring_deflection: f32,
    pub rollbar_force: f32, // [N]
    pub spring_force: f32, // [N]
    pub damper_force: f32, // [N]
    pub strut_force: f32, // [N]
    pub helper_spring_is_active: i32,
    pub damper: Damper,
    pub wheel: Wheel,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Damper {
    pub damage: f32,
    pub piston_velocity: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Wheel {
    pub brake_disk: BrakeDisk,
    pub tire: Tire,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct BrakeDisk {
    pub layer_temperature: f32,
    pub temperature: f32,
    pub wear: f32,
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Tire {
    pub pressure: f32,
    pub temperature: f32,
    pub carcass_temperature: f32,
    pub tread_temperature: f32,
    pub current_segment: u32,
    pub segments: [TireSegment; 8],
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct TireSegment {
    pub temperature: f32,
    pub wear: f32,
}
