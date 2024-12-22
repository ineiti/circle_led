use common::{Game, PlayColor};
use dioxus::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[route("/")]
    Home {},
    #[route("/play/:color")]
    Play {color: PlayColor},
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
mod games;
#[cfg(feature = "server")]
mod server;

fn main() {
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

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        div {
            Link {to: Route::Display{}, "Display"}
        }
        div {
            id: "color-grid",
            Link {to: Route::Play{color: PlayColor::Red},
                class:"color-block", style:"background-color: #ff8888;",
                "Rouge"
            }
            Link {to: Route::Play{color: PlayColor::Blue},
                class:"color-block", style:"background-color: #8888ff;",
                 "Bleue"
            }
            Link {to: Route::Play{color: PlayColor::Green},
                class:"color-block", style:"background-color: #88ff88;",
                "Vert"
            }
            Link {to: Route::Play{color: PlayColor::Yellow},
                class:"color-block", style:"background-color: #ffff88;",
                "Jaune"
            }
            Link {to: Route::Play{color: PlayColor::Magenta},
                class:"color-block", style:"background-color: #ff88ff;",
                "Rose"
            }
            Link {to: Route::Play{color: PlayColor::Cyan},
                class:"color-block", style:"background-color: #88ffff;",
                "Cyan"
            }
        }
    }
}

#[component]
pub fn Play(color: PlayColor) -> Element {
    use_effect(move || {
        let script = include_str!("../play.js").to_string() + &format!("playerLED('{color}')");
        document::eval(&script);
        // document::eval("console.log('Im here');");
        // document::eval("setTimeout( () => { document.location = '/' }, 1000 );");
    });

    rsx! {
        div {
            class: "centered-div",
            div { id: "circle-container" }
        }
    }
}

#[component]
pub fn Display() -> Element {
    use_effect(move || {
        document::eval(include_str!("../display.js"));
    });

    rsx! {
        div {
            class: "centered-div",
            div { id: "circle-container" }
        }
    }
}

#[server(endpoint = "game_state")]
async fn game_state() -> Result<Game, ServerFnError> {
    print!("Game_state");
    let FromContext(plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.game_state())
}

#[server(endpoint = "game_join")]
async fn game_join(c: PlayColor) -> Result<(), ServerFnError> {
    print!("Game_join");
    let FromContext(plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.game_join(c))
}

#[server(endpoint = "get_circle")]
async fn get_circle() -> Result<String, ServerFnError> {
    print!("get_circle");
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.get_circle())
}

#[server(endpoint = "player_pos")]
async fn player_pos(i: usize, c: PlayColor) -> Result<(), ServerFnError> {
    print!("player_pos");
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.player_pos(i, c);
    Ok(())
}

#[server(endpoint = "player_click")]
async fn player_click(c: PlayColor) -> Result<(), ServerFnError> {
    print!("player_click");
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.player_click(c);
    Ok(())
}
