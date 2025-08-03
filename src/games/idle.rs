use crate::display::Display;

#[derive(Debug)]
pub struct PlatformIdle {
    display: Display,
}

impl PlatformIdle {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
        }
    }

    pub fn get_circle(&self) -> String {
        self.display.get_circle()
    }

    pub fn message(&mut self) {
        self.display.tick();
        self.display.rainbow();
    }
}
