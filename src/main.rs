use std::time::Duration;

use async_std::task::sleep;
use common::{Game, PlayColor, LED_COUNT};
use dioxus::prelude::*;
use tracing::Level;

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

// The entry point for the server
#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum::{
        response::sse::{Event, Sse},
        routing::get,
    };
    use futures::Stream;
    use server::Platform;
    use std::{convert::Infallible, time::SystemTime};
    use std::time::Duration;
    use tokio::{sync::mpsc, task};

    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    async fn sse_handler(
        mut platform: Platform,
    ) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
        let (tx, rx) = mpsc::channel(10);

        task::spawn(async move {
            let mut start = SystemTime::now();
            loop {
                if tx
                    .send(Ok(Event::default().data(platform.get_circle())))
                    .await
                    .is_err()
                {
                    break;
                }
                sleep(Duration::from_millis(50) - start.elapsed().unwrap()).await;
                tracing::info!("Elapsed: {:?}", start.elapsed());
                start = SystemTime::now();
            }
        });

        // Convert the receiver into a stream
        Sse::new(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    // Create a global instance of Platform, and pass it to the axum router as the context.
    let platform = server::Platform::new();
    let router = axum::Router::new()
        .route(
            "/get_circle",
            get({
                let platform = platform.clone();
                move || sse_handler(platform.clone())
            }),
        )
        .serve_dioxus_application(
            ServeConfigBuilder::new().context_providers(std::sync::Arc::new(vec![Box::new(
                move || Box::new(platform.clone()),
            )])),
            App,
        );

    let router = router.into_make_service();
    let address = dioxus_cli_config::fullstack_address_or_localhost();
    let listener = tokio::net::TcpListener::bind(address).await.unwrap();
    axum::serve(listener, router).await.unwrap();
}

// For any other platform, we just launch the app
#[cfg(not(feature = "server"))]
fn main() {
    dioxus_logger::init(Level::INFO).expect("failed to init logger");

    let url = web_sys::window().unwrap().location().origin().unwrap();
    server_fn::client::set_server_url(url.leak());

    LaunchBuilder::new().launch(App);
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
    use_effect(move || {
        current_player.set(None);
    });

    let join = move |player: PlayColor| async move {
        document::eval(include_str!("../fullscreen.js"));
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
