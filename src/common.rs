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
        self.to_hex_pastel()
        // match self {
        //     PlayColor::Red => "ff0000",
        //     PlayColor::Green => "00ff00",
        //     PlayColor::Blue => "0000ff",
        //     PlayColor::Yellow => "ffff00",
        //     PlayColor::Cyan => "00ffff",
        //     PlayColor::Magenta => "ff00ff",
        // }.into()
    }

    pub fn to_hex_pastel(&self) -> String {
        match self {
            PlayColor::Red => "ff8888",
            PlayColor::Green => "88ff88",
            PlayColor::Blue => "8888ff",
            PlayColor::Yellow => "ffff88",
            PlayColor::Cyan => "88ffff",
            PlayColor::Magenta => "ff88ff",
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
