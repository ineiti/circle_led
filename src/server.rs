use std::{
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use crate::{games::snake_board::{AnswerSnake, MessagesSnake, PlatformSnake}};

#[derive(Clone, Debug)]
pub struct Platform {
    snake: Arc<Mutex<PlatformSnake>>,
}

impl Platform {
    pub fn new() -> Self {
        let out = Self {
            snake: Arc::new(Mutex::new(PlatformSnake::new())),
        };

        let snake = out.snake.clone();

        thread::spawn(move || loop {
            snake.lock().unwrap().message(MessagesSnake::Tick);
            thread::sleep(Duration::from_millis(50));
        });

        out
    }

    pub fn get_circle(&mut self) -> String {
        self.snake.lock().unwrap().get_circle()
    }

    pub fn snake_message(&mut self, msg: MessagesSnake) -> Option<AnswerSnake>{
        self.snake.lock().unwrap().message(msg)
    }
}
