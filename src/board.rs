use std::collections::HashMap;

use crate::{
    common::{Game, PlayColor, LED_COUNT},
    display::Display,
};

const JUMP_HEIGHT: usize = 20;
const JUMP_COOLDOWN: usize = 80;
const OBSTACLE_INTERVAL: usize = 50;
const BONUS_INTERVAL: usize = 200;
const LIFE_INIT: usize = 15;

#[derive(Debug)]
pub struct Board {
    players: HashMap<PlayColor, Player>,
    obstacles: Vec<Blob>,
    boni: Vec<Blob>,
}

impl Board {
    pub fn new(player_colors: Vec<PlayColor>) -> Self {
        let players = player_colors
            .iter()
            .enumerate()
            .map(|(i, p)| {
                (
                    *p,
                    Player::new(i * LED_COUNT / player_colors.len(), *p, LIFE_INIT),
                )
            })
            .collect::<HashMap<_, _>>();
        Self {
            players,
            obstacles: vec![],
            boni: vec![],
        }
    }

    pub fn player_pos(&mut self, c: PlayColor, i: usize) {
        if let Some(player) = self.players.get_mut(&c) {
            player.set_pos(i);
        } else {
            self.players.insert(c, Player::new(i, c, 10));
        }
    }

    pub fn player_click(&mut self, c: PlayColor) {
        if let Some(player) = self.players.get_mut(&c) {
            player.jump();
        }
    }

    pub fn tick(&mut self, display: &mut Display) -> Game {
        self.obstacles.retain_mut(|o| o.tick_visible());
        self.boni.retain_mut(|b| b.tick_visible());
        self.check_collision(vec![]);

        // Only check collisions for players who moved.
        let players_ignore: Vec<PlayColor> = self
            .players
            .iter_mut()
            .filter_map(|(color, player)| {
                let orig = player.pos;
                player.tick();
                (player.pos == orig).then(|| *color)
            })
            .collect();
        self.check_collision(players_ignore);

        if rand::random::<f32>() < 1. / OBSTACLE_INTERVAL as f32 {
            self.obstacles.push(Blob::rand());
        }
        if rand::random::<f32>() < 1. / BONUS_INTERVAL as f32 {
            self.boni.push(Blob::rand());
        }

        display.draw_players(self.players.values().cloned().collect());
        display.draw_obstacles(self.obstacles.iter().map(|o| o.pos()).collect());
        display.draw_boni(self.boni.iter().map(|b| b.pos()).collect());

        if self.players.len() > 1 {
            Game::Play(self.players.keys().cloned().collect())
        } else if let Some(winner) = self.players.iter().next() {
            Game::Winner(*winner.0)
        } else {
            Game::Draw
        }
    }

    fn check_collision(&mut self, players_ignore: Vec<PlayColor>) {
        self.players.retain(|_, p: &mut Player| p.lifes > 0);
        for (_, player) in self.players.iter_mut() {
            if player.jump == 0 && !players_ignore.contains(&player.color) {
                for o in &mut self.obstacles {
                    if o.pos() == player.pos {
                        player.lifes -= 1;
                        o.clear = true;
                    }
                }

                for b in &mut self.boni {
                    if b.pos() == player.pos {
                        player.lifes += 1;
                        b.clear = true;
                    }
                }
            }
        }
        self.obstacles.retain(|o| !o.clear);
        self.boni.retain(|b| !b.clear);
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub pos: Position,
    dest: Position,
    pub color: PlayColor,
    pub lifes: usize,
    pub jump: usize,
    pub jump_recover: usize,
}

impl Player {
    fn new(pos: usize, color: PlayColor, lifes: usize) -> Self {
        let pos = Position(pos);
        Self {
            pos,
            dest: pos,
            color,
            lifes,
            jump: 0,
            jump_recover: 0,
        }
    }

    fn set_pos(&mut self, dest: usize) {
        self.dest = Position(dest);
    }

    fn tick(&mut self) {
        if self.dest != self.pos {
            self.pos = if self.pos.direction(self.dest) > 0 {
                self.pos.add(-1)
            } else {
                self.pos.add(1)
            }
        }
        if self.jump > 0 {
            self.jump -= 1;
        }
        if self.jump_recover > 0 {
            self.jump_recover -= 1;
        }
    }

    fn jump(&mut self) {
        if self.jump_recover == 0 {
            self.jump = JUMP_HEIGHT;
            self.jump_recover = JUMP_HEIGHT + JUMP_COOLDOWN;
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position(pub usize);

impl Position {
    pub fn distances(a: usize, b: usize) -> i32 {
        let dist = ((a + LED_COUNT - b) % LED_COUNT) as i32;
        if dist > LED_COUNT as i32 / 2 {
            dist - LED_COUNT as i32
        } else {
            dist
        }
    }

    pub fn direction(&self, other: Position) -> i32 {
        Self::distances(self.0, other.0)
    }

    pub fn add(&self, delta: i32) -> Self {
        let d = delta.rem_euclid(LED_COUNT as i32) as usize;
        Self((self.0 + d) % LED_COUNT)
    }

    pub fn sub(&self, delta: i32) -> Self {
        self.add(-delta)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Blob {
    init: Position,
    counter: i32,
    direction: i32,
    clear: bool,
}

impl Blob {
    pub fn rand() -> Self {
        Self {
            init: Position(rand::random::<usize>() % LED_COUNT),
            counter: LED_COUNT as i32,
            direction: if rand::random() { -1 } else { 1 },
            clear: false,
        }
    }

    pub fn pos(&self) -> Position {
        self.init.add(self.counter * self.direction)
    }

    pub fn tick_visible(&mut self) -> bool {
        if self.counter > 0 {
            self.counter -= 1;
            return true;
        }
        false
    }
}
