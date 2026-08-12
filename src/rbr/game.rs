use crate::rbr::GameMode::{Driving, ExitingToMenu, FinishEnd, GameStarting, Loading, Main, Pause, Quit, Replay, RotateAroundCar, Unknown};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GameMode {
    Unknown(i32),
    Driving,
    Pause,
    Main,
    Loading,
    ExitingToMenu,
    Quit,
    Replay,
    FinishEnd,
    RotateAroundCar,
    GameStarting
}

impl From<i32> for GameMode {
    fn from(value: i32) -> Self {
        match value {
            1 => Driving,
            2 => Pause,
            3 => Main,
            5 => Loading,
            6 => ExitingToMenu,
            7 => Quit,
            8 => Replay,
            9 => FinishEnd,
            0x0A => RotateAroundCar,
            0x0C => GameStarting,
            e => Unknown(e) 
        }
    }
}
