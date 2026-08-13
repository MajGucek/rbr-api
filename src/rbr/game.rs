macro_rules! rbr_value_enum {
    (
        $(#[$enum_meta:meta])*
        pub enum $name:ident {
            $(
                $(#[$variant_meta:meta])*
                $variant:ident = $raw:literal
            ),+ $(,)?
        }
    ) => {
        $(#[$enum_meta])*
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum $name {
            $(
                $(#[$variant_meta])*
                $variant,
            )+
            // Non-documented value by RBRAPI.
            Unknown(i32),
        }

        impl $name {
            pub const fn from_raw(value: i32) -> Self {
                match value {
                    $(
                        $raw => Self::$variant,
                    )+
                    value => Self::Unknown(value),
                }
            }

            pub const fn raw(self) -> i32 {
                match self {
                    $(
                        Self::$variant => $raw,
                    )+
                    Self::Unknown(value) => value,
                }
            }

            // Helpers
            pub const fn is_known(self) -> bool {
                !matches!(self, Self::Unknown(_))
            }
        }

        impl From<i32> for $name {
            fn from(value: i32) -> Self {
                Self::from_raw(value)
            }
        }

        impl From<$name> for i32 {
            fn from(value: $name) -> Self {
                value.raw()
            }
        }
    };
}

/*
 * Camera
*/

rbr_value_enum! {
    pub enum CameraType {
        ExternalBackNear = 0x01,
        ExternalBack = 0x02,
        Bumper = 0x03,
        Bonnet = 0x04,
        Internal = 0x05,
        InternalBackseat = 0x06,
        RoadsideReplay = 0x07,
        BirdsEye = 0x09,
        SpinAround = 0x0A,
        Chase = 0x0B,
    }
}

/*
 * Game state
*/

rbr_value_enum! {
    pub enum GameMode {
        // active stage OR countdown < 5sec
        Driving = 0x01,

        Pause = 0x02,

        /// main menu OR plugin menu
        Menu = 0x03,

        LoadingTrack = 0x05,

        ExitingToMenu = 0x06,

        Replay = 0x08,

        // lesson, stage, retirement, replay is ending
        SessionEnding = 0x09,

        // Camera spinning around the car
        PreStart = 0x0A,

        MenuTransition = 0x0C,

        // before PreStart
        LoadingCompleteTransition = 0x0D,
    }
}

rbr_value_enum! {
    pub enum GameModeExtra {
        // Racing is active and car movement is normally updated.
        RacingActive = 0x00,

        LoadingReplay = 0x01,

        // Replay movement is active.
        Replay = 0x02,

        // A plugin menu is open.
        PluginMenu = 0x03,

        // Replay movement is paused.
        ReplayPaused = 0x04,
    }
}

/*
 * Vehicle
*/

rbr_value_enum! {
    pub enum Gear {
        Reverse = 0,
        Neutral = 1,
        First = 2,
        Second = 3,
        Third = 4,
        Fourth = 5,
        Fifth = 6,
    }
}

rbr_value_enum! {
    pub enum TransmissionType {
        Manual = 0,
        Automatic = 1,
    }
}

/*
 * Race progress
*/

rbr_value_enum! {
    pub enum SplitReached {
        StartLine = 0,
        FirstSplit = 1,
        SecondSplit = 2,
    }
}

rbr_value_enum! {
    pub enum GhostCarReplayMode {
        Disabled = 0,
        Recording = 1,
        Replaying = 2,
    }
}

/*
 * Stage configuration
*/

rbr_value_enum! {
    pub enum TyreType {
        DryTarmac = 0,
        IntermediateTarmac = 1,
        WetTarmac = 2,
        DryGravel = 3,
        IntermediateGravel = 4,
        WetGravel = 5,
        Snow = 6,
    }
}

rbr_value_enum! {
    pub enum WeatherType {
        Good = 0,
        Random = 1,
        Bad = 2,
    }
}

rbr_value_enum! {
    pub enum DamageType {
        Disabled = 0,
        Safe = 1,
        Reduced = 2,
        Realistic = 3,
    }
}

rbr_value_enum! {
    pub enum SkyCloudType {
        Clear = 0,
        PartlyCloudy = 1,
        LightCloud = 2,
        HeavyCloud = 3,
    }
}

rbr_value_enum! {
    pub enum SurfaceWetness {
        Dry = 0,
        Damp = 1,
        Wet = 2,
    }
}

rbr_value_enum! {
    pub enum SurfaceAge {
        New = 0,
        Normal = 1,
        Worn = 2,
    }
}

rbr_value_enum! {
    pub enum TimeOfDay {
        Morning = 0,
        Noon = 1,
        Evening = 2,
    }
}

rbr_value_enum! {
    pub enum SkyType {
        Crisp = 0,
        Hazy = 1,
        NoRain = 2,
        LightRain = 3,
        HeavyRain = 4,
        NoSnow = 5,
        LightSnow = 6,
        HeavySnow = 7,
        LightFog = 8,
        HeavyFog = 9,
    }
}

/*
 * Controller
*/

// An index into RBR's fixed array of 21 controller inputs.
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ControllerAxis {
    SteeringAnalog = 0,
    LeftDigital = 1,
    RightDigital = 2,
    ThrottleAnalog = 3,
    CombinedThrottleBrakeAnalog = 4,
    Brake = 5,
    Handbrake = 6,
    GearUp = 7,
    GearDown = 8,
    ChangeCamera = 9,
    Pause = 10,
    Clutch = 11,
    Ignition = 12,
    Reverse = 13,
    Neutral = 14,
    FirstGear = 15,
    SecondGear = 16,
    ThirdGear = 17,
    FourthGear = 18,
    FifthGear = 19,
    SixthGear = 20,
}

impl ControllerAxis {
    pub const fn index(self) -> usize {
        self as usize
    }

    pub const fn from_index(index: usize) -> Option<Self> {
        match index {
            0 => Some(Self::SteeringAnalog),
            1 => Some(Self::LeftDigital),
            2 => Some(Self::RightDigital),
            3 => Some(Self::ThrottleAnalog),
            4 => Some(Self::CombinedThrottleBrakeAnalog),
            5 => Some(Self::Brake),
            6 => Some(Self::Handbrake),
            7 => Some(Self::GearUp),
            8 => Some(Self::GearDown),
            9 => Some(Self::ChangeCamera),
            10 => Some(Self::Pause),
            11 => Some(Self::Clutch),
            12 => Some(Self::Ignition),
            13 => Some(Self::Reverse),
            14 => Some(Self::Neutral),
            15 => Some(Self::FirstGear),
            16 => Some(Self::SecondGear),
            17 => Some(Self::ThirdGear),
            18 => Some(Self::FourthGear),
            19 => Some(Self::FifthGear),
            20 => Some(Self::SixthGear),
            _ => None,
        }
    }
}