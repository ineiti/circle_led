use dioxus::prelude::*;
use strum_macros::{Display, EnumString};

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

mod server;

fn main() {
    LaunchBuilder::new()
        .with_context(server_only! {
            server::Platform::new(100)
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
                class:"color-block", style:"background-color: #88ff88;",
                 "Bleue"
            }
            Link {to: Route::Play{color: PlayColor::Green},
                class:"color-block", style:"background-color: #8888ff;",
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

#[derive(Display, EnumString, Clone, PartialEq, Debug)]
pub enum PlayColor {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
}

#[component]
pub fn Play(color: PlayColor) -> Element {
    use_effect(move || {
        document::eval(include_str!("../play.js"));
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

#[server(endpoint = "get_circle")]
async fn get_circle() -> Result<String, ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    Ok(plat.get_circle())
}

#[server(endpoint = "touch_led")]
async fn touch_led(i: usize) -> Result<(), ServerFnError> {
    let FromContext(mut plat): FromContext<server::Platform> = extract().await?;
    plat.touch_led(i);
    Ok(())
}
