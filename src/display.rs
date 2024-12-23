use crate::{
    board::{Player, Position},
    common::{PlayColor, LED_COUNT},
};

const BLINK_JUMP: usize = 5;
const BLINK_RECOVER: usize = 10;

#[derive(Debug)]
pub struct Display {
    leds: Vec<LED>,
    counter: usize,
}

impl Display {
    pub fn new() -> Self {
        Self {
            leds: (0..LED_COUNT).map(|_| LED::white()).collect(),
            counter: 0,
        }
    }

    pub fn get_circle(&self) -> String {
        self.leds
            .iter()
            .map(|l| l.to_string())
            .collect::<Vec<String>>()
            .join("")
    }

    pub fn draw_players(&mut self, players: Vec<Player>) {
        let mut leds: Vec<LED> = (0..LED_COUNT).map(|_| LED::black()).collect();
        for p in players {
            if p.jump == 0 || (self.counter % BLINK_JUMP < BLINK_JUMP / 2) {
                leds[p.pos.0].xor(p.color.into());
            }
            if p.jump == 0
                && (p.jump_recover == 0 || (self.counter % BLINK_RECOVER < BLINK_RECOVER / 2))
            {
                for dist in 1..p.lifes * 3 {
                    leds[p.pos.add(dist as i32).0].xor(LED::from(p.color).brightness(0.2));
                    leds[p.pos.sub(dist as i32).0].xor(LED::from(p.color).brightness(0.2));
                }
            }
        }

        for (i, l) in leds.into_iter().enumerate() {
            if !l.is_black() {
                self.leds[i] = l;
            }
        }
    }

    pub fn draw_obstacles(&mut self, obstacles: Vec<Position>) {
        for o in obstacles {
            self.leds[o.0] = LED::white();
        }
    }

    pub fn draw_boni(&mut self, boni: Vec<Position>) {
        for b in boni {
            self.leds[b.0] = LED::from_hex("22ff22");
        }
    }

    pub fn rainbow(&mut self) {
        self.leds[self.counter % LED_COUNT] = LED::from_hue((self.counter % 192) as u8);
        self.leds = (0..self.leds.len())
            .map(|i| self.mean_leds(i).brightness(0.6))
            .collect::<Vec<LED>>();
        let first = self.leds.remove(0);
        self.leds.push(first);
    }

    pub fn flow(&mut self) {
        let bright = [0.9, 0.8, 0.6, 0.4, 1.0, 0.3, 0.6, 0.7][self.counter / 100 % 8];
        self.leds = (0..self.leds.len())
            .map(|i| self.mean_leds(i).brightness(bright))
            .collect::<Vec<LED>>();
    }

    pub fn game_draw(&mut self, counter: usize) {
        self.leds = (0..LED_COUNT)
            .map(|i| {
                if i < counter {
                    LED::white()
                } else {
                    LED::black()
                }
            })
            .collect();
    }

    pub fn game_winner(&mut self, winner: PlayColor, counter: usize) {
        let bright = ((counter % 10) as f32 - 5.0).abs() / 5.0;
        self.leds = (0..LED_COUNT)
            .map(|_| LED::from(winner).brightness(bright))
            .collect();
        self.leds[counter % LED_COUNT] = LED::black();
        self.leds[(counter + LED_COUNT / 2) % LED_COUNT] = LED::black();
    }

    pub fn game_signup(&mut self, players: Vec<PlayColor>, counter: usize) {
        self.game_draw(counter);
        let player_width = LED_COUNT / 6;
        for (i, p) in players.iter().enumerate() {
            for j in 0..player_width {
                self.leds[i * player_width + j] = (*p).into();
            }
        }
    }

    pub fn tick(&mut self) {
        self.counter += 1;
    }

    pub fn reset(&mut self) {
        self.counter = 0;
        self.game_draw(0);
    }

    fn neighbors(&self, i: usize) -> (LED, LED) {
        let prev = if i > 0 {
            self.leds[i - 1]
        } else {
            self.leds[self.leds.len() - 1]
        };
        let next = if i < self.leds.len() - 1 {
            self.leds[i + 1]
        } else {
            self.leds[0]
        };
        (prev, next)
    }

    fn mean_leds(&self, i: usize) -> LED {
        let (p, n) = self.neighbors(i);
        self.leds[i].mean(vec![p, n])
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LED {
    red: u8,
    green: u8,
    blue: u8,
}

impl LED {
    pub fn white() -> Self {
        Self::from_hex("ffffff")
    }

    pub fn black() -> Self {
        Self::from_hex("000000")
    }

    pub fn from_hue(hue: u8) -> LED {
        let bright = hue % 64;
        let hue = hue / 64;
        let (one, two) = (255 - bright * 2, 128 + bright * 2);
        match hue {
            0 => LED {
                red: one,
                green: two,
                blue: 64,
            },
            1 => LED {
                red: 64,
                green: one,
                blue: two,
            },
            2 => LED {
                red: two,
                green: 64,
                blue: one,
            },
            _ => LED::white(),
        }
    }

    pub fn from_hex(hex: &str) -> LED {
        let mut l = LED {
            red: 0xff,
            green: 0xff,
            blue: 0xff,
        };
        if hex.len() == 6 {
            if let Ok(red) = u8::from_str_radix(&hex[0..2], 16) {
                l.red = red;
            }
            if let Ok(green) = u8::from_str_radix(&hex[2..4], 16) {
                l.green = green;
            }
            if let Ok(blue) = u8::from_str_radix(&hex[4..6], 16) {
                l.blue = blue;
            }
        }
        l
    }

    pub fn is_black(&self) -> bool {
        self.red == 0 && self.green == 0 && self.blue == 0
    }

    pub fn brightness(&mut self, delta: f32) -> LED {
        Self {
            red: Self::calc_bright(self.red, delta),
            green: Self::calc_bright(self.green, delta),
            blue: Self::calc_bright(self.blue, delta),
        }
    }

    pub fn mean(&self, others: Vec<Self>) -> Self {
        let (mut red, mut green, mut blue) =
            (self.red as usize, self.green as usize, self.blue as usize);
        let s = others.len() + 1;
        for o in others {
            red += o.red as usize;
            green += o.green as usize;
            blue += o.blue as usize;
        }
        Self {
            red: (red / s) as u8,
            green: (green / s) as u8,
            blue: (blue / s) as u8,
        }
    }

    pub fn to_string(&self) -> String {
        format!("{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }

    pub fn xor(&mut self, other: LED) {
        self.red ^= other.red;
        self.green ^= other.green;
        self.blue ^= other.blue;
    }

    fn calc_bright(c: u8, delta: f32) -> u8 {
        let res = c as f32 * delta;
        if res < 0.0 {
            0
        } else if res > 255. {
            255
        } else {
            res as u8
        }
    }
}

impl From<PlayColor> for LED {
    fn from(value: PlayColor) -> Self {
        LED::from_hex(&value.to_hex())
    }
}
