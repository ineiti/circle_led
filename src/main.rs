use std::time::Duration;

use async_std::task::sleep;
use common::LED_COUNT;
use dioxus::prelude::*;
use tracing::Level;

use crate::common::Game;
use crate::games::{drop::Drop, snake::Snake};

mod common;

mod display;
mod games;
#[cfg(feature = "server")]
mod server;

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

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");

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
    use std::time::Duration;
    use std::{convert::Infallible, time::SystemTime};
    use tokio::{
        net::UdpSocket,
        sync::{broadcast::channel, mpsc},
        task,
    };

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
                    tracing::error!("Streaming aborted");
                    return;
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

    let (tx, rx) = channel::<Vec<u8>>(1);
    let mut plat = platform.clone();
    task::spawn(async move {
        loop {
            sleep(Duration::from_millis(50)).await;
            match hex::decode(plat.get_circle()) {
                Ok(leds) => {
                    if let Err(e) = tx.send(leds) {
                        tracing::error!("While sending circle data: {e:?}");
                    }
                }
                Err(e) => tracing::error!("Couldn't convert leds to binary: {e:?}"),
            }
        }
    });

    task::spawn(async move {
        loop {
            let socket = UdpSocket::bind("0.0.0.0:8081")
                .await
                .expect("Binding to port");
            // Receives a single datagram message on the socket. If `buf` is too small to hold
            // the message, it will be cut off.
            let mut buf = [0; 10];
            if let Ok((_rcv, src)) = socket.recv_from(&mut buf).await {
                // Redeclare `buf` as slice of the received data and send reverse data back to origin.
                // tracing::info!("Got {} bytes from {src:?}, waiting for data", rcv);
                let mut rx = rx.resubscribe();
                if let Ok(answer) = rx.recv().await {
                    // tracing::info!("Sending {} bytes through UDP", answer.as_bytes().len());
                    if answer.len() > 1450 {
                        tracing::error!("Sending more than 1450 bytes over UDP works rarely!");
                    }
                    match socket.send_to(answer.as_slice(), &src).await {
                        Ok(_s) => {
                            // tracing::info!("Sent {_s} bytes");
                        }
                        Err(e) => tracing::error!("While sending back: {e:?}"),
                    }
                }
            }
        } // the socket is closed here
    });

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

#[component]
fn Home() -> Element {
    let mut game = use_signal(|| Game::Idle);

    use_future(move || async move {
        loop {
            game.set(get_game().await.unwrap());
            sleep(Duration::from_millis(500)).await;
        }
    });

    rsx! {
        div {
            match game(){
                Game::Idle => rsx!{Idle{game}},
                Game::Snake => rsx!{Snake{}},
                Game::Drop => rsx!{Drop{}}
            }
        }
    }
}

#[component]
fn Idle(game: Signal<Game>) -> Element {
    rsx! {
        div {
            id: "color-grid",

            button {onclick: move |_| async move {
                if let Err(_) = set_game(Game::Snake).await{
                };
            },
                class:"color-block", style:"background-color: #ffdddd;",
                "Snake"
            }

            button {onclick: move |_| async move {
                if let Err(_) = set_game(Game::Drop).await{
                };
            },
                class:"color-block", style:"background-color: #ddffdd;",
                "Drop"
            }
        }
    }
}

fn document_eval(parts: &[&str]) {
    document::eval(&parts.join("\n"));
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

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

#[server(endpoint = "game_reset")]
async fn game_reset() -> Result<(), ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.reset();
    Ok(())
}

#[server(endpoint = "get_circle")]
async fn get_circle() -> Result<String, ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.get_circle())
}

#[server(endpoint = "set_game")]
async fn set_game(game: Game) -> Result<Game, ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.set_game(game))
}

#[server(endpoint = "get_game")]
async fn get_game() -> Result<Game, ServerFnError> {
    let FromContext(plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.get_game())
}
