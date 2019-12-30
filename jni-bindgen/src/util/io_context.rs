use super::*;
use std::time::Duration;

pub struct IoContext {
    progress: Progress,
}

impl IoContext {
    pub fn new() -> Self {
        Self {
            progress: Progress::with_duration(Duration::from_secs(1)),
        }
    }

    pub fn update(&mut self, msg: &str) {
        self.progress.update(msg);
    }
}
