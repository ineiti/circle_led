use std::time::Duration;

use async_std::task::sleep;
use common::{Game, PlayColor, LED_COUNT};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/display")]
    Display {},
    #[route("/:..route")]
    PageNotFound { route: Vec<String> },
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

mod common;

#[cfg(feature = "server")]
mod board;
#[cfg(feature = "server")]
mod display;
#[cfg(feature = "server")]
mod server;

fn main() {
    #[cfg(not(feature = "server"))]
    server_fn::client::set_server_url("https://circle.gasser.blue");

    LaunchBuilder::new()
        .with_context(server_only! {
            server::Platform::new()
        })
        .launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS }
        Router::<Route> {}
    }
}

/// Main Choice
#[component]
fn Home() -> Element {
    let mut game = use_signal(|| Game::Idle);
    let current_player: Signal<Option<PlayColor>> = use_signal(|| None);

    use_future(move || async move {
        loop {
            game.set(game_state().await.unwrap());
            sleep(Duration::from_millis(500)).await;
        }
    });

    rsx! {
        div {
            // Link {to: Route::Display{}, "Display"}
            match game() {
                Game::Idle => rsx!{Join{joined: vec![], current_player}},
                Game::Signup(joined) => if current_player().is_some() {
                    rsx!{WaitJoin{ joined }}
                } else {
                    rsx!{Join{joined, current_player}}
                },
                Game::Play(players) => if let Some(player) = current_player() {
                    rsx!{Play { players, player }}
                } else {
                    rsx!{WaitWinner {  }}
                },
                Game::Winner(winner) => rsx!{Winner {winner, player: current_player() }},
                Game::Draw => rsx!{Draw {}},
            }
        }
    }
}

#[component]
fn Join(joined: Vec<PlayColor>, current_player: Signal<Option<PlayColor>>) -> Element {
    current_player.set(None);

    let join = move |player: PlayColor| async move {
        if game_join(player).await.unwrap() {
            current_player.set(Some(player));
        }
    };

    rsx! {
        div {
            id: "color-grid",

            for color in PlayColor::all() {
                button {onclick: move |_| async move {join(color).await},
                    class:"color-block", style:"background-color: #{color.to_hex()};",
                    "{color.to_string()}"
                }
            }
        }
    }
}

#[component]
fn WaitJoin(joined: Vec<PlayColor>) -> Element {
    rsx! {
        div {
            class: "centered-div",

            "En attente d'autres joueurs - {joined:?}"
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

fn document_eval(parts: &[&str]) {
    document::eval(&parts.join("\n"));
}

#[component]
pub fn Play(players: Vec<PlayColor>, player: PlayColor) -> Element {
    use_effect(move || {
        document_eval(&[
            &format!("const LED_COUNT = {LED_COUNT};"),
            include_str!("../play.js"),
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
                document::eval(include_str!("../fireworks.js"));
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

#[component]
pub fn Display() -> Element {
    use_effect(move || {
        document_eval(&[
            &format!("const LED_COUNT = {LED_COUNT};"),
            include_str!("../display.js"),
        ]);
    });

    rsx! {
        div {
            class: "centered-div",
            div { id: "circle-container" }
            button {
                onclick: move |_| async move { game_reset().await.unwrap();},
                class: "centered-div",
                "Reset"
            }
        }
    }
}

#[server(endpoint = "game_reset")]
async fn game_reset() -> Result<(), ServerFnError> {
    let FromContext(plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.game_reset())
}

#[server(endpoint = "game_state")]
async fn game_state() -> Result<Game, ServerFnError> {
    let FromContext(plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.game_state())
}

#[server(endpoint = "game_join")]
async fn game_join(c: PlayColor) -> Result<bool, ServerFnError> {
    let FromContext(plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.game_join(c))
}

#[server(endpoint = "get_circle")]
async fn get_circle() -> Result<String, ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.get_circle())
}

#[server(endpoint = "player_pos")]
async fn player_pos(i: usize, c: PlayColor) -> Result<(), ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.player_pos(i, c);
    Ok(())
}

#[server(endpoint = "player_click")]
async fn player_click(c: PlayColor) -> Result<(), ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.player_click(c);
    Ok(())
}
