use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

#[derive(Clone, Debug)]
pub struct Platform {
    inner: Arc<Mutex<PlatformInner>>,
}

impl Platform {
    pub fn new(size: usize) -> Self {
        let out = Self {
            inner: Arc::new(Mutex::new(PlatformInner::new(size))),
        };

        let inner = out.inner.clone();

        thread::spawn(move || loop {
            inner.lock().unwrap().tick();
            thread::sleep(Duration::from_millis(50));
        });

        out
    }

    pub fn get_circle(&mut self) -> String {
        self.inner.lock().unwrap().get_circle()
    }

    pub fn touch_led(&mut self, i: usize) {
        self.inner.lock().unwrap().touch_led(i);
    }
}

#[derive(Debug, Clone, Copy)]
struct LED {
    red: u8,
    green: u8,
    blue: u8,
}

impl LED {
    fn white() -> Self {
        Self {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
        }
    }

    fn to_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    fn mean(&self, other: Self) -> Self {
        Self {
            red: ((self.red as u16 + other.red as u16) / 2) as u8,
            green: ((self.green as u16 + other.green as u16) / 2) as u8,
            blue: ((self.blue as u16 + other.blue as u16) / 2) as u8,
        }
    }

    fn from_color(color: u8) -> LED {
        let hue = color / 64;
        let bright = color % 64;
        let (one, two) = (192 + bright, 255 - bright);
        match hue {
            0 => LED {
                red: one,
                green: two,
                blue: 0,
            },
            1 => LED {
                red: 0,
                green: one,
                blue: two,
            },
            2 => LED {
                red: two,
                green: 0,
                blue: one,
            },
            _ => LED::white(),
        }
    }
}

#[derive(Debug)]
struct PlatformInner {
    leds: Vec<LED>,
    color: u8,
}

impl PlatformInner {
    fn new(size: usize) -> Self {
        Self {
            leds: (0..size).map(|_| LED::white()).collect(),
            color: 0,
        }
    }

    fn get_circle(&self) -> String {
        self.leds
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    fn touch_led(&mut self, i: usize) {
        if let Some(l) = self.leds.get_mut(i) {
            *l = LED::from_color(self.color);
        }
    }

    fn neighbors(&self, i: usize) -> (LED, LED) {
        let prev = if i > 0 {
            self.leds[i - 1]
        } else {
            self.leds[self.leds.len() - 1]
        };
        let next = if i < self.leds.len() - 2 {
            self.leds[i + 1]
        } else {
            self.leds[0]
        };
        (prev, next)
    }

    fn tick(&mut self) {
        self.leds = (0..self.leds.len())
            .map(|i| self.neighbors(i))
            .map(|(p, n)| p.mean(n))
            .collect::<Vec<LED>>();
        let first = self.leds.remove(0);
        self.leds.push(first);
        self.color = (self.color + 2) % 192;
    }
}
