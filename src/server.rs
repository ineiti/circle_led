use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    board::{Board, CIRCLE_SIZE},
    common::{Game, PlayColor},
    display::Display,
};

#[derive(Clone, Debug)]
pub struct Platform {
    inner: Arc<Mutex<PlatformInner>>,
}

impl Platform {
    pub fn new() -> Self {
        println!("New Platform");
        let out = Self {
            inner: Arc::new(Mutex::new(PlatformInner::new())),
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

    pub fn player_pos(&mut self, i: usize, c: PlayColor) {
        self.inner.lock().unwrap().player_pos(i, c);
    }

    pub fn player_click(&mut self, c: PlayColor) {
        self.inner.lock().unwrap().player_click(c);
    }

    pub fn game_state(&self) -> Game {
        self.inner.lock().unwrap().game_state()
    }

    pub fn game_join(&self, c: PlayColor) {
        self.inner.lock().unwrap().game_join(c)
    }
}

#[derive(Debug)]
struct PlatformInner {
    display: Display,
    board: Option<Board>,
    game: Game,
    countdown: usize,
}

impl PlatformInner {
    fn new() -> Self {
        Self {
            display: Display::new(),
            board: None,
            game: Game::Idle,
            countdown: 0,
        }
    }

    fn get_circle(&self) -> String {
        self.display.get_circle()
    }

    fn player_pos(&mut self, i: usize, c: PlayColor) {
        self.board.as_mut().map(|b| b.player_pos(c, i));
    }

    fn player_click(&mut self, c: PlayColor) {
        self.board.as_mut().map(|b| b.player_click(c));
    }

    fn game_state(&self) -> Game {
        self.game.clone()
    }

    fn game_join(&mut self, c: PlayColor) {
        match self.game.clone() {
            Game::Idle => self.game = Game::Play(vec![c]),
            Game::Signup(vec) => self.game = Game::Play(vec![vec, vec![c]].concat()),
            _ => {}
        }
    }

    fn tick(&mut self) {
        match self.game.clone() {
            Game::Idle => self.display.rainbow(),
            Game::Signup(players) => {
                if players.len() == 1 && self.countdown < CIRCLE_SIZE / 2 {
                    self.countdown = CIRCLE_SIZE;
                }
                self.display.game_signup(players, self.countdown);
            }
            Game::Play(_) => {
                self.display.rainbow();
                if let Some(board) = self.board.as_mut() {
                    self.game = board.tick(&mut self.display);
                }
            }
            Game::Winner(winner) => {
                self.display.game_winner(winner, self.countdown);
            }
            Game::Draw => {
                self.display.game_draw(self.countdown);
            }
        }

        if self.countdown > 0 {
            self.countdown -= 1;
            if self.countdown == 0 {
                self.game = match self.game.clone() {
                    Game::Signup(players) => Game::Play(players),
                    _ => Game::Idle,
                }
            }
        }
    }
}
