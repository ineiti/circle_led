use crate::common::LED_COUNT;

#[derive(Debug)]
pub struct PlatformIdle {
}

impl PlatformIdle {
    pub fn new() -> Self {
        Self {}
    }

    pub fn get_circle(&self) -> String {
        "000000".repeat(LED_COUNT).into()
    }

    pub fn message(&mut self) {}
}