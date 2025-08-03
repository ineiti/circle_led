use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{
    common::Game,
    games::{
        drop::PlatformDrop,
        idle::PlatformIdle,
        snake_board::{AnswerSnake, MessagesSnake, PlatformSnake},
    },
};

#[derive(Debug)]
pub enum GamePlatform {
    Idle(PlatformIdle),
    Snake(PlatformSnake),
    Drop(PlatformDrop),
}

#[derive(Clone, Debug)]
pub struct Platform {
    game: Arc<Mutex<GamePlatform>>,
}

impl Platform {
    pub fn new() -> Self {
        let out = Self {
            game: Arc::new(Mutex::new(GamePlatform::Idle(PlatformIdle::new()))),
        };

        let game = out.game.clone();

        thread::spawn(move || loop {
            game.lock().unwrap().tick();
            thread::sleep(Duration::from_millis(50));
        });

        out
    }

    pub fn get_circle(&mut self) -> String {
        self.game.lock().unwrap().get_circle()
    }

    pub fn snake_message(&mut self, msg: MessagesSnake) -> Option<AnswerSnake> {
        self.game.lock().unwrap().snake_message(msg)
    }

    pub fn reset(&mut self) {}

    pub fn get_game(&self) -> Game {
        self.game.lock().unwrap().get_game()
    }

    pub fn set_game(&mut self, game: Game) -> Game {
        let current_game = self.game.lock().unwrap().get_game();
        if game != current_game {
            *self.game.lock().unwrap() = match game {
                Game::Idle => GamePlatform::Idle(PlatformIdle::new()),
                Game::Snake => GamePlatform::Snake(PlatformSnake::new()),
                Game::Drop => GamePlatform::Drop(PlatformDrop::new()),
            };
        }
        game
    }
}

impl GamePlatform {
    fn get_circle(&self) -> String {
        match self {
            GamePlatform::Idle(platform_idle) => platform_idle.get_circle(),
            GamePlatform::Snake(platform_snake) => platform_snake.get_circle(),
            GamePlatform::Drop(platform_drop) => platform_drop.get_circle(),
        }
    }

    fn tick(&mut self) {
        match self {
            GamePlatform::Idle(platform_idle) => platform_idle.message(),
            GamePlatform::Snake(platform_snake) => {
                platform_snake.message(MessagesSnake::Tick);
            }
            GamePlatform::Drop(platform_drop) => platform_drop.message(),
        }
    }

    fn snake_message(&mut self, msg: MessagesSnake) -> Option<AnswerSnake> {
        if let GamePlatform::Snake(snake) = self {
            snake.message(msg)
        } else {
            None
        }
    }

    fn get_game(&self) -> Game {
        match self {
            GamePlatform::Idle(_) => Game::Idle,
            GamePlatform::Snake(_) => Game::Snake,
            GamePlatform::Drop(_) => Game::Drop,
        }
    }
}
