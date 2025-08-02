use std::time::Duration;

use crate::{
    common::{PlayColor, LED_COUNT}, games::snake_board::{AnswerSnake, MessagesSnake}, server
};
use async_std::task::sleep;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SnakeGame {
    Idle,
    Signup(Vec<PlayColor>),
    Play(Vec<PlayColor>),
    Winner(PlayColor),
    Draw,
}

/// Main Choice
#[component]
pub fn Snake() -> Element {
    let mut game = use_signal(|| SnakeGame::Idle);
    let current_player: Signal<Option<PlayColor>> = use_signal(|| None);

    use_future(move || async move {
        loop {
            // current_player.set(Some(PlayColor::Red));
            // game.set(Game::Play(vec![PlayColor::Red]));
            game.set(game_state().await.unwrap());
            sleep(Duration::from_millis(500)).await;
        }
    });

    rsx! {
        div {
            // Link {to: Route::Display{}, "Display"}
            match game() {
                SnakeGame::Idle => rsx!{Join{joined: vec![], current_player}},
                SnakeGame::Signup(joined) => if current_player().is_some() {
                    rsx!{WaitJoin{ joined }}
                } else {
                    rsx!{Join{joined, current_player}}
                },
                SnakeGame::Play(players) => if let Some(player) = current_player() {
                    rsx!{Play { players, player }}
                } else {
                    rsx!{WaitWinner {  }}
                },
                SnakeGame::Winner(winner) => rsx!{Winner {winner, player: current_player() }},
                SnakeGame::Draw => rsx!{Draw {}},
            }
        }
    }
}

#[component]
fn Join(joined: Vec<PlayColor>, current_player: Signal<Option<PlayColor>>) -> Element {
    use_effect(move || {
        current_player.set(None);
    });

    let join = move |player: PlayColor| async move {
        document::eval(include_str!("../../fullscreen.js"));
        if game_join(player).await.unwrap() {
            current_player.set(Some(player));
        }
    };

    rsx! {
        div {
            id: "color-grid",

            for color in PlayColor::all() {
                button {onclick: move |_| async move {join(color).await},
                    class:"color-block", style:"background-color: #{color.to_hex_pastel()};",
                    "{color.to_string()}"
                }
            }
        }
    }
}

#[component]
fn WaitJoin(joined: Vec<PlayColor>) -> Element {
    let colors: Vec<String> = joined
        .iter()
        .map(|j| j.to_string())
        .collect::<Vec<String>>();
    let colors_str = colors.join(" : ");
    rsx! {
        div {
            class: "centered-div",

            "En attente d'autres joueurs - {colors_str}"
        }
    }
}

#[component]
fn WaitWinner() -> Element {
    rsx! {
        div {
            class: "centered-div",

            "Un jeu est en cours - faut patienter!"
        }
    }
}

#[component]
pub fn Play(players: Vec<PlayColor>, player: PlayColor) -> Element {
    use_effect(move || {
        document_eval(&[
            &format!("const LED_COUNT = {LED_COUNT};"),
            include_str!("../../play.js"),
            &format!("playerLED('{}')", player),
        ]);
    });

    rsx! {
        div {
            class: "centered-div",
            div { id: "circle-container" }
        }
    }
}

#[component]
fn Winner(winner: PlayColor, player: Option<PlayColor>) -> Element {
    use_effect(move || {
        if let Some(p) = player {
            if winner == p {
                document::eval(include_str!("../../fireworks.js"));
            }
        }
    });

    rsx! {
        div {
            class: "centered-div",

            "La couleur gagnante est {winner.to_string()}"
            br{}
            br{}
            canvas {display: "block", id: "fireworks"}

            if let Some(p) = player {
                if p == winner {
                    "Toutes nos félicitations!"
                } else {
                    "Bonne chance la prochaine fois!"
                }
            }
        }
    }
}

#[component]
fn Draw() -> Element {
    rsx! {
        div {
            class: "centered-div",

            "C'était chaud - personne n'a gagné..."
        }
    }
}

#[server(endpoint = "game_state")]
async fn game_state() -> Result<SnakeGame, ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    if let Some(AnswerSnake::State(state)) = plat.snake_message(MessagesSnake::GetState){
        Ok(state)
    } else {
        Err(ServerFnError::ServerError("didn't get state".into()))
    }
}

#[server(endpoint = "game_join")]
async fn game_join(c: PlayColor) -> Result<bool, ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    if let Some(AnswerSnake::Joined(joined)) = plat.snake_message(MessagesSnake::Join(c)){
        Ok(joined)
    } else {
        Err(ServerFnError::ServerError("didn't get join state".into()))
    }
}

fn document_eval(parts: &[&str]) {
    document::eval(&parts.join("\n"));
}

#[server(endpoint = "player_pos")]
async fn player_pos(i: usize, c: PlayColor) -> Result<(), ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.snake_message(MessagesSnake::PlayerPos(c, i));
    Ok(())
}

#[server(endpoint = "player_click")]
async fn player_click(c: PlayColor) -> Result<(), ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.snake_message(MessagesSnake::PlayerClick(c));
    Ok(())
}
