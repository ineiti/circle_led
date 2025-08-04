use std::f32::consts::{PI, TAU};

use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{
    common::{PlayColor, LED_COUNT},
    display::{Blob, Display},
    games::snake_board::Position,
    server, Route,
};

#[component]
pub fn Drop() -> Element {
    rsx! {
        div {
            id: "drop-grid",

            for color in PlayColor::all(){
                button {onclick: move |_| async move {
                    if let Err(e) = drop_color(Side::Left, color).await{
                        tracing::error!("{e:?}");
                    }
                },
                    class:"color-block", style:"background-color: #{color.to_hex_pastel()};",
                    "{color.to_string()}"
                }
                div{
                    class: "color-block",

                    if color == PlayColor::Red {
                        Link {to: Route::Reset{}, style: "text-align: center; width: 100%;", "Home"}
                    }
                }
                button {onclick: move |_| async move {
                    if let Err(e) = drop_color(Side::Right, color).await{
                        tracing::error!("{e:?}");
                    }
                },
                    class:"color-block", style:"background-color: #{color.to_hex_pastel()};",
                    "{color.to_string()}"
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct PlatformDrop {
    display: Display,
    drops: Vec<DropBlob>,
}

impl PlatformDrop {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
            drops: vec![],
        }
    }

    pub fn get_circle(&self) -> String {
        self.display.get_circle()
    }

    pub fn message(&mut self, msg: MessagesDrop) -> Option<AnswerDrop> {
        match msg {
            MessagesDrop::Tick => {
                self.display.clear();
                let drops = self
                    .drops
                    .iter_mut()
                    .map(|drop| {
                        drop.tick();
                        drop.display()
                    })
                    .collect();
                self.display.draw_blobs(drops);
            }
            MessagesDrop::DropColor(side, play_color) => {
                self.drops.push(DropBlob::new(side, play_color));
                if self.drops.len() > 10 {
                    self.drops.remove(0);
                }
            }
        }
        None
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Side {
    Left,
    Right,
}

impl Side {
    pub fn angle_start(&self) -> f32 {
        if self == &Side::Left {
            TAU - 0.01
        } else {
            0.01
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagesDrop {
    Tick,
    DropColor(Side, PlayColor),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnswerDrop {}

#[derive(Debug)]
struct DropBlob {
    angle: f32,
    speed: f32,
    color: PlayColor,
}

impl DropBlob {
    fn new(side: Side, color: PlayColor) -> Self {
        Self {
            angle: side.angle_start(),
            speed: 0.,
            color,
        }
    }

    fn tick(&mut self) {
        self.speed += self.angle.sin() / 400.;
        self.speed *= 0.999;
        self.angle += self.speed;
    }

    fn display(&self) -> Blob {
        Blob::Drop(
            Position((LED_COUNT as f32 * self.angle / 2. / PI).floor() as usize),
            self.color,
            self.speed * 40.,
        )
    }
}

#[server]
pub async fn drop_color(side: Side, color: PlayColor) -> Result<Option<AnswerDrop>, ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.drop_message(MessagesDrop::DropColor(side, color)))
}
