use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use crossbeam::queue::SegQueue;

use crate::bot_http::BotHttp;

pub struct Task {
    pub interval: Duration,
    pub handler: fn(bot: &BotHttp),
}
impl Task {
    pub fn new(interval: Duration, handler: fn(bot: &BotHttp)) -> Self {
        Self { interval, handler }
    }
}

pub struct TaskPool {
    pool: SegQueue<Task>,
}

impl TaskPool {
    pub fn new() -> Self {
        Self {
            pool: SegQueue::<Task>::new(),
        }
    }

    pub fn add_task(&mut self, task: Task) {
        self.pool.push(task);
    }

    pub fn start_handler(self, bot: BotHttp) {
        spawn(move || loop {
            if let Some(task) = self.pool.pop() {
                let bot = bot.clone();
                spawn(move || loop {
                    (task.handler)(&bot);
                    sleep(task.interval);
                });
            }
            sleep(Duration::from_nanos(1));
        });
    }
}
