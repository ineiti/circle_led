use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const LED_COUNT: usize = 200;

#[derive(Display, EnumString, Clone, PartialEq, Debug, Deserialize, Serialize, Hash, Eq, Copy)]
pub enum PlayColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

impl PlayColor {
    pub fn to_hex(&self) -> String {
        match self {
            PlayColor::Red => "ff4444",
            PlayColor::Green => "44ff44",
            PlayColor::Blue => "4444ff",
            PlayColor::Yellow => "ffff44",
            PlayColor::Cyan => "44ffff",
            PlayColor::Magenta => "ff44ff",
        }.into()
    }

    pub fn to_string(&self) -> String {
        match self{
            PlayColor::Red => "Rouge",
            PlayColor::Green => "Vert",
            PlayColor::Blue => "Bleue",
            PlayColor::Yellow => "Jaune",
            PlayColor::Cyan => "Cyan",
            PlayColor::Magenta => "Rose",
        }.into()
    }

    pub fn all() -> Vec<PlayColor>{
        vec![Self::Red, Self::Green, Self::Blue, Self::Yellow, Self::Cyan, Self::Magenta]
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Game {
    Idle,
    Signup(Vec<PlayColor>),
    Play(Vec<PlayColor>),
    Winner(PlayColor),
    Draw,
}
