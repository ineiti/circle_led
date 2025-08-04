use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const LED_COUNT: usize = 288;

pub const FREQUENCY: usize = 50;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Game {
    Idle,
    Snake,
    Drop,
}

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
            PlayColor::Red => "ff0000",
            PlayColor::Green => "00ff00",
            PlayColor::Blue => "0000ff",
            PlayColor::Yellow => "ffff00",
            PlayColor::Cyan => "00ffff",
            PlayColor::Magenta => "ff00ff",
        }
        .into()
    }

    pub fn to_hex_pastel(&self) -> String {
        match self {
            PlayColor::Red => "ff8888",
            PlayColor::Green => "88ff88",
            PlayColor::Blue => "8888ff",
            PlayColor::Yellow => "ffff88",
            PlayColor::Cyan => "88ffff",
            PlayColor::Magenta => "ff88ff",
        }
        .into()
    }

    pub fn all() -> Vec<PlayColor> {
        vec![
            Self::Red,
            Self::Green,
            Self::Blue,
            Self::Yellow,
            Self::Cyan,
            Self::Magenta,
        ]
    }
}
