use std::time::*;

pub struct Progress {
    can_next_log:   Instant,
    debounce:       Duration,
}

impl Progress {
    pub fn with_duration(debounce: Duration) -> Self {
        Self {
            can_next_log:   Instant::now(),
            debounce,
        }
    }

    pub fn can_update(&self) -> bool {
        Instant::now() >= self.can_next_log
    }

    pub fn force_update(&mut self, msg: &str) {
        self.can_next_log = Instant::now() + self.debounce;
        println!("{}", msg);
    }

    pub fn update(&mut self, msg: &str) {
        if !self.can_update() { return; }
        self.force_update(msg);
    }
}
