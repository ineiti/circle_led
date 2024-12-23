use std::collections::HashMap;

use crate::{
    common::{Game, PlayColor, LED_COUNT},
    display::Display,
};

const JUMP_HEIGHT: usize = 20;
const OBSTACLE_INTERVAL: usize = 50;
const LIFE_INIT: usize = 5;

#[derive(Debug)]
pub struct Board {
    players: HashMap<PlayColor, Player>,
    obstacles: Vec<Obstacle>,
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
            self.obstacles.push(Obstacle::rand());
        }

        display.players(self.players.values().cloned().collect());
        display.obstacles(self.obstacles.iter().map(|o| o.pos()).collect());

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
                for o in &self.obstacles {
                    if o.pos() == player.pos {
                        player.lifes -= 1;
                    }
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Player {
    pub pos: Position,
    dest: Position,
    pub color: PlayColor,
    pub lifes: usize,
    pub jump: usize,
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
    }

    fn jump(&mut self) {
        self.jump = JUMP_HEIGHT;
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
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Obstacle {
    init: Position,
    counter: i32,
}

impl Obstacle {
    pub fn rand() -> Self {
        Self {
            init: Position(rand::random::<usize>() % LED_COUNT),
            counter: LED_COUNT as i32,
        }
    }

    pub fn pos(&self) -> Position {
        self.init.add(self.counter)
    }

    pub fn tick_visible(&mut self) -> bool {
        if self.counter > 0 {
            self.counter -= 1;
            return true;
        }
        false
    }
}
