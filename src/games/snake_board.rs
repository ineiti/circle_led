use serde::{Deserialize, Serialize};

use crate::{
    common::{PlayColor, FREQUENCY, LED_COUNT},
    display::{Blob, Display},
    games::snake::SnakeGame,
};
use std::collections::HashMap;

#[cfg(debug_assertions)]
const COUNTDOWN_PLAY: usize = 2;
#[cfg(not(debug_assertions))]
const COUNTDOWN_PLAY: usize = crate::common::LED_COUNT;

const COUNTDOWN_WINNER: usize = 4 * FREQUENCY;
const OBSTACLE_INTERVAL: usize = FREQUENCY * 3;
const OBSTACLE_INCREASE_SEC: usize = 10;
const BONUS_INTERVAL: usize = FREQUENCY * 10;
const LIFE_INIT: usize = 5;
const PLAYER_SPEED: usize = 60 / FREQUENCY;
const JUMP_DURATION: usize = 4 * FREQUENCY;
const JUMP_COOLDOWN: usize = 8 * FREQUENCY;

pub enum MessagesSnake {
    PlayerTurn(PlayColor, Option<TurnDir>),
    PlayerJump(PlayColor),
    Join(PlayColor),
    GetState,
    Tick,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TurnDir {
    Right,
    Left,
}

pub enum AnswerSnake {
    Joined(bool),
    State(SnakeGame),
}

#[derive(Debug)]
pub struct PlatformSnake {
    display: Display,
    board: Option<Board>,
    game: SnakeGame,
    countdown: usize,
}

impl PlatformSnake {
    pub fn new() -> Self {
        Self {
            display: Display::new(),
            board: None,
            game: SnakeGame::Idle,
            countdown: 0,
        }
    }

    pub fn get_circle(&self) -> String {
        self.display.get_circle()
    }

    pub fn message(&mut self, msg: MessagesSnake) -> Option<AnswerSnake> {
        match msg {
            MessagesSnake::PlayerTurn(player, dir) => self.player_turn(player, dir),
            MessagesSnake::PlayerJump(play_color) => self.player_click(play_color),
            MessagesSnake::Join(play_color) => return Some(self.game_join(play_color)),
            MessagesSnake::GetState => return Some(AnswerSnake::State(self.game.clone())),
            MessagesSnake::Tick => self.tick(),
        }
        None
    }

    fn player_turn(&mut self, c: PlayColor, d: Option<TurnDir>) {
        self.board.as_mut().map(|b| b.player_turn(c, d));
    }

    fn player_click(&mut self, c: PlayColor) {
        self.board.as_mut().map(|b| b.player_click(c));
    }

    fn game_join(&mut self, c: PlayColor) -> AnswerSnake {
        match self.game.clone() {
            SnakeGame::Idle => self.game = SnakeGame::Signup(vec![c]),
            SnakeGame::Signup(vec) => {
                if vec.contains(&c) {
                    return AnswerSnake::Joined(false);
                }
                self.game = SnakeGame::Signup(vec![vec, vec![c]].concat());
                self.countdown = COUNTDOWN_PLAY;
            }
            _ => {}
        }
        AnswerSnake::Joined(true)
    }

    fn tick(&mut self) {
        self.display.tick();

        match self.game.clone() {
            SnakeGame::Idle => self.display.rainbow(),
            SnakeGame::Signup(players) => {
                if players.len() == 1 {
                    self.countdown = COUNTDOWN_PLAY;
                }
                self.display.game_signup(players, self.countdown);
            }
            SnakeGame::Play(_) => {
                self.display.flow();
                if let Some(board) = self.board.as_mut() {
                    self.game = board.tick(&mut self.display);
                    if matches!(self.game, SnakeGame::Winner(_)) {
                        self.countdown = COUNTDOWN_WINNER;
                    }
                }
            }
            SnakeGame::Winner(winner) => {
                self.display.game_winner(winner, self.countdown);
            }
            SnakeGame::Draw => {
                self.display.game_draw(self.countdown);
            }
        }

        if self.countdown > 0 {
            self.countdown -= 1;
            if self.countdown == 0 {
                self.game = match self.game.clone() {
                    SnakeGame::Signup(players) => {
                        self.board = Some(Board::new(players.clone()));
                        self.display.reset();
                        SnakeGame::Play(players)
                    }
                    _ => SnakeGame::Idle,
                }
            }
        }
        // tracing::debug!("New game state is: {:?} - {}", self.game, self.countdown);
    }
}

#[derive(Debug)]
pub struct Board {
    players: HashMap<PlayColor, Player>,
    obstacles: Vec<Drop>,
    boni: Vec<Drop>,
    obstacle: usize,
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
            obstacle: OBSTACLE_INTERVAL * 20,
        }
    }

    pub fn player_turn(&mut self, c: PlayColor, d: Option<TurnDir>) {
        if let Some(player) = self.players.get_mut(&c) {
            player.set_turn(d);
        }
    }

    pub fn player_click(&mut self, c: PlayColor) {
        if let Some(player) = self.players.get_mut(&c) {
            player.jump();
        }
    }

    pub fn tick(&mut self, display: &mut Display) -> SnakeGame {
        self.obstacles.retain_mut(|o| o.tick_visible());
        self.boni.retain_mut(|b| b.tick_visible());
        self.check_collision(vec![]);

        // Only check collisions for players who moved.
        for _ in 0..PLAYER_SPEED {
            let positions: Vec<Player> = self.players.values().cloned().collect();
            let players_ignore: Vec<PlayColor> = self
                .players
                .iter_mut()
                .filter_map(|(color, player)| {
                    let orig = player.pos;
                    player.tick(positions.clone());
                    (player.pos == orig).then(|| *color)
                })
                .collect();
            self.check_collision(players_ignore);
        }

        let speed_up = display.counter % (FREQUENCY * OBSTACLE_INCREASE_SEC);
        if speed_up == 0 && self.obstacle > 10 {
            self.obstacle = self.obstacle * 2 / 3;
        }

        if rand::random::<f32>() < 1. / (self.obstacle as f32 / 10.0) {
            self.obstacles.push(Drop::rand());
        }
        if rand::random::<f32>() < 1. / BONUS_INTERVAL as f32 {
            self.boni.push(Drop::rand());
        }

        display.clear();
        display.draw_blobs(
            self.players
                .values()
                .cloned()
                .map(|p| Blob::Player(p))
                .collect(),
        );
        display.draw_blobs(self.obstacles.iter().map(|o| o.obstacle()).collect());
        display.draw_blobs(self.boni.iter().map(|b| b.bonus()).collect());
        if speed_up < FREQUENCY {
            display.shine(speed_up as f32 / FREQUENCY as f32 * 2.0);
        }

        if self.players.len() > 1 {
            SnakeGame::Play(self.players.keys().cloned().collect())
        } else if let Some(winner) = self.players.iter().next() {
            SnakeGame::Winner(*winner.0)
        } else {
            SnakeGame::Draw
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
    turn: Option<TurnDir>,
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
            turn: None,
            color,
            lifes,
            jump: 0,
            jump_recover: 0,
        }
    }

    fn set_turn(&mut self, dir: Option<TurnDir>) {
        self.turn = dir;
    }

    fn tick(&mut self, players: Vec<Player>) {
        if let Some(dir) = self.turn.as_ref() {
            let others: Vec<&Player> = players.iter().filter(|p| p.color != self.color).collect();
            let new_pos = if dir == &TurnDir::Left {
                self.pos.add(-1)
            } else {
                self.pos.add(1)
            };
            if others
                .iter()
                .all(|o| o.pos.direction(new_pos).abs() as usize >= (o.lifes + self.lifes) * 3 / 2)
            {
                self.pos = new_pos;
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
            self.jump = JUMP_DURATION;
            self.jump_recover = JUMP_DURATION + JUMP_COOLDOWN;
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
pub struct Drop {
    init: Position,
    counter: i32,
    direction: i32,
    clear: bool,
}

impl Drop {
    pub fn rand() -> Self {
        Self {
            init: Position(rand::random::<usize>() % LED_COUNT),
            counter: LED_COUNT as i32,
            direction: if rand::random() { -1 } else { 1 },
            clear: false,
        }
    }

    pub fn pos(&self) -> Position {
        self.init
            .add(self.counter * self.direction * 20 / FREQUENCY as i32)
    }

    pub fn tick_visible(&mut self) -> bool {
        if self.counter > 0 {
            self.counter -= 1;
            return true;
        }
        false
    }

    fn bonus(&self) -> Blob {
        Blob::Bonus(self.pos())
    }

    fn obstacle(&self) -> Blob {
        Blob::Obstacle(self.pos())
    }
}
