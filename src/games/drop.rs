use dioxus::prelude::*;

use crate::common::LED_COUNT;

#[component]
pub fn Drop() -> Element {
    rsx! {
        div {
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
