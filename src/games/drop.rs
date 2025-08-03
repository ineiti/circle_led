use dioxus::prelude::*;

use crate::{Route, common::LED_COUNT};

#[component]
pub fn Drop() -> Element {
    rsx! {
        div {
            Link {to: Route::Reset{}, style: "text-align: center; width: 100%;", "Home"}
            h1 {
                "Idlying around"
            }
        }
    }
}

#[derive(Debug)]
pub struct PlatformDrop {}

impl PlatformDrop {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_circle(&self) -> String {
        "000000".repeat(LED_COUNT).into()
    }

    pub fn message(&mut self) {}
}
