use std::{
    thread::{sleep, spawn},
    time::Duration,
};

use crossbeam::queue::SegQueue;

use crate::bot_http::BotHttp;

///
/// The Task for the task pool
///
/// Example on how to create a task without using [::new()](Task::new)
///
/// ```
/// use std::time::Duration;
/// use guilded_rs::{ models::ChatMessage as Message, Task};
///
/// let task = Task {
///     interval: Duration::from_secs(10),
///     handler: |bot| {
///         let mut message = Message::default();
///         message.set_content("Content of the message");
///         let channel_id = "2b203b41-5409-436e-9ade-a3c1b640b594";
///         match bot.send_chat_message(message, channel_id) {
///             Some(msg) => {
///                 // msg is the message that just got created
///             }
///             None => {
///                 // Returns None if the request failed.
///                 // in this case look at the console for the error message.
///             }
///         }
///     }
/// };
/// ```
/// And here's and example on how to create a task using [::new()](Task::new)
///
/// ```
/// use std::time::Duration;
/// use guilded_rs::{ models::ChatMessage as Message, Task};
///
/// let task = Task::new(Duration::from_secs(10), |bot| {
///         let mut message = Message::default();
///         message.set_content("Content of the message");
///         let channel_id = "2b203b41-5409-436e-9ade-a3c1b640b594";
///         match bot.send_chat_message(message, channel_id) {
///             Some(msg) => {
///                 // msg is the message that just got created
///             }
///             None => {
///                 // Returns None if the request failed.
///                 // in this case look at the console for the error message.
///             }
///         }
///     });
/// ```
///
pub struct Task {
    #[allow(rustdoc::invalid_rust_codeblocks)]
    ///
    /// How often the task should execute
    /// ```compile_fail
    /// // Please be careful of the value as low Durations may increase the CPU load.
    /// ```
    ///<br/>
    pub interval: Duration,

    ///
    /// The function that will get executed
    ///<br/>
    pub handler: fn(bot: &mut BotHttp),
}
impl Task {
    ///
    /// Creates new task for the task pool to consume and handle.
    ///
    ///
    pub fn new(interval: Duration, handler: fn(bot: &mut BotHttp)) -> Self {
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

    #[cfg(not(doc))]
    pub fn start_handler(self, bot: BotHttp) {
        spawn(move || loop {
            if let Some(task) = self.pool.pop() {
                let mut bot = bot.clone();
                spawn(move || loop {
                    (task.handler)(&mut bot);
                    sleep(task.interval);
                });
            }
            sleep(Duration::from_nanos(1));
        });
    }
}
