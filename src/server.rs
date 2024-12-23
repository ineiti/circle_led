use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    board::Board,
    common::{Game, PlayColor, LED_COUNT},
    display::Display,
};

#[derive(Clone, Debug)]
pub struct Platform {
    inner: Arc<Mutex<PlatformInner>>,
}

impl Platform {
    pub fn new() -> Self {
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

    pub fn game_join(&self, c: PlayColor) -> bool {
        self.inner.lock().unwrap().game_join(c)
    }

    pub fn game_reset(&self) {
        self.inner.lock().unwrap().game_reset()
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

    fn game_join(&mut self, c: PlayColor) -> bool {
        match self.game.clone() {
            Game::Idle => self.game = Game::Signup(vec![c]),
            Game::Signup(vec) => {
                if vec.contains(&c) {
                    return false;
                }
                self.game = Game::Signup(vec![vec, vec![c]].concat());
                // DEBUG
                // self.countdown = LED_COUNT;
                self.countdown = 1;
            }
            _ => {}
        }
        true
    }

    fn game_reset(&mut self) {
        self.game = Game::Idle;
    }

    fn tick(&mut self) {
        self.display.tick();

        match self.game.clone() {
            Game::Idle => self.display.rainbow(),
            Game::Signup(players) => {
                if players.len() == 1 {
                    self.countdown = LED_COUNT;
                }
                self.display.game_signup(players, self.countdown);
            }
            Game::Play(_) => {
                self.display.flow();
                if let Some(board) = self.board.as_mut() {
                    self.game = board.tick(&mut self.display);
                    if matches!(self.game, Game::Winner(_)) {
                        self.countdown = 200;
                    }
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
                    Game::Signup(players) => {
                        self.board = Some(Board::new(players.clone()));
                        self.display.reset();
                        Game::Play(players)
                    }
                    _ => Game::Idle,
                }
            }
        }
        // tracing::debug!("New game state is: {:?} - {}", self.game, self.countdown);
    }
}
